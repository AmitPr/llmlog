mod cli;
mod diff;
mod ollama;
mod prompt;
mod repo;

use cli::Cli;
use diff::*;
use ollama::*;
use prompt::*;

use color_eyre::{
    eyre::{eyre, Context, Result},
    Section,
};
use git2::Repository;
use repo::extract_github_info;
use serde::Serialize;
use std::{env, io::Write};

#[derive(Debug, Serialize)]
struct CreateChangelogRequest {
    organization: String,
    name: String,
    title: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    let cwd = env::current_dir().context("Fetching CWD")?;
    let repo = Repository::discover(cwd)
        .wrap_err("Discovering git repository")
        .suggestion("Are you sure you are in a git repository?")?;
    let (org, name) = extract_github_info(&repo)?;

    let revspec = repo
        .revparse(&args.commits)
        .wrap_err("Parsing commit range")?;

    let from = revspec
        .from()
        .ok_or_else(|| eyre!("Invalid 'from' specification"))?
        .id();
    let to = revspec
        .to()
        .ok_or_else(|| eyre!("Invalid 'to' specification"))?
        .id();

    let mut revwalk = repo.revwalk()?;
    revwalk.push(to)?;
    revwalk.hide(from)?;

    let ollama = OllamaClient::new(args.ollama_endpoint.clone(), args.model.clone());
    let client = reqwest::Client::new();
    let mut summaries = Vec::new();

    println!("🔍 Analyzing commits...");

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        let commit_info = get_commit_diff(&repo, &commit).await?;

        print!("Processing commit {}... ", &commit_info.id[..8]);
        std::io::stdout().flush()?;
        let summary = summarize_commit(&ollama, &commit_info).await?;
        println!("✓");

        summaries.push(summary);
    }

    print!("📝 Drafting Changelog...");
    std::io::stdout().flush()?;
    let changelog = create_changelog(&ollama, &name, summaries).await?;
    

    println!("📤 Sending to preview server...");

    let request = CreateChangelogRequest {
        organization: org.clone(),
        name: name.clone(),
        title: changelog.title,
        content: changelog.content,
    };
    let endpoint = format!("{}/api/create", args.api_endpoint);
    let response = client.post(&endpoint).json(&request).send().await?;

    let status = response.status();
    let response_data: serde_json::Value = response.json().await?;
    let id = response_data
        .as_object()
        .and_then(|obj| obj.get("changelog"))
        .and_then(|cl| cl.get("id"))
        .and_then(|id| id.as_number())
        .ok_or_else(|| eyre!("Invalid response"))?;

    if status.is_success() {
        let endpoint = format!("{}/{}/{}/preview/{}", args.api_endpoint, org, name, id);
        println!("🌐 Opening preview at {endpoint}");
        open::that(endpoint)?;
    } else {
        return Err(eyre!(
            "Failed to send to preview server: {}",
            response_data.to_string()
        ));
    }

    Ok(())
}

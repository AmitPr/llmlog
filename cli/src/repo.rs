use color_eyre::{
    eyre::{eyre, Result, WrapErr},
    Section,
};
use git2::Repository;
use url::Url;

// Add this helper function to extract GitHub info
pub fn extract_github_info(repo: &Repository) -> Result<(String, String)> {
    let remote = repo
        .find_remote("origin")
        .wrap_err("Failed to find 'origin' remote")?;

    let url = remote
        .url()
        .ok_or_else(|| eyre!("No URL found for origin remote"))?;

    // Handle different GitHub URL formats
    // https://github.com/org/repo.git
    // git@github.com:org/repo.git
    if let Ok(parsed) = Url::parse(url) {
        let path_segments: Vec<&str> = parsed.path().trim_matches('/').split('/').collect();
        if path_segments.len() >= 2 {
            let org = path_segments[0].to_string();
            let name = path_segments[1].trim_end_matches(".git").to_string();
            return Ok((org, name));
        }
    } else if url.starts_with("git@github.com:") {
        let path = url.trim_start_matches("git@github.com:");
        let segments: Vec<&str> = path.split('/').collect();
        if segments.len() >= 2 {
            let org = segments[0].to_string();
            let name = segments[1].trim_end_matches(".git").to_string();
            return Ok((org, name));
        }
    }

    Err(eyre!(
        "Could not parse GitHub organization and repository from remote URL"
    ))
    .suggestion("Make sure your repository has a valid GitHub remote URL")
}

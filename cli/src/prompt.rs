use color_eyre::eyre::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ollama_schema, CommitInfo, OllamaClient, OllamaMessage, OllamaRequest};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Feature,
    Fix,
    Refactor,
    Breaking,
    Other,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CommitSummary {
    pub commit_id: String,
    #[serde(rename = "type")]
    pub change_type: ChangeType,
    pub summary: String,
    pub changes: Vec<String>,
}

pub async fn summarize_commit(
    client: &OllamaClient,
    commit_info: &CommitInfo,
) -> Result<CommitSummary> {
    let schema = ollama_schema::<CommitSummary>();

    let prompt = format!(
        r#"
Commit Message: {}
```diff
{}
```
Given the git commit and its changes, please analyze and summarize it into a changelog entry.
- Remember to mention the files that were changed, and what was changed
- The list of changes should be quite detailed. Try to include at least 3 changes per commit, scaling up if there are more changes, or down if there are fewer
- Explain the 'why' behind changes as much as possible.
- If unsure about the consequence of a change, do NOT make something up. Instead, just describe the change as best as you can
- Tone: Emojis are ok. Be expressive. Feel free to be profane, but don't be offensive
- You need to use categorize the change (fix, feat, refactor, style, test, docs, chore, nit, etc)
You must follow the JSON schema:
```json
{}
```
"#,
        commit_info.message,
        commit_info.diff,
        serde_json::to_string_pretty(&schema)?
    );

    let request = OllamaRequest {
        messages: vec![OllamaMessage::user(prompt)],
        stream: false,
        format: schema,
    };
    let summary: CommitSummary = client.execute(request).await?;
    Ok(summary)
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Changelog {
    pub title: String,
    pub content: String,
}

pub async fn create_changelog(
    client: &OllamaClient,
    repo: &str,
    summaries: Vec<CommitSummary>,
) -> Result<Changelog> {
    let schema = ollama_schema::<Changelog>();

    let prompt = format!(
        r#"Project name: {}
Commit Summaries:
```json
{}
```
Given the list of commit summaries, please create a changelog entry.
- The title should be a concise summary of the changes
- The content should be a markdown-formatted post.
- The content post should be a summary of the major changes, targeted at a non-technical audience
- The content will be posted on the company blog. Keep your tone playful and expressive!
- Don't over-hype the changes, but don't undersell them either. The post shouldn't be a "fluff piece"
You must follow the JSON schema:
```json
{}
```
"#,
        repo,
        serde_json::to_string_pretty(&summaries)?,
        serde_json::to_string_pretty(&schema)?
    );

    let request = OllamaRequest {
        messages: vec![OllamaMessage::user(prompt)],
        stream: false,
        format: schema,
    };

    let changelog: Changelog = client.execute(request).await?;
    Ok(changelog)
}

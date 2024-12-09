use color_eyre::eyre::{eyre, Result};
use git2::{Commit, Repository};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;

const DEFAULT_IGNORE_PATTERNS: &[&str] = &[
    "package-lock.json",
    "yarn.lock",
    "Cargo.lock",
    "pnpm-lock.yaml",
    "composer.lock",
    "dist/*",
    "build/*",
    "target/*",
    "*.gen.*",
    "*.generated.*",
    "*.min.*",
    "*_pb.rs",
    "*.pb.go",
    "*.pb.js",
    "*.csv",
    "*.json",
    "*.sql",
    "*.dump",
    "*.png",
    "*.jpg",
    "*.jpeg",
    "*.gif",
    "*.ico",
    "*.pdf",
];

const MAX_FILE_SIZE: u64 = 1024 * 1024; // 1MB

#[derive(Debug)]
pub struct DiffFilter {
    gitignore: Gitignore,
}

impl DiffFilter {
    pub fn new(repo_path: &Path) -> Result<Self> {
        let mut builder = GitignoreBuilder::new(repo_path);

        for pattern in DEFAULT_IGNORE_PATTERNS {
            builder.add_line(None, pattern)?;
        }

        let gitignore_path = repo_path.join(".gitignore");
        if gitignore_path.exists() {
            builder.add(gitignore_path).map_or(Ok(()), Err)?;
        }

        let llmlogignore_path = repo_path.join(".llmlogignore");
        if llmlogignore_path.exists() {
            builder.add(llmlogignore_path).map_or(Ok(()), Err)?;
        }

        Ok(Self {
            gitignore: builder.build()?,
        })
    }

    fn should_include_file(&self, path: &Path, size: u64) -> bool {
        if size > MAX_FILE_SIZE {
            return false;
        }
        !self.gitignore.matched(path, false).is_ignore()
    }
}

pub async fn get_commit_diff<'a>(
    repo: &'a Repository,
    commit: &'a Commit<'a>,
) -> Result<CommitInfo> {
    let parent = commit.parent(0).ok();
    let tree = commit.tree()?;
    let parent_tree = parent.as_ref().map(|p| p.tree()).transpose()?;

    let repo_path = repo
        .path()
        .parent()
        .ok_or_else(|| eyre!("Cannot find repo root"))?;
    let filter = DiffFilter::new(repo_path)?;

    let mut diff_opts = git2::DiffOptions::new();
    let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut diff_opts))?;

    let mut diff_content = String::new();
    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(new_file) = delta.new_file().path() {
            let size = delta.new_file().size();
            if !filter.should_include_file(new_file, size) {
                return true;
            }
        }

        if let Ok(content) = std::str::from_utf8(line.content()) {
            diff_content.push_str(content);
        }
        true
    })?;

    Ok(CommitInfo {
        id: commit.id().to_string(),
        message: commit.message().unwrap_or("").to_string(),
        diff: diff_content,
    })
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub diff: String,
}

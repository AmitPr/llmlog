use clap::Parser;


// TODO: CLI can disable handling gitignore, llmignore, etc.

/// A tool for generating changelogs from git commit ranges using AI
#[derive(Parser, Debug)]
#[clap(name = "llmlog", about = "Generate changelogs from git commit ranges")]
pub struct Cli {
    /// Git commit range (e.g., "HEAD~5..HEAD", "main~10..main" or a specific commit hash)
    #[clap(value_parser)]
    pub commits: String,

    /// Specify the model to use for generating commit messages
    #[clap(long, short, default_value = "llama3.2:latest")]
    pub model: String,

    /// API endpoint to post results
    #[clap(long, default_value = "http://localhost:5173")]
    pub api_endpoint: String,

    /// Ollama endpoint, to query models
    #[clap(long, default_value = "http://localhost:11434")]
    pub ollama_endpoint: String,

    /// Maximum diff size in KB to include in summarization prompts
    #[clap(long, default_value = "1024")]
    pub max_file_size: u64,

    /// Additional patterns to ignore
    #[clap(long)]
    pub ignore: Vec<String>,
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }
}

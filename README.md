# llmlog

A local LLM-powered changelog generator and publisher, that helps developers maintain clean changelogs for their projects.

## Features

- **Local LLM Summarizers**: Uses [Ollama](https://ollama.ai/) to run models on local git repositories. Keeps data local and secure.
- **Project-Aware**: Uses git diffs, ignore files, and existing commit messages to generate human-readable commit summaries.
- **Drafts and Editing**: LLMs generate drafts of changelogs, which can be edited before publishing.

### TODO/Unimplemented

- User Authentication
- Proper Project management panes
- Automatic git tags/version tracking, associate changelogs with commits and versions in the repository.
- Better prompting methods: Diff selection, repository retrieval, etc.
- Nicer error handling
- UI/UX improvements.

## Running

`llmlog` requires:

- Bun
- Rust/Cargo
- Ollama 0.5+

### Local Setup

```bash
# Clone the repository
git clone https://github.com/AmitPr/llmlog && cd llmlog

# Install CLI
cd cli
cargo build --release
# If you want to install the CLI globally, run:
cargo install --path .

# Install Web Interface
cd ../web
bun install && bun setup
```

### Run

```bash
# Run development-mode web interface
cd web && bun dev
```

## Usage

### Generate a Changelog

```bash
# Generate changelog for last 5 commits
llmlog HEAD~5..HEAD # (use /path/to/llmlog if not installed globally)

# Use a specific model (llama3.2:latest, by default)
llmlog -m llama2:latest HEAD~5..HEAD
```

Output of `llmlog --help`:

```bash
Generate changelogs from git commit ranges

Usage: llmlog [OPTIONS] <COMMITS>

Arguments:
  <COMMITS>  Git commit range (e.g., "HEAD~5..HEAD", "main~10..main" or a specific commit hash)

Options:
  -m, --model <MODEL>
          Specify the model to use for generating commit messages [default: llama3.2:latest]
      --api-endpoint <API_ENDPOINT>
          API endpoint to post results [default: http://localhost:5173]
      --ollama-endpoint <OLLAMA_ENDPOINT>
          Ollama endpoint, to query models [default: http://localhost:11434]
      --max-file-size <MAX_FILE_SIZE>
          Maximum diff size in KB to include in summarization prompts [default: 1024]
      --ignore <IGNORE>
          Additional patterns to ignore
  -h, --help
          Print help
```

### Web Interface

The web interface is available at `http://localhost:5173` by default. You can:

- View all changelogs by project
- Edit generated changelogs, before publishing.

## Architecture

### CLI Component (`/cli`)

- Written in Rust, for writing easy and ergonomic CLI tools.
- Uses Ollama for local LLM inference
- Filters out irrelevant files (binaries, lock files, etc.)
- Creates Summaries of each commit in the range.
- Feeds commit summaries into a full changelog generator, which is uploaded to the web interface for editing.

### Web Interface (`/web`)

- Built with SvelteKit, TailwindCSS, Drizzle (ORM, sqlite backing).
- Very simple REST API, with endpoints for creating, updating, and publishing changelogs.
- WYSIWYG editor via TipTap, markdown data stored in the database.
- SSRs sanitized markdown, prevents FOUC/waterfall issues.

## Database Schema

```sql
CREATE TABLE `projects` (
        `id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
        `organization` text NOT NULL,
        `name` text NOT NULL,
        `createdAt` text DEFAULT CURRENT_TIMESTAMP NOT NULL,
        `updatedAt` text DEFAULT CURRENT_TIMESTAMP NOT NULL
)
CREATE UNIQUE INDEX `projectUniqueIdx` ON `projects` (lower("organization"),lower("name"))

CREATE TABLE `changelogs` (
        `id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
        `projectId` integer NOT NULL,
        `title` text NOT NULL,
        `content` text NOT NULL,
        `isDraft` integer DEFAULT true NOT NULL,
        `version` text,
        `createdAt` text DEFAULT CURRENT_TIMESTAMP NOT NULL,
        `updatedAt` text DEFAULT CURRENT_TIMESTAMP NOT NULL,
        FOREIGN KEY (`projectId`) REFERENCES `projects`(`id`) ON UPDATE no action ON DELETE cascade
)

```

## License

Licensed under the MIT License.

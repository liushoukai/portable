# Git Commit Generator

This tool leverages an AI model to generate conventional `git commit` commands based on your staged changes. It streamlines the commit process by providing a ready-to-use commit command, including the message.

## Features

- Analyzes staged Git changes (`git diff --cached`).
- Generates a concise and conventional commit message using an AI API.
- Outputs the complete `git commit -m "..."` command, ready for execution.

## Prerequisites

Before using this tool, ensure you have the following installed:

-   **Rust**: For building the application. You can install it via `rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
-   **Git**: The tool interacts with your Git repository.
-   **An AI API Key, URL, and Model**: The tool requires access to an AI model (e.g., Google Gemini, OpenAI GPT). You'll need an API key, the endpoint URL for the API, and the specific model identifier you wish to use.

## Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-repo/git_commit_generator.git
    cd git_commit_generator
    ```

2.  **Create a `.env` file:**
    In the root of the project, create a file named `.env` and add your AI API credentials and configuration. Replace the placeholder values with your actual information.

    ```dotenv
    AI_API_KEY="YOUR_GEMINI_API_KEY"
    AI_API_URL="https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent"
    AI_MODEL="gemini-pro"
    ```
    *   **`AI_API_KEY`**: Your API key for the AI service.
    *   **`AI_API_URL`**: The API endpoint URL. (Example shown for Google Gemini)
    *   **`AI_MODEL`**: The specific model name to use. (Example shown for Google Gemini)

3.  **Build the project:**
    Navigate to the project root and build the application.

    ```bash
    cargo build --release
    # Or for a debug build (less optimized, but faster to compile):
    # cargo build
    ```
    This will create an executable named `git_commit_generator` in the `target/release/` (or `target/debug/`) directory.

## Usage

1.  **Stage your changes:**
    Before running the tool, ensure you have staged the changes you want to commit using `git add`.

    ```bash
    git add .
    # or git add src/main.rs
    ```

2.  **Run the tool:**
    Execute the compiled tool from the project root.

    ```bash
    ./target/release/git_commit_generator
    # Or for the debug version:
    # ./target/debug/git_commit_generator
    ```

3.  **Commit your changes:**
    The tool will output a complete `git commit -m "..."` command. Copy this command and paste it into your terminal to perform the commit.

    ```bash
    # Example output from the tool:
    # git commit -m "feat: Add AI-powered commit message generation"

    # Paste and execute the command:
    git commit -m "feat: Add AI-powered commit message generation"
    ```

## Example Workflow

```bash
# Make some changes to your code
echo "println!(\"Hello, Rust!\");" >> src/main.rs

# Stage the changes
git add src/main.rs

# Run the commit message generator
./target/release/git_commit_generator

# Output (example):
# git commit -m "feat: Add AI-powered commit message generation"

# Execute the commit command
git commit -m "feat: Add AI-powered commit message generation"

# Verify the commit
git log -1 --oneline

cargo build --bin git_commit_generator
```

```shell
# 在 cargo run 命令中，需要使用 -- 来分隔 cargo 的参数和程序的参数
cargo run --package git_commit_generator --bin git_commit_generator -- --help

```

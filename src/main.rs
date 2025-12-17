use std::process::Command;
use std::env;
use clap::Parser;

// 导入模型模块
mod model;
use model::{Cli, ApiRequestBody, ApiResponse, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // 1. Load environment variables from .env file
    dotenvy::dotenv().ok();

    // 2. Check if the current directory is a Git repository
    let git_check_output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()?;

    if !git_check_output.status.success() {
        eprintln!("Error: Not a git repository.");
        return Ok(());
    }

    // 3. Get staged code changes
    let diff_output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()?;

    let diff = String::from_utf8(diff_output.stdout)?;

    if diff.trim().is_empty() {
        println!("No staged changes found. Nothing to commit.");
        return Ok(());
    }

    // 4. Read API configuration from environment variables
    let api_key = env::var("AI_API_KEY").expect("AI_API_KEY must be set");
    let api_url = env::var("AI_API_URL").expect("AI_API_URL must be set");
    let model = env::var("AI_MODEL").expect("AI_MODEL must be set");

    // 5. Construct the prompt for the AI
    let prompt = format!(
        "Please generate a conventional git commit message for the following code changes. The message should follow the standard format: a concise title (subject) line, followed by a blank line, and then a more detailed explanatory body. The body MUST NOT exceed 50 characters. Provide only the commit message text, without any introductory phrases.\n\n        Diff:\n```diff\n{}\n```",
        diff
    );

    // 6. Build the request body
    let request_body = ApiRequestBody {
        model,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    // 7. Send HTTP POST request to the AI API
    let client = reqwest::Client::new();
    println!("Generating commit message...");

    let response = client.post(&api_url)
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?;

    // 8. Parse the response and either print or execute the command
    // 处理 API 调用失败的情况
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("Error: Failed to call API. Status: {}", status);
        eprintln!("Response: {}", error_text);
        return Ok(());
    }

    // 解析 API 响应
    let api_response = response.json::<ApiResponse>().await?;
    let Some(choice) = api_response.choices.get(0) else {
        eprintln!("Error: No commit message was generated.");
        return Ok(());
    };

    let commit_message = choice.message.content.trim();

    // 如果不是自动提交模式，直接打印命令并返回
    if !cli.auto {
        println!("\nGenerated command:");
        println!("git commit -m \"{}\"", commit_message);
        return Ok(());
    }

    // 自动提交模式
    println!("--auto flag detected. Automatically executing commit...");
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()?;

    // 处理 git commit 失败的情况
    if !output.status.success() {
        eprintln!("\nError: Git commit failed.");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }

    // 提交成功
    println!("\nCommit successful!");
    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}

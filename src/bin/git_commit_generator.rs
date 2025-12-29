use std::process::Command;
use std::env;
use std::time::Duration;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

// 从共享库导入模型模块
use portable::model::{Cli, ApiRequestBody, ApiResponse, Message};

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

    // 6. 创建进度显示
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .expect("Failed to set progress style")
    );
    spinner.set_message(format!("正在调用 AI API (模型: {})，请稍候...", model));
    spinner.enable_steady_tick(Duration::from_millis(80));

    // 7. Build the request body
    let request_body = ApiRequestBody {
        model,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let client = reqwest::Client::new();

    let response = client.post(&api_url)
        .bearer_auth(api_key)
        .json(&request_body)
        .timeout(Duration::from_millis(cli.timeout))
        .send()
        .await?;

    // 8. Parse the response and either print or execute the command
    // 处理 API 调用失败的情况
    if !response.status().is_success() {
        spinner.finish_with_message("❌ API 调用失败");
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("Error: Failed to call API. Status: {}", status);
        eprintln!("Response: {}", error_text);
        return Ok(());
    }

    // 解析 API 响应
    let api_response = response.json::<ApiResponse>().await?;
    let Some(choice) = api_response.choices.get(0) else {
        spinner.finish_with_message("❌ 未生成 commit message");
        eprintln!("Error: No commit message was generated.");
        return Ok(());
    };

    let commit_message = choice.message.content.trim();

    // 成功生成，先清除 spinner，然后在新行显示成功消息
    spinner.finish_and_clear();
    println!("✅ Commit message 生成成功！");

    // 如果不是自动提交模式，直接打印命令并返回
    if !cli.auto {
        println!("\n📝 Generated Command:");
        println!("git commit -m \"{}\"", commit_message);
        return Ok(());
    }

    // 自动提交模式
    println!("\n🚀 Auto mode: Executing commit...");
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

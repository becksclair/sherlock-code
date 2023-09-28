use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, CreateCompletionRequestArgs, Role,
    },
    Client,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, process::Command};

#[derive(Serialize)]
struct DescribePrContext {
    title: String,
    commit_messages: String,
}

#[derive(Serialize)]
struct GithubReviewRequestDiffPrompt {
    diff: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrAuthor {
    email: String,
    id: String,
    login: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrCommit {
    authored_date: String,
    authors: Vec<PrAuthor>,
    committed_date: String,
    message_body: String,
    message_headline: String,
    oid: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrInfo {
    commits: Vec<PrCommit>,
    title: String,
}

pub async fn fetch_pr_info(pr_url: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(pr_url)
        .arg("--json")
        .arg("title,commits")
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let pr_json = String::from_utf8_lossy(&output.stdout);
    Ok(pr_json.to_string())
}

pub async fn fetch_pr_diff(pr_url: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("diff")
        .arg(pr_url)
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    let diff = String::from_utf8_lossy(&output.stdout);
    Ok(diff.to_string())
}

fn split_diff_into_files(content: &str) -> Vec<String> {
    let mut file_diffs = Vec::new();
    let mut current_diff = String::new();
    for line in content.lines() {
        if line.starts_with("diff --git") {
            if !current_diff.is_empty() {
                file_diffs.push(current_diff.clone());
                current_diff.clear();
            }
        }
        current_diff.push_str(line);
        current_diff.push('\n');
    }
    if !current_diff.is_empty() {
        file_diffs.push(current_diff);
    }
    file_diffs
}

pub async fn generate_diff_messages(
    pr_url: &str,
) -> Result<Vec<ChatCompletionRequestMessage>, Box<dyn Error>> {
    let diff_prompt = include_str!("../data/diff_prompt.md");
    let diff = fetch_pr_diff(pr_url).await?;

    let diff_chuncks = split_diff_into_files(&diff);
    let mut messages: Vec<ChatCompletionRequestMessage> = Vec::new();

    diff_chuncks.iter().for_each(|chunk| {
        let message = ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(diff_prompt.replace("{diff}", chunk))
            .build()
            .unwrap();
        messages.push(message);
    });
    Ok(messages)
}

// Fetch the pull request info from github
pub async fn review_pr(pr_url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let review_system_prompt = include_str!("../data/review_system_prompt.md");
    let review_prompt = include_str!("../data/review_prompt.md");
    let review_prompt_finish = include_str!("../data/review_prompt_finish.md");

    let diff_messages = generate_diff_messages(pr_url).await?;
    let pr_info_json = fetch_pr_info(pr_url).await?;
    let pr_info: PrInfo =
        serde_json::from_str(&pr_info_json).expect("Unable to parse PR info");

    let rendered_review_prompt = review_prompt
        .replace("{title}", pr_info.title.as_str())
        .replace(
            "{commit_messages}",
            pr_info
                .commits
                .iter()
                .map(|c| c.message_headline.clone())
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        );

    println!("rendered_review_prompt: {}", rendered_review_prompt);

    // Attempt to query the AI directly
    let mut messages: Vec<ChatCompletionRequestMessage> = Vec::new();
    messages.push(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::System)
            .content(review_system_prompt)
            .build()?,
    );
    messages.push(diff_messages.first().unwrap().clone());
    messages.push(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(&rendered_review_prompt)
            .build()?,
    );
    messages.extend(diff_messages);
    messages.push(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(review_prompt_finish)
            .build()?,
    );

    messages
        .iter()
        .for_each(|m| println!("message: {}", m.content.as_ref().unwrap()));

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(1512u16)
        .model("gpt-3.5-turbo-16k")
        .temperature(0.0)
        .messages(messages.as_slice())
        .build()?;

    println!("\nGenerating AI review...\n");
    // println!("\nRequest:\n{:?}\n", messages.as_slice());
    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        if choice.message.role == Role::System {
            // Discard the system message
            continue;
        }
        let review_response = choice.message.content.unwrap();
        println!("Review finished.\n");
        // println!("{}\n", review_response);
        return Ok(review_response);
    }
    Err("No response from AI".into())
}

pub async fn describe_pr(
    pr_url: &str,
) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
    let client = Client::new();
    let describe_pr_prompt = include_str!("../data/describe_pr_prompt.md");
    let pr_info_json = fetch_pr_info(pr_url).await?;
    let pr_info: PrInfo =
        serde_json::from_str(&pr_info_json).expect("Unable to parse PR info");

    let rendered_review_prompt = describe_pr_prompt
        .replace("{prompt}", pr_info.title.as_str())
        .replace(
            "{commit_messages}",
            pr_info
                .commits
                .iter()
                .map(|c| c.message_headline.clone())
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        );

    let request = CreateCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo-instruct")
        .temperature(0.0)
        .prompt(rendered_review_prompt.clone())
        .build()?;

    println!("\nGenerating AI review...\n");

    let response = client.completions().create(request).await?;
    println!("Response:\n");

    let response = match response.choices.first() {
        Some(it) => it,
        None => return Err("No response from AI".into()),
    }
    .text
    .clone();

    println!("{}\n", response);
    Ok(response.to_string())
}

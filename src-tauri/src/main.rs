// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use copypasta::{ClipboardContext, ClipboardProvider};

pub mod reviewer;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
async fn review_pr(pr_url: String) -> String {
    let fut = tauri::async_runtime::spawn(async move {
        match reviewer::review_pr(pr_url.as_str()).await {
            Ok(pr_info) => return pr_info,
            Err(e) => format!("Error: {}", e),
        }
    });
    match fut.await {
        Ok(pr_info) => pr_info.to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn describe_pr(pr_url: String) -> String {
    let fut = tauri::async_runtime::spawn(async move {
        match reviewer::describe_pr(pr_url.as_str()).await {
            Ok(pr_info) => return pr_info,
            Err(e) => format!("Error: {}", e),
        }
    });
    match fut.await {
        Ok(pr_info) => pr_info.to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

#[tauri::command]
fn copy_to_clipboard(s: &str) {
    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard.set_contents(s.to_string()).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            review_pr,
            describe_pr,
            copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

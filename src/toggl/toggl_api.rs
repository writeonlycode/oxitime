use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
struct TimeEntry {
    description: String,
    workspace_id: i64,
    duration: i64,
    start: String,
    created_with: String,
}

pub fn create_time_entry(
    api_token: &str,
    workspace_id: &str,
    description: &str,
    duration: Duration,
) -> Result<()> {
    let client = Client::new();

    let ws_id: i64 = workspace_id
        .parse()
        .context("workspace_id must be a numeric value")?;

    let now = chrono::Utc::now();
    let start = now - chrono::Duration::from_std(duration).unwrap();
    let start_rfc3339 = start.to_rfc3339();

    let entry = TimeEntry {
        description: description.to_string(),
        workspace_id: ws_id,
        duration: duration.as_secs() as i64,
        start: start_rfc3339,
        created_with: "oxitime".to_string(),
    };

    let url = format!(
        "https://api.track.toggl.com/api/v9/workspaces/{}/time_entries",
        workspace_id
    );

    eprintln!("[oxitime::debug] POST {url}");
    eprintln!("[oxitime::debug] body: {}", serde_json::to_string(&entry).unwrap());

    let response = client
        .post(&url)
        .basic_auth(api_token, Some("api_token"))
        .json(&entry)
        .send()
        .context("Failed to send request to Toggl API")?;

    let status = response.status();
    let body = response.text().unwrap_or_default();
    eprintln!("[oxitime::debug] response status: {status}");
    eprintln!("[oxitime::debug] response body: {body}");

    if !status.is_success() {
        anyhow::bail!("Toggl API returned status {status}: {body}");
    }

    Ok(())
}

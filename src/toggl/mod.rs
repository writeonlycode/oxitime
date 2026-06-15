mod toggl_api;

use anyhow::Result;
use std::time::Duration;

pub struct TogglSyncer {
    api_token: String,
    workspace_id: String,
    description: String,
}

impl TogglSyncer {
    pub fn new(api_token: String, workspace_id: String, description: String) -> Self {
        TogglSyncer {
            api_token,
            workspace_id,
            description,
        }
    }

    pub fn sync_pomodoro(&self, duration: Duration) -> Result<()> {
        eprintln!("[oxitime::debug] TogglSyncer::sync_pomodoro(duration={}s)", duration.as_secs());
        let result = toggl_api::create_time_entry(&self.api_token, &self.workspace_id, &self.description, duration);
        eprintln!("[oxitime::debug] TogglSyncer::sync_pomodoro result: {result:?}");
        result
    }
}

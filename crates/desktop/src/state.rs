//! Application state management for the desktop app.
//!
//! This module provides global state management similar to React Context,
//! using GPUI's Model system for reactive updates.

use crate::api::{ApiClient, Event, MergeQueueEntry, Snapshot, Task, TaskState};
use gpui::{AppContext, Context, Model};
use std::time::Duration;

const MAX_EVENTS: usize = 200;
const POLL_INTERVAL: Duration = Duration::from_secs(5);

/// Connection status to the backend server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
}

/// Global application state.
#[derive(Debug)]
pub struct AppState {
    /// Current snapshot from the server.
    pub snapshot: Option<Snapshot>,
    /// Recent events buffer.
    pub events: Vec<Event>,
    /// Connection status.
    pub connection_status: ConnectionStatus,
    /// Last error message, if any.
    pub error: Option<String>,
    /// Currently selected project ID (None = all projects).
    pub selected_project: Option<String>,
    /// API client instance.
    api: ApiClient,
}

impl AppState {
    /// Create a new app state with default values.
    pub fn new(api_url: Option<String>) -> Self {
        let api = match api_url {
            Some(url) => ApiClient::new(url),
            None => ApiClient::default(),
        };

        Self {
            snapshot: None,
            events: Vec::new(),
            connection_status: ConnectionStatus::Connecting,
            error: None,
            selected_project: None,
            api,
        }
    }

    /// Get tasks filtered by the selected project.
    pub fn filtered_tasks(&self) -> Vec<&Task> {
        let tasks = match &self.snapshot {
            Some(snap) => &snap.tasks,
            None => return Vec::new(),
        };

        match &self.selected_project {
            Some(project_id) => tasks.iter().filter(|t| &t.project == project_id).collect(),
            None => tasks.iter().collect(),
        }
    }

    /// Get merge queue entries filtered by the selected project.
    pub fn filtered_merge_queue(&self) -> Vec<&MergeQueueEntry> {
        let (entries, tasks) = match &self.snapshot {
            Some(snap) => (&snap.merge_queue, &snap.tasks),
            None => return Vec::new(),
        };

        match &self.selected_project {
            Some(project_id) => {
                // Get task IDs belonging to this project
                let project_task_ids: std::collections::HashSet<&str> = tasks
                    .iter()
                    .filter(|t| &t.project == project_id)
                    .map(|t| t.id.as_str())
                    .collect();

                entries
                    .iter()
                    .filter(|e| project_task_ids.contains(e.task_id.as_str()))
                    .collect()
            }
            None => entries.iter().collect(),
        }
    }

    /// Get active tasks (running, question, testing, awaiting_merge).
    pub fn active_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<_> = self
            .filtered_tasks()
            .into_iter()
            .filter(|t| t.state.is_active())
            .collect();

        // Sort by updated_at descending (most recent first)
        tasks.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        tasks.truncate(10);
        tasks
    }

    /// Count tasks in a specific state.
    pub fn count_by_state(&self, state: TaskState) -> usize {
        self.filtered_tasks()
            .iter()
            .filter(|t| t.state == state)
            .count()
    }

    /// Get recent events (max 15).
    pub fn recent_events(&self) -> Vec<&Event> {
        self.events.iter().take(15).collect()
    }

    /// Add a new event to the buffer.
    pub fn push_event(&mut self, event: Event) {
        self.events.insert(0, event);
        if self.events.len() > MAX_EVENTS {
            self.events.truncate(MAX_EVENTS);
        }
    }

    /// Set the selected project filter.
    pub fn set_selected_project(&mut self, project_id: Option<String>) {
        self.selected_project = project_id;
    }

    /// Update the snapshot.
    pub fn update_snapshot(&mut self, snapshot: Snapshot) {
        self.snapshot = Some(snapshot);
        self.connection_status = ConnectionStatus::Connected;
        self.error = None;
    }

    /// Set an error state.
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.connection_status = ConnectionStatus::Disconnected;
    }

    /// Get a reference to the API client.
    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}

/// Create a new AppState model and start background polling.
pub fn create_app_state(cx: &mut AppContext, api_url: Option<String>) -> Model<AppState> {
    let state = cx.new_model(|_| AppState::new(api_url));

    // Start the polling timer
    let state_handle = state.clone();
    cx.spawn({
        let state = state_handle.clone();
        |mut cx| async move {
            loop {
                // Fetch snapshot
                let api = cx
                    .update_model(&state, |state, _| state.api().clone())
                    .ok();

                if let Some(api) = api {
                    match api.fetch_snapshot().await {
                        Ok(snapshot) => {
                            let _ = cx.update_model(&state, |state, cx| {
                                state.update_snapshot(snapshot);
                                cx.notify();
                            });
                        }
                        Err(e) => {
                            let _ = cx.update_model(&state, |state, cx| {
                                state.set_error(e.to_string());
                                cx.notify();
                            });
                        }
                    }
                }

                // Wait for next poll
                cx.background_executor()
                    .timer(POLL_INTERVAL)
                    .await;
            }
        }
    })
    .detach();

    state
}

//! HTTP API client for communicating with the Tasks server.

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// The base URL for the API (configurable).
pub const DEFAULT_API_URL: &str = "http://localhost:4800";

/// Operating mode for the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Stop,
    Pause,
    Play,
}

/// Task state enum matching the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskState {
    Waiting,
    Blocked,
    Running,
    Question,
    Testing,
    AwaitingMerge,
    Conflict,
    Completed,
    Failed,
    Cancelled,
}

impl TaskState {
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            TaskState::Running | TaskState::Question | TaskState::Testing | TaskState::AwaitingMerge
        )
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TaskState::Waiting => "waiting",
            TaskState::Blocked => "blocked",
            TaskState::Running => "running",
            TaskState::Question => "question",
            TaskState::Testing => "testing",
            TaskState::AwaitingMerge => "awaiting merge",
            TaskState::Conflict => "conflict",
            TaskState::Completed => "completed",
            TaskState::Failed => "failed",
            TaskState::Cancelled => "cancelled",
        }
    }
}

/// Task source (origin).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TaskSource {
    GithubIssue {
        owner: String,
        repo: String,
        number: u64,
    },
    GithubPr {
        owner: String,
        repo: String,
        number: u64,
    },
    Internal,
}

/// A task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub source: TaskSource,
    pub title: String,
    pub description: Option<String>,
    pub state: TaskState,
    pub parent_id: Option<String>,
    pub blocked_by: Vec<String>,
    pub project: String,
    pub labels: Vec<String>,
    pub priority: Option<i32>,
    pub session_id: Option<String>,
    pub workspace_id: Option<String>,
    pub retry_count: u32,
    pub last_failure_at: Option<String>,
    pub source_created_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// A project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub repo: String,
    pub default_branch: String,
    pub config: serde_json::Value,
}

/// Merge queue entry status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeStatus {
    Pending,
    Approved,
    Rejected,
    Merged,
    Conflict,
}

/// A merge queue entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeQueueEntry {
    pub id: String,
    pub task_id: String,
    pub pr_url: String,
    pub status: MergeStatus,
    pub queued_at: String,
}

/// Actor types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Actor {
    Human,
    Orchestrator,
    Scheduler,
    Agent,
    System,
}

impl Actor {
    pub fn display_name(&self) -> &'static str {
        match self {
            Actor::Human => "human",
            Actor::Orchestrator => "orchestrator",
            Actor::Scheduler => "scheduler",
            Actor::Agent => "agent",
            Actor::System => "system",
        }
    }
}

/// An event from the event log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub task: String,
    pub actor: Actor,
    pub ts: String,
    pub data: serde_json::Value,
}

/// Slot utilization stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotUtilization {
    pub active: u32,
    pub max: u32,
}

/// System snapshot - full state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub mode: Mode,
    pub projects: Vec<Project>,
    pub tasks: Vec<Task>,
    pub merge_queue: Vec<MergeQueueEntry>,
    pub slot_utilization: SlotUtilization,
    pub human_present: bool,
}

/// API client for the Tasks server.
#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    /// Fetch the full system snapshot.
    pub async fn fetch_snapshot(&self) -> Result<Snapshot, reqwest::Error> {
        let url = format!("{}/api/snapshot", self.base_url);
        self.client.get(&url).send().await?.json().await
    }

    /// Fetch a single task by ID.
    pub async fn fetch_task(&self, id: &str) -> Result<Task, reqwest::Error> {
        let url = format!("{}/api/tasks/{}", self.base_url, id);
        self.client.get(&url).send().await?.json().await
    }

    /// Fetch events for a task.
    pub async fn fetch_task_events(&self, id: &str) -> Result<Vec<Event>, reqwest::Error> {
        let url = format!("{}/api/tasks/{}/events", self.base_url, id);
        self.client.get(&url).send().await?.json().await
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new(DEFAULT_API_URL)
    }
}

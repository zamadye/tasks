//! Tasks Desktop — GPUI-based desktop application.
//!
//! This crate provides a native desktop UI for the Tasks platform,
//! built with GPUI (Zed's GPU-accelerated UI framework).

pub mod api;
pub mod sse;
pub mod state;

// Re-export commonly used types
pub use api::{
    ApiClient, ApiError, Event, MergeQueueEntry, MergeStatus, Mode, Project, Snapshot,
    SlotUtilization, Task, TaskSource, TaskState, DEFAULT_SERVER_URL,
};
pub use sse::{SseClient, SseClientEvent, SseConnectionState, SseFilters};
pub use state::{AppState, AppStateEvent, ConnectionStatus, create_app_state};

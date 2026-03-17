//! Tasks Desktop — GPUI-based desktop application.
//!
//! This crate provides a native desktop UI for the Tasks platform,
//! built with GPUI (Zed's GPU-accelerated UI framework).
//!
//! # Modules
//!
//! - [`sse`] - Server-Sent Events client for real-time updates
//! - [`theme`] - Theming and styling system

mod sse;
pub mod theme;

pub use sse::{SseClient, SseClientEvent, SseConnectionState, SseFilters};
pub use theme::{colors, radius, spacing, style_helpers, typography, Theme};

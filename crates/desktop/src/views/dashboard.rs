//! Dashboard view - the main landing page of the app.
//!
//! Displays:
//! - Stats cards (active sessions, running tasks, waiting tasks, merge queue)
//! - Active tasks list (running, question, testing, awaiting_merge)
//! - Recent events list

use crate::api::TaskState;
use crate::state::AppState;
use crate::theme::{Colors, Radius, Spacing, TextSize};
use crate::ui::{Badge, Card, StatCard};
use gpui::{div, prelude::*, Model, Render, Styled, View, ViewContext};

/// The Dashboard view component.
pub struct Dashboard {
    state: Model<AppState>,
}

impl Dashboard {
    pub fn new(state: Model<AppState>) -> Self {
        Self { state }
    }
}

impl Render for Dashboard {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state = self.state.read(cx);

        // Check if we have data
        let Some(snapshot) = &state.snapshot else {
            return div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_color(Colors::muted())
                        .text_size(gpui::px(TextSize::sm()))
                        .child("No data yet"),
                )
                .into_any_element();
        };

        // Calculate stats
        let active_sessions = snapshot.slot_utilization.active;
        let max_sessions = snapshot.slot_utilization.max;
        let running_count = state.count_by_state(TaskState::Running);
        let waiting_count = state.count_by_state(TaskState::Waiting);
        let merge_queue_count = state.filtered_merge_queue().len();

        // Get active tasks and recent events
        let active_tasks = state.active_tasks();
        let recent_events = state.recent_events();

        // Check if viewing all projects
        let selected_project = state.selected_project.clone();

        div()
            .size_full()
            .p(Spacing::lg())
            .flex()
            .flex_col()
            .gap(Spacing::xl())
            .bg(Colors::background())
            .child(
                // Stats cards row
                div()
                    .flex()
                    .gap(Spacing::md())
                    .child(
                        StatCard::new("Active Sessions", active_sessions.to_string())
                            .subtitle(max_sessions.to_string()),
                    )
                    .child(StatCard::new("Running Tasks", running_count.to_string()))
                    .child(StatCard::new("Waiting Tasks", waiting_count.to_string()))
                    .child(StatCard::new("Merge Queue", merge_queue_count.to_string())),
            )
            .child(
                // Active Tasks section
                div()
                    .flex()
                    .flex_col()
                    .gap(Spacing::sm())
                    .child(
                        div()
                            .text_size(gpui::px(TextSize::base()))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(Colors::foreground())
                            .child("Active Tasks"),
                    )
                    .child(if active_tasks.is_empty() {
                        div()
                            .text_color(Colors::muted())
                            .text_size(gpui::px(TextSize::sm()))
                            .child("No active tasks right now.")
                            .into_any_element()
                    } else {
                        div()
                            .flex()
                            .flex_col()
                            .gap(Spacing::sm())
                            .children(active_tasks.iter().map(|task| {
                                task_row(task, selected_project.is_none())
                            }))
                            .into_any_element()
                    }),
            )
            .child(
                // Recent Events section
                div()
                    .flex()
                    .flex_col()
                    .gap(Spacing::sm())
                    .child(
                        div()
                            .text_size(gpui::px(TextSize::base()))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(Colors::foreground())
                            .child("Recent Events"),
                    )
                    .child(if recent_events.is_empty() {
                        div()
                            .text_color(Colors::muted())
                            .text_size(gpui::px(TextSize::sm()))
                            .child("No events yet.")
                            .into_any_element()
                    } else {
                        div()
                            .flex()
                            .flex_col()
                            .gap(Spacing::xs())
                            .children(recent_events.iter().map(|event| event_row(event)))
                            .into_any_element()
                    }),
            )
            .into_any_element()
    }
}

/// Render a single task row.
fn task_row(task: &crate::api::Task, show_project: bool) -> impl IntoElement {
    let state_badge = Badge::for_task_state(task.state);
    let title = task.title.clone();
    let project = task.project.clone();
    let updated_at = format_relative_time(&task.updated_at);

    div()
        .rounded(gpui::px(Radius::lg()))
        .border_1()
        .border_color(Colors::border())
        .bg(Colors::card())
        .p(Spacing::sm())
        .hover(|style| style.bg(Colors::card_hover()))
        .cursor_pointer()
        .flex()
        .items_center()
        .gap(Spacing::sm())
        .child(state_badge)
        .child(
            div()
                .flex_1()
                .text_color(Colors::foreground())
                .font_weight(gpui::FontWeight::MEDIUM)
                .overflow_hidden()
                .text_ellipsis()
                .child(title),
        )
        .when(show_project, |el| {
            el.child(
                div()
                    .text_color(Colors::muted())
                    .text_size(gpui::px(TextSize::sm()))
                    .flex_shrink_0()
                    .child(project),
            )
        })
        .child(
            div()
                .text_color(Colors::muted())
                .text_size(gpui::px(TextSize::sm()))
                .flex_shrink_0()
                .child(updated_at),
        )
}

/// Render a single event row.
fn event_row(event: &crate::api::Event) -> impl IntoElement {
    let timestamp = format_relative_time(&event.ts);
    let event_type = event.event_type.clone();
    let actor = event.actor.display_name().to_string();
    let task_id = if event.task.len() > 8 {
        event.task[..8].to_string()
    } else {
        event.task.clone()
    };
    let data_preview = event_data_preview(&event.data);

    div()
        .rounded(gpui::px(Radius::lg()))
        .border_1()
        .border_color(Colors::border())
        .bg(Colors::card())
        .px(Spacing::sm())
        .py(Spacing::xs())
        .flex()
        .items_start()
        .gap(Spacing::sm())
        .text_size(gpui::px(TextSize::sm()))
        .child(
            div()
                .text_color(Colors::muted())
                .flex_shrink_0()
                .pt(gpui::px(2.0))
                .child(timestamp),
        )
        .child(Badge::outline(event_type))
        .child(
            div()
                .text_color(Colors::muted())
                .flex_shrink_0()
                .child(actor),
        )
        .when(!task_id.is_empty(), |el| {
            el.child(
                div()
                    .font_family("monospace")
                    .text_color(Colors::muted())
                    .flex_shrink_0()
                    .child(task_id),
            )
        })
        .child(
            div()
                .text_color(Colors::muted())
                .flex_1()
                .overflow_hidden()
                .text_ellipsis()
                .child(data_preview),
        )
}

/// Format a timestamp as a relative time string.
fn format_relative_time(timestamp: &str) -> String {
    // Parse ISO 8601 timestamp
    let Ok(dt) = chrono::DateTime::parse_from_rfc3339(timestamp) else {
        return timestamp.to_string();
    };

    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(dt.with_timezone(&chrono::Utc));

    if duration.num_seconds() < 60 {
        return "just now".to_string();
    }

    if duration.num_minutes() < 60 {
        let mins = duration.num_minutes();
        return format!("{}m ago", mins);
    }

    if duration.num_hours() < 24 {
        let hours = duration.num_hours();
        return format!("{}h ago", hours);
    }

    let days = duration.num_days();
    format!("{}d ago", days)
}

/// Generate a preview of event data (truncated JSON).
fn event_data_preview(data: &serde_json::Value) -> String {
    let raw = data.to_string();
    if raw.len() > 80 {
        format!("{}...", &raw[..80])
    } else {
        raw
    }
}

//! Tasks Desktop — Main entry point.
//!
//! A GPUI-based native desktop application for the Tasks platform.

use gpui::{
    actions, div, px, App, AppContext as _, Application, Context, Entity, IntoElement,
    ParentElement, Render, Styled, Window, WindowOptions,
};
use std::sync::Arc;
use tasks_desktop::{
    AppState, AppStateEvent, ConnectionStatus, Event, Mode, TaskState, create_app_state,
};
use tracing_subscriber::EnvFilter;

actions!(desktop, [Quit]);

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    Application::new().run(|cx: &mut App| {
        // Register quit action
        cx.on_action(|_: &Quit, cx: &mut App| cx.quit());
        cx.bind_keys([gpui::KeyBinding::new("cmd-q", Quit, None)]);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds {
                    origin: gpui::Point::default(),
                    size: gpui::Size {
                        width: px(900.),
                        height: px(700.),
                    },
                })),
                ..Default::default()
            },
            |_window, cx| {
                let view = cx.new(|cx| TasksApp::new(cx));
                view
            },
        )
        .unwrap();

        cx.activate(true);
    });
}

/// Main application view.
struct TasksApp {
    /// Application state (shared across views).
    app_state: Entity<AppState>,
    /// Cached values for display
    connection_status: ConnectionStatus,
    mode: Option<Mode>,
    task_count: usize,
    running_count: usize,
    waiting_count: usize,
    merge_queue_count: usize,
    event_count: usize,
    last_event: Option<Arc<Event>>,
    selected_project: Option<String>,
}

impl TasksApp {
    fn new(cx: &mut Context<Self>) -> Self {
        // Create the app state
        let app_state = cx.new(|cx| create_app_state(cx));

        // Subscribe to state changes
        cx.subscribe(&app_state, |this: &mut Self, state, event: &AppStateEvent, cx| {
            match event {
                AppStateEvent::SnapshotUpdated => {
                    this.update_from_state(&state, cx);
                }
                AppStateEvent::ConnectionStatusChanged(status) => {
                    this.connection_status = *status;
                    cx.notify();
                }
                AppStateEvent::EventReceived(event) => {
                    this.event_count += 1;
                    this.last_event = Some(event.clone());
                    cx.notify();
                }
                AppStateEvent::SelectedProjectChanged(project) => {
                    this.selected_project = project.clone();
                    this.update_from_state(&state, cx);
                }
                AppStateEvent::Error(_) => {
                    cx.notify();
                }
            }
        })
        .detach();

        // Initialize cached values
        let mut app = Self {
            app_state: app_state.clone(),
            connection_status: ConnectionStatus::Connecting,
            mode: None,
            task_count: 0,
            running_count: 0,
            waiting_count: 0,
            merge_queue_count: 0,
            event_count: 0,
            last_event: None,
            selected_project: None,
        };

        // Initial update from state
        app.update_from_state(&app_state, cx);

        app
    }

    fn update_from_state(&mut self, state: &Entity<AppState>, cx: &mut Context<Self>) {
        state.read(cx).snapshot().map(|snapshot| {
            self.mode = Some(snapshot.mode);
            self.task_count = snapshot.tasks.len();
            self.merge_queue_count = snapshot.merge_queue.len();
        });

        let state_ref = state.read(cx);
        self.running_count = state_ref.running_count();
        self.waiting_count = state_ref.waiting_count();
        self.connection_status = state_ref.connection_status();

        cx.notify();
    }
}

impl Render for TasksApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.app_state.read(cx);

        // Status indicator
        let (status_text, status_color) = match self.connection_status {
            ConnectionStatus::Disconnected => ("Disconnected", gpui::rgb(0xef4444)),
            ConnectionStatus::Connecting => ("Connecting...", gpui::rgb(0xeab308)),
            ConnectionStatus::Connected => ("Connected", gpui::rgb(0x22c55e)),
            ConnectionStatus::Reconnecting => ("Reconnecting...", gpui::rgb(0xeab308)),
            ConnectionStatus::Failed => ("Connection Failed", gpui::rgb(0xef4444)),
        };

        // Mode display
        let mode_text = self
            .mode
            .map(|m| m.display_name())
            .unwrap_or("Unknown");

        let mode_color = match self.mode {
            Some(Mode::Play) => gpui::rgb(0x22c55e),
            Some(Mode::Pause) => gpui::rgb(0xeab308),
            Some(Mode::Stop) => gpui::rgb(0xef4444),
            None => gpui::rgb(0x888888),
        };

        // Last event display
        let last_event_text = self
            .last_event
            .as_ref()
            .map(|e| format!("{}: {}", e.event_type, e.task))
            .unwrap_or_else(|| "No events yet".to_string());

        // Active tasks (limited to 5)
        let active_tasks: Vec<_> = state
            .active_tasks()
            .into_iter()
            .take(5)
            .collect();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui::rgb(0x0a0a0a))
            .text_color(gpui::rgb(0xfafafa))
            .p_4()
            .gap_4()
            // Header
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("Tasks Desktop"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(gpui::rgb(0x71717a))
                                    .child("GPUI-based desktop client"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().w(px(10.)).h(px(10.)).rounded_full().bg(status_color))
                            .child(div().text_sm().child(status_text)),
                    ),
            )
            // Stats row
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(stat_card("Mode", mode_text, Some(mode_color)))
                    .child(stat_card("Tasks", &self.task_count.to_string(), None))
                    .child(stat_card("Running", &self.running_count.to_string(), Some(gpui::rgb(0x3b82f6))))
                    .child(stat_card("Waiting", &self.waiting_count.to_string(), Some(gpui::rgb(0x71717a))))
                    .child(stat_card("Merge Queue", &self.merge_queue_count.to_string(), None)),
            )
            // Active tasks section
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("Active Tasks"),
                    )
                    .child(active_tasks_list(&active_tasks)),
            )
            // Events section
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child(format!("Events ({})", self.event_count)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .bg(gpui::rgb(0x18181b))
                            .rounded_lg()
                            .p_3()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(gpui::rgb(0xa1a1aa))
                                    .child(format!("Last: {}", last_event_text)),
                            ),
                    ),
            )
    }
}

/// Render the active tasks list.
fn active_tasks_list(tasks: &[&tasks_desktop::Task]) -> impl IntoElement {
    let mut container = div()
        .flex()
        .flex_col()
        .gap_1()
        .bg(gpui::rgb(0x18181b))
        .rounded_lg()
        .p_3();

    if tasks.is_empty() {
        container = container.child(
            div()
                .text_sm()
                .text_color(gpui::rgb(0x71717a))
                .child("No active tasks"),
        );
    } else {
        for task in tasks {
            container = container.child(task_row(task));
        }
    }

    container
}

/// Render a stat card.
fn stat_card(label: &str, value: &str, color: Option<gpui::Rgba>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .bg(gpui::rgb(0x18181b))
        .rounded_lg()
        .p_3()
        .min_w(px(100.))
        .child(
            div()
                .text_sm()
                .text_color(gpui::rgb(0x71717a))
                .child(label.to_string()),
        )
        .child(
            div()
                .text_xl()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(color.unwrap_or(gpui::rgb(0xfafafa)))
                .child(value.to_string()),
        )
}

/// Render a task row.
fn task_row(task: &tasks_desktop::Task) -> impl IntoElement {
    let state_color = match task.state {
        TaskState::Running => gpui::rgb(0x3b82f6),    // blue
        TaskState::Question => gpui::rgb(0xeab308),   // yellow
        TaskState::Testing => gpui::rgb(0x8b5cf6),    // purple
        TaskState::AwaitingMerge => gpui::rgb(0x22c55e), // green
        _ => gpui::rgb(0x71717a),                     // gray
    };

    div()
        .flex()
        .items_center()
        .gap_3()
        .py_1()
        .child(
            div()
                .px_2()
                .py(px(2.))
                .rounded(px(4.))
                .bg(state_color)
                .text_xs()
                .font_weight(gpui::FontWeight::MEDIUM)
                .child(task.state.display_name().to_string()),
        )
        .child(
            div()
                .flex_1()
                .text_sm()
                .overflow_hidden()
                .child(task.title.clone()),
        )
        .child(
            div()
                .text_xs()
                .text_color(gpui::rgb(0x71717a))
                .child(task.project.clone()),
        )
}

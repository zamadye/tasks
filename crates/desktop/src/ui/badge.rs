//! Badge component for displaying status indicators.

use crate::api::TaskState;
use crate::theme::{Colors, Radius, Spacing, TextSize};
use gpui::{div, prelude::*, Hsla, IntoElement, Styled};

/// Badge variant determining the visual style.
#[derive(Debug, Clone, Copy, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Destructive,
    /// Custom color background with white text.
    Custom(Hsla),
}

/// A badge component for displaying labels and status indicators.
pub struct Badge {
    label: String,
    variant: BadgeVariant,
}

impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: BadgeVariant::Default,
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Create a badge for a task state with appropriate styling.
    pub fn for_task_state(state: TaskState) -> Self {
        let (label, variant) = match state {
            TaskState::Running => ("running", BadgeVariant::Custom(Colors::running())),
            TaskState::Question => ("question", BadgeVariant::Secondary),
            TaskState::Testing => ("testing", BadgeVariant::Outline),
            TaskState::AwaitingMerge => ("awaiting merge", BadgeVariant::Secondary),
            TaskState::Completed => ("completed", BadgeVariant::Custom(Colors::completed())),
            TaskState::Failed => ("failed", BadgeVariant::Destructive),
            TaskState::Waiting => ("waiting", BadgeVariant::Outline),
            TaskState::Blocked => ("blocked", BadgeVariant::Outline),
            TaskState::Conflict => ("conflict", BadgeVariant::Destructive),
            TaskState::Cancelled => ("cancelled", BadgeVariant::Outline),
        };

        Self {
            label: label.to_string(),
            variant,
        }
    }

    /// Create an outline badge (used for event types).
    pub fn outline(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: BadgeVariant::Outline,
        }
    }
}

impl IntoElement for Badge {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        let (bg_color, text_color, border_color) = match self.variant {
            BadgeVariant::Default => (Colors::foreground(), Colors::background(), Colors::foreground()),
            BadgeVariant::Secondary => (Colors::secondary(), Colors::foreground(), Colors::secondary()),
            BadgeVariant::Outline => (Colors::background(), Colors::foreground(), Colors::outline()),
            BadgeVariant::Destructive => (Colors::failed(), Colors::foreground(), Colors::failed()),
            BadgeVariant::Custom(bg) => (bg, Colors::foreground(), bg),
        };

        div()
            .px(Spacing::sm())
            .py(Spacing::xs() / 2.0)
            .rounded(gpui::px(Radius::sm()))
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .text_size(gpui::px(TextSize::xs()))
            .text_color(text_color)
            .child(self.label)
    }
}

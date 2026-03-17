//! Card component for content containers.

use crate::theme::{Colors, Radius, Spacing};
use gpui::{div, prelude::*, AnyElement, IntoElement, Styled};

/// A card component with optional header and content sections.
pub struct Card {
    header: Option<AnyElement>,
    children: Vec<AnyElement>,
    on_click: Option<Box<dyn Fn(&mut gpui::WindowContext) + 'static>>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            header: None,
            children: Vec::new(),
            on_click: None,
        }
    }

    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: IntoElement,
    {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut gpui::WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Card {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        let mut card = div()
            .rounded(gpui::px(Radius::lg()))
            .border_1()
            .border_color(Colors::border())
            .bg(Colors::card())
            .overflow_hidden();

        if self.on_click.is_some() {
            card = card.hover(|style| style.bg(Colors::card_hover())).cursor_pointer();
        }

        let mut content = div().flex().flex_col();

        if let Some(header) = self.header {
            content = content.child(
                div()
                    .px(Spacing::md())
                    .py(Spacing::sm())
                    .child(header),
            );
        }

        if !self.children.is_empty() {
            content = content.child(
                div()
                    .px(Spacing::md())
                    .pb(Spacing::md())
                    .children(self.children),
            );
        }

        card.child(content)
    }
}

/// A simple stat card with a title and value.
pub struct StatCard {
    title: String,
    value: String,
    subtitle: Option<String>,
}

impl StatCard {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            subtitle: None,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
}

impl IntoElement for StatCard {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        let header = div()
            .text_size(gpui::px(14.0))
            .text_color(Colors::muted())
            .font_weight(gpui::FontWeight::MEDIUM)
            .child(self.title);

        let mut value_row = div()
            .text_size(gpui::px(16.0))
            .font_weight(gpui::FontWeight::BOLD)
            .text_color(Colors::foreground())
            .child(self.value);

        if let Some(subtitle) = self.subtitle {
            value_row = value_row.child(
                div()
                    .text_color(Colors::muted())
                    .font_weight(gpui::FontWeight::NORMAL)
                    .child(format!(" / {}", subtitle)),
            );
        }

        Card::new()
            .header(header)
            .child(value_row)
            .into_element()
    }
}

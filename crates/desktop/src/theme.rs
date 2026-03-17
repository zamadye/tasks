//! Theme definitions for the desktop UI
//!
//! Provides colors, spacing, and other design tokens that match
//! the web frontend's shadcn/ui theme.

use gpui::{hsla, px, Hsla, Pixels};

/// Design tokens for the UI theme
#[derive(Clone, Debug)]
pub struct Theme {
    // Background colors
    pub background: Hsla,
    pub foreground: Hsla,
    pub card: Hsla,
    pub card_foreground: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,

    // Primary colors
    pub primary: Hsla,
    pub primary_foreground: Hsla,

    // Secondary colors
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,

    // Muted colors
    pub muted: Hsla,
    pub muted_foreground: Hsla,

    // Accent colors
    pub accent: Hsla,
    pub accent_foreground: Hsla,

    // Destructive colors
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,

    // Border and input
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla,

    // Spacing
    pub radius_sm: Pixels,
    pub radius_md: Pixels,
    pub radius_lg: Pixels,
    pub radius_xl: Pixels,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    /// Light theme matching shadcn/ui defaults
    pub fn light() -> Self {
        Self {
            background: hsla(0.0, 0.0, 1.0, 1.0),
            foreground: hsla(222.2 / 360.0, 0.84, 0.049, 1.0),
            card: hsla(0.0, 0.0, 1.0, 1.0),
            card_foreground: hsla(222.2 / 360.0, 0.84, 0.049, 1.0),
            popover: hsla(0.0, 0.0, 1.0, 1.0),
            popover_foreground: hsla(222.2 / 360.0, 0.84, 0.049, 1.0),
            primary: hsla(222.2 / 360.0, 0.473, 0.112, 1.0),
            primary_foreground: hsla(210.0 / 360.0, 0.40, 0.98, 1.0),
            secondary: hsla(210.0 / 360.0, 0.40, 0.961, 1.0),
            secondary_foreground: hsla(222.2 / 360.0, 0.473, 0.112, 1.0),
            muted: hsla(210.0 / 360.0, 0.40, 0.961, 1.0),
            muted_foreground: hsla(215.4 / 360.0, 0.163, 0.469, 1.0),
            accent: hsla(210.0 / 360.0, 0.40, 0.961, 1.0),
            accent_foreground: hsla(222.2 / 360.0, 0.473, 0.112, 1.0),
            destructive: hsla(0.0 / 360.0, 0.843, 0.60, 1.0),
            destructive_foreground: hsla(210.0 / 360.0, 0.40, 0.98, 1.0),
            border: hsla(214.3 / 360.0, 0.318, 0.912, 1.0),
            input: hsla(214.3 / 360.0, 0.318, 0.912, 1.0),
            ring: hsla(222.2 / 360.0, 0.84, 0.049, 1.0),
            radius_sm: px(6.0),
            radius_md: px(8.0),
            radius_lg: px(10.0),
            radius_xl: px(12.0),
        }
    }

    /// Dark theme matching shadcn/ui defaults
    pub fn dark() -> Self {
        Self {
            background: hsla(222.2 / 360.0, 0.84, 0.049, 1.0),
            foreground: hsla(210.0 / 360.0, 0.40, 0.98, 1.0),
            card: hsla(222.2 / 360.0, 0.84, 0.049, 1.0),
            card_foreground: hsla(210.0 / 360.0, 0.40, 0.98, 1.0),
            popover: hsla(222.2 / 360.0, 0.84, 0.049, 1.0),
            popover_foreground: hsla(210.0 / 360.0, 0.40, 0.98, 1.0),
            primary: hsla(210.0 / 360.0, 0.40, 0.98, 1.0),
            primary_foreground: hsla(222.2 / 360.0, 0.473, 0.112, 1.0),
            secondary: hsla(217.2 / 360.0, 0.327, 0.176, 1.0),
            secondary_foreground: hsla(210.0 / 360.0, 0.40, 0.98, 1.0),
            muted: hsla(217.2 / 360.0, 0.327, 0.176, 1.0),
            muted_foreground: hsla(215.0 / 360.0, 0.204, 0.651, 1.0),
            accent: hsla(217.2 / 360.0, 0.327, 0.176, 1.0),
            accent_foreground: hsla(210.0 / 360.0, 0.40, 0.98, 1.0),
            destructive: hsla(0.0 / 360.0, 0.625, 0.306, 1.0),
            destructive_foreground: hsla(210.0 / 360.0, 0.40, 0.98, 1.0),
            border: hsla(217.2 / 360.0, 0.327, 0.176, 1.0),
            input: hsla(217.2 / 360.0, 0.327, 0.176, 1.0),
            ring: hsla(212.7 / 360.0, 0.267, 0.839, 1.0),
            radius_sm: px(6.0),
            radius_md: px(8.0),
            radius_lg: px(10.0),
            radius_xl: px(12.0),
        }
    }
}

//! Theme and color definitions for the desktop app.

use gpui::Hsla;

/// Convert a hex color string to GPUI's Hsla.
fn hex_to_hsla(hex: &str) -> Hsla {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;

    // Convert RGB to HSL
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < f32::EPSILON {
        return Hsla {
            h: 0.0,
            s: 0.0,
            l,
            a: 1.0,
        };
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < f32::EPSILON {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if (max - g).abs() < f32::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };

    Hsla {
        h: h / 6.0,
        s,
        l,
        a: 1.0,
    }
}

/// Color palette for the app, matching the web frontend's Tailwind colors.
pub struct Colors;

impl Colors {
    // Background colors
    pub fn background() -> Hsla {
        hex_to_hsla("#0a0a0a") // zinc-950
    }

    pub fn card() -> Hsla {
        hex_to_hsla("#18181b") // zinc-900
    }

    pub fn card_hover() -> Hsla {
        hex_to_hsla("#27272a") // zinc-800
    }

    // Text colors
    pub fn foreground() -> Hsla {
        hex_to_hsla("#fafafa") // zinc-50
    }

    pub fn muted() -> Hsla {
        hex_to_hsla("#a1a1aa") // zinc-400
    }

    // Border colors
    pub fn border() -> Hsla {
        hex_to_hsla("#27272a") // zinc-800
    }

    // State badge colors
    pub fn running() -> Hsla {
        hex_to_hsla("#2563eb") // blue-600
    }

    pub fn question() -> Hsla {
        hex_to_hsla("#ca8a04") // yellow-600
    }

    pub fn completed() -> Hsla {
        hex_to_hsla("#16a34a") // green-600
    }

    pub fn failed() -> Hsla {
        hex_to_hsla("#dc2626") // red-600
    }

    pub fn outline() -> Hsla {
        hex_to_hsla("#3f3f46") // zinc-700
    }

    pub fn secondary() -> Hsla {
        hex_to_hsla("#27272a") // zinc-800
    }
}

/// Text sizes
pub struct TextSize;

impl TextSize {
    pub fn xs() -> f32 {
        12.0
    }

    pub fn sm() -> f32 {
        14.0
    }

    pub fn base() -> f32 {
        16.0
    }

    pub fn lg() -> f32 {
        18.0
    }
}

/// Spacing values (in pixels)
pub struct Spacing;

impl Spacing {
    pub fn xs() -> f32 {
        4.0
    }

    pub fn sm() -> f32 {
        8.0
    }

    pub fn md() -> f32 {
        16.0
    }

    pub fn lg() -> f32 {
        24.0
    }

    pub fn xl() -> f32 {
        32.0
    }
}

/// Border radius values
pub struct Radius;

impl Radius {
    pub fn sm() -> f32 {
        4.0
    }

    pub fn md() -> f32 {
        8.0
    }

    pub fn lg() -> f32 {
        12.0
    }
}

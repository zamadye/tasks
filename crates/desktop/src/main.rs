//! Desktop application entry point
//!
//! Demonstrates the UI primitive components.
//!
//! Note: This demo requires macOS with a display. GPUI is designed primarily
//! for macOS, and running on other platforms requires additional configuration.

#[cfg(target_os = "macos")]
fn main() {
    use gpui::{
        div, px, App, AppContext, Bounds, Context, IntoElement, ParentElement, Render, Styled,
        Window, WindowBounds, WindowOptions,
    };

    use desktop::{
        Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardContent, CardDescription,
        CardHeader, CardTitle, Theme,
    };

    struct MainView;

    impl Render for MainView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let theme = Theme::default();

            div()
                .flex()
                .flex_col()
                .gap_6()
                .p_8()
                .bg(theme.background)
                .size_full()
                .child(
                    // Buttons section
                    Card::new()
                        .child(
                            CardHeader::new()
                                .child(CardTitle::new("Buttons"))
                                .child(CardDescription::new(
                                    "Button component with various variants and sizes",
                                )),
                        )
                        .child(
                            CardContent::new().child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_2()
                                    .child(Button::new("btn-default", "Default"))
                                    .child(
                                        Button::new("btn-secondary", "Secondary")
                                            .variant(ButtonVariant::Secondary),
                                    )
                                    .child(
                                        Button::new("btn-outline", "Outline")
                                            .variant(ButtonVariant::Outline),
                                    )
                                    .child(
                                        Button::new("btn-destructive", "Destructive")
                                            .variant(ButtonVariant::Destructive),
                                    )
                                    .child(
                                        Button::new("btn-ghost", "Ghost")
                                            .variant(ButtonVariant::Ghost),
                                    )
                                    .child(
                                        Button::new("btn-link", "Link").variant(ButtonVariant::Link),
                                    ),
                            ),
                        ),
                )
                .child(
                    // Button sizes section
                    Card::new()
                        .child(
                            CardHeader::new()
                                .child(CardTitle::new("Button Sizes"))
                                .child(CardDescription::new("Different button size variants")),
                        )
                        .child(
                            CardContent::new().child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Button::new("btn-xs", "Extra Small").size(ButtonSize::Xs),
                                    )
                                    .child(Button::new("btn-sm", "Small").size(ButtonSize::Sm))
                                    .child(
                                        Button::new("btn-md", "Default").size(ButtonSize::Default),
                                    )
                                    .child(Button::new("btn-lg", "Large").size(ButtonSize::Lg)),
                            ),
                        ),
                )
                .child(
                    // Badges section
                    Card::new()
                        .child(
                            CardHeader::new()
                                .child(CardTitle::new("Badges"))
                                .child(CardDescription::new("Badge variants for status display")),
                        )
                        .child(
                            CardContent::new().child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_2()
                                    .child(Badge::new("Default"))
                                    .child(Badge::new("Secondary").secondary())
                                    .child(Badge::new("Destructive").destructive())
                                    .child(Badge::new("Outline").outline())
                                    .child(Badge::new("Running").variant(BadgeVariant::Default))
                                    .child(Badge::new("Pending").variant(BadgeVariant::Secondary))
                                    .child(
                                        Badge::new("Failed").variant(BadgeVariant::Destructive),
                                    ),
                            ),
                        ),
                )
        }
    }

    gpui::Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, gpui::size(px(800.0), px(600.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_cx| MainView),
        )
        .unwrap();
    });
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("This demo application requires macOS.");
    eprintln!("The desktop library can still be used as a dependency.");
    eprintln!("For testing on other platforms, use the library directly in tests.");
}

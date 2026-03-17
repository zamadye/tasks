//! Tasks Desktop - Native GPUI frontend for the Tasks platform.
//!
//! This is the entry point for the desktop application.

use desktop::state::create_app_state;
use desktop::theme::Colors;
use desktop::views::Dashboard;
use gpui::{actions, div, prelude::*, px, App, AppContext, Menu, MenuItem, Styled, WindowOptions};

actions!(tasks_desktop, [Quit]);

fn main() {
    App::new().run(|cx: &mut AppContext| {
        // Register quit action
        cx.on_action(|_: &Quit, cx| cx.quit());

        // Set up menu bar
        cx.set_menus(vec![Menu {
            name: "Tasks".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        // Create app state
        let api_url = std::env::var("TASKS_API_URL").ok();
        let state = create_app_state(cx, api_url);

        // Open the main window
        cx.open_window(
            WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds {
                    origin: gpui::Point::default(),
                    size: gpui::Size {
                        width: px(1200.0),
                        height: px(800.0),
                    },
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Tasks".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_| Dashboard::new(state))
            },
        )
        .expect("Failed to open window");
    });
}

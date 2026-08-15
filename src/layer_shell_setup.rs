// src/layer_shell_setup.rs
//
// Handles all Wayland layer-shell configuration.
//
// This module is intentionally separate from the rest of the application.
// If we ever need to change how the overlay interacts with Niri or the
// layer-shell protocol, this is the main place we should need to touch.

use gtk4::ApplicationWindow;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::config::{Position, Settings};

/// Configures a GTK window as a Wayland layer-shell overlay.
pub fn setup(window: &ApplicationWindow, settings: &Settings, state: &crate::app_state::SharedState,) {
    // Initialize layer-shell support for this GTK window.
    window.init_layer_shell();

    // Put the overlay on the top layer.
    //
    // Layer::Top means the surface is above normal application windows.
    window.set_layer(Layer::Top);

    match settings.position {
        Position::TopLeft => {
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Left, true);

            window.set_margin(Edge::Top, settings.margin);
            window.set_margin(Edge::Left, settings.margin);
        }

        Position::TopRight => {
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Right, true);

            window.set_margin(Edge::Top, settings.margin);
            window.set_margin(Edge::Right, settings.margin);
        }

        Position::BottomLeft => {
            window.set_anchor(Edge::Bottom, true);
            window.set_anchor(Edge::Left, true);

            window.set_margin(Edge::Bottom, settings.margin);
            window.set_margin(Edge::Left, settings.margin);
        }

        Position::BottomRight => {
            window.set_anchor(Edge::Bottom, true);
            window.set_anchor(Edge::Right, true);

            window.set_margin(Edge::Bottom, settings.margin);
            window.set_margin(Edge::Right, settings.margin);
        }
    }

    // We don't want the overlay to reserve space in the desktop layout.
    //
    // This is important because the chat overlay should sit on top of
    // applications rather than pushing them around.
    window.set_exclusive_zone(-1);

    // Don't let the layer-shell surface take keyboard focus.
    //
    // We aren't building a text-input overlay yet, so keyboard input
    // should continue going to the application underneath.
    window.set_keyboard_mode(KeyboardMode::None);

    if state.borrow().click_through {
        window.connect_realize(|window| {
            if let Some(surface) = window.surface() {
                let region = cairo::Region::create();
                surface.set_input_region(&region);
            }
        });
    }
}
/// Updates the mouse input behavior of the overlay.
///
/// true  = mouse passes through the overlay.
/// false = overlay receives mouse input.
pub fn set_click_through(
    window: &ApplicationWindow,
    click_through: bool,
) {
    println!(
        "set_click_through: {} | size={}x{}",
        click_through,
        window.width(),
        window.height()
    );

    if let Some(surface) = window.surface() {
        println!("GDK surface: {:?}", surface);

        if click_through {
            let region = cairo::Region::create();
            surface.set_input_region(&region);
            window.queue_draw();

            println!("Applied EMPTY input region");
        } else {
            let width = window.width();
            let height = window.height();

            if width > 0 && height > 0 {
                let rectangle =
                    cairo::RectangleInt::new(0, 0, width, height);

                let region =
                    cairo::Region::create_rectangle(&rectangle);

                surface.set_input_region(&region);
                window.queue_draw();

                println!(
                    "Applied FULL input region: {}x{}",
                    width, height
                );
            }
        }
    } else {
        println!("NO GDK SURFACE!");
    }
}

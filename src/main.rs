use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::time::Duration;

mod config;
mod app_state;
mod layer_shell_setup;
mod ipc;

use config::Settings;
use app_state::SharedState;

use gtk4::prelude::*;
use gtk4::Application;
use gtk4::CssProvider;
use webkit6::prelude::*;
use webkit6::WebView;

const APP_ID: &str = "com.penguin.TwitchOverlay";

fn main() {
    let settings = Settings::load();

    let state: SharedState =
    app_state::create(settings.click_through);

    let (command_tx, command_rx) = mpsc::channel();

    let command_rx = Rc::new(RefCell::new(Some(command_rx)));

    ipc::start_listener(command_tx);

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    let command_rx = Rc::clone(&command_rx);

    app.connect_activate(move |app| {
        let command_rx = command_rx
            .borrow_mut()
            .take()
            .expect("Application activated more than once");

        build_ui(app, &settings, &state, command_rx);
    });

    app.run();
}

/// Builds the application's main window.
fn build_ui(app: &Application, settings: &Settings, state: &SharedState, command_rx: mpsc::Receiver<ipc::Command>,) {
    let window = gtk4::ApplicationWindow::builder()
        .application(app)
        .title("Twitch Overlay")
        .default_width(settings.width)
        .default_height(settings.height)
        .build();

    layer_shell_setup::setup(&window, settings, state);

    let css = CssProvider::new();

    css.load_from_data(
        "
window {
background: transparent;
}

box {
background: transparent;
}
",
    );

    gtk4::style_context_add_provider_for_display(
        &gtk4::prelude::WidgetExt::display(&window),
        &css,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    container.set_hexpand(true);
    container.set_vexpand(true);

    let webview = WebView::new();

    webview.set_background_color(
        &gdk4::RGBA::new(0.0, 0.0, 0.0, 0.0),
    );

    webview.set_hexpand(true);
    webview.set_vexpand(true);

    webview.load_uri(&settings.url);

    if settings.debug {
        let debug_html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                html, body {
                    margin: 0;
                    width: 100%;
                    height: 100%;
                    background: rgba(255, 0, 0, 0.15);
                }

                body {
                    color: white;
                    font-family: sans-serif;
                    font-size: 28px;
                    font-weight: bold;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    border: 3px solid red;
                    box-sizing: border-box;
                }
            </style>
        </head>

        <body>
            400 × 600 DEBUG :3
        </body>
        </html>
        "#;

        webview.load_html(debug_html, None);
    }

    container.append(&webview);

    window.set_child(Some(&container));

    let click = gtk4::GestureClick::new();

    click.connect_pressed(|_, _, x, y| {
        println!("OVERLAY CLICKED at {x}, {y}");
    });

    container.add_controller(click);

    window.present();

    layer_shell_setup::set_click_through(
        &window,
        state.borrow().click_through,
    );

    let window = window.clone();
    let state = state.clone();
    let app = app.clone();

    gtk4::glib::timeout_add_local(
        Duration::from_millis(50),
        move || {
            while let Ok(command) = command_rx.try_recv() {
                match command {
                    ipc::Command::Toggle => {
                        let new_value = !state.borrow().click_through;

                        state.borrow_mut().click_through = new_value;

                        layer_shell_setup::set_click_through(
                            &window,
                            new_value,
                        );

                        println!(
                            "Click-through: {}",
                            new_value
                        );
                    }

                    ipc::Command::Enable => {
                        state.borrow_mut().click_through = true;

                        layer_shell_setup::set_click_through(
                            &window,
                            true,
                        );

                        println!("Click-through: true");
                    }

                    ipc::Command::Disable => {
                        state.borrow_mut().click_through = false;

                        layer_shell_setup::set_click_through(
                            &window,
                            false,
                        );

                        println!("Click-through: false");
                    }

                    ipc::Command::Show => {
                        window.present();
                        println!("Overlay: shown");
                    }

                    ipc::Command::Hide => {
                        window.hide();
                        println!("Overlay: hidden");
                    }

                    ipc::Command::Reload => {
                        webview.reload();
                        println!("WebView: reloaded");
                    }

                    ipc::Command::Quit => {
                        println!("Overlay: quitting");
                        app.quit();
                    }
                }
            }

            gtk4::glib::ControlFlow::Continue
        },
    );
}

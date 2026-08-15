# Twitch Overlay for Niri / Wayland

A lightweight Twitch/chat overlay built with **Rust, GTK4, WebKitGTK 6,
and gtk4-layer-shell**, designed for Wayland compositors such as
**Niri**.

## Features

-   Transparent GTK4 overlay
-   Wayland layer-shell support
-   Configurable width, height, position, and margin
-   Web-based chat through WebKitGTK
-   Runtime click-through control
-   Show/hide without restarting
-   WebView reload
-   Clean shutdown
-   Local Unix socket IPC
-   Python command-line controller
-   Niri keybind integration
-   Debug mode for testing

## Requirements

-   Linux
-   Wayland
-   A compositor with layer-shell support
-   Rust/Cargo
-   GTK4
-   WebKitGTK 6
-   gtk4-layer-shell
-   Cairo
-   Python 3

The project was developed and tested with Niri.

## Build

``` bash
git clone <your-repository-url>
cd twitch-overlay
cargo build
```

Run:

``` bash
cargo run
```

Release build:

``` bash
cargo build --release
```

## Configuration

Example:

``` toml
url = "https://example.com"
opacity = 0.85
position = "TopRight"
width = 400
height = 600
margin = 20
click_through = true
```

Supported positions:

``` text
TopLeft
TopRight
BottomLeft
BottomRight
```

Set `url` to the chat/widget page you want to display.

> **Privacy:** Do not commit a private/account-specific widget URL to a
> public repository if it contains a secret or sensitive identifier.

## IPC

The overlay listens on:

``` text
/tmp/twitch-overlay.sock
```

Commands:

  Command     Action
  ----------- -----------------------
  `ping`      Test the connection
  `toggle`    Toggle click-through
  `enable`    Enable click-through
  `disable`   Disable click-through
  `show`      Show the overlay
  `hide`      Hide the overlay
  `reload`    Reload the WebView
  `quit`      Quit the overlay

## Python Controller

`control.py` provides a simple command-line controller using Python's
standard `socket` module.

Make it executable:

``` bash
chmod +x control.py
```

Test:

``` bash
./control.py ping
```

Expected:

``` text
pong
```

Examples:

``` bash
./control.py show
./control.py hide
./control.py enable
./control.py disable
./control.py toggle
./control.py reload
./control.py quit
```

## Niri Keybinds

Example:

``` kdl
// ─── Twitch Overlay ───
Mod+Shift+C       hotkey-overlay-title="Toggle Twitch Chat Click-Through" { spawn-sh "/home/your-user/twitch-overlay/control.py toggle"; }
Mod+Shift+H       hotkey-overlay-title="Hide Twitch Chat" { spawn-sh "/home/your-user/twitch-overlay/control.py hide"; }
Mod+Shift+J       hotkey-overlay-title="Show Twitch Chat" { spawn-sh "/home/your-user/twitch-overlay/control.py show"; }
Mod+Shift+K       hotkey-overlay-title="Reload Twitch Chat" { spawn-sh "/home/your-user/twitch-overlay/control.py reload"; }
```

Replace `/home/your-user/twitch-overlay/` with your actual project path.

## Click-Through

When click-through is enabled, pointer input passes through the overlay
to the application underneath.

When disabled, the overlay receives pointer input.

The implementation uses GDK's input region support with Cairo regions
and refreshes the surface after changing the input region.

## Architecture

``` text
Niri keybind
     |
     v
control.py
     |
     | Unix socket
     v
ipc.rs
     |
     | mpsc channel
     v
main.rs / GTK main loop
     |
     v
GTK4 + WebKitGTK + gtk4-layer-shell
```

Project structure:

``` text
twitch-overlay/
├── Cargo.toml
├── control.py
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── app_state.rs
│   ├── layer_shell_setup.rs
│   └── ipc.rs
└── target/
```

### `main.rs`

Creates the GTK application, overlay window, WebView, and runtime
command handling.

### `config.rs`

Loads and represents overlay configuration.

### `app_state.rs`

Stores shared runtime state such as click-through status.

### `layer_shell_setup.rs`

Configures Wayland layer-shell behavior, positioning, margins, keyboard
interaction, and pointer input regions.

### `ipc.rs`

Creates the local Unix socket, receives commands, and forwards them to
the GTK application.

## Manual IPC Test

You can test the socket without `control.py`:

``` bash
python3 -c 'import socket; s=socket.socket(socket.AF_UNIX); s.connect("/tmp/twitch-overlay.sock"); s.sendall(b"ping
"); print(s.recv(1024).decode().strip()); s.close()'
```

Expected:

``` text
pong
```

## Debug Mode

Debug mode can display a temporary test page such as:

``` text
400 × 600 DEBUG :3
```

This is useful for checking:

-   overlay dimensions
-   transparency
-   layer-shell placement
-   WebView sizing

The debug page is intended for development rather than production.

## Streaming Workflow

For a stream:

1.  Start the overlay.
2.  Confirm the configured chat widget is visible.
3.  Start the stream.
4.  Enable click-through while gaming.
5.  Use the Niri keybinds to show, hide, toggle, or reload the chat.

The overlay displays the configured WebView URL; the chat service itself
is responsible for providing live chat updates.

## Current Status

-   [x] GTK4 overlay
-   [x] Transparent background
-   [x] Niri layer-shell integration
-   [x] Top-right positioning
-   [x] Configurable dimensions
-   [x] WebView chat
-   [x] Click-through
-   [x] Runtime click-through toggle
-   [x] Local IPC
-   [x] Python controller
-   [x] Show/hide
-   [x] WebView reload
-   [x] Clean quit
-   [x] Niri keybind control

## Planned Features

-   Dynamic width and height
-   Dynamic position at runtime
-   Output/monitor selection
-   Runtime configuration changes
-   Saving configuration changes
-   Overlay presets
-   Improved debug/reload behavior
-   Easier installation and packaging

## Security and Privacy

The IPC socket is local:

``` text
/tmp/twitch-overlay.sock
```

No network IPC server is required.

Third-party chat widget URLs may contain account or widget identifiers.
Avoid publishing private URLs, credentials, or sensitive identifiers in
Git repositories, screenshots, logs, or issue reports.

## License

Add your preferred license before publishing.

For example:

``` text
MIT License
```

If using MIT, add a `LICENSE` file containing the standard MIT license
text.

## Contributing

Issues, compositor compatibility fixes, and feature contributions are
welcome.

When reporting a problem, include:

-   Linux distribution
-   Wayland compositor
-   GTK version
-   Rust version
-   Relevant terminal output
-   Relevant configuration

Do not post private chat widget URLs or other sensitive account
information in issues.

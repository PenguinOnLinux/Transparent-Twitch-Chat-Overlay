# Twitch Overlay for Niri / Wayland

A lightweight Twitch/chat overlay built with **Rust, GTK4, WebKitGTK 6, and gtk4-layer-shell**, designed for Wayland compositors such as **Niri**.

The overlay provides a transparent, configurable chat window that can stay above your applications while gaming or streaming.

## Features

- Transparent GTK4 overlay
- Wayland layer-shell support
- Configurable width and height
- Configurable position and margins
- Web-based chat through WebKitGTK
- Runtime click-through control
- Show/hide without restarting
- WebView reload
- Clean shutdown
- Local Unix socket IPC
- Python command-line controller
- Niri keybind integration
- Debug mode for testing

## Requirements

- Linux
- Wayland
- A compositor with layer-shell support
- Rust and Cargo
- GTK4
- WebKitGTK 6
- gtk4-layer-shell
- Cairo
- Python 3

The project was developed and tested with **Niri**.

## Build

Clone the repository:

```bash
git clone https://github.com/PenguinOnLinux/Transparent-Twitch-Chat-Overlay.git
cd Transparent-Twitch-Chat-Overlay
```

Build the project:

```bash
cargo build
```

Run:

```bash
cargo run
```

For a release build:

```bash
cargo build --release
```

The compiled files are placed in Cargo's `target/` directory. This directory is generated automatically and is not part of the Git repository.

## Configuration

The overlay can be configured through a TOML configuration file.

Example:

```toml
url = "https://example.com"
opacity = 0.85
position = "TopRight"
width = 400
height = 600
margin = 20
click_through = true
```

### Configuration options

| Option | Description |
|---|---|
| `url` | Web page or chat/widget URL displayed by the overlay |
| `opacity` | Overlay opacity |
| `position` | Overlay position |
| `width` | Overlay width |
| `height` | Overlay height |
| `margin` | Distance from the selected screen edges |
| `click_through` | Whether pointer input passes through the overlay |

### Supported positions

```text
TopLeft
TopRight
BottomLeft
BottomRight
```

Set `url` to the chat or widget page you want to display.

> **Privacy:** Do not commit private or account-specific widget URLs to a public repository if they contain secrets or sensitive identifiers.

## IPC

The overlay provides a local Unix socket for runtime control:

```text
/tmp/twitch-overlay.sock
```

Available commands:

| Command | Action |
|---|---|
| `ping` | Test the connection |
| `toggle` | Toggle click-through |
| `enable` | Enable click-through |
| `disable` | Disable click-through |
| `show` | Show the overlay |
| `hide` | Hide the overlay |
| `reload` | Reload the WebView |
| `quit` | Quit the overlay |

## Python Controller

`control.py` provides a simple command-line controller using Python's standard `socket` module.

Make it executable:

```bash
chmod +x control.py
```

Test the connection:

```bash
./control.py ping
```

Expected output:

```text
pong
```

Examples:

```bash
./control.py show
./control.py hide
./control.py enable
./control.py disable
./control.py toggle
./control.py reload
./control.py quit
```

## Niri Keybinds

Example Niri configuration:

```kdl
// ─── Twitch Overlay ───
Mod+Shift+C hotkey-overlay-title="Toggle Twitch Chat Click-Through" {
    spawn-sh "/home/your-user/twitch-overlay/control.py toggle";
}

Mod+Shift+H hotkey-overlay-title="Hide Twitch Chat" {
    spawn-sh "/home/your-user/twitch-overlay/control.py hide";
}

Mod+Shift+J hotkey-overlay-title="Show Twitch Chat" {
    spawn-sh "/home/your-user/twitch-overlay/control.py show";
}

Mod+Shift+K hotkey-overlay-title="Reload Twitch Chat" {
    spawn-sh "/home/your-user/twitch-overlay/control.py reload";
}
```

Replace `/home/your-user/twitch-overlay/` with the actual path to your project.

## Click-Through

When click-through is enabled, pointer input passes through the overlay to the application underneath.

When click-through is disabled, the overlay receives pointer input normally.

The implementation uses GDK input-region support with Cairo regions and refreshes the surface after changing the input region.

## Architecture

```text
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

## Project Structure

```text
twitch-overlay/
├── Cargo.toml
├── Cargo.lock
├── control.py
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── app_state.rs
│   ├── layer_shell_setup.rs
│   └── ipc.rs
└── README.md
```

### `main.rs`

Creates the GTK application, overlay window, WebView, and runtime command handling.

### `config.rs`

Loads and represents the overlay configuration.

### `app_state.rs`

Stores shared runtime state such as click-through status.

### `layer_shell_setup.rs`

Configures Wayland layer-shell behavior, positioning, margins, keyboard interaction, and pointer input regions.

### `ipc.rs`

Creates the local Unix socket, receives commands, and forwards them to the GTK application.

## Manual IPC Test

You can test the socket without using `control.py`:

```bash
python3 -c 'import socket; s=socket.socket(socket.AF_UNIX); s.connect("/tmp/twitch-overlay.sock"); s.sendall(b"ping\n"); print(s.recv(1024).decode().strip()); s.close()'
```

Expected output:

```text
pong
```

## Debug Mode

Debug mode can display a temporary test page such as:

```text
400 × 600 DEBUG :3
```

This is useful for checking:

- Overlay dimensions
- Transparency
- Layer-shell placement
- WebView sizing

The debug page is intended for development and testing rather than production use.

## Streaming Workflow

For a typical streaming setup:

1. Start the overlay.
2. Confirm that the configured chat widget is visible.
3. Start the stream.
4. Enable click-through while gaming.
5. Use the Niri keybinds to show, hide, toggle, or reload the chat.

The overlay only displays the configured WebView URL. The embedded chat/widget service is responsible for providing and updating the live chat content.

## Current Status

- [x] GTK4 overlay
- [x] Transparent background
- [x] Niri layer-shell integration
- [x] Top-right positioning
- [x] Configurable dimensions
- [x] WebView chat
- [x] Click-through
- [x] Runtime click-through toggle
- [x] Local IPC
- [x] Python controller
- [x] Show/hide
- [x] WebView reload
- [x] Clean quit
- [x] Niri keybind control

## Planned Features

- [ ] Runtime width and height changes
- [ ] Dynamic position changes at runtime
- [ ] Output/monitor selection
- [ ] Runtime configuration changes
- [ ] Saving configuration changes
- [ ] Overlay presets
- [ ] Improved debug/reload behavior
- [ ] Easier installation and packaging

## Security and Privacy

The IPC socket is local:

```text
/tmp/twitch-overlay.sock
```

No network IPC server is required.

Third-party chat widget URLs may contain account or widget identifiers. Avoid publishing private URLs, credentials, or other sensitive information in:

- Git repositories
- Screenshots
- Logs
- Issue reports
- Public configuration files

## License

This project is licensed under the **GNU General Public License v3.0**.

See the `LICENSE` file for the full license text.

## Contributing

Issues, compositor compatibility fixes, and feature contributions are welcome.

When reporting a problem, please include:

- Linux distribution
- Wayland compositor
- GTK version
- Rust version
- Relevant terminal output
- Relevant configuration

Please do not post private chat widget URLs, credentials, or other sensitive account information in issues.

---

Made with 🐧 and Rust.

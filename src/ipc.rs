// src/ipc.rs
//
// Local IPC control for the Twitch overlay.

use interprocess::local_socket::{
    GenericFilePath, ListenerOptions, Stream, ToFsName,
};
use interprocess::local_socket::traits::ListenerExt;

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;

const SOCKET_NAME: &str = "twitch-overlay.sock";

/// Commands that can be sent to the running overlay.
#[derive(Debug, Clone, Copy)]
pub enum Command {
    Toggle,
    Enable,
    Disable,
    Show,
    Hide,
    Reload,
    Quit,
}

/// Returns the path used for the overlay's local IPC socket.
fn socket_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(SOCKET_NAME);
    path
}

/// Starts the IPC listener thread.
pub fn start_listener(sender: Sender<Command>) {
    let path = socket_path();

    // Remove a stale socket from a previous crashed run.
    let _ = std::fs::remove_file(&path);

    let listener = ListenerOptions::new()
        .name(
            path.to_string_lossy()
                .as_ref()
                .to_fs_name::<GenericFilePath>()
                .expect("Failed to create IPC socket name"),
        )
        .create_sync()
        .expect("Failed to create IPC socket");

    println!("IPC socket: {}", path.display());

    thread::spawn(move || {
        for connection in listener.incoming() {
            match connection {
                Ok(stream) => {
                    let sender = sender.clone();

                    thread::spawn(move || {
                        handle_connection(stream, sender);
                    });
                }

                Err(e) => {
                    eprintln!("IPC connection error: {e}");
                }
            }
        }
    });
}

/// Handles one IPC connection.
fn handle_connection(
    mut stream: Stream,
    sender: Sender<Command>,
) {
    let mut command = String::new();

    {
        let mut reader = BufReader::new(&mut stream);

        if let Err(e) = reader.read_line(&mut command) {
            eprintln!("IPC read error: {e}");
            return;
        }
    }

    let command = command.trim();

    println!("IPC command: {command}");

    let response = match command {
        "ping" => "pong\n",

        "toggle" => {
            if sender.send(Command::Toggle).is_ok() {
                "ok\n"
            } else {
                "error\n"
            }
        }

        "enable" => {
            if sender.send(Command::Enable).is_ok() {
                "ok\n"
            } else {
                "error\n"
            }
        }

        "disable" => {
            if sender.send(Command::Disable).is_ok() {
                "ok\n"
            } else {
                "error\n"
            }
        }

        "show" => {
            if sender.send(Command::Show).is_ok() {
                "ok\n"
            } else {
                "error\n"
            }
        }

        "hide" => {
            if sender.send(Command::Hide).is_ok() {
                "ok\n"
            } else {
                "error\n"
            }
        }

        "reload" => {
            if sender.send(Command::Reload).is_ok() {
                "ok\n"
            } else {
                "error\n"
            }
        }
 
        "quit" => {
            if sender.send(Command::Quit).is_ok() {
                "ok\n"
            } else {
                "error\n"
            }
        }

        _ => "unknown\n",
    };

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("IPC write error: {e}");
    }
}

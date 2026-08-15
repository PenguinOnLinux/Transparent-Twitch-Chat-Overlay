// src/config.rs
//
// Handles all persistent settings for the overlay: the target URL,
// opacity, screen position, and click-through state.
// This module has NO GTK dependencies on purpose — it's pure data +
// file I/O, so it can be tested independently of the GUI.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Which corner of the screen the overlay should anchor to.
/// We use anchor presets (not free-form dragging) because Wayland's
/// layer-shell protocol positions windows via anchor + margin, not
/// arbitrary x/y coordinates like a normal desktop window.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for Position {
    fn default() -> Self {
        Position::TopRight
    }
}

/// All user-configurable settings for the overlay.
/// Derives Serialize/Deserialize so `serde` + `toml` can automatically
/// convert this struct to/from the config file on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// The URL to load in the overlay (e.g. a Botrix chat widget link).
    pub url: String,

    /// Overlay opacity, from 0.0 (fully transparent) to 1.0 (fully opaque).
    pub opacity: f64,

    /// Which screen corner the overlay is anchored to.
    pub position: Position,

    /// Distance in pixels from the anchored edges.
    pub margin: i32,

    /// Width of the overlay in pixels.
    pub width: i32,

    /// Height of the overlay in pixels.
    pub height: i32,

    /// Shows a visible debug background/border around the overlay.
    pub debug: bool,

    /// Whether mouse clicks pass through the overlay to windows beneath it.
    pub click_through: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            url: String::from("https://example.com"),
            opacity: 0.85,
            position: Position::default(),
            margin: 20,
            width: 400,
            height: 600,
            debug: false,
            click_through: true,
        }
    }
}

impl Settings {
    /// Returns the full path to the config file, e.g.
    /// ~/.config/twitch-overlay/config.toml
    ///
    /// Uses the `dirs` crate to correctly resolve the XDG config
    /// directory rather than hardcoding "~/.config", which isn't
    /// guaranteed to be correct on every system.
    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir()
            .expect("Could not determine config directory");
        path.push("twitch-overlay");
        path.push("config.toml");
        path
    }

    /// Loads settings from disk. If no config file exists yet
    /// (e.g. first run), creates one with default values and returns those.
    pub fn load() -> Settings {
        let path = Self::config_path();

        if let Ok(contents) = fs::read_to_string(&path) {
            match toml::from_str(&contents) {
                Ok(settings) => return settings,
                Err(e) => {
                    eprintln!("Failed to parse config file, using defaults: {e}");
                }
            }
        }

        // No file yet, or it was invalid — fall back to defaults
        // and write them out so the user has something to edit.
        let defaults = Settings::default();
        defaults.save();
        defaults
    }

    /// Writes the current settings to disk as TOML,
    /// creating the config directory if it doesn't exist yet.
    pub fn save(&self) {
        let path = Self::config_path();

        if let Some(parent_dir) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent_dir) {
                eprintln!("Failed to create config directory: {e}");
                return;
            }
        }

        match toml::to_string_pretty(self) {
            Ok(toml_string) => {
                if let Err(e) = fs::write(&path, toml_string) {
                    eprintln!("Failed to write config file: {e}");
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize settings: {e}");
            }
        }
    }
}

// src/app_state.rs
//
// Runtime state shared by the running overlay.

use std::cell::RefCell;
use std::rc::Rc;

/// Runtime state of the overlay.
#[derive(Debug)]
pub struct AppState {
    /// Whether mouse input currently passes through the overlay.
    pub click_through: bool,
}

/// Shared runtime state.
pub type SharedState = Rc<RefCell<AppState>>;

/// Creates runtime state from the startup configuration.
pub fn create(click_through: bool) -> SharedState {
    Rc::new(RefCell::new(AppState {
        click_through,
    }))
}

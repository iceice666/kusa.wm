//! penrose :: minimal configuration
//!
//! This file will give you a functional if incredibly minimal window manager that
//! has multiple workspaces and simple client / workspace movement.
mod keybindings;

use keybindings::raw_key_bindings;
use penrose::{
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    x11rb::RustConn,
    Result,
};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let wm = WindowManager::new(Config::default(), key_bindings, HashMap::new(), conn)?;

    wm.run()
}

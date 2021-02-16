//! Visualization of scenes rendered by Tracy using the `imgui-rs` crate.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

use ui::TracyUi;

mod scene;
mod ui;

fn main() {
    TracyUi::new("Tracy - a ray tracing renderer", 1280, 640).run();
}

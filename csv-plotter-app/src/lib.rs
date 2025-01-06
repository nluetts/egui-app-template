#![warn(clippy::all, rust_2018_idioms)]

mod backend_state;
mod egui;

pub use backend_state::BackendAppState;
pub use egui::EguiApp;
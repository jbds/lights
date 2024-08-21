#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod central_panel;
mod dmx_send;

pub use app::TemplateApp;
//pub use dmx_send::spawn_receiver;

#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod dmx_send;
mod json_storage;
mod panels;
mod utilities;

pub use app::LightsApp;

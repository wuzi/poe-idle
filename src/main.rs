#![allow(
    clippy::cargo,
    clippy::complexity,
    clippy::expect_used,
    clippy::nursery,
    clippy::pedantic,
    clippy::style
)]

mod app;
mod components;
mod constants;
mod data;
mod gameplay;
mod ui;
mod visual;

fn main() {
    app::run();
}

#![allow(non_snake_case)]

use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use log::LevelFilter;
mod constants;
mod hooks;
mod router;
mod views;

fn App(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    log::info!("starting app");
    dioxus_web::launch(App);
}

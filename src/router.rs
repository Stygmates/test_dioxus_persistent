use crate::views::{header::Header, home::Home};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Header)]
    #[route("/")]
    Home {},
}

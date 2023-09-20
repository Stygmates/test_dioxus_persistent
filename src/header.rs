use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;

use crate::{
    constants::COUNTER,
    hooks::{use_persistent, UsePersistent},
    router::Route,
    views::home::CounterState,
};

#[component]
pub fn Header(cx: Scope) -> Element {
    let counter_state: &UsePersistent<CounterState> = use_persistent(cx, COUNTER);
    cx.render(rsx! {
        div{"Header value: ", counter_state.get().0.to_string()}
        Outlet::<Route> {}
    })
}

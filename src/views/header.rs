use dioxus::prelude::*;

use crate::{
    constants::COUNTER,
    hooks::{use_persistent, UsePersistent},
    views::home::CounterState,
};

#[component]
pub fn Header(cx: Scope) -> Element {
    let counter_persistent_state: &UsePersistent<CounterState> = use_persistent(cx, COUNTER);
    render!(div{"Header counter: ", counter_persistent_state.get().0.to_string()})
}

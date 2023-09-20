use std::fmt::Display;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    constants::COUNTER,
    hooks::{use_persistent, UsePersistent},
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CounterState(pub u64);

impl Display for CounterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[component]
pub fn Home(cx: Scope) -> Element {
    let counter_state: &UsePersistent<CounterState> = use_persistent(cx, COUNTER);
    cx.render(rsx! {
        button{onclick: move |_event| counter_state.set(CounterState(counter_state.get().0 + 1)), "Add counter" }
        div{"Home value: ", counter_state.get().0.to_string()}
    })
}

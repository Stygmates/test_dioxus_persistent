# Dioxus persistent state

This project was created to demonstrate the bug with [this](https://dioxuslabs.com/learn/0.4/cookbook/state/custom_hooks?phantom=composing-hooks#composing-hooks) example on the dioxus docsite, the state is updated inside the current component but not on another one that references it.
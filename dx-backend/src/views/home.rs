use crate::components::{Echo, Title};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Title {}
        // Hero {}
        Echo {}
    }
}

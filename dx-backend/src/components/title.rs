use dioxus::prelude::*;

#[component]
pub fn Title() -> Element {
    rsx! {
        div { id: "title",
            div { id: "header",
                h1 { "Harmoniq" }
            }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "🔒 Login" }
            }
        }
    }
}

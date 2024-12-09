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
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

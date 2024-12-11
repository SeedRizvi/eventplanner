use dioxus::prelude::*;

const CHAT_CSS: Asset = asset!("/assets/styling/chat.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());
    let mut enter = use_signal(|| false);
    
    let change_evt = move |evt: KeyboardEvent| match evt.key() {
        Key::Enter => {
            println!("Enter pressed!");
            enter.set(true);
        },
        _ => {println!("Enter NOT pressed");}
    };
    
    rsx! {
        document::Link { rel: "stylesheet", href: CHAT_CSS }
        div { id: "chat",
            h2 { "Message Demo" }
            input {
                placeholder: "Type here to send message...",
                oninput: move |event| async move {
                    let val = enter.read().clone();
                    println!("{:?}", val);
                    if enter.read().clone() == true {
                        enter.set(false);
                        let data = chat_server(event.value()).await.unwrap();
                        response.set(data);
                    }
                },
                p {
                    if !response().is_empty() {
                        "Message: "
                        i { "{response}" }
                    }
                }
            }
        }
    }
}

// input {
//     placeholder: "Type here to send message...",
//     oninput: move |event| async move {
//         let data = chat_server(event.value()).await.unwrap();
//         response.set(data);
//     }
// }

/// Writes the user input to chat on the server.
#[server(ChatServer)]
async fn chat_server(input: String) -> Result<String, ServerFnError> {
    // TODO: Make a POST to a chat thing
    println!("{input} of len {}", input.len());
    if (input.ends_with("\n")) {
        return Ok(input);
    }
    Ok(String::new())
}

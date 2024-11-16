
use dioxus::prelude::*;
{% if router %}
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
{% endif %}

fn main() {
    dioxus::launch(app);
}

{% if router %}
fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
{% else %}
fn app() -> Element {
    // Build cool things ✌️
    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/main.css") }
        img { id: "header", src: asset!("/assets/header.svg") }
        div { id: "links",
            a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
            a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { href: "https://github.com/DioxusLabs/dioxus-sdk", "⚙️ Dioxus Development Kit" }
            a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                "💫 VSCode Extension"
            }
            a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    }
}
{% endif %}

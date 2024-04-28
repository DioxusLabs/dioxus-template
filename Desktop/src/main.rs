#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
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
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    {% if styling == "Tailwind" %}
    let cfg = dioxus::desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
    {% else %}
    dioxus::launch(App);
    {% endif %}
}

{% if router %}
#[component]
fn App() -> Element {
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
#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        img { src: "header.svg", id: "header" }
        div { id: "links",
            a { href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
            a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
            a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                "💫 VSCode Extension"
            }
            a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    }
}
{% endif %}

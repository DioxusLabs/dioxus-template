#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
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
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    {% if styling == "Tailwind" %}
    let cfg = dioxus::Desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    let builder = LaunchBuilder::desktop().with_cfg(cfg);
    builder.launch(App);
    {% else %}
    launch(App);
    {% endif %}
}

{% if router %}
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
fn App() -> Element {
    rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
    )
}
{% endif %}
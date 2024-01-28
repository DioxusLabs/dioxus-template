#![allow(non_snake_case)]
{% if platform == "fullstack" %}
#![allow(unused)]
use dioxus_fullstack::prelude::*;
{% endif %}
{% if router %}
use dioxus_router::prelude::*;
{% endif %}
use dioxus::prelude::*;
use log::LevelFilter;

{% if platform == "liveview" %}
{% if backend == "Axum" %}
#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 8080).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = axum::Router::new()
        .route(
            "/",
            axum::routing::get(move || async move {
                axum::response::Html(format!(
                    r#"
            <!DOCTYPE html>
            <html>
                <head> <title>Dioxus LiveView with axum</title>  </head>
                <body> <div id="main"></div> </body>
                {glue}
            </html>
            "#,
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
                ))
            }),
        )
        .route(
            "/ws",
            axum::routing::get(move |ws: axum::extract::ws::WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    _ = view.launch(dioxus_liveview::axum_socket(socket), app).await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
{% endif %}
{% if backend == "Salvo" %}
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 8080).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let router = salvo::Router::new()
        .hoop(salvo::affix::inject(std::sync::Arc::new(view)))
        .get(index)
        .push(salvo::Router::with_path("ws").get(connect));

    println!("Listening on http://{}", addr);

    salvo::Server::new(salvo::listener::TcpListener::bind(addr))
        .serve(router)
        .await;
}

#[salvo::handler]
fn index(_depot: &mut salvo::Depot, res: &mut salvo::Response) {
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 8080).into();
    res.render(salvo::writer::Text::Html(format!(
        r#"
            <!DOCTYPE html>
            <html>
                <head> <title>Dioxus LiveView with Salvo</title>  </head>
                <body> <div id="main"></div> </body>
                {glue}
            </html>
            "#,
        glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
    )));
}

#[salvo::handler]
async fn connect(
    req: &mut salvo::Request,
    depot: &mut salvo::Depot,
    res: &mut salvo::Response,
) -> Result<(), salvo::http::StatusError> {
    let view = depot
        .obtain::<std::sync::Arc<dioxus_liveview::LiveViewPool>>()
        .unwrap()
        .clone();

    salvo::ws::WebSocketUpgrade::new()
        .upgrade(req, res, |ws| async move {
            _ = view.launch(dioxus_liveview::salvo_socket(ws), app).await;
        })
        .await
}
{% endif %}
{% if backend == "Warp" %}
#[tokio::main]
async fn main() {
    use warp::Filter;

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 8080).into();

    let index = warp::path::end().map(move || {
        warp::reply::html(format!(
            r#"
            <!DOCTYPE html>
            <html>
                <head> <title>Dioxus LiveView with Warp</title>  </head>
                <body> <div id="main"></div> </body>
                {glue}
            </html>
            "#,
            glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws/"))
        ))
    });

    let pool = dioxus_liveview::LiveViewPool::new();

    let ws = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || pool.clone()))
        .map(
            move |ws: warp::ws::Ws, pool: dioxus_liveview::LiveViewPool| {
                ws.on_upgrade(|ws| async move {
                    let _ = pool
                        .launch(
                            dioxus_liveview::adapters::warp_adapter::warp_socket(ws),
                            app,
                        )
                        .await;
                })
            },
        );

    println!("Listening on http://{}", addr);

    warp::serve(index.or(ws)).run(addr).await;
}
{% endif %}
{% endif %}
{% if platform == "web" %}
fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}
{% endif %}
{% if platform == "desktop" %}
fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    {% if styling == "Tailwind" %}
    dioxus_desktop::launch_cfg(app, dioxus_desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string()));
    {% else %}
    dioxus_desktop::launch(app);
    {% endif %}
}
{% endif %}
{% if platform == "TUI" %}
fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_tui::launch(app);
}
{% endif %}
{% if platform == "fullstack" %}
fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    {% if router %}
    LaunchBuilder::<FullstackRouterConfig<Route>>::router().launch();
    {% else %}
    LaunchBuilder::new(app).launch();
    {% endif %}
}
{% endif %}

{% if router %}
{% if platform != "fullstack" %}
fn app(cx: Scope) -> Element {
    render!{
        Router::<Route> {}
    }
}
{% endif %}

{% if platform != "fullstack" %}
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
{% endif %}
{% if platform == "fullstack" %}
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
{% endif %}

#[component]
fn Blog(cx: Scope, id: i32) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    {% if platform == "fullstack" %}
    let text = use_state(cx, || "...".to_string());
    {% endif %}

    cx.render(rsx! {
        Link {
            to: Route::Blog {
                id: *count.get()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            {% if platform == "fullstack" %}
            button {
                onclick: move |_| {
                    to_owned![text];
                    async move {
                        if let Ok(data) = get_server_data().await {
                            println!("Client received: {}", data);
                            text.set(data.clone());
                            post_server_data(data).await.unwrap();
                        }
                    }
                },
                "Run server function!"
            }
            "Server said: {text}"
            {% endif %}
        }
    })
}
{% else %}
fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
    ))
}
{% endif %}

{% if platform == "fullstack" %}
#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
{% endif %}

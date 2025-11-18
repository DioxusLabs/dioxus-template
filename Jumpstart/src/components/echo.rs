use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    // use_signal is a hook. Hooks in dioxus must be run in a consistent order every time the component is rendered.
    // That means they can't be run inside other hooks, async blocks, if statements, or loops.
    //
    // use_signal is a hook that creates a state for the component. It takes a closure that returns the initial value of the state.
    // The state is automatically tracked and will rerun any other hooks or components that read it whenever it changes.
    let mut input = use_signal(|| String::new());

    // TODO: Explain use_resource here.
    let echo = use_resource(move || async move {
        echo_server(input()).await.unwrap()
    });

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }

        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                // `oninput` is an event handler that will run when the input changes.
                // It can return either nothing or a future that will be run when the event runs.
                oninput:  move |event| {
                    input.set(event.value());
                },
            }

            // Signals can be called like a function to clone the current value of the signal
            // Since we read the signal inside this component, the component "subscribes" to the signal. Whenever
            // the signal changes, the component will rerun.
            if let Some(response) = echo() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

// Server functions let us define public APIs on the server that can be called like a normal async function from the client.
// Each server function needs to be annotated with the `#[post]`/`#[get]` attributes, accept and return serializable types, and return
// a `Result` with the error type [`ServerFnError`].
//
// When the server function is called from the client, it will just serialize the arguments, call the API, and deserialize the
// response.
#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String> {
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.
    Ok(input)
}

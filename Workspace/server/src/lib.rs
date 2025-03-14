//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

#[cfg(feature = "server")]
pub mod server {
    use super::*;
}

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
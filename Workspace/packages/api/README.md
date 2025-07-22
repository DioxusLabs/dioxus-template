# API

This crate contains all shared fullstack server functions. This is a great place to place any server-only logic you would like to expose in multiple platforms like a method that accesses your database or a method that sends an email.

This crate will be built twice:
1. Once for the server build with the `dioxus/server` feature enabled
2. Once for the client build with the client feature disabled

During the server build, the server functions will be collected and hosted on a public API for the client to call. During the client build, the server functions will be compiled into the client build.

## Dependencies

Most server dependencies (like sqlx and tokio) will not compile on client platforms like WASM. To avoid building server dependencies on the client, you should add platform specific dependencies under the `server` feature in the [Cargo.toml](../Cargo.toml) file. More details about managing server only dependencies can be found in the [Dioxus guide](https://dioxuslabs.com/learn/0.6/guides/fullstack/managing_dependencies#adding-server-only-dependencies).

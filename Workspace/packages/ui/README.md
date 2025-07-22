# UI

This crate contains all shared components for the workspace. This is a great place to place any UI you would like to use in multiple platforms like a common `Button` or `Navbar` component.

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform{% if is_fullstack %}
│  ├─ echo.rs # The shared echo component that communicates with the server{% endif %}{% if is_router %}
│  ├─ navbar.rs # The Navbar component that will be used in the layout of every platform's router{% endif %}
```

## Dependencies

Since this crate is shared between multiple platforms, it should not pull in any platform specific dependencies. For example, if you want to use the `web_sys` crate in the web build of your app, you should not add it to this crate. Instead, you should add platform specific dependencies to the [web](../web/Cargo.toml), [desktop](../desktop/Cargo.toml), or [mobile](../mobile/Cargo.toml) crates.

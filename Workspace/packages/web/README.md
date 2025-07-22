# Development

The web crate defines the entrypoint for the web app along with any assets, components and dependencies that are specific to web builds. The web crate starts out something like this:

```
web/
├─ assets/ # Assets used by the web app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the web app.{% if is_router -%} It also defines the routes for the web platform
│  ├─ views/ # The views each route will render in the web version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route{%- endif %}
├─ Cargo.toml # The web crate's Cargo.toml - This should include all web specific dependencies
```

## Dependencies
{% if is_fullstack -%}
Since you have fullstack enabled, the web crate will be built two times:
1. Once for the server build with the `server` feature enabled
2. Once for the client build with the `web` feature enabled

You should make all web specific dependencies optional and only enabled in the `web` feature. This will ensure that the server builds don't pull in web specific dependencies which cuts down on build times significantly.
{%- else -%}
This crate will only be included in the web build, so you should add all web specific dependencies to this crate's [Cargo.toml](../Cargo.toml) file instead of the shared [ui](../ui/Cargo.toml) crate.
{%- endif %}

### Serving Your Web App

You can start your web app with the following command:

```bash
dx serve
```

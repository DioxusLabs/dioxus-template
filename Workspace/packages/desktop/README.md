# Development

The desktop crate defines the entrypoint for the desktop app along with any assets, components and dependencies that are specific to desktop builds. The desktop crate starts out something like this:

```
desktop/
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app.{% if is_router -%} It also defines the routes for the desktop platform
│  ├─ views/ # The views each route will render in the desktop version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route{%- endif %}
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

## Dependencies
{% if is_fullstack -%}
Since you have fullstack enabled, the desktop crate will be built two times:
1. Once for the server build with the `server` feature enabled
2. Once for the client build with the `desktop` feature enabled

You should make all desktop specific dependencies optional and only enabled in the `desktop` feature. This will ensure that the server builds don't pull in desktop specific dependencies which cuts down on build times significantly.
{%- else -%}
This crate will only be included in the desktop build, so you should add all desktop specific dependencies to this crate's [Cargo.toml](../Cargo.toml) file instead of the shared [ui](../ui/Cargo.toml) crate.
{%- endif %}

### Serving Your Desktop App

You can start your desktop app with the following command:

```bash
dx serve
```

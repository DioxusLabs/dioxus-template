# Development

{% if is_fullstack -%}Your new workspace contains a member crate for each of the web, desktop and mobile platforms, a `ui` crate for shared components and a `api` crate for shared backend logic:{%- else -%}
Your new workspace contains a member crate for each of the web, desktop and mobile platforms, and a `ui` crate for components that are shared between multiple platforms:{%- endif %}

```
your_project/
├─ README.md
├─ Cargo.toml
└─ packages/
   ├─ web/
   │  └─ ... # Web specific UI/logic
   ├─ desktop/
   │  └─ ... # Desktop specific UI/logic
   ├─ mobile/
   │  └─ ... # Mobile specific UI/logic{% if is_fullstack %}
   ├─ api/
   │  └─ ... # All shared server logic{% endif %}
   └─  ui/
      └─ ... # Component shared between multiple platforms
```

## Platform crates

Each platform crate contains the entry point for the platform, and any assets, components and dependencies that are specific to that platform. For example, the desktop crate in the workspace looks something like this:

```
desktop/ # The desktop crate contains all platform specific UI, logic and dependencies for the desktop app
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app.{% if is_router %} It also defines the routes for the desktop platform
│  ├─ views/ # The views each route will render in the desktop version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route{% endif %}
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

When you start developing with the workspace setup each of the platform crates will look almost identical. The UI starts out exactly the same on all platforms. However, as you continue developing your application, this setup makes it easy to let the views for each platform change independently.

## Shared UI crate

The workspace contains a `ui` crate with components that are shared between multiple platforms. You should put any UI elements you want to use in multiple platforms in this crate. You can also put some shared client side logic in this crate, but be careful to not pull in platform specific dependencies. The `ui` crate starts out something like this:

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform{% if is_fullstack %}
│  ├─ echo.rs # The shared echo component that communicates with the server{%- endif %}{% if is_router %}
│  ├─ navbar.rs # The Navbar component that will be used in the layout of every platform's router{% endif %}
```

{% if is_fullstack -%}
## Shared backend logic

The workspace contains a `api` crate with shared backend logic. This crate defines all of the shared server functions for all platforms. Server functions are async functions that expose a public API on the server. They can be called like a normal async function from the client. When you run `dx serve`, all of the server functions will be collected in the server build and hosted on a public API for the client to call. The `api` crate starts out something like this:

```
api/
├─ src/
│  ├─ lib.rs # Exports a server function that echos the input string
```
{%- endif %}

### Serving Your App

Navigate to the platform crate of your choice:
```bash
cd web
```

and serve:

```bash
dx serve
```

{% if is_mobile -%}
Mobile platforms are shared in a single crate. To serve mobile, you need to explicitly set your target device, `android` or `ios`:
```bash
dx serve --platform android
```
{%- endif %}

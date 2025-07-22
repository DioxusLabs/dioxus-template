# Development

The mobile crate defines the entrypoint for the mobile app along with any assets, components and dependencies that are specific to mobile builds. The mobile crate starts out something like this:

```
mobile/
├─ assets/ # Assets used by the mobile app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the mobile app.{% if is_router -%} It also defines the routes for the mobile platform
│  ├─ views/ # The views each route will render in the mobile version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route{%- endif %}
├─ Cargo.toml # The mobile crate's Cargo.toml - This should include all mobile specific dependencies
```

## Dependencies
{% if is_fullstack -%}
Since you have fullstack enabled, the mobile crate will be built two times:
1. Once for the server build with the `server` feature enabled
2. Once for the client build with the `mobile` feature enabled

You should make all mobile specific dependencies optional and only enabled in the `mobile` feature. This will ensure that the server builds don't pull in mobile specific dependencies which cuts down on build times significantly.
{%- else -%}
This crate will only be included in the mobile build, so you should add all mobile specific dependencies to this crate's [Cargo.toml](../Cargo.toml) file instead of the shared [ui](../ui/Cargo.toml) crate.
{%- endif %}

### Serving Your Mobile App

Mobile platforms are shared in a single crate. To serve mobile, you need to explicitly set your target device to `android` or `ios`:

```bash
dx serve --platform android
```
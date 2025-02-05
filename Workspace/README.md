# Development

Depending on your selected options, your new workspace project contains a workspace member for each platform.
If you chose to develop with the router feature, each platform crate will have a `views` folder for your platform-specific views.
You are provided with a `ui` crate for shared UI and if you chose to use fullstack, you will have a `server` crate for your shared server functions. 
Code gated with the `server` feature in the `server` crate will only be available on the actual server side and can be used e.g. for `axum` integration such as authentication middleware. This distinction is required because the declaration of [server functions](https://crates.io/crates/server_fn) also declares their respective clients.

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
# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # The entrypoint for the app.{% if is_router %} It also defines the routes for the app.{% endif %}
│  ├─ components/
│  │  ├─ mod.rs # Defines the components module
│  │  ├─ hero.rs # The Hero component for use in the home page{% if is_fullstack %}
│  │  ├─ echo.rs # The echo component uses server functions to communicate with the server{%- endif %}{% if is_router %}
│  ├─ views/ # The views each route will render in the app.
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route{% endif %}
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

{% if is_tailwind -%}
### Automatic Tailwind (Dioxus 0.7+)

As of Dioxus 0.7, there no longer is a need to manually install tailwind. Simply `dx serve` and you're good to go!

Automatic tailwind is supported by checking for a file called `tailwind.css` in your app's manifest directory (next to Cargo.toml). To customize the file, use the dioxus.toml:

```toml
[application]
tailwind_input = "my.css"
tailwind_output = "assets/out.css"
```

### Tailwind Manual Install

To use tailwind plugins or manually customize tailwind, you can install the Tailwind CLI and use it directly.

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation/tailwind-cli
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```
{%- endif %}

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

{% if is_fullstack -%}
```bash
dx serve --platform {{default_platform}}
```
{%- else -%}
```bash
dx serve
```
{%- endif %}

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

{% if is_mobile -%}
To serve on mobile, you need to explicitly set your target device, `android` or `ios`:
```bash
dx serve --platform android
```
{%- endif %}

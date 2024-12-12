# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

{% if is_tailwind -%}
### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
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
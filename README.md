# Development
{% if styling == "Tailwind" %}
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```
{% endif %}
{% if platform == "desktop" %}
Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload --platform desktop
```
{% else %}
{% if platform == "TUI" %}
Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload --platform desktop
```
{% else %}
{% if platform == "web" %}
Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080
{% else %}
{% if platform == "Fullstack" %}
Launch the Dioxus Fullstack app:

```bash
dx build --features web --release
cargo run --features ssr --release
```
{% else %}
Launch the Dioxus app:

```bash
cargo run
```
{% endif %}
{% endif %}
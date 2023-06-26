# Development
{% if styling == "Tailwind" %}
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind css compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```
{% endif %}
{% if platform == "web" %}
Run the following command in the root of the project to start the dioxus dev server:

```bash
dioxus serve --hot-reload
```

- Open the browser to http://localhost:8080
{% else %}
{% if platform == "Fullstack" %}
Launch the dioxus fullstack app:

```bash
dioxus build --features web
cargo run --features ssr
```
{% else %}
Launch the dioxus app:

```bash
cargo run
```
{% endif %}
{% endif %}
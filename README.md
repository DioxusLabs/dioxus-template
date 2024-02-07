# Overview

**This repo is not intended to be `git cloned`**.

This repo is used by `dx create` when starting new projects. So by running `dx create` you are effectively running this code.

# Development
{% if styling == "Tailwind" %}
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```sh
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```
{% endif %}
Launch the Dioxus app:

```sh
dx serve --hot-reload
```{%
  assign platform_array = "web,fullstack,liveview" | split: "," %}{%
  if platform_array contains platform %}

- Open the browser to http://localhost:8080{% endif %}

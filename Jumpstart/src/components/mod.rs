//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to define common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component {% if is_fullstack -%} and an Echo component for fullstack apps {%- endif %} to be used in our app.

mod hero;
pub use hero::Hero;

{% if is_fullstack -%}
mod echo;
pub use echo::Echo;
{%- endif %}

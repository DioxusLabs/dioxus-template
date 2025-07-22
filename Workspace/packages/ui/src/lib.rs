//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

{% if is_router -%}
mod navbar;
pub use navbar::Navbar;
{%- endif %}

{% if is_fullstack -%}
mod echo;
pub use echo::Echo;
{%- endif %}

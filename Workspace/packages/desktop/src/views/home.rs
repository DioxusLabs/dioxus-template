use dioxus::prelude::*;
{% if is_fullstack -%}
use ui::{Hero, Echo};
{%- else -%}
use ui::Hero;
{%- endif %}

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        {% if is_fullstack -%}
        Echo {}
        {%- endif %}
    }
}

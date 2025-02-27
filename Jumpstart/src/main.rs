use dioxus::prelude::*;

{% if is_router -%}
use components::Navbar;
{%- else -%}
{% if is_fullstack -%}
use components::{Hero, Echo};
{%- else -%}
use components::Hero;
{%- endif %}
{%- endif %}
{% if is_router -%}
use views::{Blog, Home};
{%- endif %}

mod components;
{% if is_router -%}
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
{%- endif %}

const FAVICON: Asset = asset!("/assets/favicon.ico");
{%- if is_tailwind -%}
const MAIN_CSS: Asset = asset!("/assets/main.css");
{%- else -%}
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
{%- endif %}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        {% if is_router -%}
        Router::<Route> {}
        {%- else -%}
        Hero {}
        {% if is_fullstack -%}
        Echo {}
        {%- endif %}
        {%- endif %}
    }
}

#![allow(dead_code)]
mod app;
mod components;
mod core;
mod pages;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| leptos::view! { <app::App/> })
}

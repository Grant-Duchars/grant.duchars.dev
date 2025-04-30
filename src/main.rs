#[allow(unused_must_use)]
mod app;
mod components;
mod pages;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| leptos::view! { <app::App/> })
}

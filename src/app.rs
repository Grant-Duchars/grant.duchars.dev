use crate::{components::TaskBar, pages::About};
use leptos::{ev, prelude::*};
use leptos_use::{use_event_listener, use_window};

#[component]
pub fn app() -> impl IntoView {
    let (window_center, set_window_center) = signal(calc_window_center());
    let _ = use_event_listener(use_window(), ev::resize, move |_| {
        set_window_center.set(calc_window_center());
    });
    provide_context(window_center);

    view! {
        <main>
            <About/>
        </main>
        <footer>
            <TaskBar/>
        </footer>
    }
}

fn calc_window_center() -> (f64, f64) {
    (get_window_width() / 2.0, get_window_height() / 2.0)
}

fn get_window_width() -> f64 {
    window()
        .inner_width()
        .expect("should have a value")
        .as_f64()
        .expect("should be a number")
}

fn get_window_height() -> f64 {
    window()
        .inner_height()
        .expect("should have a value")
        .as_f64()
        .expect("should be a number")
        - 46.0
}

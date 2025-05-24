use crate::{
    components::{Desktop, Taskbar},
    core::{DesktopItems, WindowData},
    pages::{InvalidPath, WindowsOpen},
};
use leptos::{ev, prelude::*};
use leptos_router::components::{Router, Routes};
use leptos_use::{use_event_listener, use_window};

#[component]
pub fn app() -> impl IntoView {
    let window_center = RwSignal::new(calc_window_center());
    let update_window_center = move |_| window_center.set(calc_window_center());
    let _ = use_event_listener(use_window(), ev::resize, update_window_center);
    provide_context(window_center.read_only());

    let desktop_items = RwSignal::new(DesktopItems::default());
    provide_context(desktop_items);

    let window_data = RwSignal::new(Vec::<WindowData>::new());
    provide_context(window_data.write_only());

    view! {
        <main>
            <Router>
                <Routes fallback=InvalidPath>
                    <WindowsOpen/>
                </Routes>
            </Router>
            <Desktop items=desktop_items/>
        </main>
        <footer>
            <Taskbar items=window_data.read_only()/>
        </footer>
    }
}

/// Calculates the browser window's viewport's center
fn calc_window_center() -> (f64, f64) {
    (get_window_width() / 2.0, get_window_height() / 2.0)
}

/// Gets the browser window's viewport's width
fn get_window_width() -> f64 {
    window()
        .inner_width()
        .expect("should have a value")
        .as_f64()
        .expect("should be a number")
}

/// Gets the browser window's viewport's height, subtracting the taskbar's height
fn get_window_height() -> f64 {
    window()
        .inner_height()
        .expect("should have a value")
        .as_f64()
        .expect("should be a number")
        - 46.0
}

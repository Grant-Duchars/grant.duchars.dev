use crate::pages::{InvalidPath, WindowsOpen};
use leptos::prelude::*;
use leptos_router::components::{Router, Routes};
use window_lib::components::{Desktop, Taskbar};
use window_lib::{browser, DesktopItems, Windows};

#[component]
pub fn app() -> impl IntoView {
    browser::setup_browser_center_listener();
    browser::setup_browser_dimensions_listener();
    // browser::setup_taskbar_styling();

    provide_context(DesktopItems::default());
    provide_context(Windows::default());

    view! {
        <main>
            <Desktop/>
            <Router>
                <Routes fallback=InvalidPath>
                    <WindowsOpen/>
                </Routes>
            </Router>
        </main>
        <footer>
            <Taskbar/>
        </footer>
    }
}

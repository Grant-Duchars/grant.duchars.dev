use crate::components::Window;
use crate::core::WindowData;
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn windows_open() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("") view=Outlet>
            <Route path=path!("") view=About/>
            <Route path=path!("/about") view=About/>
        </ParentRoute>
    }
    .into_inner()
}

#[component]
pub fn invalid_path() -> impl IntoView {
    let data = WindowData::new("public/close-window.svg", "Invalid Path").centered();
    view! {
        <Window data>
            <h1 style="text-align:center;vertical-align:middle;">"Oops"</h1>
            <p style="text-align:center;vertical-align:middle;">
                "It seems the path you were trying to reach doesn't exist."
            </p>
        </Window>
    }
}

#[component]
pub fn about() -> impl IntoView {
    let data = WindowData::new("public/about.svg", "About").centered();
    view! {
        <Window data>
            <h1>"Site Under Construction"</h1>
            <p>
                "Hey! Welcome to my website! I am still in the process of fleshing it out. Everything here is not final and subject to change."
            </p>
        </Window>
    }
}

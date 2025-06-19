use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Route};
use leptos_router::{path, MatchNestedRoutes};
use window_lib::components::Window;
use window_lib::{icons, WindowData};

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
    let data = WindowData::new(icons::CLOSE, "Invalid Path")
        .centered()
        .create_desktop_item(false);
    view! {
        <Window data>
            <h1 style="text-align:center;">"Oops"</h1>
            <p style="text-align:center;">
                "It seems the path you were trying to reach doesn't exist."
            </p>
        </Window>
    }
}

#[component]
pub fn about() -> impl IntoView {
    let data = WindowData::new(icons::ABOUT, "About").centered();
    view! {
        <Window data>
            <h1>"Yo! Merry Christmas!"</h1>
            <p>
                "Hi, I'm Grant Duchars, a recent computer science graduate from Lindenwood Univsersity.
                I was born in Pheonix, AZ and raised in St. Louis, MO. Some of my hobbies include playing 
                and theorizing about Magic: The Gathering, seeking out all the achievements in various 
                videogames, collecting soundtracks, and working on personal coding projects such as this one."
            </p>
        </Window>
    }
}

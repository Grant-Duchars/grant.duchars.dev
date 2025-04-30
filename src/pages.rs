use crate::components::Window;
use leptos::prelude::*;

#[component]
pub fn about() -> impl IntoView {
    let window_center = expect_context::<ReadSignal<(f64, f64)>>().read_untracked();
    let dimensions = (400.0, 250.0);
    let offset = (dimensions.0 / 2.0, dimensions.1 / 2.0);
    let position = (window_center.0 - offset.0, window_center.1 - offset.1);
    view! {
        <Window icon="public/about.svg" title="About" position dimensions>
            <h1>"Site Under Construction"</h1>
            <p>
                "Hey! Welcome to my website! I am still in the process of fleshing it out. Everything here is not final and subject to change."
            </p>
            <br/>
            <h2>"Links"</h2>
            <ul>
                <li>
                    <a href="https://github.com/Grant-Duchars">"Github"</a>
                </li>
                <li>
                    <a href="https://www.linkedin.com/in/grant-duchars/">"Linkedin"</a>
                </li>

            </ul>
        </Window>
    }
}

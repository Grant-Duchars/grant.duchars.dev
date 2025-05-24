use crate::core::WindowData;
use leptos::prelude::*;

#[component]
pub fn taskbar(items: ReadSignal<Vec<WindowData>>) -> impl IntoView {
    view! {
        <div id="task-bar">
            <For
                each=move || items.get()
                key=|item| item.title
                children=move |data| view! { <TaskbarItem data/> }
            />
        </div>
    }
}

#[component]
fn taskbar_item(data: WindowData) -> impl IntoView {
    view! {
        <Show when=move || data.is_open.get() fallback=|| ()>
            <button
                type="button"
                on:click=move |_| data.is_minimized.set(false)
                on:dblclick=move |_| data.reset_position()
            >
                <img src=data.icon/>
            </button>
        </Show>
    }
}

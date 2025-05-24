use crate::core::{DesktopItem, DesktopItemFunction, DesktopItems};
use leptos::prelude::*;

#[component]
pub fn desktop(items: RwSignal<DesktopItems>) -> impl IntoView {
    view! {
        <div id="desktop">
            <For
                each=move || items.get()
                key=|item| item.title
                children=move |item| view! { <DesktopItem item/> }
            />
        </div>
    }
}

#[component]
fn desktop_item(item: DesktopItem) -> impl IntoView {
    use DesktopItemFunction::*;
    match item.func {
        Window(is_open) => view! {
            <button type="button" on:dblclick=move |_| is_open.set(true)>
                <img src=item.icon/>
                <p>{item.title}</p>
            </button>
        }
        .into_any(),

        ExternalLink(link) => {
            let use_link = move |_| {
                let _ = location().set_href(link);
            };
            view! {
                <button type="button" on:dblclick=use_link>
                    <a href=link rel="external" on:click=|e| e.prevent_default()>
                        <img src=item.icon/>
                        <img src="public/external.svg"/>
                        <p>{item.title}</p>
                    </a>
                </button>
            }
        }
        .into_any(),
    }
}

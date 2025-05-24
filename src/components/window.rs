use crate::core::{DesktopItems, WindowData};
use leptos::prelude::*;

#[component]
pub fn window(data: WindowData, children: ChildrenFn) -> impl IntoView {
    let taskbar: WriteSignal<Vec<WindowData>> = expect_context();
    taskbar.update(|v| v.push(data));

    let WindowData {
        icon,
        title,
        node_ref,
        dimensions,
        position,
        is_open,
        is_minimized,
        desktop_item,
        ..
    } = data;

    if desktop_item {
        let desktop_items: RwSignal<DesktopItems> = expect_context();
        desktop_items.update(|v| {
            for item in v.iter() {
                if item.title == title {
                    return;
                }
            }
            v.push(data.into());
        })
    }

    let update_position = move || {
        let dimensions = dimensions;
        let position = *position.read();
        format!(
            "width:{}px;height:{}px;left:{}px;top:{}px;",
            dimensions.w, dimensions.h, position.x, position.y,
        )
    };

    view! {
        <Show when=move || is_open.get() && !is_minimized.get() fallback=|| ()>
            <div class="window" style=update_position>
                <div node_ref=node_ref class="window-bar">
                    <img src=icon/>
                    <p>{title}</p>
                    <button type="button" on:click=move |_| is_minimized.set(true)>
                        <img src="public/minimize-window.svg"/>
                    </button>
                    <button type="button">
                        <img src="public/maximize-window.svg"/>
                    </button>
                    <button type="button" on:click=move |_| is_open.set(false)>
                        <img src="public/close-window.svg"/>
                    </button>
                </div>
                <div class="window-content">{children()}</div>
            </div>
        </Show>
    }
}

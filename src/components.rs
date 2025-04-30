use leptos::{html, prelude::*};
use leptos_use::{
    core::Position, use_draggable_with_options, UseDraggableOptions, UseDraggableReturn,
};

#[component]
pub fn window(
    #[prop(default = "")] icon: &'static str,
    #[prop(default = "Window")] title: &'static str,
    #[prop(default = (0.0, 0.0))] position: (f64, f64),
    #[prop(default = (400.0, 250.0))] dimensions: (f64, f64),
    #[prop(default = true)] open: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let (x, y) = position;
    let (w, h) = dimensions;
    let window_bar = NodeRef::<html::Div>::new();
    let UseDraggableReturn { style, .. } = use_draggable_with_options(
        window_bar,
        UseDraggableOptions::default().initial_value(Position { x, y }),
    );
    let (open, set_open) = signal(open);
    view! {
        <Show when=move || open.get() fallback=|| ()>
            <div
                class="window"
                style=move || { format!("width: {}px; height: {}px; {}", w, h, style.get()) }
            >

                <div node_ref=window_bar class="window-bar">
                    <img src=icon/>
                    <p>{title}</p>
                    <button>
                        <img src="public/minimize-window.svg" alt="minimize window"/>
                    </button>
                    <button>
                        <img src="public/maximize-window.svg" alt="maximize window"/>
                    </button>
                    <button on:click=move |_| set_open.set(false)>
                        <img src="public/close-window.svg" alt="close window"/>
                    </button>
                </div>
                <div class="window-content">{children()}</div>
            </div>
        </Show>
    }
}

#[component]
pub fn task_bar() -> impl IntoView {
    view! { <div id="task-bar"></div> }
}

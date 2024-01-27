use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
#[allow(non_snake_case)]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(5);

    view! {
        <button
            on:click=move |_| set_count.update(|n| *n += 1)
            class=("red", move || count() % 2 == 1)
        >
            "Click me: "
            {count}
        </button>
    }
}

use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button
            style="display:block"
            on:click=move |_| {
                set_count.update(|x| *x += 1);
            }
        >
            "Click me: "
            {move || count()}
        </button>
        <button
            style="display:block"
            on:click=move |_| {
                set_count.update(|x| *x -= 1);
            }
        >
            "Click me: "
            {move || count()}
        </button>
    }
}
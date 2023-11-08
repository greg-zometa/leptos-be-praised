use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
        class=("red", move || count() > 0 && count() % 2 == 1)
        class=("blue", move || count() > 0 && count() % 2 == 0)
        on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>
          "Click me: "{count}
        </button>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}

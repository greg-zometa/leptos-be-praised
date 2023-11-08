use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
        on:click=move |_| {
            set_count.update(|n| *n += 1);
        }
        // the class: syntax reactively updates a single class
        // here, we'll set the `red` class when `count` is odd
        class:red=move || count() % 2 == 1
    >
        "Click me"
    </button>
            /* insert the rest of the view */
    <progress
    max="50"
    // we use it once here
    value=double_count
    />
    <p>
    "Double Count: "
    // and again here
    {double_count}
    </p>
        }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}

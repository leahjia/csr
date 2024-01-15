use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let double_count = move || count() * 2;
    let html = "<p>Some inner HTML stuff.</p>";

    view! {
        <div inner_html=html/>

        <button
            on:click=move |_| set_count.update(|n| *n += 1)
            class=("btn-blue", move || count() % 2 == 1)

            // style="position: absolute"
            // style:left=move || format!("{}px", x())
            // style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            // style:max-width="400px"
            // style=("--columns", x)

            >
            "Click"
            // {count}
        </button>

        <br/>

        <progress
            max="50"
            // same as `value=move || count.get()`
            value=count
        />

        <br/>

        <progress
            max="50"
            value=double_count
        />

        <p> "Single Count: " {count} </p>
        <p> "Double Count: " {double_count} </p>
    }
}

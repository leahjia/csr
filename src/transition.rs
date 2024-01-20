use leptos::*;
use gloo_timers::future::TimeoutFuture;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (id, set_id) = create_signal(0);

    let user_data = create_resource(id, |id| async move { api_call(id).await });

    view! {
        <div class="buttons">
            <button
                on:click=move |_| set_id(0)
                class:selected=move || id() == 0
            >
                "Tab A"
            </button>
            <button
                on:click=move |_| set_id(1)
                class:selected=move || id() == 1
            >
                "Tab B"
            </button>
            <button
                on:click=move |_| set_id(2)
                class:selected=move || id() == 2
            >
                "Tab C"
            </button>
            {move || if user_data.loading().get() {
                "Loading..."
            } else {
                ""
            }}
        </div>
        <Transition
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <p>
                {move || user_data.get()}
            </p>
        </Transition>
    }
}

async fn api_call(id: usize) -> String {
    TimeoutFuture::new(1_000).await;
    match id {
        0 => "Bob",
        1 => "Alice",
        2 => "Leah",
        _ => "User not found",
    }.to_string()
}


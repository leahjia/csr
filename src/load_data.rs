use leptos::*;
use gloo_timers::future::TimeoutFuture;

fn main() {
    leptos::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let async_data = create_resource(
        count,
        |value| async move { load_data(value).await },
    );

    let load_once = create_resource(|| (), |_| async move {
        load_data(1).await
    });

    let async_res = move || {
        async_data
            .get()
            .map(|val| format!("Server returned {val:?}"))
            .unwrap_or_else(|| "Loading...".into())
    };

    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Waiting for request." };

    view! {
        <button on:click=move |_| set_count.update(|n| *n += 2)>
            "Double it"
        </button>
        <p>
            <code>"load_once"</code>": " {move || load_once.get()}
        </p>
        <p>
            <code>"count"</code>": " {count}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_res}
            <br/>
            {is_loading}
        </p>
    }
}

async fn load_data(value: i32) -> i32 {
    TimeoutFuture::new(1_000).await;
    value * 10
}


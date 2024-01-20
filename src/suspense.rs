use leptos::*;
use gloo_timers::future::TimeoutFuture;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal("Leah".to_string());

    let async_data = create_resource(name, |str| async move { api_call(str).await });

    view! {
        <input
            on:input=move |ev| set_name(event_target_value(&ev))
            prop:value=name
        />
        <p><code>"name:"</code>{name}</p>

        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <p>"Your cap name is " {move || async_data.get()}</p>
        </Suspense>
    }
}

async fn api_call(str: String) -> String {
    TimeoutFuture::new(1_000).await;
    str.to_ascii_uppercase()
}

use leptos::*;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    let msg = move || is_odd().then(|| "Some odd message");
    let message = move || {
        match value() {
            0 => "Zero",
            1 => "One",
            n if is_odd() => "Odd",
            _ => "Even"
        }
    };

    view! {
        <main>
            <h2>"Control Flow"</h2>
            <button on:click=move |_| set_value.update(|n| *n += 1)>
                "+1"
            </button>
            <p>{msg}</p>
            <p>{message}</p>

            <h2><code>"Option<T>"</code></h2>

            {move || match is_odd() {
                true if value() == 1 => {
                    view! { <pre>"One"</pre> }.into_any()
                },
                false if value() == 2 => {
                    view! { <p>"Two"</p> }.into_any()
                }
                _ => view! { <textarea>{value()}</textarea>}.into_any()
            }}
        </main>

    }
}


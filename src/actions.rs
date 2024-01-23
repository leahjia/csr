use leptos::{*, html::Input};
use gloo_timers::future::TimeoutFuture;
use uuid::Uuid;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let add_todo = create_action(|input: &String| {
        let input = input.to_owned();
        async move { add_request(&input).await }
    });

    let submit = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();
    let input_ref = create_node_ref::<Input>();

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                add_todo.dispatch(input.value());
            }
        >
            <label>
                "What do you wanna do?"
                <input type="text" node_ref=input_ref />
            </label>
            <button type="submit">"Add todo"</button>
        </form>
        <p>{move || pending().then(|| "Loading...")}</p>
        <p>
            "Submitted: "
            <code>{move || format!("{:#?}", submit())}</code>
        </p>
        <p>
            "Pending: "
            <code>{move || format!("{:#?}", pending())}</code>
        </p>
        <p>
            "Todo ID: "
            <code>{move || format!("{:#?}", todo_id())}</code>
        </p>
    }
}

async fn add_request(text: &str) -> Uuid {
    _ = text;
    TimeoutFuture::new(1000).await;
    Uuid::new_v4()
}


use leptos::*;

fn main() {
    mount_to_body(App)
}

#[derive(Copy, Clone)]
struct LogContext(RwSignal<Vec<String>>);

fn log(msg: impl std::fmt::Display) {
    let log = use_context::<LogContext>().unwrap().0;
    log.update(|log| log.push(msg.to_string()));
}

#[component]
fn App() -> impl IntoView {
    let log = create_rw_signal(vec![]);
    let logged = move || log().join("\n");

    // some non-required logging
    provide_context(LogContext(log));

    view! {
        <CreateEffect/>
        <pre>{logged}</pre>
    }
}

#[component]
fn CreateEffect() -> impl IntoView {
    let (firsty, set_firsty) = create_signal(String::new());
    let (lasty, set_lasty) = create_signal(String::new());
    let (use_last, set_use_last) = create_signal(true);

    create_effect(move |_| {
        log(if use_last() {
            with!(|firsty, lasty| format!("{firsty} {lasty}"))
        } else {
            firsty()
        })
    });

    view! {
        <h1>
            <code>"create_effect"</code>
        </h1>
        <form>
            <label>
                "First Name"
                <input
                    input="text"
                    name="first"
                    prop:value=firsty
                    on:change=move |ev| set_firsty(event_target_value(&ev))
                /> 
            </label>
            <label>
                "Last Name"
                <input
                    input="text"
                    name="last"
                    prop:value=lasty
                    on:change=move |ev| set_lasty(event_target_value(&ev))
                /> 
            </label>
            <label>
                "Show Last Name"
                <input
                    type="checkbox"
                    name="use_last"
                    prop:checked=use_last
                    on:change=move |ev| set_use_last(event_target_checked(&ev))
                /> 
            </label>
        </form>
    }
}


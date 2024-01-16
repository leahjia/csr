use leptos::*;
use web_sys::MouseEvent;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggle? " {toggled}</p>
        <ButtonA setter=set_toggled/>
        <ButtonB on_click=move |_| set_toggled.update(|val| *val = !*val)/>
    }
}

#[component]
pub fn ButtonB(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <button on:click=on_click>"Toggle B"</button>
    }
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|val| *val = !*val)
        >
            "Toggle A"
        </button>
    }
}


use leptos::*;
use web_sys::MouseEvent;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (blue, set_blue) = create_signal(false);
    let (right, set_right) = create_signal(false);
    let (italics, set_italics) = create_signal(false);
    let (lowercase, set_lowercase) = create_signal(false);

    provide_context(set_lowercase);

    view! {
        <p
            class:blue=blue
            class:right=right
            class:italics=italics
            class:lowercase=lowercase
            >"Whatever the message is"</p>
        <ButtonA setter=set_blue/>
        <ButtonB on_click=move |_| set_right.update(|val| *val = !*val)/>
        <ButtonC on:click=move |_| set_italics.update(|val| *val = !*val)/>

        <Layout/>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <header>
            <h2>"through a layout"</h2>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD/>
        </div>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    // search up the context tree for this signal
    let setter = use_context::<WriteSignal<bool>>().expect("to have setter provided");

    view! {
        <button on:click=move |_| setter.update(|val| *val = !*val)>
            "Toggle D: lowercase"
        </button>
    }
}

#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>"Toggle C: italics"</button>
    }
}

#[component]
pub fn ButtonB<F>(on_click: F) -> impl IntoView 
where
    F: Fn(MouseEvent) + 'static
{
    view! {
        <button on:click=on_click>"Toggle B: go right"</button>
    }
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|val| *val = !*val)
        >
            "Toggle A: blue"
        </button>
    }
}


use leptos::*;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1><code>"<TakesChildren/>"</code></h1>
        <TakesChildren render_prop=|| view! { <p>"yo dawg"</p> }>
            "Some text"
        </TakesChildren>

        <h1><code>"<WrapsChildren/>"</code></h1>
        <WrapsChildren>"A""B""C"</WrapsChildren>
    }
}

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();

    view! {
        <ul>{children}</ul>
    }
}

#[component]
pub fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView
{
    view! {
        <h2>"Render Prop"</h2>
        {render_prop()}

        <h2>"Children"</h2>
        {children()}
    }
}


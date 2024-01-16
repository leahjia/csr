use leptos::{*, html::Input};
use web_sys::SubmitEvent;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (input, set_input) = create_signal("Enter some text...".to_string());

    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        set_name(value);
    };

    let (value, set_value) = create_signal("B".to_string());

    view! {
        // controlled
        <input type="text"
            on:input=move |ev| {
                set_input(event_target_value(&ev));
            }
            prop:value=input
        />
        <p>{input}</p>

        // uncontrolled
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            // value not prop:value
            <input type="submit" value="Submit"/>
        </form>
        <p>{name}</p>

        <select on:change=move |ev| {
            let new_val = event_target_value(&ev);
            set_value(new_val);
        }>
            <SelectOption value is="A"/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>
        </select>
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option
            value=is
            selected=move || value() == is
        >
            {is}
        </option>
    }
}


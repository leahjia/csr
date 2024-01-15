use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "golden".to_string(),
            value: 60
        },
        DatabaseEntry {
            key: "retriever".to_string(),
            value: 160
        }
    ]);

    view! {
        // clickers
        <button
            on:click=move |_| set_count.update(|n| *n += 1)
            class=("btn-blue", move || count() % 2 == 1)
            >
            "Click"
        </button>

        <br/>
        <ProgressBar progress=count/>
        <br/>
        <ProgressBar progress=Signal::derive(double_count)/>

        <p> "Single Count: " {count} </p>
        <p> "Double Count: " {double_count} </p>


        // iter
        <br/>
        // <h3>"Static List"</h3>
        // <StaticList length=5/>
        // <h3>"Dynamic List"</h3>
        // <DynamicList length=5/>


        // iter - complex
        <button on:click=move |_| set_data.update(|data| {
            for row in data {
                row.value *= 2;
            }
        })>
            "Update Values"
        </button>
        // display data
        <For
            each=data
            key=|state| (state.key.clone(), state.value)
            // same as children=|child| view! { <p>{child.value}</p> }
            let:child
        >
            <p>{child.key}": "{child.value}</p>
        </For>
    }
}

#[component]
fn DynamicList(length: usize) -> impl IntoView {
    let mut next_counter = length;
    let init_counters = (0..length).map(|id| (id, create_signal(id + 1))).collect::<Vec<_>>();
    let (counters, set_counters) = create_signal(init_counters);
    let add_counter = move |_| {
        // signal for new counter
        let signal = create_signal(next_counter + 1);
        // add new counter to counters
        set_counters.update(move |counters| counters.push((next_counter, signal)));
        next_counter += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
               "Add Counter"
            </button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click=move |_| set_count.update(|n| *n += 1)>
                                    {count}
                                </button>
                                <button on:click=move |_| {
                                    set_counters.update(|counters| {
                                        counters.retain(|(counter_id, _)| counter_id != &id)
                                    });
                                }>
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn StaticList(length: usize) -> impl IntoView {
    let counters = (1..=length).map(|idx| create_signal(idx));
    let counter_btns = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>
                    {count}
                </button>
                    </li>
            }
        })
    .collect_view();
    view! {
        <ul>{counter_btns}</ul>
    }
}

#[component]
fn ProgressBar(
    // max val of the progress bar
    #[prop(default = 100)] max: u16,
    // curr progress to display
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

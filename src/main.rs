use leptos::*;

/// Shows progress towards the max.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: MaybeSignal<i32>,
    // #[prop(optional)] progress: Option<Box<dyn Fn() -> i32>>
) -> impl IntoView {
    view! {
        <progress
            max=max
            // hmm... where will we get this from?
            value=progress
        />
        <br />
    }
}

#[component]
fn App() -> impl IntoView {
    // Checkout https://docs.rs/leptos/latest/leptos/struct.ReadSignal.html and https://docs.rs/leptos/latest/leptos/struct.WriteSignal.html
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
            class:red=move || count() % 2 == 1
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {count}
        </button>
        <br />
        <ProgressBar max=50 progress=count/>
        <ProgressBar progress=count/>
        <ProgressBar max=50 progress=Signal::derive(double_count)/>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

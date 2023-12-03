use leptos::*;
mod basics;
mod forms;

fn main() {
    leptos::mount_to_body(|| {
        view! {
            <basics::App/>
            <br />
            <forms::App />
        }
    })
}

use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <div class="flex w-screen h-screen bg-black">
                <p class="text-white">"Hello, world!"</p>
            </div>
        }
    })
}

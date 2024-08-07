mod app;
mod emulator;

use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || view! {
        <p>"Hello, world!"</p>
        <app::App />
        <emulator::Emulator />
    })
}
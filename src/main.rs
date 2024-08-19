use leptos::*;

mod emulator;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || view! { <emulator::Emulator /> })
}

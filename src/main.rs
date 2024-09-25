use leptos::mount::mount_to_body;

pub mod emulator;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(emulator::Emulator);
}

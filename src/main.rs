use emu_lib::cpu::z80::Z80;
use leptos::mount_to_body;

pub mod emulator;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || emulator::emulator::<Z80>());
}

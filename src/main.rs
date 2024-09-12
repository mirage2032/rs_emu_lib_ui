use leptos::*;
use emu_lib::cpu::z80::Z80;

mod emulator;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || emulator::emulator::<Z80>());
}

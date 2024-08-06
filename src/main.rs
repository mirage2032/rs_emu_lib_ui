mod app;
mod editor;

use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    let mut emulator = emu_lib::emulator::Emulator::new(emu_lib::cpu::CPUType::Z80);
    let rom_data = include_bytes!("../deps/rs_emu_lib/emu_cli/roms/rom.z80.bin");
    let test = "AAAAAAAABBBBBBBBCCCCCCCCDDDDDDDDEEEEEEEEFFFFFFFFGGGGGGGGHHHHHHHH".as_bytes();
    emulator.memory.load(test).unwrap();
    let (emu_read, emu_write) = create_signal(emulator);
    mount_to_body(move || view! {
        <p>"Hello, world!"</p>
        <app::App />
        <editor::Editor emu_read emu_write />
    })
}
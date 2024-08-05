mod app;
mod editor;

use std::path::PathBuf;
use emu_lib::memory::MemoryDevice;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    let mut emulator = emu_lib::emulator::Emulator::new(emu_lib::cpu::CPUType::Z80);
    let rom_data = include_bytes!("../deps/rs_emu_lib/emu_cli/roms/rom.z80.bin");
    println!("Loading rom");
    emulator.memory.lock().unwrap().load(rom_data).expect("Error loading rom");
    let (mem_in,mem_out) = create_signal(emulator.memory.clone());

    mount_to_body(move || view! {
        <p>"Hello, world!"</p>
        <app::App />
        <editor::Editor changes_in=mem_in changes_out=mem_out />
    })
}
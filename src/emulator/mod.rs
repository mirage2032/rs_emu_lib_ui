use leptos::{component, create_signal, IntoView, view};

pub mod memory;
mod disasm;
mod registers;

#[component]
pub fn Emulator() -> impl IntoView {
    let mut emulator = emu_lib::emulator::Emulator::new(emu_lib::cpu::CPUType::Z80);
    let rom_data = include_bytes!("../../deps/rs_emu_lib/emu_cli/roms/rom.z80.bin");
    // let test = "AAAAAAAABBBBBBBBCCCCCCCCDDDDDDDDEEEEEEEEFFFFFFFFGGGGGGGGHHHHHHHHIIIIIIII".to_string();//.repeat(5);
    // emulator.memory.load(test.as_bytes()).unwrap();
    emulator.memory.load(rom_data).unwrap();
    let (emu_read, emu_write) = create_signal(emulator);
    view! {
        <memory::MemEditor emu_read emu_write width=0x10 rows=10 />
        <disasm::Disassembler rows=10 emu_read emu_write />
        <registers::Registers emu_read emu_write />
    }
}


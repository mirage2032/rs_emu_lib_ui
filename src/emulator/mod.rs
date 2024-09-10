use emu_lib::cpu::z80::Z80;
use leptos::{component, create_node_ref, create_signal, view, IntoView, SignalGet};
use leptos::html::Div;
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};
use leptos_use::core::Position;
use stylance::import_style;

mod control;
mod disasm;
pub mod memory;
mod registers;

import_style!(style, "table.module.scss");
#[component]
pub fn Emulator() -> impl IntoView {
    let mut emulator: emu_lib::emulator::Emulator<Z80> = emu_lib::emulator::Emulator::new();
    let rom_data = include_bytes!("../../deps/rs_emu_lib/emu_cli/roms/color.bin");
    // let test = "AAAAAAAABBBBBBBBCCCCCCCCDDDDDDDDEEEEEEEEFFFFFFFFGGGGGGGGHHHHHHHHIIIIIIII".to_string();//.repeat(5);
    // emulator.memory.load(test.as_bytes()).unwrap();
    emulator.memory.load(rom_data).unwrap();
    let (emu_read, emu_write) = create_signal(emulator);
    let el = create_node_ref::<Div>();

    // `style` is a helper string "left: {x}px; top: {y}px;"
    let UseDraggableReturn {
        x,
        y,
        style,
        ..
    } = use_draggable_with_options(
        el,
        UseDraggableOptions::default().initial_value(Position { x: 0.0, y: 0.0 }),
    );
    view! {
        // <div node_ref=el
        // style=move || format!("position:fixed;height:7rem;width:7rem;background-color:green; {}",style.get())
        // ></div>
        <div
        style:width="32rem">
            <memory::MemEditor emu_read emu_write width=0x10 rows=10 />
            <disasm::Disassembler rows=10 emu_read emu_write />
            <registers::Registers emu_read emu_write />
            <control::Control emu_read emu_write />
        </div>
    }
}

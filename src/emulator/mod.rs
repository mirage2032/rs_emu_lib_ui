use crate::emulator::display::gen_dsp;
use emu_lib::cpu::z80::Z80;
use emu_lib::cpu::Cpu;
use emu_lib::memory::memdevices::RAM;
use emu_lib::memory::Memory;
use leptos::html::{Canvas, Div};
use leptos::{
    component, create_node_ref, create_signal, view, HtmlElement, IntoView, Signal, SignalGet,
    SignalWith,
};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};
use stylance::import_style;

mod control;
mod disasm;
mod display;
pub mod memory;
mod registers;

import_style!(style, "table.module.scss");
// #[component]
pub fn emulator<T: Cpu + 'static>() -> impl IntoView {
    let res = (256, 192);
    let refresh_rate = 50.08;
    let (dsp, dsp_view) = gen_dsp(res.0 * res.1, res.0 as usize, 2.0);
    let mut memory = Memory::new();
    memory.add_device(Box::new(RAM::new(0x1000)));
    memory.add_device(Box::new(dsp));
    memory.add_device(Box::new(RAM::new(
        0x10000 - res.0 as usize * res.1 as usize - 0x1000,
    )));

    let mut emulator: emu_lib::emulator::Emulator<T> =
        emu_lib::emulator::Emulator::new_w_mem(memory);
    let rom_data = include_bytes!("../../deps/rs_emu_lib/emu_cli/roms/color2.bin");
    // let test = "AAAAAAAABBBBBBBBCCCCCCCCDDDDDDDDEEEEEEEEFFFFFFFFGGGGGGGGHHHHHHHHIIIIIIII".to_string();//.repeat(5);
    // emulator.memory.load(test.as_bytes()).unwrap();
    emulator.memory.load(rom_data).unwrap();
    let (emu_read, emu_write) = create_signal(emulator);
    let dsp_update = Signal::derive(move || emu_read.with(|_| ()));
    // let display_view = dsp.display(dsp_update);
    let el = create_node_ref::<Div>();

    // `style` is a helper string "left: {x}px; top: {y}px;"
    let UseDraggableReturn { x, y, style, .. } = use_draggable_with_options(
        el,
        UseDraggableOptions::default().initial_value(Position { x: 0.0, y: 0.0 }),
    );
    view! {
        <div style:width="38rem">
            <memory::MemEditor emu_read emu_write width=0x10 rows=10 />
            <disasm::Disassembler rows=10 emu_read emu_write />
            <registers::Registers emu_read emu_write />
            <control::Control emu_read emu_write />
            <div>{dsp_view(dsp_update)}</div>
        </div>
    }
}

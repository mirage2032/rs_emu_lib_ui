use crate::emulator::display::gen_dsp;
use emu_lib::cpu::z80::Z80;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use emu_lib::memory::memdevices::RAM;
use emu_lib::memory::Memory;
use leptos::{component, create_signal, island, provide_context, view, IntoView, ReadSignal, Signal, SignalWith, WriteSignal};
use stylance::import_style;

pub mod control;
pub mod disasm;
pub mod display;
pub mod memory;
pub mod registers;
import_style!(
    #[allow(dead_code)]
    style,
    "table.module.scss"
);

#[derive(Clone)]
pub struct EmuSignals{
    pub read: ReadSignal<Emulator<Z80>>,
    pub write: WriteSignal<Emulator<Z80>>
}
// #[component]
// pub fn emu_z80() -> impl IntoView {
//     emulator::<Z80>()
// }

// #[component]
// pub fn emu_i8080() -> impl IntoView {
//     emulator::<emu_lib::cpu::i8080::I8080>()
// }

pub fn emu_with(
    emu_read: ReadSignal<emu_lib::emulator::Emulator<Z80>>,
    emu_write: WriteSignal<emu_lib::emulator::Emulator<Z80>>,
) -> impl IntoView {
    view! {
        <div style:width="38rem">
            <memory::MemEditor width=0x10 rows=10 />
            <disasm::Disassembler rows=10 />
            <registers::z80::Registers />
            <control::Control />
        </div>
    }
}

#[island]
pub fn emulator() -> impl IntoView {
    let res = (256, 192);
    // let refresh_rate = 50.08;
    let (dsp, dsp_view) = gen_dsp(res.0 * res.1, res.0 as usize, 2.0);
    let mut memory = Memory::new();
    memory.add_device(Box::new(RAM::new(0x1000)));
    memory.add_device(Box::new(dsp));
    memory.add_device(Box::new(RAM::new(
        0x10000 - res.0 as usize * res.1 as usize - 0x1000,
    )));

    let mut emulator: emu_lib::emulator::Emulator<Z80> =
        emu_lib::emulator::Emulator::new_w_mem(memory);
    let rom_data = include_bytes!("../../color2.bin");
    // let test = "AAAAAAAABBBBBBBBCCCCCCCCDDDDDDDDEEEEEEEEFFFFFFFFGGGGGGGGHHHHHHHHIIIIIIII".to_string();//.repeat(5);
    // emulator.memory.load(test.as_bytes()).unwrap();
    emulator.memory.load(rom_data).unwrap();
    let (emu_read, emu_write) = create_signal(emulator);
    let dsp_update = Signal::derive(move || emu_read.with(|_| ()));
    // let el = create_node_ref::<Div>();

    // `style` is a helper string "left: {x}px; top: {y}px;"
    // let UseDraggableReturn { x, y, style, .. } = use_draggable_with_options(
    //     el,
    //     UseDraggableOptions::default().initial_value(Position { x: 0.0, y: 0.0 }),
    // );
    provide_context(EmuSignals{read:emu_read, write:emu_write});
    view! {
        <div class=style::maincontainer style:width="38rem">
            <memory::MemEditor width=0x10 rows=10 />
            <disasm::Disassembler rows=10 />
            <registers::z80::Registers/>
            <control::Control />
            <div>{dsp_view(dsp_update)}</div>
        </div>
    }
}

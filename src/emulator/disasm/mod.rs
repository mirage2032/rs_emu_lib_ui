use emu_lib::emulator::Emulator;
use leptos::{component, IntoView, ReadSignal, view, WriteSignal};
use stylance::import_style;

import_style!(style, "disasm.module.scss");

#[component]
pub fn DisasmTr(
    pos: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    view! {}
}

pub fn DisasmThead() -> impl IntoView {
    view! {
        <thead>
            <tr>
                <th class=style::tableleft>
                    // <input disabled value="" style="width: 6.5ch" />
                </th>
                <th class=style::tabletop>
                    <span >"Addddr"</span>
                </th>
                <th class=style::tabletop>
                    <span >"Op"</span>
                </th>
                <th class=style::tabletop>
                    <span>"Arg"</span>
                </th>
            </tr>
        </thead>
    }
}

#[component]
pub fn Disassembler(
    pos: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    view! {
        <table
        class=style::memtable
        >
            <DisasmThead />
            <DisasmTr pos emu_read emu_write />
        </table>
    }
}

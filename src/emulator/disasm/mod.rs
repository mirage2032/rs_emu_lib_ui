use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::{CollectView, component, IntoView, ReadSignal, SignalWith, view, WriteSignal};
use stylance::import_style;

import_style!(style, "../table.module.scss");

pub fn DisasmThead() -> impl IntoView {
    view! {
        <thead>
            <tr>
                <th class=style::tabletop>
                    <span>"Address"</span>
                </th>
                <th class=style::tabletop>
                    <span>"Hex"</span>
                </th>
                <th class=style::tabletop>
                    <span>"Asm"</span>
                </th>
            </tr>
        </thead>
    }
}

#[component]
pub fn DisasmTr(
    address: u16,
    instruction: Result<Box<dyn emu_lib::cpu::instruction::BaseInstruction>, String>,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    view! {
        <tr>
            <td class=style::tableleft>
                <span>{format!("{:04X}", address)}</span>
            </td>
            {match instruction {
                Ok(ins) => {
                    let ins_hexstr = ins
                        .to_bytes()
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<String>();
                    view! {
                        <td class=style::tableleft>
                            <span>{ins_hexstr}</span>
                        </td>
                        <td class=style::tablecell>
                            <span>{ins.to_string()}</span>
                        </td>
                    }
                }
                Err(_) => {
                    view! {
                        <td class=style::tableleft>
                            <span>
                                {emu_read
                                    .with(|emu| { emu.memory.read_8(address) })
                                    .map(|b| format!("{:02X}", b))
                                    .unwrap_or_else(|_| "??".to_string())}
                            </span>
                        </td>
                        <td class=style::tablecell>
                            <span>"N/A"</span>
                        </td>
                    }
                }
            }}
        </tr>
    }
}

#[component]
pub fn DisasmTbody(
    rows: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    view! {
        <tbody>
            {
                let rows = move || {
                    let mut pc = 0;
                    (0..rows)
                        .map(|_| {
                            let instruction = {
                                emu_read
                                    .with(|emu| {
                                        emu.cpu.parser().ins_from_mem(&emu.memory, pc as u16)
                                    })
                            };
                            let size = match &instruction {
                                Ok(ins) => ins.common().get_length() as usize,
                                Err(e) => 1,
                            };
                            pc += size;
                            view! {
                                <DisasmTr
                                    address=(pc - size) as u16
                                    instruction
                                    emu_read
                                    emu_write
                                />
                            }
                        })
                        .collect::<Vec<_>>()
                };
                rows.into_view()
            }
        </tbody>
    }
}

#[component]
pub fn Disassembler(
    rows: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    view! {
        <table class=style::table>
            <DisasmThead />
            <DisasmTbody rows emu_read emu_write />
        </table>
    }
}

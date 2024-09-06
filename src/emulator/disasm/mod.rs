use emu_lib::cpu::z80::Z80;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::logging::log;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use stylance::{classes, import_style};

import_style!(style, "../table.module.scss");

#[component]
pub fn FollowPCSwitch<T:Cpu+Default+'static>(
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
    start_pos_read: ReadSignal<Option<u16>>,
    start_pos_write: WriteSignal<Option<u16>>,
) -> impl IntoView {
    let elem_class = move || match start_pos_read.get() {
        Some(_) => style::tablebutton,
        None => style::tablebuttoninvert,
    };
    view! {
        <tr>
            <td colspan=3>
                <div style:display="flex">
                    <button
                        class=elem_class
                        style:width="100%"
                        style:overflow="hidden"
                        on:click=move |_| {
                            start_pos_write
                                .update(|v| {
                                    if v.is_none() {
                                        *v = Some(emu_read.with(|emu| emu.cpu.registers().pc))
                                    } else {
                                        *v = None
                                    }
                                });
                        }
                    >
                        "Follow PC"
                    </button>
                    <Show when=move || start_pos_read.get().is_some()>
                        <input
                            class=style::tablecount
                            style:outline="none"
                            style:border="none"
                            on:change=move |event| {
                                event
                                    .target()
                                    .map(|target| {
                                        let element = target
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .unwrap();
                                        let result = u16::from_str_radix(&element.value(), 16);
                                        match result {
                                            Ok(val) => {
                                                start_pos_write.set(Some(val));
                                                element.set_value(&format!("{:04X}", val));
                                            }
                                            Err(_) => {
                                                log!("Invalid hex value");
                                                element
                                                    .set_value(&format!("{:04X}", start_pos_read().unwrap()));
                                            }
                                        }
                                    });
                            }
                            style:width="5ch"
                            maxlength=4
                            value=move || format!("{:04X}", start_pos_read().unwrap())
                        />
                    </Show>
                </div>
            </td>
        </tr>
    }
}

#[component]
pub fn DisasmThead() -> impl IntoView {
    view! {
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
    }
}

#[component]
pub fn DisasmTr<T:Cpu+Default+'static>(
    address: u16,
    instruction: Result<Box<dyn emu_lib::cpu::instruction::BaseInstruction>, String>,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    let elem_class = move || match emu_read.with(|emu| emu.cpu.registers().pc == address) {
        true => classes! {
            style::colorfocus,
            style::tableleft
        },
        false => style::tableleft.to_string(),
    };
    view! {
        <tr>
            <td class=elem_class>
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
                        <td class=elem_class>
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
pub fn DisasmTbody<T:Cpu+Default+'static>(
    rows: usize,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
    start_pos_read: ReadSignal<Option<u16>>,
) -> impl IntoView {
    view! {
        <tbody>
            {
                let rows = move || {
                    let mut pc = start_pos_read
                        .with(|val| val.unwrap_or(emu_read.with(|emu| emu.cpu.registers().pc)))
                        as usize;
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
pub fn Disassembler<T:Cpu+Default+'static>(
    rows: usize,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    let (start_pos_read, start_pos_write) = create_signal(None);
    view! {
        <table class=style::table>
            <thead>
                <FollowPCSwitch emu_read emu_write start_pos_read start_pos_write />
                <DisasmThead />
            </thead>
            <DisasmTbody rows emu_read emu_write start_pos_read />
        </table>
    }
}

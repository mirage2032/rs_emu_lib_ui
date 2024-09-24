use super::{style, EmuSignals};
use emu_lib::cpu::Cpu;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::logging::log;
use leptos::*;
use stylance::classes;
use web_sys::wasm_bindgen::JsCast;

#[island]
pub fn FollowPCSwitch(
) -> impl IntoView {
    let emu_signals = expect_context::<EmuSignals>();
    let start_pos_signals = expect_context::<StartPosSignals>();
    let elem_class = move || match start_pos_signals.read.get() {
        Some(_) => style::tablebutton,
        None => style::tablebuttoninvert,
    };
    view! {
        <tr>
            <td colspan=4>
                <div style:display="flex">
                    <button
                        class=elem_class
                        style:width="100%"
                        style:overflow="hidden"
                        on:click=move |_| {
                            start_pos_signals
                                .write
                                .update(|v| {
                                    if v.is_none() {
                                        *v = Some(
                                            emu_signals.read.with(|emu| *emu.cpu.registers().pc),
                                        )
                                    } else {
                                        *v = None
                                    }
                                });
                        }
                    >
                        "Follow PC"
                    </button>
                    <Show when=move || start_pos_signals.read.get().is_some()>
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
                                                start_pos_signals.write.set(Some(val));
                                                element.set_value(&format!("{:04X}", val));
                                            }
                                            Err(_) => {
                                                log!("Invalid hex value");
                                                element
                                                    .set_value(
                                                        &format!("{:04X}", start_pos_signals.read.get().unwrap()),
                                                    );
                                            }
                                        }
                                    });
                            }
                            style:width="5ch"
                            maxlength=4
                            prop:value=move || {
                                format!("{:04X}", start_pos_signals.read.get().unwrap())
                            }
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
                <span>"Bk"</span>
            </th>
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

#[island]
pub fn DisasmTr(
    address: u16,
    // instruction: Result<Box<dyn emu_lib::cpu::instruction::ExecutableInstruction<Z80>>, String>,
    instruction: Option<(String,String)>
) -> impl IntoView {
    let emu_signals = expect_context::<EmuSignals>();
    let class_is_bk = move || emu_signals.read.with(|emu| emu.breakpoints.contains(&address));
    let switch_bk = move |_| {
        emu_signals.write.update(|emu| {
            if emu.breakpoints.contains(&address) {
                emu.breakpoints.retain(|&x| x != address);
            } else {
                emu.breakpoints.push(address);
            }
        });
    };
    let class_is_pc = move || match emu_signals.read.with(|emu| *emu.cpu.registers().pc == address) {
        true => classes! {
            style::colorfocus,
            style::tableleft
        },
        false => style::tableleft.to_string(),
    };
    view! {
        <tr>
            <td class=class_is_pc on:click=switch_bk>
                <Show when=class_is_bk>
                    <div style:display="flex" style:justify-content="center">
                        <div class=style::breakpoint></div>
                    </div>
                </Show>
            </td>
            <td class=class_is_pc>
                <span>{format!("{:04X}", address)}</span>
            </td>
            {match instruction {
                Some((bytes, asm)) => {
                    view! {
                        <td class=style::tablecell style:text-align="left">
                            <input style:width="10ch" prop:value=bytes />
                        </td>
                        <td class=style::tablecell>
                            <input prop:value=asm />
                        </td>
                    }
                }
                None => {
                    view! {
                        <td class=style::tableleft>
                            <span>
                                {emu_signals
                                    .read
                                    .with(|emu| { emu.memory.read_8(address) })
                                    .map(|b| format!("{:02X}", b))
                                    .unwrap_or_else(|_| "??".to_string())}
                            </span>
                        </td>
                        <td class=style::tablecell>
                            <span>"UNKNOWN"</span>
                        </td>
                    }
                }
            }}
        </tr>
    }
}

#[island]
pub fn DisasmTbody(
    rows: usize,
) -> impl IntoView {
    let emu_signals = expect_context::<EmuSignals>();
    let start_pos_signals = expect_context::<StartPosSignals>();
    view! {
        <tbody>
            {
                let rows = move || {
                    let mut pc = start_pos_signals
                        .read
                        .with(|val| {
                            val.unwrap_or(emu_signals.read.with(|emu| *emu.cpu.registers().pc))
                        }) as usize;
                    (0..rows)
                        .map(|_| {
                            let instruction = {
                                emu_signals
                                    .read
                                    .with(|emu| {
                                        emu.cpu.parser().ins_from_mem(&emu.memory, pc as u16)
                                    })
                            };
                            let size = match &instruction {
                                Ok(ins) => ins.common().length as usize,
                                Err(_) => 1,
                            };
                            let string_instruction = match &instruction {
                                Ok(ins) => {
                                    Some((
                                        ins
                                            .to_bytes()
                                            .iter()
                                            .map(|b| format!("{:02X}", b))
                                            .collect::<String>(),
                                        ins.to_string(),
                                    ))
                                }
                                Err(_) => None,
                            };
                            pc += size;
                            view! {
                                <DisasmTr
                                    address=(pc - size) as u16
                                    instruction=string_instruction
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

#[derive(Clone)]
pub struct StartPosSignals {
    pub read: ReadSignal<Option<u16>>,
    pub write: WriteSignal<Option<u16>>
}

#[island]
pub fn Disassembler(
    rows: usize
) -> impl IntoView {
    let (start_pos_read, start_pos_write) = create_signal(None);
    provide_context(StartPosSignals {
        read: start_pos_read.clone(),
        write: start_pos_write.clone()
    });
    view! {
        <table class=style::table style:width="100%">
            <thead>
                <FollowPCSwitch />
                <DisasmThead />
            </thead>
            <DisasmTbody rows />
        </table>
    }
}

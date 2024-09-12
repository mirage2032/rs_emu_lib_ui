use super::style;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::logging::log;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use stylance::classes;

#[component]
pub fn FollowPCSwitch<T: Cpu + 'static>(
    emu_read: ReadSignal<Emulator<T>>,
    start_pos_read: ReadSignal<Option<u16>>,
    start_pos_write: WriteSignal<Option<u16>>,
) -> impl IntoView {
    let elem_class = move || match start_pos_read.get() {
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
                            start_pos_write
                                .update(|v| {
                                    if v.is_none() {
                                        *v = Some(emu_read.with(|emu| *emu.cpu.registers().pc))
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
                            prop:value=move || format!("{:04X}", start_pos_read().unwrap())
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

#[component]
pub fn DisasmTr<T: Cpu + 'static>(
    address: u16,
    instruction: Result<Box<dyn emu_lib::cpu::instruction::ExecutableInstruction<T>>, String>,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    let class_is_bk = move || {
        let is_bk = emu_read.with(|emu| emu.breakpoints.contains(&address));
        match is_bk {
            true => classes! {
                style::colorbreakpoint,
                style::tableleft
            },
            false => style::tableleft.to_string(),
        }
    };
    let switch_bk = move |_| {
        emu_write.update(|emu| {
            if emu.breakpoints.contains(&address) {
                emu.breakpoints.retain(|&x| x != address);
            } else {
                emu.breakpoints.push(address);
            }
        });
    };
    let class_is_pc = move || match emu_read.with(|emu| *emu.cpu.registers().pc == address) {
        true => classes! {
            style::colorfocus,
            style::tableleft
        },
        false => style::tableleft.to_string(),
    };
    view! {
        <tr>
            <td class=class_is_bk
            on:click=switch_bk
            >
            </td>
            <td class=class_is_pc>
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
                        <td class=style::tablecell style:text-align="left" >
                            <input style:width="8ch" prop:value=ins_hexstr/>
                        </td>
                        <td class=style::tablecell>
                            <input prop:value=ins.to_string() />
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
                            <span>"UNKNOWN"</span>
                        </td>
                    }
                }
            }}
        </tr>
    }
}

#[component]
pub fn DisasmTbody<T: Cpu + 'static>(
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
                        .with(|val| val.unwrap_or(emu_read.with(|emu| *emu.cpu.registers().pc)))
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
                                Ok(ins) => ins.common().length as usize,
                                Err(_) => 1,
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
pub fn Disassembler<T: Cpu + 'static>(
    rows: usize,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    let (start_pos_read, start_pos_write) = create_signal(None);
    view! {
        <table class=style::table style:width="100%">
            <thead>
                <FollowPCSwitch emu_read start_pos_read start_pos_write />
                <DisasmThead />
            </thead>
            <DisasmTbody rows emu_read emu_write start_pos_read />
        </table>
    }
}

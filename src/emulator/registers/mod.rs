use emu_lib::cpu::registers::BaseRegister;
use emu_lib::cpu::z80::Z80;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use leptos::html::Input;
use leptos::logging::log;
use leptos::wasm_bindgen::JsCast;
use leptos::{
    component, create_effect, create_node_ref, create_signal, view, web_sys, CollectView, For,
    IntoView, ReadSignal, Show, Signal, SignalGet, SignalUpdate, SignalWith, WriteSignal,
};
use stylance::import_style;

import_style!(style, "../table.module.scss");

#[component]
fn GPRegister(
    name: &'static str,
    suffix: String,
    read_full: Signal<u16>,
    read_left: Signal<u8>,
    read_right: Signal<u8>,
    write_full: impl Fn(u16) -> () + 'static,
    write_left: impl Fn(u8) -> () + 'static,
    write_right: impl Fn(u8) -> () + 'static,
) -> impl IntoView {
    let full_ref = create_node_ref::<Input>();
    let left_ref = create_node_ref::<Input>();
    let right_ref = create_node_ref::<Input>();
    create_effect(move |_| {
        full_ref
            .get()
            .unwrap()
            .set_value(&format!("{:04X}", read_full.get()));
        left_ref
            .get()
            .unwrap()
            .set_value(&format!("{:02X}", read_left.get()));
        right_ref
            .get()
            .unwrap()
            .set_value(&format!("{:02X}", read_right.get()));
    });
    view! {
        <table class=style::table>
            <thead>
                <tr>
                    <th colspan=2 class=style::tabletop>
                        {name}
                        {&suffix}
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td colspan=2 class=style::tablecell>
                        <input
                            maxlength=4
                            style:width="6ch"
                            _ref=full_ref
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
                                                write_full(val);
                                                element.set_value(&format!("{:04X}", val));
                                            }
                                            Err(_) => {
                                                log!("Invalid hex value");
                                                element.set_value(&format!("{:04X}", read_full.get()));
                                            }
                                        }
                                    });
                            }
                        />
                    </td>
                </tr>
                <tr>
                    <td class=style::tabletop>{name.chars().nth(0)}{&suffix}</td>
                    <td class=style::tabletop>{name.chars().nth(1)}{&suffix}</td>
                </tr>
                <tr>
                    <td class=style::tablecell>
                        <input
                            maxlength=2
                            style:width="3ch"
                            _ref=left_ref
                            on:change=move |event| {
                                event
                                    .target()
                                    .map(|target| {
                                        let element = target
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .unwrap();
                                        let result = u8::from_str_radix(&element.value(), 16);
                                        match result {
                                            Ok(val) => {
                                                write_left(val);
                                                element.set_value(&format!("{:02X}", val));
                                            }
                                            Err(_) => {
                                                log!("Invalid hex value");
                                                element.set_value(&format!("{:02X}", read_left.get()));
                                            }
                                        }
                                    });
                            }
                        />
                    </td>
                    <td class=style::tablecell>
                        <input
                            maxlength=2
                            style:width="3ch"
                            _ref=right_ref
                            on:change=move |event| {
                                event
                                    .target()
                                    .map(|target| {
                                        let element = target
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .unwrap();
                                        let result = u8::from_str_radix(&element.value(), 16);
                                        match result {
                                            Ok(val) => {
                                                write_right(val);
                                                element.set_value(&format!("{:02X}", val));
                                            }
                                            Err(_) => {
                                                log!("Invalid hex value");
                                                element.set_value(&format!("{:02X}", read_right.get()));
                                            }
                                        }
                                    });
                            }
                        />
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
#[component]
fn GPRegisterGroup<T:Cpu+Default+'static>(
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
    gp_idx: usize,
) -> impl IntoView {
    view! {
        <div style:display="flex">
            <GPRegister
                name="AF"
                suffix="'".repeat(gp_idx)
                read_full=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].af)
                })
                read_left=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].a)
                })
                read_right=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].f.into())
                })
                write_full=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].af = val)
                }
                write_left=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].a = val)
                }
                write_right=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].f = val.into())
                }
            />

            <GPRegister
                name="BC"
                suffix="'".repeat(gp_idx)
                read_full=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].bc)
                })
                read_left=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].b)
                })
                read_right=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].c)
                })
                write_full=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].bc = val)
                }
                write_left=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].b = val)
                }
                write_right=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].c = val)
                }
            />

            <GPRegister
                name="DE"
                suffix="'".repeat(gp_idx)
                read_full=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].de)
                })
                read_left=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].d)
                })
                read_right=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].e)
                })
                write_full=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].de = val)
                }
                write_left=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].d = val)
                }
                write_right=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].e = val)
                }
            />

            <GPRegister
                name="HL"
                suffix="'".repeat(gp_idx)
                read_full=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].hl)
                })
                read_left=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].h)
                })
                read_right=Signal::derive(move || {
                    emu_read.with(|emu| emu.cpu.registers().gp[gp_idx].l)
                })
                write_full=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].hl = val)
                }
                write_left=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].h = val)
                }
                write_right=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().gp[gp_idx].l = val)
                }
            />
        </div>
    }
}
#[component]
fn GPAllRegisters<T:Cpu+Default+'static>(
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    let gp_groups = move || emu_read.with(|emu| emu.cpu.registers().gp.len());
    let (read_current_gp, write_current_gp) = create_signal(0);
    view! {
        <div style:display="flex">
            <Show when=move || { gp_groups() > 1 }>
                <table class=style::table>
                    <tr>
                        <td
                            class=style::tablebutton
                            on:click=move |_| {
                                write_current_gp.update(|val| *val = (*val + 1) % gp_groups());
                            }
                            style="vertical-align: middle;text-align:center"
                        >
                            <p style:padding="0.1rem">Swap <br />view</p>
                        </td>
                    </tr>
                    <tr>
                        <td
                            class=style::tablebutton
                            on:click=move |_| {
                                emu_write
                                    .update(|emu| {
                                        let main_gp = emu.cpu.registers_mut().gp[0].clone();
                                        emu.cpu.registers_mut().gp[0] = emu
                                            .cpu
                                            .registers()
                                            .gp[1]
                                            .clone();
                                        emu.cpu.registers_mut().gp[1] = main_gp;
                                    });
                            }
                            style="vertical-align: middle;text-align:center"
                        >
                            <p style:padding="0.1rem">Swap <br />content</p>
                        </td>
                    </tr>
                </table>
            </Show>
            <For each=move || 0..gp_groups() key=|gp_idx| gp_idx.clone() let:gp_idx>
                <Show when=move || gp_idx == read_current_gp.get()>
                    <GPRegisterGroup emu_read=emu_read emu_write=emu_write gp_idx />
                </Show>
            </For>

        </div>
    }
}

#[component]
pub fn WordRegister(
    name: &'static str,
    register_read: Signal<u16>,
    register_write: impl Fn(u16) -> () + 'static,
) -> impl IntoView {
    view! {
        <table class=style::table>
            <thead>
                <tr>
                    <th class=style::tabletop>{name}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td class=style::tablecell>
                        <input
                            maxlength=4
                            style:width="6ch"
                            value=move || format!("{:04X}", register_read())
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
                                                register_write(val);
                                                element.set_value(&format!("{:04X}", val));
                                            }
                                            Err(_) => {
                                                log!("Invalid hex value");
                                                element.set_value(&format!("{:04X}", register_read()));
                                            }
                                        }
                                    });
                            }
                        />
                    </td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
pub fn PCSPRegisters<T:Cpu+Default+'static>(
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    view! {
        <div style:display="flex">
            <WordRegister
                name="PC"
                register_read=Signal::derive(move || emu_read.with(|emu| emu.cpu.registers().pc))
                register_write=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().pc = val)
                }
            />
            <WordRegister
                name="SP"
                register_read=Signal::derive(move || emu_read.with(|emu| emu.cpu.registers().sp))
                register_write=move |val| {
                    emu_write.update(|emu| emu.cpu.registers_mut().sp = val)
                }
            />
        </div>
    }
}

#[component]
pub fn ByteRegister(
    name: &'static str,
    register_read: Signal<u8>,
    register_write: impl Fn(u8) -> () + 'static,
) -> impl IntoView {
    view! {
        <table class=style::table>
            <tr>
                <td class=style::tabletop>{name}</td>
            </tr>
            <tr>
                <td class=style::tablecell>
                    <input
                        maxlength=2
                        style:width="3ch"
                        value=move || format!("{:02X}", register_read())
                        on:change=move |event| {
                            event
                                .target()
                                .map(|target| {
                                    let element = target
                                        .dyn_into::<web_sys::HtmlInputElement>()
                                        .unwrap();
                                    let result = u8::from_str_radix(&element.value(), 16);
                                    match result {
                                        Ok(val) => {
                                            register_write(val);
                                            element.set_value(&format!("{:02X}", val));
                                        }
                                        Err(_) => {
                                            log!("Invalid hex value");
                                            element.set_value(&format!("{:02X}", register_read()));
                                        }
                                    }
                                });
                        }
                    />
                </td>
            </tr>
        </table>
    }
}

#[component]
pub fn OtherRegisters<T:Cpu+Default+'static>(
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    let views = move || {
        let mut views = Vec::new();
        emu_read.with(|emu_rd| {
            for (name, register) in emu_rd.cpu.registers().other.iter() {
                let name_deref = *name;
                match register {
                    BaseRegister::Bit8(val) => {
                        views.push(view! {
                            <ByteRegister
                                name=name
                                register_read=Signal::derive(move || {
                                    emu_read
                                        .with(|emu| {
                                            match emu.cpu.registers().other[name_deref] {
                                                BaseRegister::Bit8(val) => val,
                                                _ => unreachable!(),
                                            }
                                        })
                                })
                                register_write=move |val| {
                                    emu_write
                                        .update(move |emu| {
                                            emu.cpu
                                                .registers_mut()
                                                .other
                                                .insert(name_deref, BaseRegister::Bit8(val));
                                        })
                                }
                            />
                        });
                    }
                    BaseRegister::Bit16(val) => {
                        views.push(view! {
                            <WordRegister
                                name=name
                                register_read=Signal::derive(move || {
                                    emu_read
                                        .with(|emu| {
                                            match emu.cpu.registers().other[name_deref] {
                                                BaseRegister::Bit16(val) => val,
                                                _ => unreachable!(),
                                            }
                                        })
                                })
                                register_write=move |val| {
                                    emu_write
                                        .update(|emu| {
                                            emu.cpu
                                                .registers_mut()
                                                .other
                                                .insert(name_deref, BaseRegister::Bit16(val));
                                        })
                                }
                            />
                        });
                    }
                }
            }
        });
        views
    };
    view! { <div style:display="flex">{views}</div> }
}

#[component]
pub fn Registers<T:Cpu+Default+'static>(
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    view! {
        <div>
            <GPAllRegisters emu_read emu_write />
            <div style:display="flex">
                <PCSPRegisters emu_read emu_write />
                <OtherRegisters emu_read emu_write />
            </div>
        </div>
    }
}

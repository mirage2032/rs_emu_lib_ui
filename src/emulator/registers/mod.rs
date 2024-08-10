use emu_lib::emulator::Emulator;
use leptos::{
    CollectView, component, create_effect, create_node_ref, For, IntoView, ReadSignal,
    Signal, SignalGet, SignalUpdate, SignalWith, view, web_sys, WriteSignal,
};
use leptos::html::Input;
use leptos::logging::log;
use leptos::wasm_bindgen::JsCast;
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
                            style:width="4.5ch"
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
                            style:width="2.5ch"
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
                            style:width="2.5ch"
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
fn GPRegisterGroup(
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
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
fn GPAllRegisters(
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    let gp_groups = move || emu_read.with(|emu| emu.cpu.registers().gp.len());
    view! {
        <div>
            <For each=move || (0..gp_groups()) key=|gp_idx| gp_idx.clone() let:gp_idx>
                <GPRegisterGroup emu_read=emu_read emu_write=emu_write gp_idx />
            </For>
        </div>
    }
}

#[component]
pub fn Registers(
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    view! {
        <div>
            <GPAllRegisters emu_read=emu_read emu_write=emu_write />
        </div>
    }
}

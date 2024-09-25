use super::{style, EmuSignals};
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::logging::{log, warn};
use leptos::*;
use std::borrow::BorrowMut;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use leptos::prelude::*;     // Added

#[component]
fn MemThead(width: usize) -> impl IntoView {
    let address_signals = expect_context::<AddressReadSignals>();
    let suffix = move |offset: u16| {
        let address: u16 = address_signals.read.get();
        if offset > (0x10 - (address & 0x00FF)) {
            let offset = (address & 0x00FF) + offset;
            format!("{:02X}", offset)
        } else {
            let offset = (address & 0x00FF) + offset;
            format!("{:01X}", offset)
        }
    };
    let view = view! {
        <thead>
            <tr>
                <th class=style::tableleft></th>
                {(0..width)
                    .map(move |x| {
                        view! {
                            <th class=style::tabletop style:min-width="2.5ch">
                                <span>{move || suffix(x as u16)}</span>
                            </th>
                        }
                    })
                    .collect_view()}
            </tr>
        </thead>
    };
    Some(view)
}

#[component]
fn MemCell(index: usize) -> impl IntoView {
    let emu_signals = expect_context::<EmuSignals>();
    let i_getval = move || -> Result<u8, &str> {
        if index >= emu_signals.read.with(|emu| emu.memory.size()) {
            return Err("??");
        }
        emu_signals.read.with(|emu| emu.memory.read_8(index as u16))
    };

    let s_getval = move || -> String {
        match i_getval() {
            Ok(val) => format!("{:02X}", val),
            Err(msg) => msg.to_string(),
        }
    };

    let i_setval = move |value: &u8| -> Result<(), &str> {
        if index > u16::MAX as usize {
            return Err("Index out of bounds");
        }
        let mut result = Err("Unknown error");
        emu_signals.write.update(|emu: &mut Emulator<Z80>| {
            result = emu.memory.write_8(index as u16, *value);
        });
        result
    };

    let s_setval = move |value: &str| -> Result<(), &str> {
        let hexval = u8::from_str_radix(value, 16);
        match hexval {
            Ok(val) => {
                i_setval(&val)?;
                Ok(())
            }
            Err(_) => Err("Invalid hex value"),
        }
    };

    let view = view! {
        <input
            maxlength=2
            prop:value=move || s_getval()
            style:width="100%"
            on:change=move |ev| {
                let elem_val = event_target_value(&ev);
                if let Err(err) = s_setval(&elem_val) {
                    warn!("Error saving value: {} at pos: {} with error: {}", elem_val,index,err);
                    let real_val = s_getval();
                    event_target::<HtmlInputElement>(&ev).borrow_mut().set_value(&real_val);
                }
            }
            on:click=move |event| {
                event
                    .target()
                    .map(|target| {
                        let element = target.dyn_into::<HtmlInputElement>().unwrap();
                        element.select();
                    });
            }
        />
    };
    Some(view)
}

#[component]
fn MemThs(width: usize, row_start: usize) -> impl IntoView {
    view! {
        <For each=move || { row_start..(row_start + width) } key=move |index| *index let:index>
            <th class=style::tablecell>
                <MemCell index />
            </th>
        </For>
    }
}

#[component]
fn MemTrCounter(width: usize) -> impl IntoView {
    let emu_signals = expect_context::<EmuSignals>();
    let address_signals = expect_context::<AddressReadSignals>();
    let memth = move || {
        let row_start = address_signals.read.get() as usize;
        view! { <MemThs width row_start /> }
    };
    let view = view! {
        <tr>
            <th class=style::tableleft style:display="flex" style:border="none">
                <span>"0x"</span>
                <input
                    class=style::tablecount
                    style:width="100%"
                    maxlength=4
                    prop:value=move || format!("{:04X}", address_signals.read.get())
                    on:change=move |event| {
                        event
                            .target()
                            .map(|target| {
                                let element = target.dyn_into::<HtmlInputElement>().unwrap();
                                let elem_val = &element.value();
                                let hexval = u16::from_str_radix(elem_val, 16);
                                match hexval {
                                    Ok(val) => {
                                        address_signals.write.set(val);
                                        element.set_value(&format!("{:04X}", val));
                                    }
                                    Err(_) => {
                                        log!("Invalid hex value");
                                        element
                                            .set_value(&format!("{:04X}", address_signals.read.get()));
                                    }
                                }
                            });
                    }
                />
            </th>
            {memth}
        </tr>
    };
    Some(view)
}

#[component]
fn MemTr(width: usize, row_start: usize) -> impl IntoView {
    view! {
        <tr>
            <th class=style::tableleft>
                <span>{format!("0x{:04X}", row_start)}</span>
            </th>
            <MemThs width row_start />
        </tr>
    }
}

#[component]
pub fn MemTbody(width: usize, rows: usize) -> impl IntoView {
    let emu_signals = expect_context::<EmuSignals>();
    let address_signals = expect_context::<AddressReadSignals>();
    let addr_start = move || address_signals.read.get() as usize + width;
    let addr_end = move || {
        emu_signals
            .read
            .with(|emu| emu.memory.size())
            .min(addr_start() + width * rows)
    };
    let view = view! {
        <tbody>
            <MemTrCounter width />
            <For
                each=move || { (addr_start()..addr_end()).step_by(width) }
                key=|row_start| *row_start
                let:row_start
            >
                <MemTr width row_start />
            </For>
        </tbody>
    };
    Some(view)
}

#[derive(Clone)]
pub struct AddressReadSignals {
    pub read: ReadSignal<u16>,
    pub write: WriteSignal<u16>,
}

#[component]
pub fn MemEditor(width: usize, rows: usize) -> impl IntoView {
    let emu_signals = expect_context::<EmuSignals>();

    let (address_read, address_write) = create_signal(0);
    provide_context(AddressReadSignals {
        read: address_read.clone(),
        write: address_write.clone(),
    });
    let view = view! {
        <table style:width="100%" class=style::table>
            <MemThead width />
            <MemTbody width rows />
        </table>
    };
    Some(view)
}

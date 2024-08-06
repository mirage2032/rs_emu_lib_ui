use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::*;
use leptos::logging::{log, warn};
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::js_sys::Atomics::add;
use stylance::import_style;

import_style!(style, "editor.module.scss");

#[component]
fn MemThead(width: usize) -> impl IntoView {
    view! {
        <thead>
            <tr>
                <th>
                    <input disabled class=style::tablecell value="" style="width: 6.5ch" />
                </th>
                {(0..width)
                    .map(move |x| {
                        view! {
                            <th>
                                <input
                                    disabled
                                    class=style::tablecell
                                    value=format!("{:X}", x)
                                    style="width: 2.5ch"
                                />
                            </th>
                        }
                    })
                    .collect_view()}
            </tr>
        </thead>
    }
}

#[component]
fn MemCell(
    //index is a derived usize
    index: Signal<usize>,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    let i_getval = move |index: usize| -> Result<u8, &str> {
        emu_read.with(|emu| emu.memory.read_8(index as u16))
    };

    let s_getval = move |index: usize| -> String {
        match i_getval(index) {
            Ok(val) => format!("{:02X}", val),
            Err(_) => "??".to_string(),
        }
    };

    let i_setval = move |index: usize, value: &u8| -> Result<(), &str> {
        let mut result = Err("Mem not written");
        emu_write.update(|emu: &mut Emulator| {
            result = emu.memory.write_8(index as u16, *value);
        });
        result
    };

    let s_setval = move |index: usize, value: &str| -> Result<(), &str> {
        let hexval = u8::from_str_radix(value, 16);
        match hexval {
            Ok(val) => i_setval(index, &val),
            Err(_) => Err("Invalid hex value"),
        }
    };

    view! {
        <input
            class=style::tablecell
            maxlength=2
            value=move || s_getval(index())
            on:change=move |event| {
                event
                    .target()
                    .map(|target| {
                        let element = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        let elem_val = &element.value();
                        let result = s_setval(index(), elem_val);
                        match result {
                            Ok(_) => {
                                log!("Saved value: {} at pos: {}", elem_val,index());
                                element.set_value(&format!("{}", elem_val));
                            }
                            Err(err) => {
                                warn!(
                                    "Error saving value: {} at pos: {} with error: {}", element.value(),index(),err
                                );
                                element.set_value(&s_getval(index()));
                            }
                        }
                    });
            }
            style="width: 2.5ch"
        />
    }
}

#[component]
fn MemThs(
    address_read: ReadSignal<u16>,
    width: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
    offset: usize,
) -> impl IntoView {
    view! {
        {
            let start = move || (offset + address_read() as usize) * width;
            (0..width)
                .map(move |i| {
                    let idx = move || start() + i;
                    view! {
                        <th>
                            <MemCell index=Signal::derive(idx) emu_read emu_write />
                        </th>
                    }
                })
                .collect_view()
        }
    }
}

#[component]
fn MemTrCounter(
    width: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
    address_read: ReadSignal<u16>,
    address_write: WriteSignal<u16>,
) -> impl IntoView {
    view! {
        <tr>
            <th>
                <input
                    class=style::tablecell
                    style="width: 6.5ch"
                    value=move || format!("{:04X}", address_read())
                    on:change=move |event| {
                        event
                            .target()
                            .map(|target| {
                                let element = target
                                    .dyn_into::<web_sys::HtmlInputElement>()
                                    .unwrap();
                                let elem_val = &element.value();
                                let hexval = u16::from_str_radix(elem_val, 16);
                                match hexval {
                                    Ok(val) => {
                                        let val = val & 0xFFF0;
                                        log!("Saved value: {:04X}", val/0x10);
                                        address_write(val/0x10);
                                        element.set_value(&format!("{:04X}", val));
                                    }
                                    Err(_) => {
                                        log!("Invalid hex value");
                                        element.set_value(&format!("{:04X}", address_read()));
                                    }
                                }
                            });
                    }
                />
            </th>
            <MemThs address_read width emu_read emu_write offset=0 />
        </tr>
    }
}
#[component]
fn MemTr(
    width: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
    address_read: ReadSignal<u16>,
    offset: usize,
) -> impl IntoView {
    view! {
        <tr>
            <th>
                <input
                    disabled
                    class=style::tablecell
                    style="width: 6.5ch"
                    value=move || format!("0x{:04X}", offset * width+address_read()as usize*0x10)
                />
            </th>
            <MemThs address_read width emu_read emu_write offset />
        </tr>
    }
}

#[component]
pub fn MemTbody(
    width: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    let rows = 0x10usize;
    let (address_read, address_write) = create_signal(0);
    view! {
        <tbody>
            <MemTrCounter
                width
                emu_read
                emu_write
                address_read
                address_write
            />
            {
                move || (1..(emu_read.with(|emu| emu.memory.size()) / width).min(rows))
                .map(|y| {
                    view! { <MemTr address_read width emu_read emu_write offset=y/> }
                })
                .collect_view()}
        </tbody>
    }
}

#[component]
pub fn Editor(emu_read: ReadSignal<Emulator>, emu_write: WriteSignal<Emulator>) -> impl IntoView {
    let width = 0x20;
    view! {
        <table style="table-collapse: collapse; border-spacing: 0;">
            <MemThead width />
            <MemTbody width emu_read emu_write />
        </table>
    }
}

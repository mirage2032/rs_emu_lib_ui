use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::*;
use leptos::logging::{log, warn};
use leptos::wasm_bindgen::JsCast;
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
                    .map(|x| {
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
    index: usize,
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
            on:change=move |event| {
                event
                    .target()
                    .map(|target| {
                        let element = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        let elem_val = &element.value();
                        let result = s_setval(index, elem_val);
                        match result {
                            Ok(_) => {
                                log!("Saved value: {} at pos: {}", elem_val,index);
                                element.set_value(&format!("{}", elem_val));
                            }
                            Err(err) => {
                                warn!(
                                    "Error saving value: {} at pos: {} with error: {}", element.value(),index,err
                                );
                                element.set_value(&s_getval(index));
                            }
                        }
                    });
            }
            value=move || s_getval(index)
            style="width: 2.5ch"
        />
    }
}

#[component]
fn MemThs(
    y: usize,
    width: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    view! {
        {
            let start = y * width;
            (0..width)
                .map(|i| {
                    let index = start + i;
                    view! {
                        <th>
                            <MemCell
                                index
                                emu_read
                                emu_write
                            />
                        </th>
                    }
                })
                .collect_view()
        }
    }
}

#[component]
fn MemTr(
    y: usize,
    width: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    view! {
        <tr>
            <th>
                <input
                    disabled
                    class=style::tablecell
                    style="width: 6.5ch"
                    value=format!("0x{:04X}", y * width)
                />
            </th>
            <MemThs y width emu_read emu_write />
        </tr>
    }
}

#[component]
pub fn MemTbody(
    width: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
    // start: usize,
    // rows: usize,
) -> impl IntoView {
    let start = 0usize;
    let rows = 0x10usize;
    let memsize = emu_read.with(|emu| emu.memory.size());
    log!("Memory size: {}", memsize);
    view! {
        <tbody>
            {(start..memsize.min(start + rows))
                .map(|y| {
                    view! {
                        <MemTr y width emu_read emu_write />
                    }
                })
                .collect_view()}
        </tbody>
    }
}

#[component]
pub fn Editor(
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    let width = 0x10;
    let memsize = emu_read.with(|emu| emu.memory.size());
    log!("Memory size: {}", memsize);
    view! {
        <table style="table-collapse: collapse; border-spacing: 0;">
            <MemThead width />
            <MemTbody width emu_read emu_write />
        </table>
    }
}

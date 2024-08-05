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
                    <input disabled class=style::tablecell value="" style="width: 5.5ch" />
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
    changes_in: ReadSignal<Emulator>,
    changes_out: WriteSignal<Emulator>,
) -> impl IntoView {
    let i_getval = move |index: usize| -> Result<u8, &str> {
        changes_in.with(|emu| emu.memory.read_8(index as u16))
    };

    let s_getval = move |index: usize| -> String {
        match i_getval(index) {
            Ok(val) => format!("{:02X}", val),
            Err(_) => "??".to_string(),
        }
    };

    let i_setval = move |index: usize, value: &u8| -> Result<(), &str> {
        let mut result = Err("Mem not written");
        changes_out.update(|emu: &mut Emulator| {
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
                event.target()
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
fn MemTr(
    y: usize,
    width: usize,
    changes_in: ReadSignal<Emulator>,
    changes_out: WriteSignal<Emulator>,
) -> impl IntoView {
    view! {
        <tr>
            <th>
                <input
                    disabled
                    class=style::tablecell
                    style="width: 5.5ch"
                    value=format!("0x{:03X}", y)
                />
            </th>
            {
                let start = y * width;
                (0..width)
                    .map(|i| {
                        let index = start + i;
                        view! {
                            <th>
                                <MemCell
                                    index
                                    changes_in=changes_in.clone()
                                    changes_out=changes_out.clone()
                                />
                            </th>
                        }
                    })
                    .collect_view()
            }
        </tr>
    }
}

#[component]
pub fn MemTbody(
    width: usize,
    changes_in: ReadSignal<Emulator>,
    changes_out: WriteSignal<Emulator>,
) -> impl IntoView {
    let memsize = changes_in.with(|emu| emu.memory.size());
    log!("Memory size: {}", memsize);
    view! {
        <tbody>
            {(0..memsize / 100 / width)
                .map(|y| {
                    view! { <MemTr y=y width=width changes_in=changes_in changes_out=changes_out /> }
                })
                .collect_view()}
        </tbody>
    }
}

#[component]
pub fn Editor(
    changes_in: ReadSignal<Emulator>,
    changes_out: WriteSignal<Emulator>,
) -> impl IntoView {
    let width = 0x10;
    let memsize = changes_in.with(|emu| emu.memory.size());
    log!("Memory size: {}", memsize);
    //style
    view! {
        <table style="table-collapse: collapse; border-spacing: 0;">
            <MemThead width=width />
            <MemTbody width=width changes_in=changes_in changes_out=changes_out />
        </table>
    }
}

use std::sync::{Arc, Mutex};

use emu_lib::memory::{Memory, MemoryDevice};
use leptos::*;
use leptos::logging::{log, warn};
use leptos::wasm_bindgen::JsCast;
use stylance::import_style;

import_style!(style, "editor.module.scss");

#[component]
fn Thead(width: usize) -> impl IntoView {
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
fn Tinput(
    index: usize,
    changes_in: ReadSignal<Arc<Mutex<Memory>>>,
    changes_out: WriteSignal<Arc<Mutex<Memory>>>,
) -> impl IntoView {
    let getval = move |index: usize| {
        changes_in.with(|mem| mem.lock().unwrap().read_8(index as u16).unwrap())
    };
    let setval = move |index: usize, value: &u8| -> Result<(), &str> {
        let mut result = Err("Mem not written");
        changes_out.update(|mem: &mut Arc<Mutex<Memory>>| {
            result = mem.lock().unwrap().write_8(index as u16, *value);
        });
        result
    };
    view! {
        <input
            class=style::tablecell
            maxlength=2
            on:change=move |e| {
                e.target()
                    .map(|x| {
                        let element = x.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        let hexval = u8::from_str_radix(&element.value(), 16);
                        match hexval {
                            Ok(elem_val) => {
                                let result = setval(index, &elem_val);
                                match result {
                                    Ok(_) => {
                                        log!("Saved value: {} at pos: {}", element.value(),index);
                                        element.set_value(&format!("{:02X}", elem_val));
                                    }
                                    Err(err) => {
                                        warn!(
                                            "Error saving value: {} at pos: {} with error: {}", element.value(),index,err
                                        );
                                        element.set_value(&format!("{:02X}", getval(index)));
                                    }
                                }
                            }
                            Err(_) => {
                                warn!("Invalid pos: {} value: {}",index, element.value());
                                element.set_value(&format!("{:02X}", getval(index)));
                            }
                        }
                    });
            }
            value=move || format!("{:02X}", getval(index))
            style="width: 2.5ch"
        />
    }
}

#[component]
fn Trow(
    y: usize,
    width: usize,
    changes_in: ReadSignal<Arc<Mutex<Memory>>>,
    changes_out: WriteSignal<Arc<Mutex<Memory>>>,
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
                                <Tinput
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
pub fn Tbody(
    width: usize,
    changes_in: ReadSignal<Arc<Mutex<Memory>>>,
    changes_out: WriteSignal<Arc<Mutex<Memory>>>,
) -> impl IntoView {
    let memsize = changes_in().lock().unwrap().size();
    log!("Memory size: {}", memsize);
    view! {
        <tbody>
            {(0..memsize/100 / width)
                .map(|y| {
                    view! {
                        <Trow
                            y=y
                            width=width
                            changes_in=changes_in.clone()
                            changes_out=changes_out.clone()
                        />
                    }
                })
                .collect_view()}
        </tbody>
    }
}

#[component]
pub fn Editor(
    changes_in: ReadSignal<Arc<Mutex<Memory>>>,
    changes_out: WriteSignal<Arc<Mutex<Memory>>>,
) -> impl IntoView {
    let width = 0x10;
    let memsize = changes_in().lock().unwrap().size();
    log!("Memory size: {}", memsize);
    //style
    view! {
        <table style="table-collapse: collapse; border-spacing: 0;">
            <Thead width=width />
            <Tbody width=width changes_in=changes_in changes_out=changes_out />
        </table>
    }
}

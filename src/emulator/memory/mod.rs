use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::*;
use leptos::logging::{log, warn};
use leptos::wasm_bindgen::JsCast;
use stylance::import_style;

import_style!(style, "../table.module.scss");

#[component]
fn MemThead(width: usize) -> impl IntoView {
    view! {
        <thead>
            <tr>
                <th class=style::tableleft></th>
                {(0..width)
                    .map(move |x| {
                        view! {
                            <th class=style::tabletop>
                                <span>{format!("{:X}", x)}</span>
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
        let address = u16::try_from(index).map_err(|_| "Address outside memory range")?;
        emu_read.with(|emu| emu.memory.read_8(address))
    };

    let s_getval = move |index: usize| -> String {
        match i_getval(index) {
            Ok(val) => format!("{:02X}", val),
            Err(_) => "??".to_string(),
        }
    };

    let i_setval = move |index: usize, value: &u8| -> Result<(), &str> {
        let address = u16::try_from(index).map_err(|_| "Address outside memory range")?;
        let mut result = Err("Mem not written");
        emu_write.update(|emu: &mut Emulator| {
            result = emu.memory.write_8(address, *value);
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
            maxlength=2
            value=move || s_getval(index())
            style:width="2.5ch"
            on:change=move |event| {
                event
                    .target()
                    .map(|target| {
                        let element = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        let elem_val = &element.value();
                        let idx = index();
                        let result = s_setval(idx, elem_val);
                        match result {
                            Ok(_) => {
                                log!("Saved value: {} at pos: {}", elem_val,idx);
                                element.set_value(&format!("{}", elem_val));
                            }
                            Err(err) => {
                                warn!(
                                    "Error saving value: {} at pos: {} with error: {}", element.value(),idx,err
                                );
                                let real_val = s_getval(idx);
                                element.set_value(&real_val);
                            }
                        }
                    });
            }
            on:click=move |event| {
                event
                    .target()
                    .map(|target| {
                        let element = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        element.select();
                    });
            }
        />
    }
}

#[component]
fn MemThs(
    width: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
    row_start: Signal<usize>,
) -> impl IntoView {
    view! {
        {(0..width)
            .map(move |i| {
                view! {
                    <th class=style::tablecell>
                        <MemCell index=Signal::derive(move || row_start() + i) emu_read emu_write />
                    </th>
                }
            })
            .collect_view()}
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
            <th class=style::tableleft>
                <span>"0x"</span>
                <input
                    class=style::tablecount
                    style:width="4.2ch"
                    maxlength=4
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
                                        address_write(val);
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
            <MemThs
                width
                emu_read
                emu_write
                row_start=Signal::derive(move || { address_read() as usize })
            />
        </tr>
    }
}

#[component]
fn MemTr(
    width: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
    row_start: usize,
) -> impl IntoView {
    view! {
        <tr>
            <th class=style::tableleft>
                <span>{format!("0x{:04X}", row_start)}</span>
            </th>
            <MemThs width emu_read emu_write row_start=Signal::derive(move || row_start) />
        </tr>
    }
}

#[component]
pub fn MemTbody(
    width: usize,
    rows: usize,
    emu_read: ReadSignal<Emulator>,
    emu_write: WriteSignal<Emulator>,
) -> impl IntoView {
    let (address_read, address_write) = create_signal(0);
    view! {
        <tbody>
            <MemTrCounter width emu_read emu_write address_read address_write />
            {
                let addr_start = move || address_read() as usize + width;
                let addr_end = move || {
                    emu_read.with(|emu| emu.memory.size()).min(addr_start() + width * rows)
                };
                move || {
                    (addr_start()..addr_end())
                        .step_by(width)
                        .map(|row_start| {
                            view! { <MemTr width emu_read emu_write row_start /> }
                        })
                        .collect_view()
                }
            }
        </tbody>
    }
}

#[component]
pub fn MemEditor(width: usize, rows: usize,
                 emu_read: ReadSignal<Emulator>, emu_write: WriteSignal<Emulator>) -> impl IntoView {
    view! {
        <table class=style::memtable>
            <MemThead width />
            <MemTbody width rows emu_read emu_write />
        </table>
    }
}

use super::style;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::logging::{log, warn};
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use web_sys::HtmlInputElement;
use std::borrow::BorrowMut;

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
fn MemCell<T: Cpu + Default + 'static>(
    index: Signal<usize>,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    let i_getval = move || -> Result<u8, &str> {
        emu_read.with(|emu| emu.memory.read_8(index() as u16))
    };

    let s_getval = move || -> String {
        match i_getval() {
            Ok(val) => format!("{:02X}", val),
            Err(_) => "??".to_string(),
        }
    };

    let i_setval = move |value: &u8| -> Result<(), &str> {
        let mut result = Err("Mem not written");
        emu_write.update(|emu: &mut Emulator<T>| {
            result = emu.memory.write_8(index() as u16 , *value);
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

    view! {
        <input
            maxlength=2
            prop:value=move || s_getval()
            style:width="100%"
            on:change=move |ev| {
                let elem_val = event_target_value(&ev);
                if let Err(err) = s_setval(&elem_val){
                        warn!(
                            "Error saving value: {} at pos: {} with error: {}", elem_val,index(),err
                        );
                        let real_val = s_getval();
                        event_target::<HtmlInputElement>(&ev)
                            .borrow_mut()
                            .set_value(&real_val);
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
    }
}

#[component]
fn MemThs<T: Cpu + Default + 'static>(
    width: usize,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
    row_start: Signal<usize>,
) -> impl IntoView {;
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
fn MemTrCounter<T: Cpu + Default + 'static>(
    width: usize,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
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
                    prop:value=move || format!("{:04X}", address_read())
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
fn MemTr<T: Cpu + Default + 'static>(
    width: usize,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
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
pub fn MemTbody<T: Cpu + Default + 'static>(
    width: usize,
    rows: usize,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    let (address_read, address_write) = create_signal(0);
    let addr_start = move || address_read() as usize + width;
    let addr_end = move || {
        emu_read
            .with(|emu| emu.memory.size())
            .min(addr_start() + width * rows)
    };
    view! {
        <tbody>
            <MemTrCounter width emu_read emu_write address_read address_write />
            <For
                each=move || { (addr_start()..addr_end()).step_by(width) }
                key=|row_start| *row_start
                let:row_start
            >
                <MemTr width emu_read emu_write row_start />
            </For>
        </tbody>
    }
}

#[component]
pub fn MemEditor<T: Cpu + Default + 'static>(
    width: usize,
    rows: usize,
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    view! {
        <table class=style::table>
            <MemThead width />
            <MemTbody width rows emu_read emu_write />
        </table>
    }
}

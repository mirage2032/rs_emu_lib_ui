use super::style;
use emu_lib::cpu::i8080::I8080;
use emu_lib::cpu::z80::Z80;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use leptos::logging::warn;
use leptos::{
    component, event_target, event_target_value, view, IntoView, ReadSignal, Signal,
    SignalWithUntracked, WriteSignal,
};
use std::borrow::BorrowMut;
use web_sys::HtmlInputElement;

mod i8080;
mod z80;

#[component]
fn Register(
    name: &'static str,
    maxlength: usize,
    get: Signal<String>,
    set: impl Fn(&str) -> Result<(), std::num::ParseIntError> + 'static,
) -> impl IntoView {
    view! {
        <table class=style::table>
            <thead>
                <tr>
                    <th class=style::tabletop>
                        <span>{name}</span>
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th class=style::tablecell>
                        <input
                            style:width="100%"
                            maxlength=maxlength
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                match set(&val) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        warn!("Invalid hex value: {}",val);
                                        event_target::<HtmlInputElement>(&ev)
                                            .borrow_mut()
                                            .set_value(&get());
                                    }
                                }
                            }
                            prop:value=get
                        />
                    </th>
                </tr>
            </tbody>
        </table>
    }
}
#[component]
pub fn Registers<T: Cpu + 'static>(
    emu_read: ReadSignal<Emulator<T>>,
    emu_write: WriteSignal<Emulator<T>>,
) -> impl IntoView {
    unsafe {
        emu_read.with_untracked(|emu| {
            if let Some(_) = (&emu.cpu as &dyn std::any::Any).downcast_ref::<Z80>() {
                let emu_read = std::mem::transmute::<
                    ReadSignal<Emulator<T>>,
                    ReadSignal<Emulator<Z80>>,
                >(emu_read);
                let emu_write = std::mem::transmute::<
                    WriteSignal<Emulator<T>>,
                    WriteSignal<Emulator<Z80>>,
                >(emu_write);
                view! { <z80::Registers emu_read emu_write /> }
            } else if let Some(_) = (&emu.cpu as &dyn std::any::Any).downcast_ref::<I8080>() {
                let emu_read = std::mem::transmute::<
                    ReadSignal<Emulator<T>>,
                    ReadSignal<Emulator<I8080>>,
                >(emu_read);
                let emu_write = std::mem::transmute::<
                    WriteSignal<Emulator<T>>,
                    WriteSignal<Emulator<I8080>>,
                >(emu_write);
                view! { <i8080::Registers _emu_read=emu_read _emu_write=emu_write /> }
            } else {
                panic!("Unknown CPU type");
            }
        })
    }
}

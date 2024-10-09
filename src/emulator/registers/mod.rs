use super::style;
use emu_lib::cpu::i8080::I8080;
use emu_lib::cpu::z80::Z80;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use leptos::logging::warn;
use leptos::prelude::*;

use std::borrow::BorrowMut;
use web_sys::HtmlInputElement;

pub mod i8080;
pub mod z80;

// #[component]
fn Register(
    name: &'static str,
    maxlength: usize,
    get: Signal<String>,
    set: impl Fn(&str) -> Result<(), std::num::ParseIntError> + 'static,
) -> impl IntoView {
    let change = move |event|
        {
            let val = event_target_value(&event);
            match set(&val) {
                Ok(_) => {}
                Err(_) => {
                    warn!("Invalid hex value: {}",val);
                    event_target::<HtmlInputElement>(&event)
                        .borrow_mut()
                        .set_value(&get());
                }
            }
        };
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
                        <input style:width="100%" maxlength=maxlength
                        on:change=change
                        prop:value=get
                        />
                    </th>
                </tr>
            </tbody>
        </table>
    }
}

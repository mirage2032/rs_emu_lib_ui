use emu_lib::cpu::i8080::I8080;
use emu_lib::emulator::Emulator;
use leptos::prelude::*;

#[component]
pub fn registers(
    _emu_read: ReadSignal<Emulator<I8080>>,
    _emu_write: WriteSignal<Emulator<I8080>>,
) -> impl IntoView {
    view! { <div>F</div> }
}

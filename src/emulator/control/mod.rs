use std::time::Duration;
use emu_lib::cpu::instruction::BaseInstruction;
use emu_lib::emulator::{Emulator, StopReason};
use leptos::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::{JsCast, JsValue};
use leptos::*;
use leptos::leptos_dom::helpers::IntervalHandle;
use stylance::import_style;
use tokio::time::sleep;
use web_sys::js_sys;

import_style!(style, "../table.module.scss");
#[component]
pub fn Control(emu_read: ReadSignal<Emulator>, emu_write: WriteSignal<Emulator>) -> impl IntoView {
    let halted = move || emu_read.with(|emu| emu.cpu.halted());
    let switch_halt = move || {
        emu_write.update(|emu| {
            emu.cpu.set_halted(!emu.cpu.halted());
        });
    };
    let (running, set_running) = create_signal::<Option<Result<IntervalHandle,JsValue>>>(None);
    let step = move || {
        emu_write.update(|emu| {
            if let Err(e) = emu.step() {
                log::error!("Error stepping: {}", e);
                if let Some(Ok(int)) = running.get() {
                    int.clear();
                    set_running.set(None);
                }
            }
        });
    };


    let toggle_running = move || {
        set_running.update(|r| {
            match r {
                Some(Ok(int)) => {
                    int.clear();
                    *r = None;
                }
                _ => {
                    *r = Some(set_interval_with_handle(
                        step,
                        Duration::from_millis(0),
                    ));
                }
            }
        });
    };


    view! {
        <table class=style::table>
            <tr>
                <th
                    class=style::tablebutton
                    style:padding="0.3rem"
                    on:click=move |_| {
                        emu_write.update(|emu| {
                            match emu.step() {
                                Ok(_) => {}
                                Err(e) => {
                                    log::error!("Error stepping: {}", e);
                                }
                            }
                        });
                    }
                >
                    Step
                </th>
                <th
                    class=style::tablebutton
                    on:click=move |_| {
                        toggle_running();
                    }
                    style:padding="0.3rem"
                >
                    Run
                </th>
                <th
                    style:padding="0.3rem"
                    class=move || {
                        if halted() { style::tablebuttoninvert } else { style::tablebutton }
                    }
                    on:click=move |_| {
                        switch_halt();
                    }
                >
                    Halted
                </th>
                <th class=style::tablebutton>
                    <input
                        on:change=move |event| {
                            event
                                .target()
                                .map(|target| {
                                    let element = target
                                        .dyn_into::<web_sys::HtmlInputElement>()
                                        .unwrap();
                                    if let Some(files) = element.files() {
                                        if let Some(file) = files.get(0) {
                                            let reader = web_sys::FileReader::new().unwrap();
                                            let reader_clone = reader.clone();
                                            let onloadend_callback = Closure::wrap(
                                                Box::new(move || {
                                                    let array_buffer = reader_clone.result().unwrap();
                                                    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                                                    let file_content = uint8_array.to_vec();
                                                    emu_write
                                                        .update(|emu| {
                                                            emu.memory.load(&file_content).unwrap();
                                                        });
                                                    log::info!("{:?}", file_content);
                                                }) as Box<dyn FnMut()>,
                                            );
                                            reader
                                                .set_onloadend(
                                                    Some(onloadend_callback.as_ref().unchecked_ref()),
                                                );
                                            reader.read_as_array_buffer(&file).unwrap();
                                            onloadend_callback.forget();
                                        }
                                    }
                                });
                        }
                        value="Load file"
                        type="file"
                        style:display="none"
                        id="file-input"
                    />
                    <label for="file-input" style:padding="0.3rem">
                        Load file
                    </label>
                </th>
            </tr>
        </table>
    }
}

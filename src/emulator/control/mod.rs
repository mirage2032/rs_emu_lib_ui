use super::{style, EmuSignals};
use emu_lib::cpu::instruction::ExecutableInstruction;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use leptos::logging::log;
use leptos::prelude::*;
use tokio::time::Duration;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use web_sys::{js_sys, MouseEvent};

#[island]
pub fn Control() -> impl IntoView {
    let emu_signals = expect_context::<EmuSignals>();
    let halted_class = move || {
        emu_signals.read.with(|emu| match emu.cpu.halted() {
            true => style::tablebuttoninvert,
            false => style::tablebutton,
        })
    };
    let switch_halt = move || {
        emu_signals.write.update(|emu| {
            emu.cpu.set_halted(!emu.cpu.halted());
        });
    };

    let runner_active: RwSignal<Option<IntervalHandle>> = RwSignal::new(None);
    let runner_class = move || match runner_active.get() {
        Some(_) => style::tablebuttoninvert,
        None => style::tablebutton,
    };
    let switch_runner = move || {
        runner_active.update(|runner_state| match runner_state {
            Some(handle) => {
                handle.clear();
                *runner_state = None;
            }
            None => {
                let interval_result = set_interval_with_handle(
                    move || {
                        emu_signals.write.update(|emu| match emu.run_ticks::<_>(400000.0,&Some(|_emu:&mut Emulator<_>,_ins:&dyn ExecutableInstruction<_>|{})) {
                            Ok(_) => {}
                            Err(err) => {
                                runner_active.update(|runner_active| {
                                    if let Some(handle) = runner_active {
                                        handle.clear();
                                    }
                                    *runner_active = None;
                                });
                                warn!("Running stopper due to an error:{:?}", err)
                            }
                        });
                    },
                    Duration::from_millis(0),
                );
                match interval_result {
                    Ok(interval) => {
                        *runner_state = Some(interval);
                    }
                    Err(err) => {
                        warn!("Running stopper due to an error:{:?}", err);
                    }
                }
            }
        });
    };

    let file_event = move |event: MouseEvent| {
        let element = event_target::<HtmlInputElement>(&event);
        if let Some(files) = element.files() {
            if let Some(file) = files.get(0) {
                log!("Loading file: {:?}", file);
                let reader = web_sys::FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let onloadend_callback = Closure::wrap(Box::new(move || {
                    let array_buffer = reader_clone.result().unwrap();
                    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                    let file_content = uint8_array.to_vec();
                    log!("Loaded file with {} bytes", file_content.len());
                    emu_signals.write.update(|emu| {
                        emu.memory.load(&file_content).unwrap();
                    });
                }) as Box<dyn FnMut()>);
                reader.set_onloadend(Some(onloadend_callback.as_ref().unchecked_ref()));
                reader.read_as_array_buffer(&file).unwrap();
                onloadend_callback.forget();
            }
        }
    };

    view! {
        <table style:width="100%" class=style::table>
            <tr>
                <th
                    class=style::tablebutton
                    style:padding="0.3rem"
                    on:click=move |_| {
                        emu_signals
                            .write
                            .update(|emu| {
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
                    class=move || runner_class
                    on:click=move |_| {
                        switch_runner();
                    }
                    style:padding="0.3rem"
                >
                    Run
                </th>
                <th
                    style:padding="0.3rem"
                    class=halted_class
                    on:click=move |_| {
                        switch_halt();
                    }
                >
                    Halted
                </th>
                <th class=style::tablebutton>
                    <input
                        on:click=move |e| file_event(e)
                        value="SAFA"
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

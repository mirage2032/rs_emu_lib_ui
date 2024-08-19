use emu_lib::emulator::Emulator;
use leptos::*;
use stylance::import_style;
import_style!(style, "../table.module.scss");
#[component]
pub fn Control(emu_read: ReadSignal<Emulator>, emu_write: WriteSignal<Emulator>) -> impl IntoView {
    view! {
        <table class=style::table>
            <tr>
                <th
                    class=style::tablebutton
                    style:padding="0.3rem"
                    on:click=move |_| {
                        emu_write
                            .update(|emu| {
                                if let Err(e) = emu.step() {
                                    log::error!("Error stepping: {}", e);
                                }
                            });
                    }
                >
                    Step
                </th>
                <th class=style::tablebutton style:padding="0.3rem">
                    Run
                </th>
            </tr>
        </table>
    }
}

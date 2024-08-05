use leptos::*;
use leptos::logging::{log, warn};
use leptos::wasm_bindgen::JsCast;
use stylance::import_style;

import_style!(style, "editor.module.scss");

fn thead(width: usize) -> impl IntoView {
    view! {
        <thead>
            <tr>
                <th>
                    <input disabled class=style::tablecell value="" style="width: 6.5ch" />
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

fn trow(y: usize,
        c: &[u8],
        changes_in: ReadSignal<Vec<u8>>,
        changes_out: WriteSignal<Vec<u8>>,
) -> impl IntoView {
    view! {
        <tr>
            <th>
                <input
                    disabled
                    class=style::tablecell
                    style="width: 6.5ch"
                    value=format!("0x{:04X}", y)
                />
            </th>
            {
                let val = move |index| changes_in.with(|x| x[index]);
                let setval = move |index: usize, value: &u8| {
                    changes_out.update(|x| x[index] = *value);
                };
                c.iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let cellnum = c.len() * y + i;
                        view! {
                            <td>
                                <input
                                    class=style::tablecell
                                    maxlength=2
                                    on:change=move |e| {
                                        e.target()
                                            .map(|x| {
                                                let element = x
                                                    .dyn_into::<web_sys::HtmlInputElement>()
                                                    .unwrap();
                                                let hexval = u8::from_str_radix(&element.value(), 16);
                                                match hexval {
                                                    Ok(val) => {
                                                        log!("Saved pos: {} value: {}",cellnum, element.value());
                                                        setval(cellnum, &val);
                                                        element.set_value(&format!("{:02X}", val));
                                                    }
                                                    Err(_) => {
                                                        warn!("Invalid pos: {} value: {}",cellnum, element.value());
                                                        element.set_value(&format!("{:02X}", val(cellnum)));
                                                    }
                                                }
                                            });
                                    }
                                    value=format!("{:02X}", val(cellnum))
                                    style="width: 2.5ch"
                                />
                            </td>
                        }
                    })
                    .collect_view()
            }
        </tr>
    }
}

#[component]
pub fn Editor(
    changes_in: ReadSignal<Vec<u8>>,
    changes_out: WriteSignal<Vec<u8>>,
) -> impl IntoView {
    //style
    view! {
        <table style="table-collapse: collapse; border-spacing: 0;">
            {thead(0x10)}
            <tbody>
                {
                    thead(0x10);
                    changes_in()
                        .chunks(0x10)
                        .enumerate()
                        .map(|(y, line)| {
                            view! {
                                // Y
                                {trow(y, line, changes_in.clone(), changes_out.clone())}
                            }
                        })
                        .collect_view()
                }
            </tbody>
        </table>
    }
}
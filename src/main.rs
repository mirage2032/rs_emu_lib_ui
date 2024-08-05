mod app;
mod editor;

use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    let (data_in , data_out) = create_signal(String::from("AAAAAABBBBBBBBCCCADASDADAGBASLKDJASLKJDALKSJDLAKJDLKSJ").repeat(10).as_bytes().iter().map(|x| *x as u8).collect::<Vec<u8>>());
    let b: &u8 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".repeat(10).as_bytes().iter().next().unwrap();
    
    mount_to_body(move || view! {
        <p>"Hello, world!"</p>
        <app::App />
        <editor::Editor changes_in=data_in changes_out=data_out />
        />
    })
}
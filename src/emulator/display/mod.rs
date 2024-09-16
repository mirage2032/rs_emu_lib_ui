use emu_lib::memory::MemoryDevice;
use leptos::html::Canvas;
use leptos::{create_effect, create_node_ref, view, HtmlElement, IntoView, Signal, SignalGet};
use std::sync::{Arc, Mutex};
use leptos::logging::log;
use web_sys::CanvasRenderingContext2d;
use leptos::wasm_bindgen::JsCast;


pub struct CanvasDisplay {
    pub buffer: Arc<Mutex<Vec<u8>>>,
    pub width: usize,
    pub height: usize,
}

impl MemoryDevice for CanvasDisplay {
    fn size(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.len()
    }
    fn read_8(&self, addr: u16) -> Result<u8, &'static str> {
        self.buffer
            .lock()
            .or(Err("Failed to lock buffer"))?
            .get(addr as usize)
            .copied()
            .ok_or("Address out of bounds")
    }

    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer[addr as usize] = data;
        Ok(())
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        self.write_8(addr, data)
    }
}

impl CanvasDisplay {
    pub fn new(size: u16, width: usize) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(vec![0; size as usize])),
            height: size.div_ceil(width as u16) as usize,
            width,
        }
    }
}

pub fn gen_dsp(size: u16, width: usize) -> (CanvasDisplay,impl Fn(Signal<()>)-> HtmlElement<Canvas>) {
    let dsp = CanvasDisplay::new(size, width);
    let buffer = dsp.buffer.clone();
    let height = dsp.height;
    let mut display = move |dsp_update: Signal<()>| -> HtmlElement<Canvas> {
        let canvas_ref = create_node_ref::<Canvas>();
        let width = width;
        let height = height;
        let buffer = buffer.clone();
        create_effect(move |_| {
            dsp_update.get();
            // log!("Updating display");
            if let Some(canvas) = canvas_ref.get() {
                let ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                canvas.set_width(width as u32);
                canvas.set_height(height as u32);
                let buffer = buffer.lock().unwrap();
                let mut i = 0;
                for y in 0..height {
                    for x in 0..width {
                        let red:u8 = buffer[i] & 0b11100000;
                        let green:u8 = (buffer[i] & 0b00011100)<<3;
                        let blue:u8 = (buffer[i] & 0b00000011)<<6;
                        ctx.set_fill_style(&format!("rgb({},{},{})",red,green,blue).into());
                        ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
                        i += 1;
                    }
                }
            }
        });
        view! {
            <canvas node_ref=canvas_ref/>
        }
        //END
    };
    (dsp,display)
}

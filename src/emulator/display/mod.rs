use emu_lib::memory::MemoryDevice;
use leptos::html::Canvas;
use leptos::logging::log;
use leptos::wasm_bindgen::JsCast;
use leptos::{create_effect, create_node_ref, view, HtmlElement, Signal, SignalWith};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use web_sys::CanvasRenderingContext2d;

pub struct CanvasDisplay {
    pub buffer: Arc<Mutex<Vec<u8>>>
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
    pub fn new(size: u16) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(vec![0; size as usize])),
        }
    }
}

pub fn gen_dsp(
    size: u16,
    width: usize,
    scale: f64,
) -> (CanvasDisplay, impl Fn(Signal<()>) -> HtmlElement<Canvas>) {
    let dsp = CanvasDisplay::new(size);
    let height = size.div_ceil(width as u16) as usize;
    let buffer = dsp.buffer.clone();
    let display = move |dsp_update: Signal<()>| -> HtmlElement<Canvas> {
        let canvas_ref = create_node_ref::<Canvas>();
        let canvas = Rc::new(RefCell::new(None));
        let ctx = Rc::new(RefCell::new(None));
        let buffer = buffer.clone();
        create_effect(move |_| {
            dsp_update.track();
            if canvas.borrow().is_none() {
                if let Some(can) = canvas_ref.get() {
                    canvas.replace(Some(can));
                    ctx.replace(Some(
                        canvas
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .get_context("2d")
                            .unwrap()
                            .unwrap()
                            .dyn_into::<CanvasRenderingContext2d>()
                            .unwrap(),
                    ));
                } else {
                    log!("Canvas not found");
                    return;
                }
            }
            canvas
                .borrow()
                .as_ref()
                .unwrap()
                .set_width((width as f64 * scale) as u32);
            canvas
                .borrow()
                .as_ref()
                .unwrap()
                .set_height((height as f64 * scale) as u32);
            let buffer = buffer.lock().unwrap();
            let mut i = 0;
            for y in 0..height {
                for x in 0..width {
                    let red: u8 = buffer[i] & 0b11100000;
                    let green: u8 = (buffer[i] & 0b00011100) << 3;
                    let blue: u8 = (buffer[i] & 0b00000011) << 6;
                    ctx.borrow()
                        .as_ref()
                        .unwrap()
                        .set_fill_style(&format!("rgb({},{},{})", red, green, blue).into());
                    ctx.borrow().as_ref().unwrap().fill_rect(
                        (x as f64 * scale) as f64,
                        (y as f64 * scale) as f64,
                        scale,
                        scale,
                    );
                    i += 1;
                }
            }
        });
        view! { <canvas node_ref=canvas_ref /> }
        //END
    };
    (dsp, display)
}

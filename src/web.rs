//! Browser entry point.
//!
//! `minifb` creates a `<canvas>` inside the `gol-container` element and copies
//! the 32-bit buffer into it. The simulation loop uses `requestAnimationFrame`
//! because the browser cannot block on `thread::sleep`.

use crate::{Grid, life_cycle};
use minifb::{Window, WindowOptions};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

const WIDTH: usize = 200;
const HEIGHT: usize = 120;
const PROBABILITY: f64 = 0.4;
const FRAMES_PER_GENERATION: u32 = 3;

/// Canvas `ImageData` expects RGBA bytes. `Cell::to_color` returns the same
/// `0RGB` value used by the desktop backend, so the alpha byte must be set
/// explicitly or every pixel is fully transparent.
const OPAQUE: u32 = 0xFF00_0000;

fn request_frame(closure: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global window")
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("requestAnimationFrame failed");
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = Window::new("gol-container", WIDTH, HEIGHT, WindowOptions::default())
        .map_err(|error| JsValue::from_str(&format!("{error:?}")))?;

    let mut grid = Grid::init_grid(WIDTH, HEIGHT, PROBABILITY);
    let mut buffer: Vec<u32> = grid
        .array
        .iter()
        .map(|cell| cell.to_color() | OPAQUE)
        .collect();

    app.update_with_buffer(&buffer, WIDTH, HEIGHT)
        .map_err(|error| JsValue::from_str(&format!("{error:?}")))?;

    let slot = Rc::new(RefCell::new(None));
    let slot_holder = slot.clone();
    let mut frame = 0u32;

    *slot_holder.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        frame += 1;

        if frame % FRAMES_PER_GENERATION == 0 {
            grid = life_cycle(&grid);

            for (pixel, cell) in buffer.iter_mut().zip(&grid.array) {
                *pixel = cell.to_color() | OPAQUE;
            }

            let _ = app.update_with_buffer(&buffer, WIDTH, HEIGHT);
        }

        request_frame(slot.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_frame(slot_holder.borrow().as_ref().unwrap());
    Ok(())
}

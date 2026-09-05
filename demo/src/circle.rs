//! A filled-and-stroked circle canvas demo.

use jarsdraw::{Canvas, Circle, Styled};
use wasm_bindgen::prelude::*;

// Minimum margin from screen
const MARGIN: f64 = 0.24;

/// Draws a filled, stroked circle, scaled to fit the canvas.
#[wasm_bindgen]
pub struct CircleCanvas {
    canvas: Canvas,
}

#[wasm_bindgen]
impl CircleCanvas {
    /// Creates a circle canvas bound to the `<canvas>` element at `selector`, then draws
    /// the circle immediately.
    ///
    /// # Parameters
    /// - `selector`: a CSS selector identifying the target `<canvas>` element.
    ///
    /// # Returns
    /// A `CircleCanvas` with the circle already drawn at the canvas's current dimensions.
    ///
    /// # Errors
    /// Returns a `JsValue` error if the element cannot be found or cast to a canvas.
    #[wasm_bindgen(constructor)]
    pub fn new(selector: &str) -> Result<Self, JsValue> {
        let canvas = Canvas::from_selector(selector)?;
        let mut demo = Self { canvas };
        demo.redraw();
        Ok(demo)
    }

    /// Clears this canvas and redraws the circle to fit the canvas's current dimensions.
    ///
    /// # Side Effects
    /// Erases the canvas, then fills and strokes a circle scaled to the canvas's current
    /// width/height. Call this after resizing the `<canvas>` element.
    pub fn redraw(&mut self) {
        self.canvas.clear();

        // Scale
        let (width, height) = self.canvas.dimensions();
        let radius = (1.0 - MARGIN) * (width.min(height) as f64) / 2.0;

        // Center
        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;

        // Draw circle
        let styled = Styled::new(Circle::new(cx, cy, radius).fill(true).stroke(true))
            .stroke("#3b82f6")
            .line_width(3.0);
        self.canvas.draw(&styled);
    }
}

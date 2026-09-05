//! A single line-segment canvas demo.

use jarsdraw::{Canvas, Line, Styled};
use wasm_bindgen::prelude::*;

// Minimum margin from screen
const MARGIN: f64 = 0.24;

/// Draws a single diagonal line, scaled to fit the canvas.
#[wasm_bindgen]
pub struct LineCanvas {
    canvas: Canvas,
}

#[wasm_bindgen]
impl LineCanvas {
    /// Creates a line canvas bound to the `<canvas>` element at `selector`, then draws the
    /// line immediately.
    ///
    /// # Parameters
    /// - `selector`: a CSS selector identifying the target `<canvas>` element.
    ///
    /// # Returns
    /// A `LineCanvas` with the line already drawn at the canvas's current dimensions.
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

    /// Clears this canvas and redraws the line to fit the canvas's current dimensions.
    ///
    /// # Side Effects
    /// Erases the canvas, then strokes a single diagonal line segment scaled to the
    /// canvas's current width/height. Call this after resizing the `<canvas>` element.
    pub fn redraw(&mut self) {
        self.canvas.clear();

        // Scale
        let (width, height) = self.canvas.dimensions();
        let scale = (1.0 - MARGIN) * (width.min(height) as f64);

        // Offset
        let offset_x = (width as f64 - scale) / 2.0;
        let offset_y = (height as f64 - scale) / 2.0;

        // Draw line
        let start = (offset_x, offset_y);
        let end = (offset_x + scale, offset_y + scale);
        let styled = Styled::new(Line::new(start, end)).stroke("#f97316").line_width(3.0);
        self.canvas.draw(&styled);
    }
}

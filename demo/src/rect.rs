//! A filled-and-stroked rectangle canvas demo.

use jarsdraw::{Canvas, Rect, Styled};
use wasm_bindgen::prelude::*;

// Minimum margin from screen
const MARGIN: f64 = 0.24;

/// Draws a filled, stroked rectangle, scaled to fit the canvas.
#[wasm_bindgen]
pub struct RectCanvas {
    canvas: Canvas,
}

#[wasm_bindgen]
impl RectCanvas {
    /// Creates a rect canvas bound to the `<canvas>` element at `selector`, then draws
    /// the rectangle immediately.
    ///
    /// # Parameters
    /// - `selector`: a CSS selector identifying the target `<canvas>` element.
    ///
    /// # Returns
    /// A `RectCanvas` with the rectangle already drawn at the canvas's current dimensions.
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

    /// Clears this canvas and redraws the rectangle to fit the canvas's current dimensions.
    ///
    /// # Side Effects
    /// Erases the canvas, then fills and strokes a rectangle scaled to the canvas's current
    /// width/height. Call this after resizing the `<canvas>` element.
    pub fn redraw(&mut self) {
        self.canvas.clear();

        // Scale
        let (width, height) = self.canvas.dimensions();
        let scale = (1.0 - MARGIN) * (width.min(height) as f64);

        // Offset
        let offset_x = (width as f64 - scale) / 2.0;
        let offset_y = (height as f64 - scale) / 2.0;

        // Draw rect
        let styled = Styled::new(Rect::new(offset_x, offset_y, scale, scale).fill(true).stroke(true))
            .stroke("#a855f7")
            .line_width(6.0)
            .fill("#d8b4fe");
        self.canvas.draw(&styled);
    }
}

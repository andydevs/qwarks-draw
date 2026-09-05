//! A radiating line-segment canvas demo.

use jarsdraw::{Canvas, Line, Styled};
use std::f64::consts::TAU;
use wasm_bindgen::prelude::*;

// Minimum margin from screen
const MARGIN: f64 = 0.24;

// Number of rays in the sunburst
const RAY_COUNT: usize = 16;

/// Draws a sunburst of independent line segments radiating from the canvas's center.
#[wasm_bindgen]
pub struct LineCanvas {
    canvas: Canvas,
}

#[wasm_bindgen]
impl LineCanvas {
    /// Creates a line canvas bound to the `<canvas>` element at `selector`, then draws the
    /// sunburst immediately.
    ///
    /// # Parameters
    /// - `selector`: a CSS selector identifying the target `<canvas>` element.
    ///
    /// # Returns
    /// A `LineCanvas` with the sunburst already drawn at the canvas's current dimensions.
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

    /// Clears this canvas and redraws the sunburst to fit the canvas's current dimensions.
    ///
    /// # Side Effects
    /// Erases the canvas, then strokes `RAY_COUNT` line segments radiating from the
    /// canvas's center, scaled to the canvas's current width/height. Call this after
    /// resizing the `<canvas>` element.
    pub fn redraw(&mut self) {
        self.canvas.clear();

        // Scale
        let (width, height) = self.canvas.dimensions();
        let radius = (1.0 - MARGIN) * (width.min(height) as f64) / 2.0;

        // Center
        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;

        // Draw rays
        for i in 0..RAY_COUNT {
            let angle = TAU * (i as f64) / (RAY_COUNT as f64);
            let end = (cx + radius * angle.cos(), cy + radius * angle.sin());
            let styled = Styled::new(Line::new((cx, cy), end)).stroke("#3b82f6").line_width(3.0);
            self.canvas.draw(&styled);
        }
    }
}

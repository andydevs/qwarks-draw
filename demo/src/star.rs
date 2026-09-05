//! A closed five-pointed star canvas demo.

use jarsdraw::{Canvas, Polyline, Styled};
use std::f64::consts::TAU;
use wasm_bindgen::prelude::*;

// Minimum margin from screen
const MARGIN: f64 = 0.24;

// Number of star points
const POINTS: usize = 5;

// Inner vertex radius, as a fraction of the outer radius
const INNER_RATIO: f64 = 0.382;

/// Draws a closed five-pointed star, scaled to fit the canvas.
#[wasm_bindgen]
pub struct StarCanvas {
    canvas: Canvas,
}

#[wasm_bindgen]
impl StarCanvas {
    /// Creates a star canvas bound to the `<canvas>` element at `selector`, then draws
    /// the star immediately.
    ///
    /// # Parameters
    /// - `selector`: a CSS selector identifying the target `<canvas>` element.
    ///
    /// # Returns
    /// A `StarCanvas` with the star already drawn at the canvas's current dimensions.
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

    /// Clears this canvas and redraws the star to fit the canvas's current dimensions.
    ///
    /// # Side Effects
    /// Erases the canvas, then strokes a closed five-pointed star scaled to the canvas's
    /// current width/height. Call this after resizing the `<canvas>` element.
    pub fn redraw(&mut self) {
        self.canvas.clear();

        // Scale
        let (width, height) = self.canvas.dimensions();
        let outer_radius = (1.0 - MARGIN) * (width.min(height) as f64) / 2.0;
        let inner_radius = outer_radius * INNER_RATIO;

        // Center
        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;

        // Alternate outer/inner vertices around the circle, starting straight up
        let vertex_count = POINTS * 2;
        let points: Vec<_> = (0..vertex_count)
            .map(|i| {
                let angle = TAU * (i as f64) / (vertex_count as f64) - TAU / 4.0;
                let radius = if i % 2 == 0 { outer_radius } else { inner_radius };
                (cx + radius * angle.cos(), cy + radius * angle.sin())
            })
            .collect();

        // Draw star
        let styled = Styled::new(Polyline::new(&points).closed(true))
            .stroke("#3b82f6")
            .line_width(3.0);
        self.canvas.draw(&styled);
    }
}

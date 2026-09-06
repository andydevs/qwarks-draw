# Jarsdraw

A Rust/WebAssembly canvas drawing library for the browser (edition 2024), compiled via
`wasm-bindgen`/`web-sys`. See [README.md](README.md) for installation and usage.

## Structure

Two Cargo crates in this repo:

- **`jarsdraw/`** — the `jarsdraw` library (`rlib`); `jarsdraw/src/`:
  - `canvas.rs` — `Canvas`: wraps an `HtmlCanvasElement` + `CanvasRenderingContext2d`;
    `Canvas::from_selector` binds to a DOM element, `Canvas::draw` dispatches to a `Draw`.
  - `draw.rs` — `Draw` trait: implemented by anything that can render itself onto a `Canvas`.
  - `macros.rs` — internal `builder_fn!`/`builder_fns!` macros for generating chainable
    "with"-style setter methods, one at a time or as a whole `impl` block, with an
    auto-generated docstring skeleton around a one-clause description per field (used by
    the `primitives/` modules and `styled.rs`); not part of the public API.
  - `primitives.rs` + `primitives/` — the `Draw` shape primitives, each re-exported from
    `primitives.rs`.
    - `primitives/polyline.rs` — `Polyline`: a `Draw` primitive rendering a connected
      sequence of points.
    - `primitives/circle.rs` — `Circle`: a `Draw` primitive rendering a circle, with
      `fill`/`stroke` booleans toggling whether each is drawn.
    - `primitives/line.rs` — `Line`: a `Draw` primitive rendering a single straight, stroked
      segment.
    - `primitives/rect.rs` — `Rect`: a `Draw` primitive rendering an axis-aligned rectangle,
      with `fill`/`stroke` booleans toggling whether each is drawn.
  - `styled.rs` — `Style`/`Styled`: chainable stroke/fill styling
    (`Styled::new(shape).stroke(..).line_width(..).fill(..)`) layered on top of any `Draw`
    shape.
  - `lib.rs` — re-exports the public API: `Canvas`, `Draw`, `Polyline`, `Circle`, `Line`,
    `Rect`, `Style`, `Styled`.
- **`demo/`** — a separate `jarsdraw-demo` crate plus a webpack/JS front end (`demo/index.js`,
  `demo/index.html`, `demo/bootstrap.js`) that exercises the library: a responsive grid of
  showcase canvases, each backed by its own standalone `#[wasm_bindgen]` demo module in
  `demo/src/` (e.g. `circle.rs` → `CircleCanvas`) that draws and resizes independently, wired
  up via `wasm-bindgen`. Current showcases: `star.rs` (a closed `Polyline` star), `circle.rs`
  (a filled/stroked `Circle`), `line.rs` (a single `Line` segment), and `rect.rs` (a
  filled/stroked `Rect`).

## Conventions

- Every public function/type/trait carries a `///` docstring with `# Parameters`, `# Returns`,
  `# Errors`/`# Panics`, and `# Side Effects` sections where applicable (see any file in
  `jarsdraw/src/` for the pattern). Trait methods and their impls are each documented
  individually — the impl's doc describes what that specific implementation does.
- Keep this file and `README.md` in sync with `jarsdraw/src/lib.rs`'s module list whenever a
  module is added, renamed, or removed.

## Commands

- `cd jarsdraw && cargo check` / `cd demo && cargo check` — type-check both crates.
- `cd jarsdraw && wasm-pack build --target bundler` — build the library crate to `pkg/`.
- `cd jarsdraw && wasm-pack test --firefox` — run the library's wasm-bindgen tests.
- `cd demo && npm start` — build the Rust WASM, watch for changes, and serve the demo page.

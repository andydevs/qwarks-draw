# Jarsdraw

A Rust/WebAssembly canvas drawing library for the browser, built with `wasm-bindgen`.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Adding as a dependency

Add `jarsdraw` to your `Cargo.toml`:

```toml
[dependencies]
jarsdraw = { path = "/path/to/jarsdraw" }
```

## Usage

### Drawing

Bind a `Canvas` to a `<canvas>` element, build a shape, and draw it:

```rust
use jarsdraw::{Canvas, Polyline, Styled};

let canvas = Canvas::from_selector("#my-canvas")?;

let line = Styled::new(Polyline::new(&[(0.0, 0.0), (100.0, 100.0), (100.0, 0.0)]))
    .stroke("red")
    .line_width(3.0);

canvas.draw(&line);
```

- `Draw` is the trait implemented by every drawable primitive (currently `Polyline`, `Circle`,
  and `Line`).
- `Circle` and `Line` each take chainable `fill(bool)`/`stroke(bool)` toggles controlling
  whether they're filled and/or stroked when drawn.
- `Styled::new(shape)` wraps any `Draw` value so its stroke color, line width, and fill color
  can be set via chained calls before drawing.

### Building the WASM Package

Build the library with `wasm-pack` targeting your preferred bundler:

```sh
wasm-pack build --target bundler
```

This outputs a `pkg/` directory containing the compiled `.wasm` file and JavaScript bindings, which can be imported in any bundler-based web project.

### Demo Application

The `demo/` directory contains a full browser demo showing a responsive grid of canvases, each drawing a different shape.

```sh
cd demo
npm install
npm start
```

This builds the Rust WASM code, starts a file watcher to rebuild on changes, and serves the page with webpack-dev-server.

### Testing

```sh
wasm-pack test --firefox
```

# MillCreator

Open-source CNC mill design and control software, written in Rust. **Early skeleton: there are no milling features yet.**

It shares its core and its interface code with [LightCreator](https://github.com/michalsarna/LightCreator) (laser)
through [baseCreator](https://github.com/michalsarna/baseCreator), pinned to a tag in `crates/millcreator/Cargo.toml`.

## What it does today

* Opens a window with the shared colour schemes and fonts.
* Opens an SVG, PDF or Illustrator (PDF-compatible) file (File menu, or the path on the command line) and shows its
  outlines. Drag to pan, the wheel zooms.

Drawing and editing shapes, layers, tool paths, G-code and machine control are not there yet; see [FEATURES.md](FEATURES.md).

## Building

```sh
cargo run -p millcreator                       # start empty
cargo run -p millcreator -- drawing.svg        # open a file
cargo test --workspace
```

Add `--features camera` for live camera capture. On Linux the GUI needs the usual X11/Wayland and OpenGL libraries
(`libxkbcommon-x11`, `libgl1`).

To work on baseCreator and MillCreator together, point Cargo at your checkout of baseCreator (do not commit this file):

```toml
# .cargo/config.toml
[patch."https://github.com/michalsarna/baseCreator"]
base-core = { path = "../baseCreator/crates/base-core" }
base-ui   = { path = "../baseCreator/crates/base-ui" }
```

## Licence

Either of [Apache License 2.0](LICENSE-APACHE) or the [MIT license](LICENSE), at your option.

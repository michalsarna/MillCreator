# MillCreator

Open-source CNC mill design and control software, written in Rust. **Early skeleton: there are no milling features yet.**

It shares its core and its interface code with [LightCreator](https://github.com/michalsarna/LightCreator) (laser)
through [baseCreator](https://github.com/michalsarna/baseCreator), pinned to a tag in `crates/millcreator/Cargo.toml`.

## What it does today

* Opens a window with the shared colour schemes, fonts and icons, a menu bar (File, Edit, Arrange, View, Settings, Help), a tool palette on the left and an arrange bar on the right.
* Opens an SVG, PDF or Illustrator (PDF-compatible) file (File menu, or the path on the command line).
* Draws and edits shapes on a work area with the editor of baseCreator: select, node edit, rectangle, ellipse, triangle,
  star, polygon, heart, spiral, line, pen, pan and zoom; selection handles to move, resize, turn and slant; the usual
  commands (undo and redo, select all, copy, paste, duplicate, delete, group, ungroup); grid and snapping (View menu and View options…).
  Keyboard: tool letters as in LightCreator (V, N, R, E, ...), Ctrl+Z / Y, Ctrl+A / C / V / D / G, Delete.

There is no project file, no layers, no tool paths, no G-code and no machine control yet; see [FEATURES.md](FEATURES.md).
The work area is a fixed 300 x 200 mm.

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

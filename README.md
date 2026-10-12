<p align="center"><img src="assets/icons/icon-256.png" alt="MillCreator icon" width="128"></p>

# MillCreator

Open-source CNC mill design and control software, written in Rust. **Early skeleton: there are no milling features yet.**

It shares its core and its interface code with [LightCreator](https://github.com/michalsarna/LightCreator) (laser)
through [baseCreator](https://github.com/michalsarna/baseCreator), pinned to a tag in `crates/millcreator/Cargo.toml`.

## What it does today

It looks like LightCreator, because the window and its parts come from baseCreator:

![The start screen](docs/screenshots/01-start.png)
![An SVG file open in the editor](docs/screenshots/02-editor-properties.png)

* The window has its own flat frame and title bar, a start screen (new design, open a file, recent files, language and
  colour scheme), a menu bar (File, Edit, Arrange, View, Settings, Help; the native menu bar on macOS), a control bar,
  a tool palette on the left, an arrange bar and a Properties tab on the right, and a status bar.
* 30 languages for the shared texts and the colour schemes of LightCreator (Settings menu); the choice is kept.
* Opens an SVG, PDF or Illustrator (PDF-compatible) file (File menu, drag and drop, or the path on the command line).
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

## Icon

`assets/icons/millcreator.svg` is the icon: the orange background and the brown base of the baseCreator icon, with an end
mill going into the base. `tools/make-icons.sh assets/icons/millcreator.svg millcreator` renders the PNGs, the `.ico` and
the `.icns` (needs inkscape; the `.icns` needs macOS). The screenshots are made by
`cargo test -p millcreator render_screenshots -- --ignored`.

## Licence

Either of [Apache License 2.0](LICENSE-APACHE) or the [MIT license](LICENSE), at your option.

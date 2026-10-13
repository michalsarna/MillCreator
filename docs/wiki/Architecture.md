🌐 **English** · [Polski](Architektura)

# Architecture

MillCreator is one binary crate, `crates/millcreator`, on top of **baseCreator** (`base-core`: geometry, scene, SVG / PDF / LightBurn readers, polygon offset, the project container; `base-ui`: window, editor, clipart, camera). baseCreator is pinned by git tag.

| Module | What it does |
|---|---|
| `tools`, `tools_ui` | the bit library: kinds, dimensions, ready-made bits, files; the window and the pictures of the bits |
| `materials` | the material and its thickness |
| `device`, `device_ui` | devices, their files, the start screen list, the window, the camera tab |
| `layers`, `layers_ui` | the 16 layers, operations, passes; the Layers and Material tabs |
| `toolpath` | from shapes to moves and G-code (uses `base_core::ops` for offsets) |
| `view3d`, `preview_ui` | the projection, drawing and controls of the 3D view; the window and the export |
| `section` | the `mill` section of the project file |
| `main`, `menu` | the application, the menus and the project open / save |

Logic modules have no UI in them and are covered by unit tests; the windows are tested by drawing them headlessly with `egui_kittest`. `cargo test -p millcreator render_screenshots -- --ignored` renders the pictures of this wiki.

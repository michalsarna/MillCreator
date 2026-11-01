🌐 **English** · [Polski](Architektura)

# Architecture

MillCreator is one binary crate, `crates/millcreator`, on top of **baseCreator** (`base-core`: geometry, scene, SVG / PDF / LightBurn readers, polygon offset, the project container; `base-ui`: window, editor, clipart, camera). baseCreator is pinned by git tag.

| Module | What it does |
|---|---|
| `tools`, `tools_ui` | the bit library: kinds, dimensions, ready-made bits, files; the window and the pictures of the bits |
| `materials`, `setup_ui` | the material, its thickness and the zero on it; the window of the project setup |
| `device`, `device_ui`, `wcs_pick` | devices, their files, the start screen list, the settings window, the camera tab; the zero of the program picked or dragged on the work area |
| `layers`, `layers_ui`, `layer_presets`, `cutting_calc` | the 64 layers and their operations; the Layers and Material tabs; the layer presets; Vc and the feed per tooth |
| `toolpath`, `passes`, `clear`, `handtabs`, `relief`, `model3d` | from shapes to moves and G-code (uses `base_core::ops` for offsets): passes, ramps and tabs, raster, rest and face, tabs placed by hand, pictures as reliefs, 3D models as depth maps |
| `view3d`, `preview_ui` | the 3D view of the job; the preview tab, the export of G-code and the Preview G-code text |
| `panes_glue`, `workspaces` | the panes of tabs of the work area and the open drawings as tabs |
| `machine_ui`, `duet_glue` | the link to the controller and the buttons that use it; a Duet board over HTTP |
| `section`, `ugs_import` | the `mill` section of the project file; the layers of a UGS design |
| `main`, `menu`, `native_menu` | the application, the menus (the common layout of `base_ui::menu::standard_menus`), the macOS menu bar, the keys and the project open / save |

Logic modules have no UI in them and are covered by unit tests; the windows are tested by drawing them headlessly with `egui_kittest`. `cargo test -p millcreator render_screenshots -- --ignored` renders the pictures of this wiki.

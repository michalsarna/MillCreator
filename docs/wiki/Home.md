🌐 **English** · [Polski](Home-PL)

<p align="center"><img src="https://raw.githubusercontent.com/michalsarna/MillCreator/main/assets/icons/icon-256.png" alt="MillCreator" width="128"></p>

# MillCreator

Open-source software to design and prepare CNC milling jobs, written in Rust. Draw or import shapes, give each layer an operation and a bit, look at the job in 3D and write G-code, on macOS, Windows and Linux. It is the sibling of [LightCreator](https://github.com/michalsarna/LightCreator) (laser) and uses the same window, editor and files through [baseCreator](https://github.com/michalsarna/baseCreator).

![Layers and the editor](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/05-layers.png)

## Where to start

| I want to… | Go to |
|---|---|
| Install the program | [Installation](Installation) |
| Prepare my first job | [Quick Start](Quick-Start) |
| Draw, edit and import shapes | [Design Tools](Design-Tools) |
| Add my machine and a camera | [Devices and Cameras](Devices-and-Cameras) |
| Say what is milled and how | [Layers and Operations](Layers-and-Operations) |
| Choose or define a bit | [Bit Library](Bit-Library) |
| Set the material | [Material](Material) |
| See the job in 3D, export G-code | [3D View and G-code](3D-View-and-G-code) |
| Drive the machine: jog, zero, probe, firmware, calibration | [Machine Control](Machine-Control) |
| Find a menu entry and what it does | [Menus](Menus) |
| Learn the keys | [Keyboard Shortcuts](Keyboard-Shortcuts) |
| Share a project with LightCreator | [Project File](Project-File) |
| Set the material and the zero of a new job | [Quick Start](Quick-Start) (Project setup) |
| See what it looks like | [Screenshots](Screenshots) |
| Find the settings file | [Settings and Files](Settings-and-Files) |
| Read about the code | [Architecture](Architecture) |
| Know what is tested | [Safety and Limitations](Safety-and-Limitations) |

## At a glance

* **Devices:** your machines (work area, travel in Z, safe height, spindle range, cameras) in one list; **I have no device** gives a stand-in called Dummy.
* **Layers:** 64 layers, each with an operation (cut outside / inside, follow the line, pocket by rings or raster, rest pocket, face, drill, bore, chamfer, dogbone slot, V-carve with inlays, pictures and 3D relief), a bit, depth, pass depth, feeds, spindle speed, tabs, ramps or a helix, climb or conventional, stock to leave, coolant; the settings can be kept as presets.
* **Bits:** a library of ready-made bits and your own (kept for every device), seven kinds, each with a picture that shows where every dimension is measured; import and export.
* **Material:** wood in three colours, metal, plastic in a colour you choose, and the thickness.
* **3D view:** the block of material, the cuts and the moves at the safe height; turn, zoom, reset; a slider plays the job.
* **G-code:** GRBL flavour, with a pause to change the bit or `T… M6` for a tool changer, your own lines (start, end, change of bit or operation), a work coordinate system and a zero on the material; Mach3 / Mach4 / UCCNC and LinuxCNC through plugins.
* **The machine:** GRBL (0.7 to 1.1 and newer), Smoothieware, or a Duet over HTTP; jogging, probing, the centre of a hole, overrides, saved positions, coolant.
* **Drawing:** DXF, SVG, PDF, Illustrator, LightBurn and UGS import, arrays, copies along a path, text on a path, trimming, nesting, QR codes and barcodes, serial text, and STL / OBJ / 3MF models as a relief.
* **Shared with LightCreator:** the editor, 310 clipart pictures, SVG / PDF / Illustrator and LightBurn import, the `.cre` project file, 30 languages (shared texts) and 14 colour schemes.

> ⚠️ **Safety:** milling machines are dangerous. Test a new job in the air above the material, wear eye and ear protection, clamp the work, never leave a running machine unattended. See [Safety and Limitations](Safety-and-Limitations).

Latest release: **v0.5.3** · [Releases](https://github.com/michalsarna/MillCreator/releases) · [Issues](https://github.com/michalsarna/MillCreator/issues) · Licence: [MIT](https://github.com/michalsarna/MillCreator/blob/main/LICENSE) or [Apache-2.0](https://github.com/michalsarna/MillCreator/blob/main/LICENSE-APACHE), at your option

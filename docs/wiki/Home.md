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
| Share a project with LightCreator | [Project File](Project-File) |
| See what it looks like | [Screenshots](Screenshots) |
| Find the settings file | [Settings and Files](Settings-and-Files) |
| Read about the code | [Architecture](Architecture) |
| Know what is tested | [Safety and Limitations](Safety-and-Limitations) |

## At a glance

* **Devices:** your machines (work area, travel in Z, safe height, spindle range, cameras) in one list; **I have no device** gives a stand-in called Dummy.
* **Layers:** 16 layers, each with an operation (cut outside, cut inside, follow the line, pocket, drill), a bit, depth, pass depth, feeds and spindle speed.
* **Bits:** a library of ready-made bits and your own (kept for every device), seven kinds, each with a picture that shows where every dimension is measured; import and export.
* **Material:** wood in three colours, metal, plastic in a colour you choose, and the thickness.
* **3D view:** the block of material, the cuts and the moves at the safe height; turn, zoom, reset; a slider plays the job.
* **G-code:** GRBL flavour, with a pause to change the bit.
* **Shared with LightCreator:** the editor, 310 clipart pictures, SVG / PDF / Illustrator and LightBurn import, the `.cre` project file, 30 languages (shared texts) and 14 colour schemes.

> ⚠️ **Safety:** milling machines are dangerous. Test a new job in the air above the material, wear eye and ear protection, clamp the work, never leave a running machine unattended. See [Safety and Limitations](Safety-and-Limitations).

Latest release: **v0.2.4** · [Releases](https://github.com/michalsarna/MillCreator/releases) · [Issues](https://github.com/michalsarna/MillCreator/issues) · Licence: [MIT](https://github.com/michalsarna/MillCreator/blob/main/LICENSE) or [Apache-2.0](https://github.com/michalsarna/MillCreator/blob/main/LICENSE-APACHE), at your option

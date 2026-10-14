# Versions

Each release gets the next version number. Newest first.

## v0.2.2

**The machine**
* MillCreator can **connect to a GRBL machine**: the device has a *Connection* page (serial port or network socket, baud rate, streaming, homing or clearing an alarm after connecting) and the Device tab has Connect, Home, Unlock, Pause, Resume, the state, the position and a console.
* The control bar has **Frame** (the tool goes round the box of the work at the safe height, spindle off), **Start** / **Stop** of the job and **Save G-code**, as in LightCreator.
* The device has a **Rotary** page (chuck or rollers, workpiece diameter, microsteps per turn); a drawing higher than the way round the workpiece is reported.

**Preview**
* The control bar has **Toolpaths** (the paths over the work area), **Preview…** and **Camera view** (a window with the live camera).
* The preview window is the same as PenCreator's and has an **animation** (back to the start, play / pause, speed, repeat) and a choice of **3D** or **From above** (the plan of the job).

**Interface**
* The pointer is the tool's only over the work area; the line of the side panel and the line between the list of layers and their options light up red and thicken under the pointer, and the latter is dragged to resize the list.
* baseCreator v0.9.0.

## v0.2.1

* MillCreator's own texts are translated into the 30 languages; warnings of the toolpaths too.
* **New** and **Open…** on the start screen without any device use the Dummy.
* The device section of the start screen is tidied and centred: *Add device…*, *Edit…*, *Duplicate*, *Import…* and, in a second row, *I have no device*; *Export…* and *Delete* are in the window of *Edit…*.
* baseCreator v0.8.1.

## v0.2.0

**Milling**
* **Devices** with a list on the start screen (add, edit, duplicate, export, import, delete), work area and travel in Z, safe height, spindle range, up to eight cameras and a live Camera tab; **I have no device** makes the stand-in Dummy.
* **Layers** (16) with operations: cut outside the line, cut inside the line, follow the line, pocket and drill, each with a bit, depth, pass depth, step over, feeds and spindle speed.
* **Bit library** with ready-made bits and your own (kept for every device): seven kinds with a picture that shows where each dimension is measured; import and export.
* **Material** tab: wood in three colours, metal, plastic in a colour you choose, thickness.
* **3D view of the job** and **G-code export** (Milling menu): the block of material, the cuts and the moves at the safe height; turn, zoom, reset; a progress slider; GRBL-style G-code with a pause to change the bit.
* The project (`.cre`) keeps a `mill` section: layers, material, a snapshot of the device and the bits used.
* Description tab (from baseCreator 0.8.0); LightBurn projects are imported with **File → Import**.
* The wiki (English and Polish) and the screenshots page.

## v0.1.1

* The shared `.cre` project file and opening LightBurn projects (baseCreator 0.7.0).

## v0.1.0

* First release: the window, the editor and the clipart gallery of baseCreator, SVG and PDF import.

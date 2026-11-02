🌐 **English** · [Polski](Szybki-start)

# Quick Start

A first job from the start: a device, the material, a shape, a layer with a bit, a look at the job in 3D and in G-code, and the file for the machine. The labels in **bold** are those of the program; Ctrl is Cmd on macOS.

## 1. The device

The start screen lists your machines under **Device**. Pick the one you will work with.

* **Add device…** makes a new one: the name, the units, the machine zero, the work area, the safe height, the spindle… (all the pages are in [Devices and Cameras](Devices-and-Cameras)). A new device can begin from **Start from → A usual machine size…**.
* **Edit…**, **Duplicate** and **Import…** change, copy or read a device from a file.
* Without a machine press **I have no device**: a stand-in called *Dummy* is made and selected (a work area of 300 × 200 mm with 60 mm in Z, safe height 5 mm, spindle 1000–24000 rpm). **New** or **Open…** with no device at all make and use it too.

Then press **New** (or **Open…** for a file you have; **Open recent** lists the last ones).

## 2. The project setup

After **New** the window **Project setup** asks about the piece of material (it opens again from **File → Project setup…**):

| Field | What it means |
|---|---|
| **Material width**, **Material height** | The size of the piece (at first the work area, at most 300 × 200 mm). |
| **Thickness** | The thickness of the piece (0.1 to 500 mm). |
| **Zero of the program** | Where X 0, Y 0 is on the piece: **Front left**, **Front right**, **Back left**, **Back right** or **Centre**. |
| **Z counts from the bottom of the material** | Off: Z = 0 is the top of the material. On: every Z is the height above its bottom. |
| **Work area is the material** | Makes the work area of the device as big as the piece. |

The material lies at the top left of the page. **Apply** writes the program for the work coordinate system **G54** (or the one chosen in the device) with its zero at the place you chose: set that zero on the machine at that place of the piece (jog there and press **X0**, **Y0** and **Z0** or **Zero all** on the **Control** tab, or probe). **Skip** leaves the zero of the device. The choices are saved with the project.

## 3. The material

The **Material** tab: **Material of the workpiece** (**Wood, light**, **Wood, medium**, **Wood, dark**, **Metal**, **Plastic** with a **Colour** you choose) and the **Thickness**. The colour is only for the 3D view; the feeds come from the bits and the layers. **First pass** can start every layer with a shallow pass along the edges. See [Material](Material).

## 4. A shape

* Draw with the tools on the left: **Rectangle (R)**, **Ellipse (E)**, **Polygon (G)**, **Text (T)**, **Polyline / pen (P)**… (the letter is the key; the same tools are in the **Insert** menu). A new shape goes on the active layer.
* Or bring one in: **File → Open…** (SVG, PDF, Illustrator, DXF), **File → Import ▸** (an image, DXF, a LightBurn project, a UGS design, a 3D model as a relief), the **Clipart** tab, or drop a file on the window.
* The **Properties** tab sets the position and the size of the selection. See [Design Tools](Design-Tools).

## 5. Layers: the operation, the bit, the depths, the feeds

A shape is milled the way its layer says. Open the **Layers** tab. The first layers of a new project are ready:

| Layer | Operation | Depth | Bit |
|---|---|---|---|
| *Cut out* | **Cut outside the line** (the part stays) | 10 mm | Flat 3.175 mm (1/8") |
| *Holes and windows* | **Cut inside the line** (the inside falls out) | 10 mm | Flat 3.175 mm (1/8") |
| *Pocket* | **Pocket** (the area is cleared) | 3 mm | Flat 3.175 mm (1/8") |
| *Engrave* | **Follow the line** | 1 mm | V-bit 60° |
| *Drill* | **Drill** (a hole at the centre of each closed shape) | 10 mm | Drill 3 mm |

1. **Put the shape on a layer.** Select it, click the layer in the list and press **Put the selected shapes on this layer**, or click the colour of the layer in the row of colours under the work area (a double click there opens the options of the layer). The list shows the layers that have shapes and the active one; **Show all 64 layers** shows the others.
2. **The operation:** **Operation** has eleven of them (profiles, pocket, drill, face, bore, chamfer, V-carve, dogbone slot, picture…): [Layers and Operations](Layers-and-Operations).
3. **The bit:** choose it in **Bit**. Choosing a bit also sets the **Feed**, **Plunge feed**, **Spindle speed**, **Pass depth** and **Step over** from its starting speeds. The **…** button opens the **Bit library** ([Bit Library](Bit-Library)).
4. **The depths:** **Depth** is the total depth below the top of the material; **Pass depth** is how deep one pass goes (0 = the full depth in one pass). The line under the settings says how many passes that makes.
5. **The feeds:** **Feed** and **Plunge feed** in mm/min, **Spindle speed** in rpm. Under them, worked out from the diameter and the flutes of the bit: **Cutting speed (Vc)** in m/min and **Feed per tooth (fz)** and **Plunge per tooth** in mm. Typing a Vc changes the spindle speed, typing an fz changes the feed; only the spindle speed and the feeds are saved.
6. **Tabs** keep a part that is cut out in its place: set them on a *Cut outside the line* layer, or place them by hand.

The tick in the list takes a layer into the toolpaths and the G-code; the eye shows or hides it. A good setting can be kept with **Save preset…** and put on another layer with **Load preset**.

## 6. Look at the job

* **Toolpaths** (the green button above the work area) draws the paths of the bit over the drawing.
* **Preview…** or **Milling → 3D view of the job…** (Ctrl+Shift+P on Windows and Linux) opens the **Preview** tab: the block of material with the cuts carved in it, the moves at the safe height, an animation with a slider, the time and the length of the cuts, and the warnings (a layer without a bit, deeper than the material, outside the work area…). See [3D View and G-code](3D-View-and-G-code).
* **Preview G-code** shows the program itself in a tab (*Preview G-code: name.nc*), coloured, without saving anything. The text can be edited; **Save G-code** in the tab writes it, **Make again** makes it from the project again, **Analyse** shows its toolpaths.

## 7. Save the project and the G-code

* **File → Save** (Ctrl+S) keeps the project as a `.cre` file.
* **Save G-code** (the button above the work area), **Milling → Export G-code…** or **Ctrl+E** (Windows and Linux; see [Keyboard Shortcuts](Keyboard-Shortcuts#macos) for macOS) asks for a file (`name.nc`) and writes the G-code, through the postprocessors of plugins that are on. With **Settings → View options… → Preview G-code after saving** the saved file opens in the G-code analysis.
* The text of the **Description** tab goes into the G-code as `(Note: …)` comments at the top.

## 8. On the machine

The program can also send the job to a GRBL machine (tested on a TwoTrees TTC450Pro, not on other machines):

1. Set the port in **Machine → Device settings… → Connection**, then **Connect** on the **Device** tab.
2. **Home**, then jog to the zero of the program and set it on the **Control** tab (**Zero X and Y**, **Z0**, or **Probe Z**).
3. **Frame** goes round the box of the job at the safe height with the spindle off: check that the job fits the material.
4. **Start** sends the job. **STOP** stops it; **Pause** and **Resume** hold it. **From line…** starts at a line of the program.

See [Machine Control](Machine-Control).

> ⚠️ Test a new job in the air above the material first, wear eye and ear protection, clamp the work and stay at the machine. See [Safety and Limitations](Safety-and-Limitations).

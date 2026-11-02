🌐 **English** · [Polski](Widok-3D-i-G-code)

# 3D View and G-code

## Toolpaths
From the shapes, the layers, the bits and the device the program makes the moves of the machine. Coordinates are those of the work area: X to the right, Y away from you (the drawing is turned over, because the editor counts Y downwards), Z = 0 at the top of the material and negative below it (or, after **Project setup** with **Z counts from the bottom of the material**, the height above its bottom).

* **Profiles** go round the shape at the radius of the bit, in passes down to the depth of the layer; the next pass follows without lifting the bit.
* **Pockets** are cleared in rings from the outline inwards, the step over apart, or by a raster (see [Layers and Operations](Layers-and-Operations)).
* **Follow the line** goes along the line and back, pass after pass.
* **Drill** pecks down in steps of the pass depth and comes out to the surface in between.
* Between cuts the bit rises to the **safe height** and moves there at the speed of moves above the material, then goes down fast to 1 mm over the surface and plunges at the plunge feed.

**Toolpaths** (the green button in the bar above the work area) draws the cuts over the drawing in the colour of their layer and the moves above the material in blue-grey; press it again to hide them. The paths follow every change of the drawing.

## The preview
**Preview…** (the button above the work area) or **Milling → 3D view of the job…** (Ctrl+Shift+P) opens the **Preview** tab, titled *Preview: name of the project*. It is a tab of the work area like the drawings: drag it to another pane, or right-click it and choose **Open in a separate window** (**Back to the main window** puts it back). Its cross closes it. The preview follows the active drawing and makes the program again after every change of the project.

It shows only the area of the job, **10 % larger than the job** in each direction, as a block of the chosen material with its thickness.

![3D view](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/16-3d-view.png)

| Control | What it does |
|---|---|
| **3D** / **From above** | the 3D picture, or the plan of the job seen from straight above (it cannot be turned) |
| **Reset view** or a double click | back to the first view |
| the two fit icons | **Fit work area** and **Fit all objects** |
| **Material**, **Cuts**, **Moves at the safe height** | show or hide each part; the blue frame is the safe height |
| the download icon | **Export the milled piece as STL…** (see below) |
| left drag | turn the view |
| right or middle drag | move the picture |
| mouse wheel | zoom |
| the animation | **Back to the start**, **Play** / **Pause**, the slider, the **Animation speed** (0.25× to 25×) and **Repeat**; the job is shown up to the slider, with the bit marked |

**Material** is the stock as it is milled: the grooves of the bits are carved into its top as far as the slider has got, with the real shape of each bit (a V-bit gets wider with depth). **Cuts** adds the paths of the bit, in the colour of their layer, over it. Under the buttons: the **Time** up to the slider and in all, the length of the cuts with an estimate of the time, and the warnings of the job (a layer without a bit, deeper than the material or the travel of the machine, a spindle speed out of range, outside the work area, a line too small for the bit…).

![Turned view](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/17-3d-view-turned.png)

The carved block is a picture on a grid (finer for a small job), and the time is an estimate from the feeds, not a simulation of the controller.

**Export the milled piece as STL…** saves the stock with the grooves in it, as far as the slider has got (at the end: the whole job), as a binary STL model in millimetres. **Milling → Export relief as STL…** is the other export: the relief of a picture alone.

**With a rotary attachment** (device settings → Rotary, switched on) the preview shows the workpiece as a **cylinder** along X: the program's Y is the way round its surface, so a drawing is wound round it, the middle of the job facing up, and cuts and the moves at the safe height follow the surface. The cuts go down towards the axis. The picture is the whole way round, so a job higher than the circumference shows twice over the same place (and the program warns). The milled piece is not exported as STL for a rotary (it would be a flat plate).

## The G-code as text: Preview G-code

**Preview G-code** (next to **Save G-code** above the work area) makes the program of the project, through the postprocessors that are on, and shows it **as text in a tab** (*Preview G-code: name.nc*), without writing a file. It is the same program **Export G-code…** writes, starting with `(MillCreator: name of the project)`. A project has one such tab; pressing the button again shows it.

* The text is **coloured** (the G and M words, the axes, the speeds and the tools, the line numbers, the comments) and **can be edited** by hand. A tab with edits is marked with a dot, and *Changed by hand* stands in its bar.
* **Save G-code** in the tab writes the text as it is to a file.
* **Make again** makes it from the project again. While there are edits, the first click asks **Lose the edits?** and the second makes it.
* **Analyse** shows the toolpaths of the text in the **G-code analysis** tab.
* While it has no edits, the text **follows the project** by itself, half a second after the last change: when the project is saved, when the text of the **Description** tab (the notes) changes, when a setting of the device changes (its own G-code too) and when a postprocessor is switched on or off. Edits by hand are never thrown away by this; changes of the drawing appear after the next save, or with **Preview G-code** or **Make again**.
* Closing a tab with edits asks **Save your work before closing?**: **Save**, **Discard** or **Cancel**.

The tab can be moved to another pane or into a window of its own like the preview.

## Export G-code
**Save G-code** (the button above the work area), **Milling → Export G-code…** (Ctrl+E) or **File → Export ▸ Export G-code…** asks for a file (`name.nc`; `.gcode` and `.ngc` can be chosen) and writes the program. The status bar says where it went and repeats the warnings of the job. With **Settings → View options… → Preview G-code after saving** (off by default) the saved file then opens in the **G-code analysis**.

The file is GRBL-style:

* the header comments `(MillCreator: name)` and `(Device: name)`, then one `(Note: …)` line for every line of the **Description** tab of the project (brackets and semicolons are changed so that the comment stays whole; at most 60 lines of 100 characters);
* `G21 G90 G17`, the work coordinate word (`G54` … `G59`) when one is chosen, and the device's own G-code for the start;
* for every layer a `(Layer: name)` comment; the spindle as `M3 S…` and, when it starts, a wait `G4 P…` of the device; `G0` for the moves above the material and `G1` with the feed for plunges and cuts;
* at the end: up to the safe height, `M5`, back over the zero, the device's own G-code for the end, and `M30`.

When the next layer uses another bit, the file stops with `M0 (Change the bit to: …)` after the spindle is switched off. With a **tool changer** the change is `T<n> M6` instead (the bit needs its **Changer number** in the library). **Coolant** (`M7` / `M8`, `M9`) and the **dust collector** lines follow the layers. With a **project setup** the program counts from the zero you chose (in `G54` unless the device has another code); with *Z counts from the bottom of the material* every Z is the height above the bottom. The device's own G-code, the very start and end of the file, its variables and **Own file format** are on the **G-code** page of the device settings ([Devices and Cameras](Devices-and-Cameras)). A **postprocessor** from a plugin (for example Mach3 / Mach4 / UCCNC or LinuxCNC from *G-code dialects*) rewrites the file at the end. A job started with **Start** is sent without the postprocessors.

The moves of every operation are in the file and in the preview exactly as they are written: ramps, helices, tabs, the rings of a V-carve at their depths.

## The G-code analysis
**File → View G-code file…** (also on the start screen, and a G-code file dropped on the window) opens any G-code file in the G-code analysis, a tab named after the file (the **G-code viewer**): its toolpath, flat or in 3D when Z changes, **Show travel moves**, **Colour by height**, the length of the cuts and the time. Drag to move (in 3D: to turn; a right drag or Shift+drag moves), the wheel zooms, Esc closes it.

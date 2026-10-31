🌐 **English** · [Polski](Widok-3D-i-G-code)

# 3D View and G-code

## Toolpaths
From the shapes, the layers, the bits and the device the program makes the moves of the machine. Coordinates are those of the work area: X to the right, Y away from you (the drawing is turned over, because the editor counts Y downwards), Z = 0 at the top of the material and negative below it.

* **Profiles** go round the shape at the radius of the bit, in passes down to the depth of the layer; the next pass follows without lifting the bit.
* **Pockets** are cleared in rings, from the outline inwards, the step over apart.
* **Follow the line** goes along the line and back, pass after pass.
* **Drill** pecks down in steps of the pass depth and comes out to the surface in between.
* Between cuts the bit rises to the **safe height** and moves there at the speed of moves above the material, then goes down fast to 1 mm over the surface and plunges at the plunge feed.

## The 3D window
**Milling → 3D view of the job…** (Ctrl/Cmd+Shift+3). It shows only the area of the project, **10 % larger than the project** in each direction, as a block of the chosen material with its thickness.

![3D view](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/16-3d-view.png)

| Control | What it does |
|---|---|
| **3D** / **From above** | the 3D picture, or the plan of the job seen from straight above (it cannot be turned) |
| drag | turn the view |
| mouse wheel | zoom |
| right or middle drag | move the picture |
| **Reset view** or double click | back to the first view |
| Material, Cuts, Moves at the safe height | show or hide each part; the blue frame is the safe height |
| Animation | back to the start, play / pause, the progress slider, the speed (×0.25 to ×25) and repeat; the job is shown up to a part of its length, with the bit marked; the material shows the cuts made so far |

**Material** is the stock as it is milled: the grooves of the bits are carved into its top, as far as the slider has got, with the real shape of each bit (a V-bit gets wider with depth). **Cuts** adds only the paths of the bit, in the colour of their layer, over it. The window shows the length of the cuts, an estimate of the time and the warnings, and makes the program again after every change of the project.

![Turned view](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/17-3d-view-turned.png)

The view is a line drawing of the toolpaths on the block: it does not remove material, and the time is an estimate from the feeds, not a simulation of the controller.

## G-code
**Export G-code…** writes a GRBL-style file: `G21 G90 G17`, the spindle with `S… M3` and the wait of the device, `G0` for moves above the material and `G1` with the feed for plunges and cuts, then `M5`, the safe height, the machine zero and `M30`. The coordinates follow the **machine zero** of the device. When the next layer uses another bit the file stops with `M0 (Change the bit to: …)` after the spindle is switched off. Warnings are added to the status bar.

**With a rotary attachment** (device settings → Rotary, switched on) the preview shows the workpiece as a **cylinder** along X: the program's Y is the way round its surface, so a drawing is wound round it, the middle of the job facing up, and cuts and the moves at the safe height follow the surface. The cuts go down towards the axis. The picture is the whole way round, so a job higher than the circumference shows twice over the same place (and the program warns). The export of the milled piece as STL is not made for a rotary (it would be a flat plate).

The moves of every operation are in the file and in the preview exactly as they are written: ramps, helices, tabs, the rings of a V-carve at their depths. With a **tool changer** the change of the bit is `T<n> M6` instead of an `M0` pause; **coolant** (`M7` / `M8`, `M9`) and the **dust collector** lines follow the layers. With a **project setup** the program starts with `G54` and counts from the zero you chose; with *Z from the bottom* every Z is the height above the bottom of the material. A **postprocessor** from a plugin (for example Mach3 / Mach4 / UCCNC or LinuxCNC from *G-code dialects*) rewrites the file when it is saved.

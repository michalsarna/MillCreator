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
| drag | turn the view |
| mouse wheel | zoom |
| right or middle drag | move the picture |
| **Reset view** or double click | back to the first view |
| Material, Cuts, Moves at the safe height | show or hide each part; the blue frame is the safe height |
| Progress slider | shows the job up to a part of its length, with the bit marked; the cuts drawn are as wide as the bit |

Cuts are drawn in the colour of their layer over a groove as wide as the bit at that depth (a V-bit grows wider with depth). The window shows the length of the cuts, an estimate of the time and the warnings, and makes the program again after every change of the project.

![Turned view](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/17-3d-view-turned.png)

The view is a line drawing of the toolpaths on the block: it does not remove material, and the time is an estimate from the feeds, not a simulation of the controller.

## G-code
**Export G-code…** writes a GRBL-style file: `G21 G90 G17`, the spindle with `S… M3` and the wait of the device, `G0` for moves above the material and `G1` with the feed for plunges and cuts, then `M5`, the safe height, the machine zero and `M30`. The coordinates follow the **machine zero** of the device. When the next layer uses another bit the file stops with `M0 (Change the bit to: …)` after the spindle is switched off. Warnings are added to the status bar.

# Features

## Available now

* **Devices:** a list kept between runs; work area, travel in Z, safe height, speed of moves, spindle range and wait, machine zero, units (mm or inch), notes; up to eight cameras (address or camera of the computer, rotation) with a live Camera tab; add, edit, duplicate, delete, export and import (checked strictly); the stand-in Dummy device.
* **Layers:** 16 layers with name, colour, operation, bit, depth, pass depth, step over, feeds and spindle speed; switch each on and off, show and hide; put the selected shapes on a layer.
* **Operations:** cut outside the line, cut inside the line, follow the line (open and closed), pocket, drill with pecks.
* **Bit library:** ready-made bits and your own, kept for every device; kinds: flat end mill, ball nose, bull nose, V-bit, engraving bit, tapered ball nose, drill; a picture of every kind with the letter of each dimension where it is measured; feeds and speeds as starting points; import and export of bits as JSON; a project carries the bits it uses.
* **Material:** wood (light, medium, dark), metal, plastic (any colour), thickness.
* **Toolpaths:** offsets by the radius of the bit, passes down to the depth, rings for pockets, a safe height, estimate of the length and the time, and warnings.
* **3D view:** block of the material, cuts as wide as the bit, moves at the safe height, turn, zoom, move, reset, a progress slider with the bit marked; the shown area is 10 % larger than the project.
* **G-code** for GRBL: spindle and wait, rapid and cutting moves with feeds, the zero of the machine, a stop to change the bit (`M0`).
* **Project file** `.cre` (shared with LightCreator): the drawing, the description and the `mill` section; the sections of other programs are kept as they were.
* The window of LightCreator's look: own frame and title bar, start screen, control bar, status bar, tabs (Properties, Layers, Material, Device, Clipart, Description, Camera), the clipart gallery (310 included pictures, your own, online search), the colour schemes, fonts and the 30 languages of baseCreator for the shared texts.
* Reading SVG, PDF and Illustrator (PDF-compatible) files and LightBurn projects (`.lbrn2`: shapes, text outlines, bitmaps) into a scene of shapes.
* The editor of baseCreator: drawing tools, selection, node editing, undo and redo, copy and paste, group, align, flip, rotate, order, grid and snapping.

## Not there (yet)

The list is a plan, not a promise.

* Machine control: connection, jogging, homing, probing, streaming.
* Tabs, ramps and helical entry, climb / conventional choice, V-carving, 3D relief, tool length offsets, collision checks with clamps.
* Other controllers than GRBL, post-processor settings.
* A simulation that removes material in the 3D view.

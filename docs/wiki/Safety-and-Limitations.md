🌐 **English** · [Polski](Bezpieczenstwo-i-ograniczenia)

# Safety and Limitations

> ⚠️ A milling machine can injure you and destroy the work, the cutter and itself. This program is young and **has been tested on a TwoTrees TTC450Pro and works there, but not on other machines**. Use it at your own risk.

* **Test in the air first:** raise the machine so that the bit cannot touch anything, run the G-code and watch the path. Then run it on a cheap piece.
* Wear eye and ear protection, clamp the work, keep a hand near the stop button, never leave a running machine unattended.
* The **speeds and feeds** of the ready-made bits are cautious starting points; they are not tested on your machine.
* **Check the zero:** the G-code counts from the machine zero set in the device (which corner) and Z = 0 at the top of the material. If you used **File → Project setup…**, it counts from the zero you chose on the material (G54 with that zero: set it on the machine there) and, when you asked, Z from the bottom of the material. Set the same on your machine and look at the first moves with the bit in the air.
* **What is tested:** the geometry of the toolpaths (the distance from the line, the passes, the safe height), the bit library with its checks and files, the device files, the project file and the G-code text, by automatic tests on every change. The pictures in this wiki are rendered by the program itself.

## What it does not do
* It can **connect to a GRBL machine and stream the job** (Connect, Home, Frame, Start, Stop); this was tried on a TwoTrees TTC450Pro, **not on other machines**. Start with the spindle off and the bit in the air, and stay at the machine; the Stop button aborts with a soft reset.
* Frame goes round the box of the work at the safe height with the spindle off: check that the safe height is really clear.
* No collision check with clamps and no tool length offsets (a tool length sensor sets Z for a new bit). Tabs are placed by the program (evenly) or by hand on the work area.
* **V-carve:** the walls are exact whatever the distance of the rings, but the flat bottom of a wide shape is cut with the V-bit itself (ridges, a long program; the program says so): use a flat bit in a Pocket layer for it. The plug of an inlay is carved on the back of a piece: mirror its shapes.
* Ramps, helices, tabs and the climb / conventional choice are tested on the geometry and against the GRBL firmware in a simulator, **not on a real machine**: try a new setting on a scrap, with the spindle off first. A tab holds a part only if it is high enough: the program warns when it is as high as the cut. A helix needs room: where it does not fit, the ramp along the contour is used.
* A shape outside the work area is reported, but not moved.
* Pictures are not milled. Text is milled as its outline (a line follows it).
* The 3D view shows the material milled as far as the slider has got (a grid, finer for a small job); it is a picture to look at, not a measurement.
* The time is an estimate.

## What is new and has not been tried on a real machine
Everything below was tested by unit tests, renders, and where it changes G-code against the GRBL firmware in a simulator, **not on real controllers**. Try each with the spindle off and the bit in the air first.

* **Tool changer** (`T<n> M6` without a pause): only for controllers that know M6 (grblHAL, FluidNC); plain GRBL answers with an error. A bit without a changer number makes the machine stop as before.
* **Coolant and dust collector** per layer (M7 / M8 / M9 and the device's own lines): switched on with the layer, off before a bit change and at the end. Not run in the simulator.
* **Overrides, saved positions, Find the centre, coolant buttons:** real-time commands and jog moves in machine coordinates (`G53`). A saved position goes up first when it is higher than the tool, then across, then down: check that nothing is in the way. The centre finder is for holes and moves the probe along X and Y; start it with the probe hanging in the hole.
* **Duet / RepRapFirmware:** the board runs the uploaded program by itself; there is **no stop of the machine from here except Stop** (an `M0` through the network) and no overrides or jogging. Keep a hand on the machine's own emergency stop. Smoothieware is streamed like GRBL and is untried on a real board.
* **Postprocessors for Mach3 / Mach4 / UCCNC and LinuxCNC** (the plugin G-code dialects) were written from the manuals: read the file before you run it.
* **3D models as a relief** are looked at from above: undercuts are lost, the highest point is left uncut and what is round the model is cleared down to its lowest point.

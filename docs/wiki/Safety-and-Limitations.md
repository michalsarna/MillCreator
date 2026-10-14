🌐 **English** · [Polski](Bezpieczenstwo-i-ograniczenia)

# Safety and Limitations

> ⚠️ A milling machine can injure you and destroy the work, the cutter and itself. This program is young and **has not been run on a machine by its author**. Use it at your own risk.

* **Test in the air first:** raise the machine so that the bit cannot touch anything, run the G-code and watch the path. Then run it on a cheap piece.
* Wear eye and ear protection, clamp the work, keep a hand near the stop button, never leave a running machine unattended.
* The **speeds and feeds** of the ready-made bits are cautious starting points; they are not tested on your machine.
* **Check the zero:** the G-code counts from the machine zero set in the device (which corner) and Z = 0 at the top of the material. Set the same on your machine.
* **What is tested:** the geometry of the toolpaths (the distance from the line, the passes, the safe height), the bit library with its checks and files, the device files, the project file and the G-code text, by automatic tests on every change. The pictures in this wiki are rendered by the program itself.

## What it does not do
* It can **connect to a GRBL machine and stream the job** (Connect, Home, Frame, Start, Stop), but **none of this has been tried on a real machine**. Start with the spindle off and the bit in the air, and stay at the machine; the Stop button aborts with a soft reset.
* Frame goes round the box of the work at the safe height with the spindle off: check that the safe height is really clear.
* No jogging and no probing.
* No tabs, no ramps or helical entry (a bit plunges straight down), no climb / conventional choice, no V-carving, no 3D relief, no tool length offsets and no collision check with clamps.
* The pocket leaves a thin strip in the middle when its width is not a multiple of the step over.
* A shape outside the work area is reported, but not moved.
* Pictures are not milled. Text is milled as its outline (a line follows it).
* The 3D view draws lines on the block; it does not remove material.
* The time is an estimate.

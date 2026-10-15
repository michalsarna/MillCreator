# Features

## Available now

* **Devices:** a list kept between runs; work area, travel in Z, safe height, speed of moves, spindle range and wait, machine zero, units (mm or inch), notes; up to eight cameras (address or camera of the computer, rotation) with a live Camera tab; add, edit, duplicate, delete, export and import (checked strictly); the stand-in Dummy device.
* **Layers:** 64 layers with name, colour, operation, bit, depth, pass depth, step over, feeds and spindle speed; switch each on and off, show and hide; put the selected shapes on a layer.
* **Operations:** cut outside the line, cut inside the line, follow the line (open and closed), pocket, drill with pecks.
* **Bit library:** ready-made bits and your own, kept for every device; kinds: flat end mill, ball nose, bull nose, V-bit, engraving bit, tapered ball nose, drill; a picture of every kind with the letter of each dimension where it is measured; feeds and speeds as starting points; import and export of bits as JSON; a project carries the bits it uses.
* **Material:** wood (light, medium, dark), metal, plastic (any colour), thickness.
* **Toolpaths:** offsets by the radius of the bit, passes down to the depth, rings for pockets, a safe height, estimate of the length and the time, and warnings.
* **The machine:** a GRBL controller on a serial port or a network socket (port, baud rate, streaming, homing or clearing an alarm after connecting, reconnecting by itself); Connect, Home, Unlock, Pause / Resume, Frame (the box of the work at the safe height), Start and Stop, a **Console** tab (the traffic with the controller, a line to send a command) and the position; Save G-code on the control bar.
* **Rotary attachment:** chuck or rollers, workpiece diameter, microsteps per turn; the drawing may not be higher than the way round the workpiece (warning).
* **Toolpaths button:** the paths drawn over the work area; **Camera view** window of the live camera.
* **3D view:** block of the material, cuts as wide as the bit, moves at the safe height, turn, zoom, move, reset, from above as a plan, an animation (play, pause, speed ×0.25 to ×25, repeat) with a progress slider and the bit marked; the shown area is 10 % larger than the project.
* **G-code** for GRBL: spindle and wait, rapid and cutting moves with feeds, the zero of the machine, a stop to change the bit (`M0`).
* **Project file** `.cre` (shared with LightCreator): the drawing, the description and the `mill` section; the sections of other programs are kept as they were.
* The window of LightCreator's look: own frame and title bar, start screen, control bar, status bar, tabs (Properties, Layers, Material, Device, Clipart, Description, Camera), the clipart gallery (310 included pictures, your own, online search), the colour schemes, fonts and the 30 languages of baseCreator for the shared texts.
* Reading SVG, PDF and Illustrator (PDF-compatible) files and LightBurn projects (`.lbrn2`: shapes, text outlines, bitmaps) and UGS Designer designs (`.ugsd`: shapes, text, pictures; the cut types, depths, feeds and spindle speeds become layers) into a scene of shapes.
* **Looking for a newer release** on GitHub when the program starts (switch in Settings → Program options…) and by hand in Help → Check for updates…; it only asks which release is newest and opens the download page, nothing is installed by itself.
* **Layer bar** under the editor, the same in all programs: one row of the 64 layer colours (arrows or the wheel scroll it; a dot marks layers that have shapes). Click a colour to put the selected shapes on that layer (or, with nothing selected, to draw on it); double-click for its options.
* **Plugins** (Settings → Plugins…, shared with the other programs on baseCreator): clipart, machine presets, translations, importers of other file formats, shape generators with a form, and G-code postprocessors, as folders or zips; scripts run in a sandbox (Rhai) and the postprocessor output is checked. An **official list** of plugins (all programs' plugins, Install / Update with a checksum, updates checked only when you ask or switch it on) is in Plugins… → Official plugins.
* The editor of baseCreator: drawing tools, selection, node editing, undo and redo, copy and paste, group, align, flip, rotate, order, grid and snapping.
* **Machine control:** the **Control** tab (work and machine position, zero of X, Y, Z and G54 … G59, jogging by steps or by holding a button, keyboard jogging, go to X0 Y0 at the safe height, probing Z and the X / Y edges with a touch plate or a 3D probe with the tool diameter taken into account, macros with variables and G-code before and after every job); the job stops at a change of the bit (a window, a Z probe, continue / stop); **Start from line…**; the program can be checked in the controller's check mode first; the settings of the controller (`$$`) with search, a filter for what differs from typical, writes after a question, an automatic backup before the first change, backup to and restore from a file, factory reset behind a typed word; calibration of steps per millimetre and of the squareness of the axes; diagnostics (input pins, firmware, jobs, running time, alarms, errors, a zip with report, settings, console and program) and maintenance reminders; alarms and errors of GRBL explained in 30 languages; a device can be read from the controller (travel, spindle, speed) and started from a usual machine size; **copies of the settings of the controller** are kept with the device (made at the first connection and whenever the settings differ from the newest copy, with a message listing what changed; by hand, restore and delete from the firmware window; part of the backup of the program); a **tool length sensor** (menu Machine): its place in machine coordinates, the reference tool measured on it, and every new bit measured at a pause of the job so that it gets the Z of the reference tool. None of it has been tried on a real machine.
* **Backup:** Help → Download backup… saves a zip with the settings of MillCreator and the shared clipart gallery; Help → Restore from backup… reads it back (settings replaced after a confirmation, pictures added to the gallery, a backup of another program refused). The same entries as in LightCreator.

## Not there (yet)

The list is a plan, not a promise.

* Jogging and probing.
* Tabs, ramps and helical entry, climb / conventional choice, V-carving, 3D relief, tool length offsets, collision checks with clamps.
* Other controllers than GRBL, post-processor settings.
* A simulation that removes material in the 3D view.

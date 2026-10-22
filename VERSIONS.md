# Versions

Each release gets the next version number. Newest first.

## v0.3.6

* **Pictures are engraved:** the *Picture* operation of a layer (dithered rows, dots, depth by tone) with the dithering methods of baseCreator; a picture put on a layer switches it to that operation.
* **3D relief:** a picture as a depth map cut in passes, with the shape of the bit taken into account, and the carved piece exported as STL.
* **Preview:** *Fit to the work area* and *Fit to the elements*, as icons in the title line (like in LightCreator).
* **Plugins:** the import window has a form, a preview and a 3D view (for example the STL projection plugin); the generator window has a preview and pictures; PDF / Illustrator import can use the layers of the file. Plugins can import a KiCad board, Gerber, EAGLE and EasyEDA boards (outline, isolation, holes) from the official list.
* The Device tab uses the shared XY jog pad; the About window shows the baseCreator version.
* egui / eframe 0.37; baseCreator v1.15.0.

## v0.3.5

* **Properties tab:** when one picture is selected, an *Image* section has the buttons **Adjust image…** and **Trace image…** (shared by all programs), so tracing a picture into outlines does not have to be looked for in the menu.
* baseCreator v1.5.0.

## v0.3.4

* **Same shape tools everywhere:** the operations on shapes that LightCreator had (union, intersection, subtract, exclusive or, offset shape, round corners, convert to curves) are now in the shared editor of baseCreator and in the Arrange menu and the right-click menu of every program (Shape operations submenu).
* baseCreator v1.4.0.

## v0.3.3

* **Links open again:** the download page from the update window, Help → Project page on GitHub and the list of plugins did nothing, because the program was built without the link-opening feature of its GUI library. A test keeps it.
* **Right-click menu** on a shape (copy, paste, duplicate, delete, group, lock, flip, turn, arrange, adjust and trace a picture), shared with the other programs.
* Documentation: design tools, pictures, the side panel and the layer bar.
* baseCreator v1.3.0.

## v0.3.2

* **First pass** (Material tab): an optional shallow pass (0.1 mm by default) along the edges of every layer before its normal passes, to break the top of the material cleanly. A pocket gets it along its edge only, drilling has none; it works with every bit.
* **Pictures:** File → Import → Import image… (or drop a PNG, JPG, BMP, GIF or WebP file) with the same import dialog as LightCreator; Arrange → Adjust image… and **Trace image…** turn a picture into outlines to mill.
* **Side panel:** the tabs can be dragged into another order (remembered, also in backups), and a button collapses the panel to a narrow strip with the tab names written vertically; a click on a name opens the panel on that tab.
* **Layer bar:** more room between the colours and at both ends, so the outline of the first one is not pressed against the arrow.
* baseCreator v1.2.0.

## v0.3.1

* **Control tab redesigned:** big position readout with X0/Y0/Z0 next to it, Zero all / Zero X and Y, Go to X0 Y0 / Z to safe height, a separate Move section (XY and Z lines, a text field for any step), speeds in their own section (loaded from the device each time), bigger centred jog buttons, a slimmer Probe section and a spindle test (Switch on / Switch off).
* **One move per click:** a jog click used to be sent again on every frame while the mouse button was down; now it moves once (unless Hold to move is on).
* **Alert window:** when the machine alarms, stops or stops answering during a job, a big window explains what happened, with red **STOP** and orange **UNLOCK** buttons.
* **Spindle:** the G-code now says `M3 S…` (speed after the command), and the speed is scaled to the controller's own `$30` range when it differs from the device settings (written to the console). Not tried on a real machine; use the spindle test and send the Console log if the spindle still does not start.
* **Layer bar:** the selected layer's outline is no longer clipped, and the bar ends exactly at the work panel.
* **Layer order:** layers can be moved earlier/later in the job (chevrons in the Layers panel); the toolpath follows that order.
* **Update check:** Help → Check for updates, and an option to look for a newer version at start (Settings → Program options).
* Windows (3D view, bit library) keep their size; double Esc selects the Select tool.

## v0.3.0

* **Layer bar** under the editor, the same in all programs: one row of **64 layer colours** (arrows or the wheel scroll it; a dot marks layers with shapes). A click puts the selected shapes on that layer (or, with nothing selected, makes it the layer to draw on); a double click opens its options. One palette in all programs. Projects with fewer layers open with the rest added.
* **Autosave and recovery:** a project with a file is saved to it while you work; one without a name goes to a recovery file, offered as a big **Recover** button on the start screen, and removed at the first save.
* **Ctrl / Cmd** click and drag add to the selection, like Shift.
* Two new buttons at the bottom of the tool strip: **Generate shape** (plugins) and **Online clipart**.
* **Plugins:** an official list (Settings → Plugins… → Official plugins) with Install / Update; the Plugins window no longer grows to the height of the screen.
* **Security:** plugin zips are installed from exactly the bytes that were checked, with real size limits; backups are restored with limits; temporary files (print, ser2net) are private; rustls 0.23.46 (four advisories); build actions pinned.
* macOS releases are only the `.dmg` (no more `.tar.gz`).
* baseCreator v1.0.0.

## v0.2.9

* **Official plugin list** (Settings → Plugins… → Official plugins): all plugins of the repository *baseCreator-plugins* (also those for other programs) with **Install** and **Update**, checksum check, and a switch to look for updates at the start (off by default). A fractals plugin (Sierpinski, Koch, Hilbert, dragon curve, Pythagoras tree) and more are in the list. Guide for authors and the way to send a plugin: the plugins repository.
* baseCreator v0.9.9.

## v0.2.8

* **Plugins** (Settings → Plugins…): packs without code (clipart, presets, translations, data files), importers of other file formats, generators of shapes with a form and G-code postprocessors; scripts (Rhai) run in a sandbox, the output of a postprocessor is checked and controller commands are refused. File → Import → *Import with a plugin…* and *Generate shape…*. Shared with the other programs on baseCreator; see the Plugins wiki page.
* Plugins can bring bits (`mill-tools` data files) and machine presets of class `mill`.
* baseCreator v0.9.8.

## v0.2.7

* **Import of UGS Designer files** (`.ugsd`, File → Import → Import UGS design…, also drag and drop): shapes, rounded rectangles, ellipses, text and pictures; every way of cutting of the file (cut type, depth, feed, spindle speed) becomes the settings of a layer. Hidden objects are drawn too. Bits of the file's tool diameter are added to the library; spindle speed is a percent of the top speed of the device.
* **Imports (LightBurn and UGS) are really imports now:** the shapes are added to the open drawing, selected and can be undone with one step; the layers of the file are used only for layers that have no shapes yet. They no longer replace the project or appear in the recent files.
* baseCreator v0.9.7.

## v0.2.6

* **Machine control:** the **Control** tab (work and machine position, zero and G54 … G59, jogging by steps or by holding a button, keyboard jogging, probing Z and the edges with a touch plate or a 3D probe, macros with variables, G-code before and after every job) and the **Machine** menu (settings of the controller `$$` with search, backup before the first change, restore, factory reset; calibration; diagnostics with statistics, maintenance reminders and a zip; **Start from line…**; **Check the program**). A job stops at a change of the bit in a window with a Z probe and continue / stop.
* **Tool length sensor** (menu Machine): its place, the reference tool, and every new bit measured at a pause of the job so that it gets the Z of the reference tool.
* **Copies of the settings of the controller** are kept with the device (made at the first connection and whenever the settings differ from the newest copy, with a message saying what changed); they are part of the backup of the program.
* Alarms and errors of GRBL are explained in the console and the status bar, in 30 languages. A device can be read from the controller and started from a usual machine size.
* No more moving of settings from the old per-program folders. baseCreator v0.9.6.

## v0.2.5

* One data folder for all programs: `baseCreator` (macOS `~/Library/Application Support/baseCreator`). The clipart gallery is now **shared** by LightCreator, MillCreator and PenCreator; settings, device profiles, materials and tools stay separate per program in `apps/<program>/app.ron`.
* The first start after the update moves the old settings and pictures into the shared folder (the old folder is left untouched, identical pictures are not duplicated). `BASECREATOR_DIR` selects another folder.
* The gallery index is written safely and re-read, so two programs open at once do not lose each other's pictures.
* **Help → Download backup…** and **Restore from backup…**: one zip with the settings of MillCreator and the shared gallery. Restoring replaces the settings and adds the pictures to the gallery (nothing is lost or doubled); a backup made by another program is refused. The same entries as in LightCreator, made in baseCreator.
* baseCreator v0.9.4.

## v0.2.4

* A **Console** tab, as in LightCreator: the traffic with the controller, status polls on request, auto-scroll, clear and a line to send a command. It replaces the small console of the Device tab.
* baseCreator v0.9.2.

## v0.2.3

* **The animation of the preview works** (its buttons were inert), in 3D and from above.
* The preview window no longer has an *Export G-code…* button: the control bar has *Save G-code*.
* baseCreator v0.9.1.

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

# Versions

Each release gets the next version number. Newest first.

## v0.5.3 (not released yet)

* **Preview opens in a pane** (never also in a window of its own); a window of a tab (preview, G-code text, camera) stays where it was put instead of sliding down.
* **Camera files:** pictures and videos go to the folder `camera` next to the project file (`camera01`, `camera02`… when it is there already), and their names start with the project file name.
* **Notes of the project** (the Description tab) are written at the start of the G-code as comments. The *Layers* tab no longer grows too wide (the rows of Vc and fz were in one line).
* **Panes made of tabs:** every pane has a bar of tabs (no list to choose from); tabs are dragged between panes or moved from the right-click menu; the last tab closed closes the pane; nothing is opened twice (a project that is open is shown by its tab). **Preview G-code** is a tab with the G-code as coloured text that can be edited (it asks to save the edits when closed). The preview has a zoom slider beside the navigation cube; the preview, the G-code text and the camera can go to a window of their own. The About window has the home page and a list of the credits. baseCreator v1.51.0.
* **Layer presets** in the shared window of baseCreator: search, rename, delete, export and import of a file; a preset whose bit is gone leaves the layer without a bit and says so. The old file is read as it was. baseCreator v1.50.0.
* **Pick the zero of the program on the work area:** a button in the device settings (the next click sets it, with snapping to the corners and middles of the shapes) and a draggable icon of the work coordinate system (G54 … G59) on the work area; one pick or drag is one step of undo.
* **Stay down between rings** for a pocket by rings (off by default): every depth goes round all the rings and the bit moves between them by a cut in the material when the whole way is inside the pocket; otherwise it rises as before.
* **Cutting speed Vc and feed per tooth fz** beside the spindle speed and the feeds of a layer and of a bit in the library; typing one changes the other. Files keep only rpm and mm/min.
* **Show variable names** (*View options*, off by default): the name of the variable beside the label of a field that has one (bit, layer, device); a click copies `{name}`. baseCreator v1.49.0.
* **Variables of the bit** in the own G-code of a change of the bit or of the operation: `{tool_number}`, `{tool_name}`, `{tool_diameter}`, `{tool_flutes}`, `{tool_rpm}`, `{tool_feed}`, `{tool_plunge}` (the changer number of the bit and *Tool changer* with `T<n> M6` were already there).
* **G-code of the device:** `{variables}` in every own G-code box (project, path, date, time, machine, the box of the cuts, the layer), *Very start of the file* and *Very end of the file*, and **Own file format** for controllers (decimal point after whole numbers, `%` lines, the dwell word, line numbers, no comments); all off by default, the file is the same as before. baseCreator v1.48.0.
* **Layer names** beside the colours (*View options*, off by default), **Preview G-code** next to *Save G-code* (shows the program without saving it) and *Preview G-code after saving* (off by default). baseCreator v1.47.0.
* **Pass depth 0** in a layer or in the bit library means the full depth in one pass (a warning when that is deeper than the cutting part of the bit). Older files read as they were.
* **Split view:** the work area in up to three panes (the active drawing, another open drawing to look at, the preview, the G-code analysis, the camera), side by side or stacked, with *Show beside* in the tab menu, and a pane can go to a window of its own on another monitor. The layout is remembered. baseCreator v1.46.0.
* **About window:** thanks to Jakub (CNC KpK) for the ideas and suggestions for improvements, with a link to the channel. baseCreator v1.45.0.

## v0.5.2

* **Insert menu:** the tools of the left palette that insert objects (and the generators of plugins and the online clipart) as menu commands.
* **Plugins:** the check for newer plugins at the start is on by default, and a window lists the plugins that are out of date (**Update all**, **Plugins**, **Later**).
* **Camera:** **RTSP** streams (decoded by ffmpeg, which the packages bring), **Take a photo**, **Record video** (Motion-JPEG AVI) and a **time-lapse** every N seconds or after each layer of the job, all into a folder you choose; a recording goes on while the camera view is closed. baseCreator v1.41.0.

## v0.5.1

* **Colours:** the background and the work-area colours of the view options work (they did nothing here) and are saved, also in a backup.
* **Preview:** the fit buttons are next to *Reset view*, the title bar has only the pin button, and *Export the milled piece as STL…* is an icon with a tooltip. With a **rotary attachment** the preview shows the workpiece as a **cylinder** with the job wound round it.
* **Background picture:** *View → Background picture…* puts a photo, a plan or a scan under the drawing, from a file or the picture of the camera, with four corners to drag and an opacity.
* **Plugins:** any number of postprocessors can be on at once (they run in the order they were ticked); the new plugin **Barcodes** makes Code 39, Code 128 and EAN-13; the *STL projection* plugin is gone (STL, OBJ and 3MF are read as a relief since v0.5.0).
* **Trace image:** zoom in, zoom out and reset-view buttons; the *Trace* button is green. baseCreator v1.40.0.

## v0.5.0

* **Safe profiles:** a layer chooses the **direction** of a profile or a pocket (as drawn, climb or conventional), **stock to leave** with a **finishing pass** at the full depth, how the bit goes **down** (plunge, zigzag ramp, ramp along the contour or helix, with the ramp angle and a ramp feed, which a bit of the library can give) and **tabs** (full, partial or ramp, spread evenly, none on holes, or placed, moved and removed by hand on the work area). Warnings for a ramp steeper than 45°, tabs as high as the cut and a helix that does not fit. Older projects open as before.
* **DXF import:** File → Import → Import DXF… (also Open and drag and drop): lines, polylines with bulges, arcs, circles, ellipses, splines and blocks, in millimetres by the unit of the file, with the layers of the DXF on the free layers of the project; lines that meet become closed paths. **Auto-join paths…**, **Close path with tolerance…** and **Export selection as SVG / DXF…** in the shape operations.
* **More 2.5D operations:** pockets by raster or rings (the strip that was left in the middle is cleared now), an outline pass, rest pockets, **Face**, **Bore**, **Chamfer**, **Dogbone slot** and trochoids for slots; a bit of the library can give a ramp feed. Older projects open as before.
* **V-carve:** a V-bit goes deeper the farther it is from the edge of the shape, so the walls slope and the corners are sharp (lettering, signs). The depth of the layer is the deepest the bit goes, the bottom is flat there, and a **start depth** saves the shallow rings. **Inlay** makes a **socket** and a **plug** with a gap between them.
* **Relief:** a layer cuts a 3D relief as **Rough** (flat bit, with stock to leave), **Finish** (one go with a ball nose) or both. **File → Import → Import 3D model as relief…** (also Open and drag and drop) reads **STL**, **OBJ** and **3MF** models and puts their depth map on the page as a picture for a relief layer.
* **Machine control:** overrides of the feed, the spindle speed and the rapids while a job runs (GRBL 1.1 and newer); coolant buttons (M7 / M8 / M9) and **coolant and dust collector per layer** (the dust collector uses the device's own lines); **saved positions** to jog back to; **Find the centre** of a hole with the probe; **Frame round the outline** (a rubber band instead of the box); a **tool changer** (`T<number> M6` without a pause, a number for every bit in the library). baseCreator v1.37.0.
* **Design tools:** grid and circular **arrays**, **copy along a path**, **text on a path**, **trim by shape**, **remove duplicates**, **break apart**, **simplify**, **serial text** (counting or from a CSV file), **QR** and **Code 39** codes, **nesting** by boxes and a **Measure…** window, after *Dogbone / T-bone corners…* in the Arrange menu. baseCreator v1.38.0.
* **Project setup:** after File → New (and File → Project setup…) a window asks for the size and thickness of the material, where the zero of X and Y is on it (a corner or the centre), whether Z counts from the bottom, and whether the work area is the material. It is saved with the project.
* **Layer presets:** save the way of cutting of a layer under a name and put it on any layer.
* **Controllers:** **Duet / RepRapFirmware** over HTTP (send and start, state, stop; no jogging) and **Smoothieware** as a firmware choice. New official plugin **G-code dialects** (Mach3 / Mach4 / UCCNC, LinuxCNC). baseCreator v1.39.2.
* The tests run ramps, tabs, a helix and a finishing pass against the real GRBL firmware (a simulator).

## v0.4.2

* **3D view:** **Material** shows the stock as it is milled: the cuts made so far (up to the slider) are carved into its top with the real shape of each bit; **Cuts** only adds the paths of the bit over it. The view can be exported as an **STL** model of the milled piece (an icon in the title line and a button in the options row; the cuts done up to the slider).
* **Own G-code:** a new page *G-code* in the device settings, with lines for the start of the program, every change of the bit, every change of the operation and the end of the program.
* **Work coordinates:** the device settings (page General) choose **G54 … G59** and the **zero of the program** (X, Y); the program starts with the word and counts from that zero, also the frame.
* **Preview as a tab:** the preview window has a button in its title line that pins it as a tab of the work area, next to the drawings and the camera view, and a button in the tab that makes it a window again.
* **Plugins:** every plugin that has something to set (a G-code postprocessor, for example *G-code tools*) has a tab of its own in the right-hand panel; the postprocessor is no longer chosen in the Plugins window. The official plugins are translated into Polish, German, Italian, Finnish, Chinese and Hindi.
* **Import / Adjust image:** the dialog follows the size of the window, with the buttons at the bottom, and a new **Levels** column: a histogram of the greys with two sliders for the range.
* **Trace image:** the dialog follows the size of the window; the preview zooms with the scroll wheel, moves by dragging and shows the whole picture again on a double click.
* **Tool palette:** when the window is too low for it (Full HD with the taskbar showing), the buttons grow smaller instead of the palette scrolling.
* Updated dependencies. baseCreator v1.34.1.

## v0.4.1

* **Cut one element at a time, or only the selection:** two options in the Control tab, under Frame / From line… (this session only, not saved; they can be used together). **One element at a time** cuts each element, a group (even over several layers) or a shape that is in no group, completely, with all its layers and all their passes, before the next one, the nearest element to the machine's zero first; it is meant for layers that use the same bit (with different bits the bit is still changed at every switch). **Selection only** cuts just the shapes that are selected on the work area when the G-code is saved or the job is started. Both change the G-code, the preview and the estimate, and Frame follows the selection.
* **Machine menu:** *Device settings…* opens the settings of the device, like the button of the Device tab.
* **Camera view:** a tab of the work area next to the drawings (there is no floating window); the picture is fitted to the window up as well as down, in steps of 0.01 so that it does not tremble. A stream that is not drawn is stopped.
* **Windows:** the pop-up windows are moved only by their title line (the pointer is a hand there, a normal one over the icons) and no longer stick to the pointer after a click. The buttons of the device window and of the other dialogs stand at the foot of the window under a line, and the lists of the bit library and the other windows fill their window.
* **Jogging:** jog, jog cancel, Home, Unlock, Pause and Resume follow the GRBL version of the Connection page; hold to move is only for GRBL 1.1 and newer.
* The tests run the generated jobs against the real GRBL firmware (a simulator) in CI.
* baseCreator v1.30.0.

## v0.4.0

* **GRBL versions:** the Connection page of the device has a *GRBL version* choice: 1.1 or newer (the default), 1.0, 0.9, 0.8 or 0.7 or older. The older boards report the position in their own format and have no `$J=` jog (the program jogs with ordinary moves); GRBL 0.7 has no real-time commands (no status, Pause, Home or Unlock) and cuts lines longer than 49 characters. A servo pen lift (`M3 S…`) works with every version.
* The firmware settings table, the quick guide and the clipart window no longer grow to the height of the screen.
* baseCreator v1.24.0.

## v0.3.9

* Small refinements to the look of the program.
* baseCreator v1.22.0.

## v0.3.8

* **Tabs of open drawings:** *New* opens a new tab (at most 10) instead of replacing the drawing; each tab has its own undo, view and file, a dot marks unsaved work, **Ctrl+1…9** (0 = the last) switch tabs and **Ctrl+W** closes one. Closing the program goes through the tabs one by one and asks about each unsaved one; every tab has its own recovery file; closing the last tab closes the program.
* **Control tab:** Home, Unlock, Pause and Resume in a block of 2 × 2 with coloured icons (also in the Device tab), START and a **STOP** that is live only while the machine works, **Frame** and **From line…** with icons, the Zero buttons in a grid of 2 × 2, Move above the jog buttons, Speed under them, Probe before Spindle, and the spindle Switch on / Switch off in a row of two under the chosen speed.
* The bar over the work area keeps the editing controls, Toolpaths, Preview…, Camera view, the size of the selection and **Save G-code**.
* The ring round the active layer colour is no longer cut off; the Generate shape… and Import windows of plugins keep their height instead of growing to the height of the program window.
* `Fit work area` / `Fit all` moved to **Ctrl+Shift+0** / **Ctrl+Shift+9** (Ctrl+0…9 switch tabs).
* baseCreator v1.20.1.

## v0.3.7

* **G-code viewer:** *File → View G-code file…*, a dropped `.nc` / `.gcode` / `.tap` file, or the **View G-code file…** button on the start screen (no project is started; closing the viewer returns to the start screen) shows the toolpath: flat, or in 3D when Z changes, with zoom, pan, rotation, **Reset view**, hidden rapid moves, colour by height, size, cutting length and a time estimate.
* **Navigation cube** (like in FreeCAD) in the 3D views: click a face, an edge or a corner to look from there, the arrows turn the view by 15°, drag the cube to rotate. In the 3D preview and in the G-code viewer (not in the plan *From above*).
* The edge of the side panel no longer lights up and takes the drag while the pointer is over a window; the list in the Plugins window is not taller than its window.
* `examples/`: a plaque and a badge with four bits as projects (`.cre`) and their G-code.
* baseCreator v1.17.0.

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

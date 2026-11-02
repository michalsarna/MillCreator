🌐 **English** · [Polski](Menu)

# Menus

The menus have the same layout in LightCreator, MillCreator and PenCreator (it comes from baseCreator), and MillCreator puts its own entries into their places:

**File · Edit · Insert · Arrange · View · Machine · Milling · Settings · Help**

* **Windows and Linux:** the menu bar is in the window, under the title bar, once a drawing is open (the start screen has no menu bar). The keys of the entries work there too (see [Keyboard Shortcuts](Keyboard-Shortcuts)).
* **macOS:** the menus are in the menu bar of the system at the top of the screen, also on the start screen (an entry that needs a drawing is grey there). The first menu, **MillCreator**, holds **About MillCreator**, the services, hide / show and **Quit** (Cmd+Q), so **File** has no *Quit* and **Help** no *About*. The keys of the menus behave a little differently on macOS: see [Keyboard Shortcuts](Keyboard-Shortcuts#macos).

The keys are written as on Windows and Linux (Ctrl); on macOS Ctrl is Cmd. An entry that cannot be used now is grey (for example **Save**, **3D view of the job…** and **Export G-code…** while no drawing is open, **Centre on each other** with fewer than two shapes selected, **Adjust image…** and **Trace image…** without a picture selected).

## File

| Entry | Key | What it does |
|---|---|---|
| **New** | Ctrl+N | Opens a new, empty drawing in a tab of its own and then the **Project setup** window. See [Quick Start](Quick-Start). |
| **Open…** | Ctrl+O | Opens a project (`.cre`) or an SVG, PDF or Illustrator drawing in a tab of its own; a project that is open already is shown, not opened twice. A DXF, a UGS design and a 3D model (STL, OBJ, 3MF) chosen here are imported into the drawing. A file dropped on the window goes the same way, and also a picture or a LightBurn project (imported) and a G-code file (shown in the G-code analysis). |
| **Open recent** | | The last files (up to 10); **Clear list** empties it. |
| **Save** | Ctrl+S | Saves the project (`.cre`). |
| **Save as…** | Ctrl+Shift+S | Saves the project under another name. |
| **Project setup…** | | The size and thickness of the material, the zero of the program on it, Z from the bottom, the work area as the material. See [Quick Start](Quick-Start#2-the-project-setup). |
| **Import ▸ Import image…** | | A picture (PNG, JPG, BMP, GIF, WebP) for the page, with the import dialog. See [Design Tools](Design-Tools). |
| **Import ▸ Import DXF…** | | An ASCII DXF as outlines; its layers go to free layers of the project. |
| **Import ▸ Import LightBurn project…** | | The shapes of a `.lbrn2` / `.lbrn` project. |
| **Import ▸ Import UGS design…** | | A `.ugsd` design of UGS Designer, with its ways of cutting as layers. |
| **Import ▸ Import 3D model as relief…** | | An STL, OBJ or 3MF model as a depth map on a relief layer. See [Layers and Operations](Layers-and-Operations#pictures). |
| **Import ▸ Import with a plugin…** | | A file that a plugin reads. See [Plugins](Plugins). |
| **Import ▸ Generate shape…** | | The window of the shape generators of plugins (also in **Insert**). |
| **Export ▸ Export G-code…** | | The same as **Milling → Export G-code…**. |
| **Export ▸ Export relief as STL…** | | The same as **Milling → Export relief as STL…**. |
| **Export ▸ Export selection as SVG…** | | Writes the selected shapes (all of them when nothing is selected) as SVG. |
| **Export ▸ Export selection as DXF…** | | The same as DXF R12, a DXF layer for every layer of the project. |
| **View G-code file…** | | Opens a G-code file in the **G-code analysis**: its toolpath in 2D or 3D, the length of the cuts and the time. |
| **Quit** | Ctrl+Q | Ends the program; it asks about unsaved work first. Not on macOS (it is in the **MillCreator** menu there). |

## Edit

| Entry | Key | What it does |
|---|---|---|
| **Undo** | Ctrl+Z | Takes the last change back. |
| **Redo** | Ctrl+Y | Does it again (Ctrl+Shift+Z too). |
| **Copy** | Ctrl+C | Copies the selected shapes. |
| **Paste** | Ctrl+V | Puts the copied shapes in. |
| **Duplicate** | Ctrl+D | Copies the selection in one step. |
| **Delete** | Del | Deletes the selection (Delete or Backspace); while nodes are selected with the node tool, the nodes. |
| **Select all** | Ctrl+A | Selects every shape. |

## Insert

The tools of the left palette that insert objects; the entry chooses the tool (the tool in use is ticked; not in the menu bar of macOS, which ticks only **Grid**, **Snap to grid**, **Snap to objects**, the language and the colour scheme), and the letter in the label is its key:

**Rectangle (R)**, **Ellipse (E)**, **Triangle (Y)**, **Star (S)**, **Polygon (G)**, **Heart (K)**, **Spiral (I)**, **Line (L)**, **Polyline / pen (P)**, **Text (T)**.

Then **Generate shape…** (the generators of plugins, see [Plugins](Plugins)) and **Online clipart…** (opens the **Clipart** tab and the search of free pictures on the internet). See [Design Tools](Design-Tools).

## Arrange

| Entry | Key | What it does |
|---|---|---|
| **Align left**, **Align centre (H)**, **Align right**, **Align top**, **Align centre (V)**, **Align bottom** | | Line up the selected shapes. |
| **Centre on bed** | | Puts the selection in the middle of the work area. |
| **Centre on each other** | | Puts the selected shapes on one centre (two or more). |
| **Flip horizontal**, **Flip vertical** | | Mirror the selection. |
| **Rotate 90° CW**, **Rotate 90° CCW** | | Turn the selection by a quarter. |
| **Group** | Ctrl+G | Makes the selected shapes one group. |
| **Ungroup** | Ctrl+Shift+G | Takes the group apart. |
| **Lock** | Ctrl+L | The shapes can no longer be moved or changed. |
| **Unlock** | Ctrl+Shift+L | Frees locked shapes. |
| **Bring to front**, **Send to back** | | The order of the shapes. |
| **Edit nodes** | N | The node tool: move the points of a line. |
| **Convert to path** | | Turns a rectangle, ellipse, star… into a path whose nodes can be edited. |
| **Union**, **Intersection**, **Subtract**, **Exclusive or** | | Boolean operations on two or more shapes. |
| **Offset shape…**, **Round corners…**, **Convert to curves** | | A larger or smaller outline, rounded corners, curves from lines. |
| **Auto-join paths…**, **Close path with tolerance…** | | Join lines that meet into paths, close nearly closed paths. |
| **Dogbone / T-bone corners…** | | Notches in inside corners, so a square part fits a slot cut by a round bit. |
| **Grid array…**, **Circular array…**, **Copy along path…**, **Text on path…** | | Copies in rows and columns, round a centre or along a path; text bent along a path. |
| **Trim by shape…**, **Remove duplicates**, **Break apart**, **Simplify…** | | Cut lines by a shape, delete double shapes, one shape per outline, fewer points. |
| **Serial text…**, **Barcode / QR code…**, **Nest shapes…**, **Measure…** | | Numbered copies of a text, codes as outlines, packing the shapes on an area, sizes and distances. |
| **Adjust image…**, **Trace image…** | | Change a picture on the page; turn it into outlines. |

All of them are explained in [Design Tools](Design-Tools).

## View

| Entry | Key | What it does |
|---|---|---|
| **Grid**, **Snap to grid**, **Snap to objects** | | Switches (ticked when on). The grid itself is set in **Settings → View options…**. |
| **Zoom in** | Ctrl++ | Zooms in about the middle of the work area (the key is Ctrl and `=`). |
| **Zoom out** | Ctrl+- | Zooms out. |
| **Fit bed to window** | Ctrl+Shift+0 | The whole work area in the window. |
| **Fit all objects** | Ctrl+Shift+9 | All the shapes in the window. |
| **Auto-fit work area to window** | | A switch (ticked when on, except on macOS): the work area keeps fitting the window while it is resized (zooming by hand switches it off). |
| **Split view: side by side**, **Split view: one above the other** | | Divide the central area into panes (a second pane is added when there is one). |
| **Single view** | | All the tabs in one pane again. |
| **Background picture…** | | A photo or plan under the drawing. See [Design Tools](Design-Tools#background-picture-view-menu-and-the-trace-dialog). |

Panes and tabs: [Design Tools](Design-Tools).

## Machine

The settings of the machine first, then what runs on it. See [Machine Control](Machine-Control).

| Entry | What it does |
|---|---|
| **Device settings…** | The settings of the device in use (the pages are in [Devices and Cameras](Devices-and-Cameras)). |
| **Firmware settings ($$)…** | The settings of the GRBL controller, with backups. |
| **Calibration…** | Steps per millimetre and the squareness of the axes. |
| **Tool length sensor…** | The wizard of the tool length sensor. |
| **Diagnostics and statistics…** | The input pins, the jobs, the hours, alarms and errors, maintenance reminders, the export of a report. |
| **Start from line…** | Starts the job at a line of the program. |
| **Check the program** | The controller goes through the program in check mode (`$C`): nothing moves, the lines it does not understand are reported. Needs a connected machine. |

## Milling

| Entry | Key | What it does |
|---|---|---|
| **Bit library…** | | The ready-made bits and yours. See [Bit Library](Bit-Library). |
| **3D view of the job…** | Ctrl+Shift+P | Opens the **Preview** tab: the material, the cuts and the moves in 3D, with the animation. See [3D View and G-code](3D-View-and-G-code). |
| **Export G-code…** | Ctrl+E | Asks for a file name (`name.nc`) and writes the G-code of the project, through the postprocessors that are on. The same as the **Save G-code** button. |
| **Export relief as STL…** | | Saves the 3D relief of a picture as an STL model (only when there is a picture on a layer with the 3D relief method). |

## Settings

| Entry | What it does |
|---|---|
| **View options…** | The main and the secondary grid, the colours of the work area, the line thickness, **Show layer names**, **Preview G-code after saving**, **Show variable names**. See [Settings and Files](Settings-and-Files). |
| **Program options…** | **Look for a newer version when the program starts**. |
| **Plugins…** | Install, switch on and off and remove plugins, the official list. See [Plugins](Plugins). |
| **Language ▸** | The 30 languages of the program. |
| **Colour scheme ▸** | The 14 colour schemes of the window, and festive ones around Christmas, the New Year, Halloween and Easter. |

## Help

| Entry | What it does |
|---|---|
| **Project page on GitHub** | Opens the repository of MillCreator in the browser. |
| **Download backup…** | Saves the settings and the shared clipart gallery in one zip. See [Settings and Files](Settings-and-Files). |
| **Restore from backup…** | Reads such a zip back. |
| **Check for updates…** | Looks for a newer release of MillCreator. |
| **About** | The version, the licence and the credits. Not on macOS (**MillCreator → About MillCreator** there). |

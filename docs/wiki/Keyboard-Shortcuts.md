🌐 **English** · [Polski](Skroty-klawiszowe)

# Keyboard Shortcuts

Ctrl on Windows and Linux is **Cmd** on macOS. The keys work in the editor (not on the start screen), and none of them acts while a text field has the keyboard: while you type a name, a number or G-code, Ctrl+C, Delete or a letter belong to the field.

## Keys of the menus

The keys shown next to the entries of the menus (see [Menus](Menus)). They work as soon as a drawing is open, on Windows, Linux and macOS, and an entry that is grey at that moment does nothing. For the menu bar of macOS see [below](#macos).

| Key | Entry | Menu |
|---|---|---|
| Ctrl+N | **New** | File |
| Ctrl+O | **Open…** | File |
| Ctrl+S | **Save** | File |
| Ctrl+Shift+S | **Save as…** | File |
| Ctrl+Q | **Quit** | File (macOS: Cmd+Q in the **MillCreator** menu) |
| Ctrl+Z | **Undo** | Edit |
| Ctrl+Y, Ctrl+Shift+Z | **Redo** | Edit |
| Ctrl+C | **Copy** | Edit |
| Ctrl+V | **Paste** | Edit |
| Ctrl+D | **Duplicate** | Edit |
| Delete or Backspace | **Delete** (shown as *Del*) | Edit |
| Ctrl+A | **Select all** | Edit |
| Ctrl+G | **Group** | Arrange |
| Ctrl+Shift+G | **Ungroup** | Arrange |
| Ctrl+L | **Lock** | Arrange |
| Ctrl+Shift+L | **Unlock** | Arrange |
| N | **Edit nodes** | Arrange (the node tool, see below) |
| Ctrl+= (shown as Ctrl++) | **Zoom in** | View |
| Ctrl+- | **Zoom out** | View |
| Ctrl+Shift+0 | **Fit bed to window** | View |
| Ctrl+Shift+9 | **Fit all objects** | View |
| Ctrl+Shift+P | **3D view of the job…** | Milling |
| Ctrl+E | **Export G-code…** | Milling |

A key belongs to one entry only (a test of the program checks it): Ctrl+Shift+G is **Ungroup**, and the G-code is exported with **Ctrl+E**.

## Other keys of the editor

| Key | What it does |
|---|---|
| Delete, Backspace | Deletes the selected shapes; with the node tool and nodes selected, the nodes. |
| Esc | Clears the selection (and the points of a line being drawn with the pen). |
| Esc twice | A second Esc within half a second goes back to the **Select** tool. |
| Ctrl+1 … Ctrl+9, Ctrl+0 | Shows the drawing in tab 1 … 9; Ctrl+0 is tab 10. |
| Ctrl+W | Closes the drawing that is shown (it asks about unsaved work). |
| Enter | Finishes the line of the pen tool (a right click or a double click does it too). |

## Tools (one letter)

A letter without Ctrl chooses a tool of the left palette. The letters are in the names of the tools (in the tooltips and in the **Insert** menu).

| Key | Tool | Key | Tool |
|---|---|---|---|
| V | **Select (V)** | L | **Line (L)** |
| N | **Node edit (N)** | P | **Polyline / pen (P)** |
| R | **Rectangle (R)** | T | **Text (T)** |
| E | **Ellipse (E)** | H | **Pan (H)** |
| Y | **Triangle (Y)** | Z | **Zoom in (Z)** |
| S | **Star (S)** | X | **Zoom out (X)** |
| G | **Polygon (G)** | | |
| K | **Heart (K)** | | |
| I | **Spiral (I)** | | |

## The mouse in the work area

| Action | What it does |
|---|---|
| Wheel, or a pinch on a touchpad | Zooms about the pointer. |
| Middle button drag | Moves the view (with the **Pan** tool also the left button). |
| Shift + click, Ctrl + click | Adds a shape to the selection, or takes it out. |
| Right click (Select tool) | Selects the shape under the pointer and opens the context menu. |
| Shift with the zoom tools | Swaps zoom in and zoom out. |
| Click on a drawing in another pane (or on its tab) | Makes that drawing the active one. |
| Middle click on a tab | Closes the tab. |

## The preview (3D view)

| Action | What it does |
|---|---|
| Left drag | Turns the view. |
| Right or middle drag | Moves the picture. |
| Wheel | Zooms. |
| Double click | Back to the first view (the same as **Reset view**). |

In the **G-code analysis** (File → View G-code file…), a drag moves a flat file and turns a 3D one (a right drag or Shift+drag moves it), the wheel zooms and **Esc** closes it.

## Windows and fields

* **Esc** cancels the question *Save your work before closing?* (as **Cancel**), and cancels **Pick on the work area** of the zero of the program.
* In the window **Layer presets**, Enter keeps a new name of a preset and Esc drops it.

## Jogging with the keyboard

On the **Control** tab, with **Keyboard** ticked (off by default): the arrow keys move X and Y and **Page Up** / **Page Down** move Z, by the step chosen above them. Nothing moves while a field has the keyboard or the machine is busy. See [Machine Control](Machine-Control).

## macOS

On macOS the program uses the menu bar of the system. All the keys on this page work there as on Windows and Linux, with Cmd for Ctrl:

* The menu bar of the system takes the keys of **New** (Cmd+N), **Open…** (Cmd+O), **Save** (Cmd+S), **Save as…** (Cmd+Shift+S) and **Quit** (Cmd+Q, in the **MillCreator** menu) and shows them next to these entries.
* The program takes all the other keys itself, the same way as on the other systems: Cmd+E, Cmd+Shift+P, Cmd+L, Cmd+Shift+L, Cmd+= / Cmd+-, Cmd+Shift+0 / Cmd+Shift+9, the editing keys, Cmd+1 … Cmd+0, Cmd+W and the letters of the tools. The menu bar of the system does not show these keys next to the entries; this page and the menu bar of Windows and Linux do.

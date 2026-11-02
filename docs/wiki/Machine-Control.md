🌐 **English** · [Polski](Sterowanie-maszyna)

# Machine control

MillCreator drives a GRBL machine from the **Control** tab and the **Machine** menu. All of it comes from baseCreator, so MillCreator, PenCreator and LightCreator behave the same. **Tested on a TwoTrees TTC450Pro only**: test with the tool lifted or no pen, and keep a hand on the stop.

## Reading the machine

Connect on the **Device** tab. The program reads the controller (`$I` for its firmware, `$$` for its settings, `$G` for its state). In the device settings, on the page **Machine**, **Use these values for the device** copies the travel, the spindle speeds and the speed of moves from the controller into the device, so you need not type them. New devices can also start from **A usual machine size…** (a starting point only).

## The Control tab

* **Position:** the work position and the machine position of X, Y and Z, in big figures. **X0 Y0 Z0** (on the left of each axis) make the place where the tool is the zero of the work (`G10 L20`); click a number to type the position. Under them: **Zero all**, **Zero X and Y**, **Go to X0 Y0** (lifts to the safe height first) and **Z to safe height**. **Coordinates** picks G54 … G59.
* **Move:** the distance of one click for XY and for Z is a field you can type in; the buttons next to it put a value there (XY 0.1 … 100, Z 0.1 … 20; the values are set in the device settings, page **Jogging**). **Speed** has the XY and Z speeds of the device (a change here lasts until the device's own change).
* **Jog buttons** (big, in the middle): the arrows, **Z▲ Z▼**; the square stops the move. One click is one move. With **Hold to move** the tool moves while the button is held down and stops when it is let go. **Keyboard** (off by default) uses the arrow keys and Page Up / Page Down by the chosen step.

* **Job options** (under Frame and From line…; this session only, not saved; used together or alone, they change the G-code, the preview and the estimate): **One element at a time** cuts each element, a group (even over several layers) or a shape that is in no group, completely, with all its layers and all their passes, before the next one, the nearest element to the machine's zero first (off: a layer after a layer). It is meant for layers that use the same bit: with different bits the bit is changed (a pause) at every switch. **Selection only** cuts just the shapes that are selected on the work area when the G-code is saved or the job is started; nothing selected means nothing to cut.
* **The preview is a tab:** **Preview…** (above the work area) opens the preview as a tab of the work area, next to the drawings and the camera view. Right-click the tab and choose **Open in a separate window** to give it a window of its own; **Back to the main window** (or closing that window) puts it back. The cross of the tab closes the preview. See [3D View and G-code](3D-View-and-G-code).
* **Export of the 3D view as STL:** the *download* icon in the bar of buttons of the preview (**Export the milled piece as STL…**) saves the milled piece as a binary STL model: the stock with the grooves of the bits in it, as far as the slider has got (at the end: the whole job), in millimetres, with the lower left corner of the view at 0, 0 and the top of the stock at the thickness of the material. The bit is swept along the cuts with its real shape (flat, ball, V…), so the surface is as fine as the smallest bit allows (a coarser grid for a big job). Milling → *Export relief as STL…* is the other export: the relief of a picture alone.
* **Spindle** (MillCreator): the speed and **Switch on** / **Switch off** (`M3 S…` and `M5`) to try the spindle. The S value is spread over the controller's range (`$30`) when that is not the top speed of the device.

## Probing (MillCreator)

In the device settings, page **Probe**, say whether you have a **touch plate** (its thickness and the thickness of its side) or a **3D touch probe** (the offset of Z at the touch and the diameter of the ball), the fail distance, the two speeds and the retract. On the Control tab:

* **Probe Z** – the tool goes down until it touches (quickly, backs off, then slowly), and Z is set to the thickness of the plate. Put the plate under the tool first.
* When the probe input is already closed, nothing is probed: the tool would be pressed against the plate. Test the wiring by touching the plate: the input shows as *closed*.
* **Macros:** buttons that send G-code you wrote (page **Macros** of the device settings). Variables in brackets are replaced when the macro is sent: `[posx] [posy] [posz]` (work position), `[mposx]` …, `[xmin] [xmax] [ymin] [ymax]` (the box of the job), `[safez]`, `[prbx] [prby] [prbz]` (the last probe). `$RST` is not allowed in a macro. The same page has G-code that runs **before** and **after** every job (also after Stop).

### Overrides, coolant and saved positions

* **Overrides** (GRBL 1.1 and newer, shown once the controller reports them): while a job runs, change the **feed** and the **spindle** speed in steps of 10 % and 1 %, go back to 100 %, or slow the **rapids** to 50 % or 25 %. The current percentages are shown. This is the realtime override of GRBL, so it works at once and does not change the program.
* **Coolant:** *Mist* (M7), *Flood* (M8) and *Off* (M9). A layer can ask for coolant by itself (see Layers and Operations). Machines often have the dust collector wired to M8.
* **Saved positions** (GRBL 1.1 and newer): *Save here* remembers where the tool is (the machine position, with a name). The name button goes back to it: up first when it is higher than the tool is, then across, then down. The list is kept in `saved_positions.json` in the shared data folder, so every program has the same places.
* **Find the centre** (in the Probe section): the probe hangs in a hole, about in the middle and at the depth you want. It touches the left and the right side, goes to the middle between them, touches the front and the back side, goes to the middle and makes it the zero of X and Y of the coordinate system in use; the status line gives the size of the hole. It does not do Z: probe that first. It is for holes; for a boss, move the probe round it by hand and probe the edges. A failed touch or an alarm stops it.
* **Tool changer:** a machine that changes the bit by itself (grblHAL, FluidNC: they know M6; plain GRBL does not) is turned on in the device settings (*Spindle*: *Tool changer*). Every bit then needs a **changer number** in the bit library, and the program asks for it with `T<number> M6` instead of stopping with an `M0` pause; the coolant and the dust collector are switched off for the change.
* **Frame round the outline:** next to the Frame button: the frame goes round the outline of the cuts (a rubber band, the convex hull) instead of their box.

## Running a job

* **Start** streams the whole job; **From line…** (also in the menu *Machine*) starts at a line of the program: the machine is first brought into the state the program has there (units, coordinate system, spindle and its speed, feed, position), at the safe height, and the rest follows. It refuses when it cannot work that out (for example after `G53` or `G92`).
* **Check the program** (menu *Machine*, or *Check the program before every run* on the page **Jogging**): the controller goes through the program in check mode (`$C`). Nothing moves, and the lines it does not understand are reported with their line number. With the option on, the real job starts only when the check found nothing.
* **Pauses:** When the program reaches a change of the bit, the machine stops there (the line before it has finished) and a window says which bit to put in. **Probe Z** sets Z again with the plate; **Continue the job** goes on, **Stop the job** ends it. While the job waits you can jog, zero and probe on the Control tab.
* Errors and alarms that come during a job are counted and the first line of each is remembered.

## Machine menu

* **Firmware settings ($$):** the settings of the controller, with search and a filter that shows only those that differ from the typical value. A change is written after a question; the settings are read again afterwards. **The first change of a session saves all settings to a file** in the data folder (`firmware-backups`). **Back up to a file…** and **Restore from a file…** do it by hand (a file is compared with the controller before anything is written, and read back afterwards); **Factory settings ($RST=$)…** needs the word RESET.
* **Calibration:** *steps per millimetre* (ask for a move, measure it, get the new `$100` / `$101` / `$102`, write it) and *squareness of the axes* (type the two diagonals of a rectangle: the angle and the shift are shown).
* **Diagnostics and statistics:** the input pins of the controller (limit switches, probe, door, hold, reset, start), the firmware, jobs, running time, alarms and errors of this machine, and **maintenance reminders** (for example "oil the rails every 50 hours"; set them on the page **Maintenance**). **Export diagnostics (zip)…** saves a report, the settings, the console and the program.

## Tool length sensor (MillCreator)

A sensor that the tip of every tool touches lets a new bit get exactly the same Z as the one before, without touching the work again. Menu **Machine → Tool length sensor…** leads through it:

1. **Where the sensor is:** home the machine, jog the tool over the middle of the sensor a few millimetres above it and press **The sensor is here**. The position is kept in *machine* coordinates, so never move the sensor. The same window has the travel height, the fail distance, the retract and the two speeds.
2. **The first tool, the reference:** put in the first bit and set Z on the work with it (probe Z, or touch the surface and press Z0). **Measure the reference tool** takes the tool to the sensor and touches it twice (quickly, backs off, slowly); the Z at the touch is remembered with the device.
3. **Every other tool:** when a job pauses for a change of the bit, the window has **Measure the tool (sensor)**: after the touch Z is set so that this place has the Z of the reference tool, and the tool goes up. The same can be done by hand in the wizard (**Measure this tool and set Z**), and **Only measure** shows how much longer or shorter a tool is, without changing anything.

The probe input of the controller must be wired to the sensor. A measurement is refused when that input is already closed, when the machine is not idle, or before the controller has reported its position. **Not tried on a real machine:** the first time, keep a hand near the stop.

## Copies of the controller settings

Every device keeps copies of the settings of its controller (`$$`). One is made the first time the machine is connected, and again whenever the settings read at connecting differ from the newest copy: the status bar and the console say how many settings changed and which (`$110: 3000 → 5000`), so you notice a change made from another program or by the controller itself. A change made here (a write in **Firmware settings**) gets its own copy without an alarm. **Make a copy now** in the window of the firmware settings, and the same in the device settings, page **Machine**, make one by hand; **Restore…** compares a copy with the controller and writes the differences back after a question; **Delete** removes one. The newest eight are kept. The copies are part of the device, so they are in the **backup of the program** and in an exported device file.

## Alarms and errors

**GRBL versions.** Choose the version of the board under *GRBL version* on the device's Connection page: **1.1 or newer** (the default; also grblHAL and FluidNC), **1.0**, **0.9**, **0.8** or **0.7 or older**. GRBL 0.8, 0.9 and 1.0 report the position as `<Idle,MPos:…,WPos:…>` and have no `$J=` jog command, so the program jogs with ordinary `G91` / `G1` moves; such a jog cannot be cancelled halfway, it runs to the end of its step. GRBL 0.7 has no real-time commands at all: no status report (so no position), no Pause, Resume, Home or Unlock, and Stop only drops the lines not yet sent while the board finishes what it already has; that firmware also cuts lines longer than 49 characters. The spindle (`M3 S…`) works the same with every version.

When GRBL says `ALARM:n` or `error:n`, the console gets a line under it that explains it, and the status bar shows it. The explanations are in the 30 languages of the program.

> ⚠️ Probing, jogging and the pauses send real moves. Keep the stop within reach.

## When the machine stops

An alarm, an error in a job or a controller that does not answer opens a **window in the middle of the screen**: what happened, and two big buttons, **STOP** (red: stops the job and the spindle) and **UNLOCK** (orange: clears the alarm, `$X`). Look at why it stopped before you unlock: the tool may be in the material.

## Other controllers: Smoothieware and Duet

* **Smoothieware:** the **GRBL version** list of the Connection page has **Smoothieware (not GRBL)** next to the GRBL versions. The board is streamed through the same layer as GRBL; it has no `$` settings, so the firmware pages do not apply. Not every button of the Control tab is meant for it.
* **Duet / RepRapFirmware over HTTP:** tick **Duet / RepRapFirmware (HTTP)** on the Connection page and give the **address** of the board (and its **password**, which is not saved). The Control tab then has **Send and start** (the program is uploaded to `0:/gcodes/` and started with `M32`), **State** (idle, printing, … and how far the file is) and **Stop**. The board runs the file by itself, so there is no jogging, no overrides and no streaming. It is for RepRapFirmware 2 and 3 in standalone mode; a Duet driven by a single-board computer (SBC mode) is not covered.

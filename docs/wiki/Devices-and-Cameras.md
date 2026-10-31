🌐 **English** · [Polski](Urzadzenia-i-kamery)

# Devices and Cameras

A **device** is a milling machine. The list of devices is kept between runs and is the same for every project. The start screen selects the device; **New** and **Open…** then use it. The Device tab of the editor shows the device in use and opens its settings.

![Add a device](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/02-add-device.png)

## Buttons on the start screen
**Add device…**, **Edit…**, **Duplicate** and **Import…** (strictly checked: a newer format, a wrong number or a file over 2 MB is refused) in the first row and **I have no device** in the second. **Export…** (one device as a JSON file) and **Delete** (asks to confirm) are in the window of **Edit…**. A copy or an imported device with a name that is taken gets a number: `Router (2)`.

## I have no device
Makes, once, a plain device called **Dummy**: a 300 × 200 mm work area with 60 mm of travel in Z, safe height 5 mm, spindle 1000–24000 rpm. It stays in the list like any other device and can be edited; the project is saved with it as its machine. **New** and **Open…** without any device make and use it too.

## Device settings
| Page | What it holds |
|---|---|
| General | name, units (mm or inch; geometry is always stored in mm), machine zero (front left, front right, back left, back right), **work coordinates** (none, or G54 … G59) and the **zero of the program** |
| Machine | work area X and Y, travel in Z, **safe height** (where the spindle moves between cuts) and the speed of moves above the material |
| Spindle | lowest and highest speed, and the wait after switching on |
| G-code | **own G-code** for the start of the program, every change of the bit, every change of the operation (a new layer) and the end of the program (see below) |
| Connection | serial port or network socket, port or USB id, baud rate, streaming, what to do after connecting |
| Rotary | a rotary attachment: kind, diameters, microsteps per turn |
| Camera | up to eight cameras: an address (MJPEG stream or JPEG snapshot, http / https) or a camera of the computer, a rotation of 0 / 90 / 180 / 270°, on / off |
| Notes | free text |

The controller is GRBL (0.7 to 1.1 and newer), or **Smoothieware** (a choice of the *GRBL version* list), or a **Duet / RepRapFirmware** board over HTTP (a check box on the Connection page with the address and a password that is not saved; see Machine Control). The **Spindle** page also has **Tool changer** (`T<n> M6`), and the **G-code** page the lines for the dust collector. The **Connection** page sets how to reach it (a serial port or a network socket, baud rate, counting or simple streaming, homing or clearing an alarm after connecting) and the **Rotary** page a rotary attachment (chuck or rollers, workpiece diameter, microsteps per turn; the Y axis turns the work). The **Device** tab of the editor has Connect, Home, Unlock, Pause, Resume, the state and position of the machine and a console.

## Camera
When the device has a camera that is switched on (and valid), the editor gets a **Camera** tab with its live picture. Pictures from an address use the stream reader of baseCreator; cameras of the computer need a build with `--features camera`. Rotation and the picture adjustments of the camera are applied.

## In the project
The project keeps a snapshot of the device it was made for (name and the three sizes of the work area). When it is opened for a device whose work area differs, the status bar says so.

**Own G-code (page G-code).** Four boxes, one command per line (at most 20 lines of 100 characters; control characters are refused): *At the start of the program* (after the header and the work coordinate word), *At every change of the bit* (also when the first bit is taken; after the pause for the change and before the spindle starts, for example `T1 M6` or a coolant command), *At every change of the operation* (a new layer; after its comment, before the bit is looked at) and *At the end of the program* (after the head is back at the zero, before `M30`). The lines go into every G-code the program makes: the file, the job that is sent, the dry run. They are the machine's own commands, so the program does not check them: try a new line without a bit in the machine first.

**Work coordinates (G54 … G59) and the zero of the program (page General).** By default the program counts from the machine zero (the corner chosen as *Machine zero*), as before. When you choose **G54 … G59**, the program starts with that word (so the machine uses the work coordinate system whose zero you set on it, for example by touching the material) and every X and Y is counted from the **zero of the program**: a point of the work area given as X, Y (the work area is counted from the corner of the machine zero). With zero 0, 0 the numbers are the same as without a code, only the word is added. The frame (*Frame*), the dry run and the job all use it, and so do the return to the zero at the end and the own G-code. Z is not changed: it is counted from the top of the material.

🌐 **English** · [Polski](Urzadzenia-i-kamery)

# Devices and Cameras

A **device** is a milling machine. The list of devices is kept between runs and is the same for every project. The start screen selects the device; **New** and **Open…** then use it. The Device tab of the editor shows the device in use and opens its settings.

![Add a device](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/02-add-device.png)

## Buttons on the start screen
**Add device…**, **I have no device**, **Edit…**, **Duplicate**, **Export…** (one device as a JSON file), **Import…** (strictly checked: a newer format, a wrong number or a file over 2 MB is refused) and **Delete** (asks to confirm). A copy or an imported device with a name that is taken gets a number: `Router (2)`.

## I have no device
Makes, once, a plain device called **Dummy**: a 300 × 200 mm work area with 60 mm of travel in Z, safe height 5 mm, spindle 1000–24000 rpm. It stays in the list like any other device and can be edited; the project is saved with it as its machine.

## Device settings
| Page | What it holds |
|---|---|
| General | name, units (mm or inch; geometry is always stored in mm), machine zero (front left, front right, back left, back right) |
| Machine | work area X and Y, travel in Z, **safe height** (where the spindle moves between cuts) and the speed of moves above the material |
| Spindle | lowest and highest speed, and the wait after switching on |
| Camera | up to eight cameras: an address (MJPEG stream or JPEG snapshot, http / https) or a camera of the computer, a rotation of 0 / 90 / 180 / 270°, on / off |
| Notes | free text |

The controller is GRBL. **MillCreator does not connect to the machine**: the device describes it so that the toolpaths fit it.

## Camera
When the device has a camera that is switched on (and valid), the editor gets a **Camera** tab with its live picture. Pictures from an address use the stream reader of baseCreator; cameras of the computer need a build with `--features camera`. Rotation and the picture adjustments of the camera are applied.

## In the project
The project keeps a snapshot of the device it was made for (name and the three sizes of the work area). When it is opened for a device whose work area differs, the status bar says so.

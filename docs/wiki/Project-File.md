🌐 **English** · [Polski](Plik-projektu)

# Project File

MillCreator saves **`.cre`**, the project file of baseCreator that LightCreator writes too: one JSON file with the drawing, the description and **one section per program**. Each program writes its own section and keeps the sections of the others untouched, so a project passes between programs without losing anything.

MillCreator's section is called `mill`. It holds:
* the **64 layers** (name, colour, operation, bit, depth, pass depth, step over, feeds, spindle speed),
* the **material** (kind, colour, thickness),
* a **snapshot of the device** (name and the work area),
* the **bits the layers use**, so the project can be opened where they are not in the library.

Opening a project from LightCreator (or a plain SVG, PDF or LightBurn file) gives the default layers; its other sections (`laser`) are kept and written back when you save. A project with a newer `mill` format than the program knows is refused with a message. A layer with wrong numbers is replaced by its default; a bit that is missing is added to **My bits**.

The device itself is not in the project, only its snapshot: the device in use is the one selected on the start screen. The status bar warns when the work area of the snapshot is not the one of that device.

The description of the project (**Description** tab) is part of the shared container, so LightCreator shows the same text.

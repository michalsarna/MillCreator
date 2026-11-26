🌐 **English** · [Polski](Plik-projektu) · [简体中文](Project-File-ZH) · [한국어](Project-File-KO)

# Project File

MillCreator saves **`.cre`**, the project file of baseCreator that LightCreator writes too: one JSON file with the drawing, the description and **one section per program**. Each program writes its own section and keeps the sections of the others untouched, so a project passes between programs without losing anything.

MillCreator's section is called `mill`. It holds:
* the **64 layers** with all their settings (name, colour, order, operation, bit, depths, step over, feeds, spindle speed, direction, entry, tabs and the tabs placed by hand, coolant…),
* the **material** (kind, colour, thickness, the first pass) and the **project setup** (the size of the piece, the zero of the program on it, Z from the bottom),
* a **snapshot of the device** (name and the work area),
* the **bits the layers use**, so the project can be opened where they are not in the library.

Opening a project from LightCreator (or a plain SVG, PDF or LightBurn file) gives the default layers; its other sections (`laser`) are kept and written back when you save. A project with a newer `mill` format than the program knows is refused with a message. A layer with wrong numbers is replaced by its default; a bit that is missing is added to **My bits**.

The device itself is not in the project, only its snapshot: the device in use is the one selected on the start screen. The status bar warns when the work area of the snapshot is not the one of that device.

The description of the project (**Description** tab) is part of the shared container, so LightCreator shows the same text. MillCreator also writes it at the top of the G-code as `(Note: …)` comments (see [3D View and G-code](3D-View-and-G-code#export-g-code)).

## Password protection

A project can be protected with passwords: in **Project setup…** of LightCreator, MillCreator and PenCreator, and in **File → Page setup…** of BaseCreator, part **Protection**.

* **Owner password**: everything (editing, copying, exporting, changing or removing the protection).
* **Open password** (optional): the project can be opened, looked at, turned into G-code and run on the machine, and the material and the layer settings (speed, power, passes, depth, bit, pen) can be changed, but the drawing cannot be edited, copied, exported as a drawing (SVG, DXF, Illustrator, LightBurn …) or printed. The built-in web page shows no drawing then.
* With only an owner password, nobody else can open the file. Without a password the file cannot be read at all, and a **forgotten password cannot be recovered**.
* A project opened with the open password can be unlocked with the owner password, without opening it again: in the **Protection** part of the setup (**Owner password**, **Unlock**) or with **Unlock with the owner password…** in the menu of its tab (right click). That menu also has **Project information…** and the setup of the project.

The file stays a JSON text with the key `protected_cre`; the project in it is encrypted with **XChaCha20-Poly1305** under a random 256-bit key, which is stored once for every password, wrapped with a key made from the password by **Argon2id** (64 MiB, 3 passes, a salt of its own). Every save uses a new nonce; a changed file does not open. Passwords need at least 8 characters, and the open password must differ from the owner's.

What the open password forbids is enforced by the programs, as with the owner password of a PDF: whoever has it can, in principle, decrypt the project with other tools. Keep the owner password for yourself.

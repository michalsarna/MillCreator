🌐 **English** · [Polski](Szybki-start)

# Quick Start

1. **Choose the device.** On the start screen pick your machine, or press **Add device…**. Without a machine press **I have no device**: a stand-in called *Dummy* (300 × 200 × 60 mm) is made and selected. If you press **New** or **Open…** with no device at all, the Dummy is made and used. See [Devices and Cameras](Devices-and-Cameras).
2. **Start the design.** **New** starts an empty design; **Open…** reads an SVG, PDF, Illustrator or `.cre` file; **File → Import → Import LightBurn project…** reads a LightBurn project. Or draw with the tools on the left.
3. **Put shapes on layers.** Select shapes, open the **Layers** tab, click the layer and press **Put the selected shapes on this layer**. The first layers are ready: *Cut out* (outside the line), *Holes and windows* (inside the line), *Pocket*, *Engrave* (follow the line) and *Drill*. See [Layers and Operations](Layers-and-Operations).
4. **Choose the bit and the depth** of each layer you use. Every ready-made bit brings its feed and speed. **Bit library…** (Milling menu or the **…** button) shows and edits them: [Bit Library](Bit-Library).
5. **Set the material** and its thickness in the **Material** tab: [Material](Material).
6. **Look at the job:** **Milling → 3D view of the job…** shows the cuts and the moves at the safe height. Warnings (no bit, deeper than the material, outside the work area) are listed above the picture. See [3D View and G-code](3D-View-and-G-code).
7. **Export G-code** from the 3D window or **Milling → Export G-code…**, and save the project (`.cre`).

> The program writes G-code; it does not control the machine. Load the file into your controller software, test it in the air first and stay at the machine. See [Safety and Limitations](Safety-and-Limitations).

🌐 **English** · [Polski](Warstwy-i-operacje)

# Layers and Operations

A project has **64 layers**. Every shape lies on one layer, and the layer says what is done with it. Click a layer in the **Layers** tab to make it the active one (new shapes go to it) and to edit it below the list.

![Layers](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/05-layers.png)

In the list: the tick takes the layer into the toolpaths and the G-code, the eye shows it in the work area, and the number is how many shapes lie on it.

### Pictures

A picture put on a layer switches that layer to the **Picture** operation (once; you can change it back). The layer then engraves the picture in one go at its **Depth** (use a shallow depth: the rows and the dots are many), by the **Picture method**: **Dithered rows** (the picture is dithered and each row is cut where it is dark), **Dots** (a hole for every dark cell) **Depth by tone** (the rows are cut with a depth that follows the tone: the darker, the deeper, down to the depth of the layer) or a **3D relief**. For the first two you choose the **Dithering** (threshold, Floyd–Steinberg, Jarvis, Stucki, Atkinson, ordered). The rows are the **Step over** of the bit apart (a percent of its diameter) and run at the **Scan angle**. 

**3D relief** (as in molino): the grey level of every pixel is a depth, black the deepest and white left uncut, down to the **Depth** of the layer. The relief is cut in passes of the **Pass depth** down to the full depth (the last pass is the relief itself), the rows are the **Step over** apart, and the **shape of the bit is taken into account** (a flat, ball nose, bull nose or V-bit does not cut into the relief anywhere under its body: a narrow pit that the bit does not fit in stays shallow; near the edge of the picture the bit stays a little higher). Adjust the picture (brightness, contrast, negative) first: *Arrange → Adjust image…*. **Milling → Export relief as STL…** saves the carved piece as a closed solid (the thickness of the material from the Material tab) to look at or to share.

A relief can be cut in two layers, as a real job is: the **Relief part** of a layer is **Rough and finish** (what it always did), **Rough** (passes down to the relief with the *Stock to leave* on it; use a flat bit) or **Finish** (one go over the relief itself, row after row at the *Step over* and the *Scan angle*; use a ball nose). Put the same picture on two layers (or copy the layer) and give the first one a flat bit and the part Rough, the second one a ball nose and the part Finish; the layers go in the order of their rank.

**File → Import → Import 3D model as relief…** (also Open and drag and drop) reads an **STL** (binary or text), **OBJ** or **3MF** model in millimetres, looks at it from above and puts the depth map on the page as a picture: the highest point of the model is white (left uncut), the lowest is black, and where the model has nothing the map is black (the material round it is cleared). The picture is as large as the model. The program puts it on a layer that already cuts a relief, or switches the first empty layer to the Picture operation with the 3D relief method, a ball nose from the library and the depth of the model (at most the thickness of the material). Models with up to five million triangles are read; 3MF components and transforms are not applied (every object is placed as it is in the file). Turn a model over in its program first if you want to cut its back.

Other shapes on the same layer are followed as lines. On a layer with another operation a picture is only shown (a warning says it is not milled).

## Settings of a layer
| Setting | Meaning |
|---|---|
| Name, colour | free; the colour is the colour of its shapes and of its cuts in the 3D view |
| Operation | see below |
| Bit | any bit of the [Bit Library](Bit-Library). Choosing a bit takes its feed, plunge feed, spindle speed, a pass depth of half its diameter and the step over |
| Depth | the total depth below the top of the material |
| Pass depth | how deep one pass goes; the last pass always ends at the full depth. **0 means the full depth in one pass** (a warning says so when that is deeper than the cutting part of the bit); the same for *Pass depth (of the diameter)* of a bit in the library |
| Step over | for a pocket: the distance between rings, as a part of the diameter |
| Feed, plunge feed | mm/min in the material and for the plunge |
| Spindle speed | rpm; a value outside the range of the device is reported |
| Direction | for a profile or a pocket: **As drawn** (what the program always did), **Climb** or **Conventional**. With a clockwise spindle a climb cut keeps the material on the left of the bit (counter-clockwise round a part, clockwise in a pocket); the rings of a hole of a part go the other way round |
| Stock to leave, Finishing pass | the passes down to the depth stay this far from the line; with *Finishing pass* one more pass at the full depth takes the rest off along the real line |
| Entry | how the bit goes down: **Plunge** (straight down at the plunge feed), **Zigzag ramp** (back and forth along the first part of the way), **Ramp along the contour** (down while going round) or **Helix** (in a pocket, on a circle that fits; otherwise the ramp along the contour). **Ramp angle** is the slope in degrees; **Ramp feed** is the feed there (0 = the feed of the bit's *Ramp feed* in the Bit Library, or else the plunge feed). A ramp steeper than 45° is reported |
| Pocket pattern, Raster angle | for a pocket and a face: **Rings** (ring after ring from the outside in; a ring along the middle clears the strip that is left when the width is not a multiple of the step over) or **Raster** (parallel lines at the *Raster angle*, back and forth, joined where the way between them stays inside) |
| Outline pass | for a pocket: a pass along the wall **Before** or **After** the clearing (or *None*), at every depth |
| Rest of layer | for a pocket: clear only what the bit of another layer (a pocket with a bigger bit) could not reach, with the bit of this layer, which has to be smaller |
| Start depth, Inlay, Inlay gap | for **V-carve**. *Inlay* makes one half of an inlay: the **Socket** is carved inside the shape (made bigger by the *Inlay gap*), the **Plug** is the shape made smaller by the gap with the sloping walls carved *round it*, outside the line, down to the depth of the layer. Carve the plug in a second piece with the same shapes turned over (mirrored), cut it out with a profile layer, and glue it into the socket |
| Relief part | for a **3D relief**: Rough and finish, Rough (with *Stock to leave*) or Finish, see Pictures above |
| Coolant, Dust collector | *Coolant* is Off, Mist (M7) or Flood (M8) while the layer is cut; *Dust collector* switches it on with the device's own lines (Device settings → G-code: *Dust collector on / off*; a layer that asks for it with no lines is reported). They stay on between layers that ask for the same, and go off before a change of the bit and at the end of the program |
| Trochoids | for "Follow the line": the bit goes in loops along the line (*Trochoid diameter*, advancing *Trochoid step* with every turn; *Half circles* swing across the line instead), so that it cuts a slot as wide as the loops and the bit with a light load |
| Tabs | **Full** (the bit rises above the material), **Partial** (to the height of the tab) or **Ramp** (partial with a slope, *Tab slope*); *Tab width* is the width of a tab at the part (the bit is added), *Tab height* is measured from the bottom of the cut. They are spread evenly: one for every *Tab spacing* of the ring, at least and at most the numbers given, none on a hole of a part when *No tabs on holes* is on. Only the passes below the top of a tab are raised. For profiles and for "Follow the line" on closed lines. **Place tabs by hand** (under the tab settings) turns the work area into a tab editor for the shapes of the layer: click on an outline to add a tab (a yellow dot), drag a dot along the outline to move it, click a dot to take it away; *Clear hand-placed tabs* removes them all. A ring that has tabs placed by hand gets only those; they are kept in the project with the layer and follow the shape when it is moved or scaled |

## Operations
| Operation | What the bit does |
|---|---|
| **Cut outside the line** | goes round the outside of the line, so the shape stays: a part is cut out of the material |
| **Cut inside the line** | goes round the inside of the line, so the shape falls out: a hole or a window |
| **Follow the line** | the centre of the bit follows the line (engraving, grooves); open lines work too; at deeper passes the bit comes back along the line |
| **Pocket** | clears all of the area inside the line, ring after ring, from the outside in |
| **Drill** | a hole at the centre of every closed shape, in pecks of the pass depth |
| **Picture** | a picture on the layer is engraved (see Pictures above) |
| **Face** | planes the top of the material over the box of all the shapes of the layer (draw a rectangle that gives the size), by rings or a raster; *Overhang* is how far the edge of the bit goes over the edge of the box (a bit goes a little over to leave no ridge) |
| **Bore** | a round hole bigger than the bit: the bit goes down round it in a spiral (the ramp along the contour, at the *Ramp angle*) and then once round at the full depth; the core falls out |
| **Chamfer** | a V-bit (or an engraving bit) follows the line at the depth that makes the chamfer *Chamfer width* wide on the surface (the depth of the layer is set by it; a flat bit is reported) |
| **V-carve** | a V-bit (or an engraving bit) goes deeper the farther it is from the edge of the shape, so the walls slope and the corners come out sharp: lettering, signs, inlays. The *Depth* is the deepest the bit goes; where the shape is wider than that, the bottom is flat (cut with the V-bit itself, which leaves ridges and takes long, so the program says so: a flat bit in a *Pocket* layer is better for it). *Start depth*: nothing is cut shallower than this (the shallow part of the wall is cut by the deeper rings anyway, so this only saves time). A flat bit is reported |
| **Dogbone slot** | as *Cut inside the line*, with a notch into every sharp corner: the centre of the bit goes on until its edge reaches the corner, so a square part fits a slot made by a round bit |

Shapes of one layer form one region (even-odd): a shape inside another one is a hole of it. Put shapes that should be cut separately on separate layers. Open lines are left out of a profile and a pocket (the program says so). Pictures are never milled.

## What is checked
A layer without a bit, a depth over the thickness of the material, a depth over the travel of the machine, a spindle speed out of range, cuts outside the work area and a line too small for the bit are reported as warnings in the 3D window and when G-code is written.

## Layer presets

* **Stay down between rings** (a layer with *Pocket*, pattern *Rings*; off by default): on every depth the rings follow each other, and the bit goes to the next ring by a cut in the material instead of rising, when the whole way lies inside the region the bit can reach (islands are never crossed, so the bit rises there as before). The first ring of a deeper pass is entered from the depth of the pass before. The preview and the time estimate show the new moves.

Next to the spindle speed and the feeds the layer (and the *Starting speeds* of a bit in the library) shows the **cutting speed Vc** (m/min, Vc = π · D · n / 1000), the **feed per tooth fz** (mm, fz = F / (n · z)) and the plunge feed per tooth, worked out from the diameter D and the number of flutes z of the bit. Typing a Vc changes the spindle speed, typing an fz changes the feed (or the plunge feed); only the spindle speed and the feeds in mm/min are saved, so the files do not change. Without a bit the rows are dimmed.

Above the settings of a layer: **Save as preset** keeps the way of cutting of the layer (the operation, the bit, depths, speeds, tabs, ramps…) under a name; the **Presets** list puts a preset on the selected layer (✖ forgets it). The name, colour and order of the layer, the layer a rest pocket takes over from and the tabs placed by hand stay as they were. The presets are kept in `mill_layer_presets.json` in the shared data folder (at most 60).

🌐 **English** · [Polski](Warstwy-i-operacje)

# Layers and Operations

A project has **64 layers**. Every shape lies on one layer, and the layer says what is done with it. Click a layer in the **Layers** tab to make it the active one (new shapes go to it) and to edit it below the list.

![Layers](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/05-layers.png)

In the list: the tick takes the layer into the toolpaths and the G-code, the eye shows it in the work area, and the number is how many shapes lie on it.

### Pictures

A picture put on a layer switches that layer to the **Picture** operation (once; you can change it back). The layer then engraves the picture in one go at its **Depth** (use a shallow depth: the rows and the dots are many), by the **Picture method**: **Dithered rows** (the picture is dithered and each row is cut where it is dark), **Dots** (a hole for every dark cell) **Depth by tone** (the rows are cut with a depth that follows the tone: the darker, the deeper, down to the depth of the layer) or a **3D relief**. For the first two you choose the **Dithering** (threshold, Floyd–Steinberg, Jarvis, Stucki, Atkinson, ordered). The rows are the **Step over** of the bit apart (a percent of its diameter) and run at the **Scan angle**. 

**3D relief** (as in molino): the grey level of every pixel is a depth, black the deepest and white left uncut, down to the **Depth** of the layer. The relief is cut in passes of the **Pass depth** down to the full depth (the last pass is the relief itself), the rows are the **Step over** apart, and the **shape of the bit is taken into account** (a flat, ball nose, bull nose or V-bit does not cut into the relief anywhere under its body: a narrow pit that the bit does not fit in stays shallow; near the edge of the picture the bit stays a little higher). Adjust the picture (brightness, contrast, negative) first: *Arrange → Adjust image…*. **Milling → Export relief as STL…** saves the carved piece as a closed solid (the thickness of the material from the Material tab) to look at or to share.

Other shapes on the same layer are followed as lines. On a layer with another operation a picture is only shown (a warning says it is not milled).

## Settings of a layer
| Setting | Meaning |
|---|---|
| Name, colour | free; the colour is the colour of its shapes and of its cuts in the 3D view |
| Operation | see below |
| Bit | any bit of the [Bit Library](Bit-Library). Choosing a bit takes its feed, plunge feed, spindle speed, a pass depth of half its diameter and the step over |
| Depth | the total depth below the top of the material |
| Pass depth | how deep one pass goes; the last pass always ends at the full depth |
| Step over | for a pocket: the distance between rings, as a part of the diameter |
| Feed, plunge feed | mm/min in the material and for the plunge |
| Spindle speed | rpm; a value outside the range of the device is reported |
| Direction | for a profile or a pocket: **As drawn** (what the program always did), **Climb** or **Conventional**. With a clockwise spindle a climb cut keeps the material on the left of the bit (counter-clockwise round a part, clockwise in a pocket); the rings of a hole of a part go the other way round |
| Stock to leave, Finishing pass | the passes down to the depth stay this far from the line; with *Finishing pass* one more pass at the full depth takes the rest off along the real line |
| Entry | how the bit goes down: **Plunge** (straight down at the plunge feed), **Zigzag ramp** (back and forth along the first part of the way), **Ramp along the contour** (down while going round) or **Helix** (in a pocket, on a circle that fits; otherwise the ramp along the contour). **Ramp angle** is the slope in degrees; **Ramp feed** is the feed there (0 = the feed of the bit's *Ramp feed* in the Bit Library, or else the plunge feed). A ramp steeper than 45° is reported |
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

Shapes of one layer form one region (even-odd): a shape inside another one is a hole of it. Put shapes that should be cut separately on separate layers. Open lines are left out of a profile and a pocket (the program says so). Pictures are never milled.

## What is checked
A layer without a bit, a depth over the thickness of the material, a depth over the travel of the machine, a spindle speed out of range, cuts outside the work area and a line too small for the bit are reported as warnings in the 3D window and when G-code is written.

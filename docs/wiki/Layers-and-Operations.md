🌐 **English** · [Polski](Warstwy-i-operacje)

# Layers and Operations

A project has **16 layers**. Every shape lies on one layer, and the layer says what is done with it. Click a layer in the **Layers** tab to make it the active one (new shapes go to it) and to edit it below the list.

![Layers](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/05-layers.png)

In the list: the tick takes the layer into the toolpaths and the G-code, the eye shows it in the work area, and the number is how many shapes lie on it.

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

## Operations
| Operation | What the bit does |
|---|---|
| **Cut outside the line** | goes round the outside of the line, so the shape stays: a part is cut out of the material |
| **Cut inside the line** | goes round the inside of the line, so the shape falls out: a hole or a window |
| **Follow the line** | the centre of the bit follows the line (engraving, grooves); open lines work too; at deeper passes the bit comes back along the line |
| **Pocket** | clears all of the area inside the line, ring after ring, from the outside in |
| **Drill** | a hole at the centre of every closed shape, in pecks of the pass depth |

Shapes of one layer form one region (even-odd): a shape inside another one is a hole of it. Put shapes that should be cut separately on separate layers. Open lines are left out of a profile and a pocket (the program says so). Pictures are never milled.

## What is checked
A layer without a bit, a depth over the thickness of the material, a depth over the travel of the machine, a spindle speed out of range, cuts outside the work area and a line too small for the bit are reported as warnings in the 3D window and when G-code is written.

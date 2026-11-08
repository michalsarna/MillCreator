🌐 **English** · [Polski](Noz-wleczony)

# Drag knife

A mill can also **cut** with a drag knife in a holder put in the collet of the spindle: vinyl, card, rubber, leather, thin sheets. The blade turns freely in the holder and its tip trails a little behind the axis, the way a castor of a chair trails behind its pin. MillCreator moves the axis so that the **tip** goes along your lines, with the **spindle still**.

## A drag knife layer

1. Give the layer the operation **Drag knife** (in the **Layers** tab). No bit is needed.
2. Pick the **Knife** from the list (or **Knife library…** to open the library). It sets **Offset of the tip**, **Turn round corners sharper than** and **Cut on past the start** at once; each of them can be changed on the layer after that.
3. Set the **Depth** of the layer: how deep the blade goes into the material. The knife cuts in **one pass** at that depth: passes back and forth would turn the blade the other way.
4. Before the layer the job stops with **Put in the drag knife** (an `M0` in the G-code) and the spindle stays off; put the holder in, zero Z on the material if your holder needs it, and press **Continue the job**.

## How the knife is made good

* **Offset of the tip:** how far the tip is behind its axis (the maker of the blade or the holder gives it). The axis goes ahead of the tip by that much, so the cut lies on the line.
* **Turn round corners sharper than:** at a sharper corner the axis goes round it on a small arc, the blade turns on the spot and the corner stays sharp.
* **Cut on past the start:** a closed line is cut on this far past its start, so it closes.
* The way the blade points is carried from one cut to the next, and every cut starts by turning it from where it was left.
* **Holes first:** the holes are cut before the parts round them, so a part stays held until it is cut out.

## The knife library

**Milling → Knife library…** opens the library: **Ready-made knives** on the left, **My knives** under them, the values of the knife chosen on the right.

| Ready-made knife | Offset of the tip | Turn round corners sharper than | Cut on past the start | For |
|---|---|---|---|---|
| Drag knife in a holder of a CNC machine | 0.5 mm | 10° | 1.0 mm | a holder in the collet of the spindle, the spindle still; set the depth on the layer |
| 30° blade | 0.15 mm | 15° | 0.4 mm | thin film and paper, fine detail and small letters |
| 45° blade | 0.25 mm | 15° | 0.5 mm | vinyl, flex, paper and thin card |
| 60° blade | 0.5 mm | 15° | 0.8 mm | thick vinyl, card, rubber, foam and sandblast stencils |

* A ready-made knife cannot be changed. **Copy** puts a copy among **My knives**, with a **Name**, the values and **Notes**; **Delete** removes a knife of your own.
* Your knives are kept with the settings of the program and in its backup.
* A layer **copies** the values of a knife: a later change in the library does not change it. When the values of a layer match no knife the list shows **Own values**.

## The first cut

Cut a small test (a square with a circle in it) in a spare piece, with the material held flat (tape or a vacuum table: a knife pulls at it). A rounded corner means the offset is too small, a loop or a hook at a corner means it is too large; a line that does not close wants a longer **Cut on past the start**. When it is right, **Copy** the knife into **My knives** with the depth that worked in its notes.

See also: [Layers and Operations](Layers-and-Operations), [Design Tools](Design-Tools#drag-knife), [Safety and Limitations](Safety-and-Limitations).

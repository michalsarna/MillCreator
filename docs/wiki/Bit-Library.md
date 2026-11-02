🌐 **English** · [Polski](Biblioteka-frezow)

# Bit Library

**Milling → Bit library…** (or the **…** button next to the bit of a layer) opens the library. On the left: the **ready-made bits** of the program and **My bits**. On the right: the picture of the selected bit, its dimensions, its starting speeds and notes.

![Bit library, V-bit](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/11-bits-vbit.png)

## The picture
Every kind of bit is drawn in section with the **letter of each dimension where it is measured**: D across the widest cutting part, L along the cutting part, O along the whole bit, S across the shank, R at the corner, A as the angle of the flanks and T across the tip. The same letters stand next to the entry fields. A long shank is drawn short, with a break.

| ![Flat](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/10-bits-flat.png) | ![Bull nose](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/12-bits-bull.png) |
|---|---|
| Flat end mill | Bull nose |
| ![Tapered](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/13-bits-tapered.png) | ![Engraver](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/14-bits-engraver.png) |
| Tapered ball nose | Engraving bit |

## Kinds and dimensions
| Kind | Dimensions |
|---|---|
| Flat end mill | D cutting diameter, L cutting length, S shank diameter, O overall length |
| Ball nose | D, L, S, O (the radius is D/2) |
| Bull nose (corner radius) | D, R corner radius, L, S, O |
| V-bit | D, A angle, L, S, O |
| Engraving bit | D, T tip diameter, A angle, L, S, O |
| Tapered ball nose | D, T tip (ball) diameter, A taper angle, L, S, O |
| Drill | D, A point angle, L, S, O |

## Your own bits
* **New bit** makes one with plain defaults; **Copy** makes a copy of the selected bit (also of a ready-made one) among yours; **Delete** removes one of yours. Ready-made bits cannot be changed or deleted; copy one to change it.
* Changes are saved as soon as the numbers are valid (the cutting length cannot be longer than the bit, the corner radius not more than half the diameter, the tip not wider than the bit). A wrong number is explained under the bit and is not saved.
* **My bits are kept once, for every device**, with the settings of the program.
* **Export…** writes all your bits to a JSON file; **Export this bit…** writes the selected one (also a ready-made one). **Import…** reads such a file: every bit is checked, gets an id and a name of its own, and a bit with a wrong number is left out and named. A newer format or a file over 4 MB is refused.
* A project carries the bits its layers use (see [Project File](Project-File)). Opened where they are missing, they are added to **My bits** and the status bar says so.

## Starting speeds
Each bit has a feed, plunge feed, **ramp feed** (for a ramp or a helix going down; 0 = the plunge feed), spindle speed, a pass depth (percent of the diameter), a step over (percent) and the number of flutes. A layer takes them when the bit is chosen. Under them the **Cutting speed (Vc)**, **Feed per tooth (fz)** and **Plunge per tooth** are worked out from the diameter and the **Flutes**; typing one changes the spindle speed or the feed (see [Layers and Operations](Layers-and-Operations#cutting-speed-and-feed-per-tooth)). The figures of the ready-made bits are cautious starting points for soft wood and plastic: **adjust them to your machine, material and cutter**.

**Changer number** is the place of the bit in a tool changer: with a device that has **Tool changer** switched on (device settings, page Spindle) the program asks for the bit with `T<number> M6` and goes on without a pause. 0 means the bit is not in the changer, and the machine stops for a change by hand as usual. The number is also the variable `{tool_number}` of the own G-code.

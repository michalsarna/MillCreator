🌐 **English** · [Polski](Material-obrabiany)

# Material

The **Material** tab sets the workpiece, which the 3D view shows and the project keeps.

![Material](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/06-material.png)

| Material | Colour |
|---|---|
| Wood, light | sand |
| Wood, medium | brown |
| Wood, dark | dark brown |
| Metal | grey |
| Plastic | white by default, **any colour you choose** |

Only the colour of a plastic can be changed; the other materials keep their own colour. The **thickness** (0.1 to 500 mm) is the thickness of the stock: the top of the material is Z = 0, and a layer deeper than the thickness is reported. **File → Project setup…** (it opens after File → New) also records the **size** of the piece, **where the zero of X and Y is** on it and whether **Z counts from the bottom** (see Quick Start). The material has no effect on feeds or speeds; the bits and layers carry those.

**First pass.** Switch on *Start with a shallow pass along the edges* and give its depth (0.1 mm by default). Every layer then first goes once along the edges it will cut, only that deep, which breaks the top of the material cleanly (veneer, laminate, coating), and then does its normal passes. A profile and *Follow the line* get it along their line, a pocket along its edge only (not over the whole area), drilling gets none; it works with every bit. A layer that is not deeper than the first pass gets no extra pass. The setting is saved in the project.

![Plastic in blue](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/18-3d-view-plastic.png)

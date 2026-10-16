🌐 **English** · [Polski](Narzedzia-projektowe)

# Design Tools

The editor is the one of baseCreator, the same as in LightCreator.

![Properties](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/04-editor-properties.png)

* **Tools** (left): select, node edit, rectangle, ellipse, triangle, star, polygon, heart, spiral, line, pen, text, pan and zoom. Letter keys select tools as in LightCreator. New shapes go to the active layer.
* **Handles** move, resize, turn and slant a selection; the **arrange bar** (right) aligns, distributes, orders, groups, flips and locks.
* **Properties** tab: position and size, rotation, mirror, alignment.
* **Edit** menu: undo / redo, select all, copy, paste, duplicate, delete, group, ungroup, boolean operations, offset and round corners.
* **Grid and snapping:** View menu and **Settings → View options…**
* **Import:** **File → Open…** reads SVG, PDF and Illustrator (PDF-compatible) files and `.cre` projects; drag and drop works too. **File → Import → Import LightBurn project…** reads `.lbrn2` / `.lbrn`: rectangles, ellipses, polygons, stars, paths with curves, groups and text as outlines come over (the layers of LightBurn mean nothing here). **Import UGS design…** reads `.ugsd` files of UGS Designer: paths, rectangles, ellipses, text and pictures, hidden objects too; each way of cutting (outside, inside, on the path, pocket, drill) becomes the settings of a layer with its depth, feed and spindle speed, and a flat bit of the file's tool diameter is added to the library. An import adds the shapes to the open drawing (one undo takes them back) and uses the layers of the file only where no shapes are yet.
* **Clipart:** the gallery of 310 pictures with your own and online search ([the same as in LightCreator](https://github.com/michalsarna/LightCreator/wiki/Clipart-Gallery)). A picture is placed as ordinary shapes.
* **Right mouse button** on a shape opens the context menu: copy, paste, duplicate, delete, select all, group, ungroup, lock, unlock, flip, turn, the *Arrange* submenu (order, alignment, centring, edit nodes, convert to path) and the *Shape operations* submenu (union, intersection, subtract, exclusive or, offset shape, round corners, convert to curves; they are also in the **Arrange** menu); for a picture also *Adjust image…* and *Trace image…*. The entries that do not fit the selection are grey.
* **Pictures (bitmaps):** **File → Import → Import image…** (or drop a PNG, JPG, BMP, GIF or WebP file on the window) opens the import dialog: rotate, flip, brightness, contrast, gamma, auto levels, negative and size. **Arrange → Adjust image…** changes a picture on the page, and **Arrange → Trace image…** turns its dark (or light) areas into outlines (threshold, speckle filter, simplification, curve smoothing) that are milled like any other shape. The picture itself is only shown, it is not milled.
* **Side panel:** drag a tab to put it in another place (the order is remembered, also in a backup). The small arrow at the left of the tabs hides the panel; a narrow strip with the names of the tabs stays, and a click on a name opens the panel on that tab.
* **Esc twice** (a second Esc within half a second) goes back to the Select tool.
* **Layer bar** under the editor: one row of 64 colours (a dot marks the layers with shapes); a click puts the selected shapes on that layer, a double click opens its options.
* The work area is the one of the device; the units are its units.

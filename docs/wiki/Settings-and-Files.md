🌐 **English** · [Polski](Ustawienia-i-pliki)

# Settings and Files

Language, colour scheme, view options, recent files, **the list of devices** and **your bits** are stored in one file, `app.ron`:

| System | Location |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/millcreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\millcreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/millcreator/app.ron`, by default `~/.local/share/baseCreator/apps/millcreator/app.ron` |

* **Shared folder.** LightCreator, MillCreator and PenCreator keep their data in one folder, `baseCreator` (the table shows the program's own settings inside it). The **clipart gallery is shared**: a picture saved or downloaded in one program shows up in the others (the `clipart` folder next to `apps`, with an `index.json` recording name, category, source, licence and author). Each program keeps its own settings, device profiles, materials and tools in `apps/<program>/app.ron`. Set the environment variable `BASECREATOR_DIR` to use another folder.
* The file is written when the program closes (and from time to time). **Delete it to reset everything**: the device list and your bits go too, so export them first (**Export…** on the start screen and in the bit library).
* **Help → Download backup…** saves a zip with the settings of MillCreator (device list, bits, language, colour scheme, recent files) and the whole shared clipart gallery, with a README. **Help → Restore from backup…** reads such a file back after a confirmation: the settings are replaced and the pictures are **added** to the gallery, so nothing saved by another program in the meantime is lost. A backup made by another program is refused. LightCreator has the same two entries.
* Projects (`.cre`), SVG and G-code are saved wherever you choose.
* The window has its own flat frame. The **Settings** menu has:
  * **View options…**: the **Main grid** and the **Secondary grid** (show, distance between lines, colour), the **Work area** colours (**Background colour**, **Workspace colour**, automatic or your own), the **Line thickness**, **Show layer names** (the names beside the colours of the layer bar), **Preview G-code after saving** (the saved G-code opens in the G-code analysis) and **Show variable names** (the names of the variables of the own G-code beside the fields; a click copies one). **Reset grid** puts the grids and colours back.
  * **Program options…**: **Look for a newer version when the program starts** (on by default; **Help → Check for updates…** looks at once).
  * **Plugins…**, **Language** (30, for the shared texts and MillCreator's own) and **Colour scheme** (14, and festive ones around a holiday).
* Other files in the shared data folder: `mill_layer_presets.json` (the layer presets), `saved_positions.json` (the saved positions of the machine), `camera_recording.json` (the folder, the video format and the time-lapse of the camera recordings) and the folder `camera` for recordings of projects not saved yet. The panes and tabs of the work area and the side panel are remembered with the settings.
* **Help → Project page on GitHub** opens the repository.

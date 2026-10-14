🌐 **English** · [Polski](Ustawienia-i-pliki)

# Settings and Files

Language, colour scheme, view options, recent files, **the list of devices** and **your bits** are stored in one file, `app.ron`:

| System | Location |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/millcreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\millcreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/millcreator/app.ron`, by default `~/.local/share/baseCreator/apps/millcreator/app.ron` |

* **Shared folder.** LightCreator, MillCreator and PenCreator keep their data in one folder, `baseCreator` (the table shows the program's own settings inside it). The **clipart gallery is shared**: a picture saved or downloaded in one program shows up in the others (the `clipart` folder next to `apps`, with an `index.json` recording name, category, source, licence and author). Each program keeps its own settings, device profiles, materials and tools in `apps/<program>/app.ron`. Set the environment variable `BASECREATOR_DIR` to use another folder. A program started for the first time after the update moves its old settings and pictures in (the old folder is left untouched, identical pictures are not duplicated).
* The file is written when the program closes (and from time to time). **Delete it to reset everything**: the device list and your bits go too, so export them first (**Export…** on the start screen and in the bit library).
* **Help → Download backup…** saves a zip with the settings of MillCreator (device list, bits, language, colour scheme, recent files) and the whole shared clipart gallery, with a README. **Help → Restore from backup…** reads such a file back after a confirmation: the settings are replaced and the pictures are **added** to the gallery, so nothing saved by another program in the meantime is lost. A backup made by another program is refused. LightCreator has the same two entries.
* Projects (`.cre`), SVG and G-code are saved wherever you choose.
* The window has its own flat frame; **Settings** has the language (30, for the shared texts and MillCreator's own) and the colour scheme (14), and **View options…**
* **Help → Project page on GitHub** opens the repository.

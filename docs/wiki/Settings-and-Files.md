🌐 **English** · [Polski](Ustawienia-i-pliki)

# Settings and Files

Language, colour scheme, view options, recent files, **the list of devices** and **your bits** are stored in one file, `app.ron`:

| System | Location |
|---|---|
| macOS | `~/Library/Application Support/millcreator/app.ron` |
| Windows | `%APPDATA%\millcreator\data\app.ron` |
| Linux | `$XDG_DATA_HOME/millcreator/app.ron`, by default `~/.local/share/millcreator/app.ron` |

* Your clipart sits in the `clipart` folder next to it.
* The file is written when the program closes (and from time to time). **Delete it to reset everything**: the device list and your bits go too, so export them first (**Export…** on the start screen and in the bit library).
* Projects (`.cre`), SVG and G-code are saved wherever you choose.
* The window has its own flat frame; **Settings** has the language (30, for the shared texts and MillCreator's own) and the colour scheme (14), and **View options…**
* **Help → Project page on GitHub** opens the repository.

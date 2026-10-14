🌐 [English](Settings-and-Files) · **Polski**

# Ustawienia i pliki

Język, schemat kolorów, opcje widoku, ostatnie pliki, **lista urządzeń** i **twoje frezy** są w jednym pliku, `app.ron`:

| System | Położenie |
|---|---|
| macOS | `~/Library/Application Support/millcreator/app.ron` |
| Windows | `%APPDATA%\millcreator\data\app.ron` |
| Linux | `$XDG_DATA_HOME/millcreator/app.ron`, domyślnie `~/.local/share/millcreator/app.ron` |

* Twoje cliparty leżą obok, w folderze `clipart`.
* Plik jest zapisywany przy zamknięciu programu (i co jakiś czas). **Usuń go, żeby wszystko zresetować**: znika też lista urządzeń i twoje frezy, więc najpierw je wyeksportuj (**Export…** na ekranie startowym i w bibliotece frezów).
* Projekty (`.cre`), SVG i G-code zapisują się tam, gdzie wskażesz.
* Okno ma własną płaską ramkę; **Settings** ma język (30, dla wspólnych tekstów i własnych MillCreatora) i schemat kolorów (14) oraz **View options…**
* **Help → Project page on GitHub** otwiera repozytorium.

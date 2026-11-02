🌐 [English](Settings-and-Files) · **Polski**

# Ustawienia i pliki

Język, schemat kolorów, opcje widoku, ostatnie pliki, **lista urządzeń** i **twoje frezy** są w jednym pliku, `app.ron`:

| System | Położenie |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/millcreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\millcreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/millcreator/app.ron`, domyślnie `~/.local/share/baseCreator/apps/millcreator/app.ron` |

* **Wspólny folder.** LightCreator, MillCreator i PenCreator trzymają dane w jednym folderze, `baseCreator` (tabela pokazuje ustawienia danego programu w jego wnętrzu). **Galeria clipartów jest wspólna**: obrazek zapisany lub pobrany w jednym programie pojawia się w pozostałych (folder `clipart` obok `apps`, z plikiem `index.json` zawierającym nazwę, kategorię, źródło, licencję i autora). Każdy program trzyma własne ustawienia, profile urządzeń, materiały i narzędzia w `apps/<program>/app.ron`. Zmienna środowiskowa `BASECREATOR_DIR` wskazuje inny folder.
* Plik jest zapisywany przy zamknięciu programu (i co jakiś czas). **Usuń go, żeby wszystko zresetować**: znika też lista urządzeń i twoje frezy, więc najpierw je wyeksportuj (**Eksportuj…** w oknie urządzenia i w bibliotece frezów).
* **Pomoc → Pobierz kopię zapasową…** zapisuje plik zip z ustawieniami programu MillCreator (lista urządzeń, frezy, język, schemat kolorów, ostatnie pliki) i całą wspólną galerią clipartów, z README. **Pomoc → Przywróć z kopii zapasowej…** wczytuje taki plik po potwierdzeniu: ustawienia są zastępowane, a obrazki **dodawane** do galerii, więc nic, co w międzyczasie zapisał inny program, nie ginie. Kopia zrobiona w innym programie jest odrzucana. LightCreator ma te same dwie pozycje.
* Projekty (`.cre`), SVG i G-code zapisują się tam, gdzie wskażesz.
* Okno ma własną płaską ramkę. Menu **Ustawienia** (Settings) ma:
  * **Opcje widoku…**: **Siatka główna** i **Siatka pomocnicza** (pokazywanie, odległość między liniami, kolor), kolory **Obszaru roboczego** (**Kolor tła**, **Kolor obszaru roboczego**, automatyczne albo własne), **Grubość linii**, **Pokaż nazwy warstw** (nazwy obok kolorów paska warstw), **Podgląd G-code po zapisaniu** (zapisany G-code otwiera się w analizie G-code) i **Pokaż nazwy zmiennych** (nazwy zmiennych własnego G-code obok pól; kliknięcie kopiuje). **Przywróć siatkę** przywraca siatki i kolory.
  * **Opcje programu…**: **Szukaj nowszej wersji przy uruchamianiu programu** (domyślnie włączone; **Pomoc → Sprawdź aktualizacje…** sprawdza od razu).
  * **Wtyczki…**, **Język** (30, dla wspólnych tekstów i własnych MillCreatora) i **Schemat kolorów** (14 oraz świąteczne w okolicy świąt).
* Inne pliki we wspólnym folderze danych: `mill_layer_presets.json` (presety warstw), `saved_positions.json` (zapisane pozycje maszyny), `camera_recording.json` (folder, format wideo i poklatkowe nagrań kamery) oraz folder `camera` na nagrania projektów jeszcze nie zapisanych. Panele i karty obszaru roboczego oraz panel boczny są zapamiętywane z ustawieniami.
* **Pomoc → Strona projektu na GitHubie** otwiera repozytorium.

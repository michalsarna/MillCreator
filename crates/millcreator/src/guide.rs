//! The built-in quick guide: short descriptions of the program's options.
//! Written in English and Polish; other interface languages show the English text.
use base_ui::dialogs::Section;
use base_ui::i18n::{lang, Lang};

pub const EN: &[Section] = &[
    Section {
        title: "Getting started",
        lines: &[
            "The start screen lists your machines. Click one, or press I have no device for a stand-in called Dummy. Add device… makes a new one: the work area, the safe height, the spindle and the connection. Then press New or Open….",
            "After New, the window of File > Project setup… asks about the piece of material and where the zero of the program is on it. Set the same zero on the machine.",
        ],
    },
    Section {
        title: "Layers and bits",
        lines: &[
            "A shape is milled the way its layer says. Select shapes and click a colour in the row under the work area to put them on that layer. On the Layers tab set the operation (profiles, pocket, drill, V-carve and others), the bit, the depth, the pass depth and the feeds. The tick takes a layer into the job.",
            "Milling > Bit library… keeps your bits; choosing a bit also sets its starting speeds. Tabs keep a part in its place while it is cut out.",
        ],
    },
    Section {
        title: "3D view and G-code",
        lines: &[
            "Milling > 3D view of the job… shows the block of material with the cuts in it, an animation, the time and the warnings. Preview G-code shows the program as text in a tab, without saving anything. Milling > Export G-code… writes the file, and Milling > Export relief as STL… writes a relief.",
        ],
    },
    Section {
        title: "Window and tabs",
        lines: &[
            "Each open drawing has a tab above the work area (File > New and File > Open… make one, up to 10); a dot marks unsaved work. File > Open recent lists the last files, and File > Print… prints the design on paper or saves it as a PDF. Settings > Program options… sets auto-save and the check for updates.",
            "Help > Download backup… saves your settings and your clipart in a zip file; Help > Restore from backup… reads it back. Help > Check for updates… looks for a newer version.",
        ],
    },
    Section {
        title: "Drawing and editing",
        lines: &[
            "The palette on the left: select (V), node edit (N), rectangle (R), ellipse (E), triangle (Y), star (S), polygon (G, asks for the number of sides), heart (K), spiral (I, asks for the number of turns), line (L), polyline (P), text (T), pan (H), zoom in (Z) and zoom out (X). Drag to draw; Shift keeps the proportions. The Insert menu has the same tools.",
            "Click to select, Shift+click to add. A frame from left to right selects what is wholly inside, from right to left also what it touches. The handles resize; a click on a handle changes them to rotate and slant. The arrows move the selection (Shift: 10 mm). Escape clears the selection, twice gives the select tool. Hold Space to pan with any tool.",
            "The Arrange menu has align, flip, rotate, group, lock, order, the operations on shapes (union, intersection, subtract, exclusive or, offset, round corners), arrays, text on a path, nesting and measuring. The right button opens the same commands at the pointer.",
        ],
    },
    Section {
        title: "Text, pictures and import",
        lines: &[
            "File > Import brings in pictures, SVG, Illustrator / PDF, DXF, LightBurn projects and UGS designs. File > Export writes SVG and PLT / DXF. The Clipart tab has ready-made pictures, and Insert > Online clipart… searches the internet; Insert > Generate shape… uses the generators of plugins.",
            "View > Overlay from… puts a photo under the drawing, or the live picture of the camera when the device has one; drag its four corners onto the material. View > Show camera overlay shows or hides it. It is saved with the project.",
        ],
    },
    Section {
        title: "Machine",
        lines: &[
            "Machine > Device settings… sets the connection, and Connect on the Device tab opens it. Machine > Switch device… goes back to the start screen. Machine > Firmware settings ($$)… reads and writes the settings of GRBL, and Machine > Calibration… measures the steps of the axes.",
            "Machine > Frame goes round the box of the job at the safe height with the spindle off. Machine > Start job sends the job; test a new job in the air first. Machine > Start from line… starts in the middle, and Machine > Check the program runs it in the check mode of GRBL: nothing moves and the lines GRBL does not take are reported.",
            "Machine > Diagnostics and statistics… shows the inputs, the jobs, the alarms and the maintenance reminders; Help > Export diagnostics (zip)… makes one file to attach to a bug report.",
        ],
    },
    Section {
        title: "Safety",
        lines: &[
            "Test a new job in the air above the material, clamp the work, wear eye and ear protection and stay at the machine. Machine > Tool length sensor… measures the bits.",
        ],
    },
    Section {
        title: "Keyboard shortcuts",
        lines: &[
            "Ctrl+N new, Ctrl+O open, Ctrl+S save, Ctrl+Shift+S save as, Ctrl+P print, Ctrl+Z undo, Ctrl+Y redo, Ctrl+C copy, Ctrl+V paste, Ctrl+D duplicate, Delete delete, Ctrl+A select all, Ctrl+G group, Ctrl+Shift+G ungroup, Ctrl+L lock, Ctrl+Shift+L unlock, F1 this guide.",
        ],
    },
];

pub const PL: &[Section] = &[
    Section {
        title: "Pierwsze kroki",
        lines: &[
            "Ekran startowy pokazuje Twoje maszyny. Kliknij jedną albo naciśnij Nie mam urządzenia, żeby użyć zastępczego urządzenia Dummy. Dodaj urządzenie… tworzy nowe: obszar roboczy, wysokość bezpieczną, wrzeciono i połączenie. Potem naciśnij Nowy albo Otwórz….",
            "Po wybraniu Nowy okno Plik > Ustawienia projektu… pyta o kawałek materiału i o to, gdzie na nim jest zero programu. To samo zero ustaw na maszynie.",
        ],
    },
    Section {
        title: "Warstwy i frezy",
        lines: &[
            "Kształt jest frezowany tak, jak mówi jego warstwa. Zaznacz kształty i kliknij kolor w rzędzie pod obszarem roboczym, żeby przenieść je na tę warstwę. Na karcie Warstwy ustaw operację (kontury, kieszeń, wiercenie, V-carve i inne), frez, głębokość, głębokość przejścia i posuwy. Zaznaczenie bierze warstwę do zadania.",
            "Frezowanie > Biblioteka frezów… przechowuje Twoje frezy; wybranie freza ustawia też jego prędkości startowe. Mostki trzymają część na miejscu podczas wycinania.",
        ],
    },
    Section {
        title: "Widok 3D i G-code",
        lines: &[
            "Frezowanie > Widok 3D zadania… pokazuje blok materiału z wycięciami, animację, czas i ostrzeżenia. Podgląd G-code pokazuje program jako tekst na karcie, niczego nie zapisując. Frezowanie > Eksportuj G-code… zapisuje plik, a eksport płaskorzeźby do STL jest w tym samym menu.",
        ],
    },
    Section {
        title: "Okno i karty",
        lines: &[
            "Każdy otwarty rysunek ma kartę nad obszarem roboczym (Plik > Nowy i Plik > Otwórz… tworzą kartę, do 10); kropka oznacza niezapisaną pracę. Plik > Otwórz ostatnie pokazuje ostatnie pliki, a Plik > Drukuj… drukuje projekt na papierze albo zapisuje go jako PDF. Ustawienia > Opcje programu… ustawiają automatyczny zapis i sprawdzanie aktualizacji.",
            "Pomoc > Pobierz kopię zapasową… zapisuje Twoje ustawienia i cliparty w pliku zip, a przywracanie z kopii w menu Pomoc wczytuje je z powrotem. Pomoc > Sprawdź aktualizacje… szuka nowszej wersji.",
        ],
    },
    Section {
        title: "Rysowanie i edycja",
        lines: &[
            "Paleta po lewej: zaznaczanie (V), edycja węzłów (N), prostokąt (R), elipsa (E), trójkąt (Y), gwiazda (S), wielokąt (G, pyta o liczbę boków), serce (K), spirala (I, pyta o liczbę zwojów), linia (L), łamana (P), tekst (T), rączka (H), powiększanie (Z) i pomniejszanie (X). Rysuj przeciągając; Shift utrzymuje proporcje. Menu Wstaw ma te same narzędzia.",
            "Kliknięcie zaznacza, Shift+kliknięcie dodaje. Ramka od lewej do prawej zaznacza to, co jest w niej w całości, od prawej do lewej także to, czego dotyka. Uchwyty zmieniają rozmiar; kliknięcie uchwytu zamienia je na obrót i pochylenie. Strzałki przesuwają zaznaczenie (Shift: 10 mm). Escape czyści zaznaczenie, dwa razy wraca do zaznaczania. Trzymana Spacja daje rączkę przy każdym narzędziu.",
            "Menu Rozmieść ma wyrównanie, odbicia, obroty, grupowanie, blokadę, kolejność, działania na kształtach (suma, część wspólna, różnica, wyłączenie, offset, zaokrąglanie rogów), szyki, tekst na ścieżce, rozkładanie i pomiary. Prawy przycisk otwiera te same polecenia przy wskaźniku.",
        ],
    },
    Section {
        title: "Tekst, obrazy i import",
        lines: &[
            "Plik > Importuj wczytuje obrazy, SVG, Illustrator / PDF, DXF, projekty LightBurn i projekty UGS. Plik > Eksport zapisuje SVG i PLT / DXF. Karta Clipart ma gotowe obrazki, Wstaw > Cliparty z internetu… szuka ich w sieci, a Wstaw > Generuj kształt… używa generatorów z wtyczek.",
            "Widok > Nakładka z… kładzie pod rysunkiem zdjęcie albo obraz na żywo z kamery, gdy urządzenie ją ma; przeciągnij jego cztery narożniki na materiał. Widok > Pokaż nakładkę z kamery pokazuje ją lub ukrywa. Jest zapisywana w projekcie.",
        ],
    },
    Section {
        title: "Maszyna",
        lines: &[
            "Maszyna > Ustawienia urządzenia… ustawiają połączenie, a Połącz na karcie Urządzenie je otwiera. Maszyna > Zmień urządzenie… wraca do ekranu startowego. Maszyna > Ustawienia firmware ($$)… czytają i zapisują ustawienia GRBL, a Maszyna > Kalibracja… mierzy kroki osi.",
            "Maszyna > Obrys obchodzi prostokąt zadania na wysokości bezpiecznej z wyłączonym wrzecionem. Maszyna > Rozpocznij zadanie wysyła zadanie; nowe zadanie najpierw sprawdź w powietrzu. Maszyna > Start od wiersza… zaczyna od środka, a Maszyna > Sprawdź program przepuszcza je przez tryb sprawdzania GRBL: nic się nie rusza, a wiersze, których GRBL nie przyjmuje, są zgłaszane.",
            "Maszyna > Diagnostyka i statystyki… pokazują wejścia, zadania, alarmy i przypomnienia o przeglądach; Pomoc > Eksportuj diagnostykę (zip)… tworzy jeden plik do dołączenia do zgłoszenia błędu.",
        ],
    },
    Section {
        title: "Bezpieczeństwo",
        lines: &[
            "Nowe zadanie sprawdź w powietrzu nad materiałem, zamocuj materiał, załóż ochronę oczu i uszu i zostań przy maszynie. Maszyna > Czujnik długości narzędzia… mierzy frezy.",
        ],
    },
    Section {
        title: "Skróty klawiszowe",
        lines: &[
            "Ctrl+N nowy, Ctrl+O otwórz, Ctrl+S zapisz, Ctrl+Shift+S zapisz jako, Ctrl+P drukuj, Ctrl+Z cofnij, Ctrl+Y ponów, Ctrl+C kopiuj, Ctrl+V wklej, Ctrl+D powiel, Delete usuń, Ctrl+A zaznacz wszystko, Ctrl+G grupuj, Ctrl+Shift+G rozgrupuj, Ctrl+L zablokuj, Ctrl+Shift+L odblokuj, F1 ten przewodnik.",
        ],
    },
];

/// The guide in the language of the interface.
pub fn sections() -> &'static [Section] {
    if lang() == Lang::Pl {
        PL
    } else {
        EN
    }
}

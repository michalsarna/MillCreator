//! The quick guide of MillCreator: what is its own. The rest is the shared guide (`base_ui::guide`).
use base_ui::dialogs::Section;
use base_ui::guide::{Own, Picture};
use base_ui::i18n::{lang, Lang};

const EN: &[Section] = &[
    Section {
        title: "Getting started",
        lines: &[
            "The start screen lists your machines. Click one, or press I have no device for a stand-in called Dummy. Add device… makes a new one: the work area, the safe height, the spindle and the connection. Then press New or Open….",
            "After New, the window of File > Project setup… asks about the piece of material and where the zero of the program is on it. Set the same zero on the machine. Sides to work on: 2 sides gives side A and side B (4 and 6 sides: a post and a block, below): a switch over the work area shows one or the other, with the ghosts of the other side turned over. For a round workpiece choose Cylinder (rotary): its diameter and length, and the rotary attachment of the device is switched on.",
        ],
    },
    Section {
        title: "Layers and bits",
        lines: &[
            "A shape is milled the way its layer says. Select shapes and click a colour in the row under the work area to put them on that layer. On the Layers tab set the operation (profiles, pocket, drill, V-carve and others), the bit, the depth, the pass depth and the feeds. The tick takes a layer into the job.",
            "Milling > Bit library… keeps your bits; choosing a bit also sets its starting speeds. Tabs keep a part in its place while it is cut out.",
            "Profiles can have a Lead in and a Lead out: a line or an arc from the waste side, so the mark where the bit starts is not on the part, and an overlap past the start.",
            "More operations: Thread mill (a helix round every circle, a nut or a screw, with the pitches of usual screws), Fluting (grooves that start and end shallow), Moulding (a roundover, cove or ogee swept along a line), Texture (scoops of a ball nose, as if carved by hand) and 3D shape (a dome, a slope or a flat top out of every closed shape). A picture layer can use Sketch carving, a V-bit cutting lines like a pencil sketch. A relief Finish with Rest of layer cuts only what a bigger bit before it could not reach.",
            "Two sides: File > Project setup… > Sides to work on > 2 sides gives side A and side B, each drawn as the material lies; the alignment holes go on the last layer. Click a ghost to align the selected shapes to it or to snap to it; a ghost never moves. Save G-code asks whether both sides go into one file, with or without a stop (M0) to turn the material over, or into two files ending in _A and _B; each side says at its head which one it is. Preview G-code asks the same and shows the whole job in one tab or each side in a tab of its own; Save G-code in that tab asks too. The 3D view shows the cuts of both faces; where they go through the material there is a hole.",
            "Posts and blocks: File > Project setup… > Sides to work on: 4 sides (post) is a post with a base a × b and a height H lying along X, milled on its four long faces A to D and rolled a quarter turn back between them (the front comes up); 6 sides (block) is a block a × b × c milled on all six faces, E and F being its left and right end turned up. Each face is a work area of its own, as big as the face, on material as thick as the object is deep there; the work area is always the material (also for 2 sides). The switch over the work area shows a face; the faces beside it are unfolded round it with their ghosts, the face under it shows through. Save G-code puts all the faces in one file (each counted from its own zero, with a stop before each face that says how to lay the object and to set the zero of Z on its top) or a file for each face (_A … _F). The 3D view shows the whole object carved on every face, lying as the machine sees the shown face; a cut through from two faces opposite each other is a hole.",
        ],
    },
    Section {
        title: "3D view and G-code",
        lines: &[
            "Milling > 3D view of the job… shows the block of material with the cuts in it, an animation, the time and the warnings. Preview G-code shows the program as text in a tab, without saving anything. Milling > Export G-code… writes the file, and Milling > Export relief as STL… writes a relief.",
        ],
    },
    Section {
        title: "Running a job",
        lines: &[
            "Machine > Frame goes round the box of the job at the safe height with the spindle off. Machine > Start job sends the job; test a new job in the air first.",
        ],
    },
];

const PL: &[Section] = &[
    Section {
        title: "Pierwsze kroki",
        lines: &[
            "Ekran startowy pokazuje Twoje maszyny. Kliknij jedną albo naciśnij Nie mam urządzenia, żeby użyć zastępczego urządzenia Dummy. Dodaj urządzenie… tworzy nowe: obszar roboczy, wysokość bezpieczną, wrzeciono i połączenie. Potem naciśnij Nowy albo Otwórz….",
            "Po wybraniu Nowy okno Plik > Ustawienia projektu… pyta o kawałek materiału i o to, gdzie na nim jest zero programu. To samo zero ustaw na maszynie. Obrabiane strony: 2 strony dają stronę A i stronę B (4 i 6 stron: słupek i kostka, niżej): przełącznik nad obszarem roboczym pokazuje jedną albo drugą, z duchami drugiej strony odwróconymi. Dla okrągłego przedmiotu wybierz Walec (obrotnica): jego średnicę i długość, a obrotnica urządzenia zostanie włączona.",
        ],
    },
    Section {
        title: "Warstwy i frezy",
        lines: &[
            "Kształt jest frezowany tak, jak mówi jego warstwa. Zaznacz kształty i kliknij kolor w rzędzie pod obszarem roboczym, żeby przenieść je na tę warstwę. Na karcie Warstwy ustaw operację (kontury, kieszeń, wiercenie, V-carve i inne), frez, głębokość, głębokość przejścia i posuwy. Zaznaczenie bierze warstwę do zadania.",
            "Kontury mogą mieć Wejście i Wyjście: linię albo łuk od strony odpadu, żeby ślad w miejscu startu freza nie był na części, oraz zakładkę za punktem startu.",
            "Więcej operacji: Frezowanie gwintu (helisa wokół każdego okręgu, nakrętka albo śruba, ze skokami typowych śrub), Rowkowanie (żłobki) (rowki płytkie na początku i końcu), Listwa profilowa (zaokrąglenie, wklęsłość albo ogee prowadzone wzdłuż linii), Tekstura (wybrania frezem kulowym jak rzeźbione ręcznie) i Kształt 3D (kopuła, skos albo płaski wierzch z każdego zamkniętego kształtu). Warstwa z obrazem może użyć metody Rzeźba szkicowa: frez V wycina linie jak szkic ołówkiem. Wykończenie płaskorzeźby z opcją Resztki po warstwie frezuje tylko to, czego większy frez przed nim nie sięgnął.",
            "Dwie strony: Plik > Ustawienia projektu… > Obrabiane strony > 2 strony dają stronę A i stronę B, każdą rysowaną tak, jak leży materiał; otwory bazujące trafiają na ostatnią warstwę. Kliknij ducha, żeby wyrównać do niego zaznaczone kształty lub je do niego przyciągać; duch nigdy się nie przesuwa. Zapisz G-code pyta, czy obie strony zapisać w jednym pliku, z zatrzymaniem (M0) na odwrócenie materiału lub bez, czy w dwóch plikach z końcówką _A i _B; każda strona ma w nagłówku informację, którą jest. Podgląd G-code pyta tak samo i pokazuje cały projekt w jednej karcie albo każdą stronę w osobnej; Zapisz G-code w tej karcie też pyta. Widok 3D pokazuje frezowania obu powierzchni; tam, gdzie przechodzą przez materiał, jest otwór.",
            "Słupki i kostki: Plik > Ustawienia projektu… > Obrabiane strony: 4 strony (słupek) to słupek o podstawie a × b i wysokości H leżący wzdłuż X, frezowany na czterech długich ścianach A–D i obracany między nimi o ćwierć obrotu do tyłu (przód idzie do góry); 6 stron (kostka) to kostka a × b × c frezowana na wszystkich sześciu ścianach, E i F to jej lewy i prawy koniec skierowany do góry. Każda ściana ma własny obszar roboczy wielkości ściany, na materiale tak grubym, jak głęboki jest tam przedmiot; obszar roboczy to zawsze materiał (także dla 2 stron). Przełącznik nad obszarem roboczym pokazuje ścianę; ściany sąsiednie są rozłożone wokół niej z duchami, a ściana pod nią prześwituje. Zapisz G-code zapisuje wszystkie ściany w jednym pliku (każda od własnego zera, z zatrzymaniem przed każdą ścianą, które mówi, jak ułożyć przedmiot i ustawić zero Z na jego górze) albo w pliku dla każdej ściany (_A … _F). Widok 3D pokazuje cały przedmiot wyfrezowany na każdej ścianie, ułożony tak, jak maszyna widzi pokazaną ścianę; przelotowe frezowanie z dwóch przeciwległych ścian daje otwór.",
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
        title: "Uruchamianie zadania",
        lines: &[
            "Maszyna > Obrys obchodzi prostokąt zadania na wysokości bezpiecznej z wyłączonym wrzecionem. Maszyna > Rozpocznij zadanie wysyła zadanie; nowe zadanie najpierw sprawdź w powietrzu.",
        ],
    },
];

const EN_LAST: &[Section] = &[
    Section {
        title: "Safety",
        lines: &[
            "Test a new job in the air above the material, clamp the work, wear eye and ear protection and stay at the machine. Machine > Tool length sensor… measures the bits.",
        ],
    },
];

const PL_LAST: &[Section] = &[
    Section {
        title: "Bezpieczeństwo",
        lines: &[
            "Nowe zadanie sprawdź w powietrzu nad materiałem, zamocuj materiał, załóż ochronę oczu i uszu i zostań przy maszynie. Maszyna > Czujnik długości narzędzia… mierzy frezy.",
        ],
    },
];

/// The guide in English or Polish.
pub fn sections_in(polish: bool) -> Vec<Section> {
    let first = if polish { PL } else { EN };
    base_ui::guide::sections(Own { first, last: if polish { PL_LAST } else { EN_LAST }, machine: true, picture: Picture::Camera }, polish)
}

/// The guide in the language of the interface.
pub fn sections() -> Vec<Section> {
    sections_in(lang() == Lang::Pl)
}

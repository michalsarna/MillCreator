//! The quick guide of MillCreator: what is its own. The rest is the shared guide (`base_ui::guide`).
use base_ui::dialogs::Section;
use base_ui::guide::{Own, Picture};
use base_ui::i18n::{lang, Lang};

const EN: &[Section] = &[
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

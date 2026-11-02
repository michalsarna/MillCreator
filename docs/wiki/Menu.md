🌐 [English](Menus) · **Polski**

# Menu

Menu ma ten sam układ w LightCreatorze, MillCreatorze i PenCreatorze (pochodzi z baseCreatora), a MillCreator wstawia w swoje miejsca własne pozycje:

**Plik · Edycja · Wstawianie · Rozmieść · Widok · Maszyna · Frezowanie · Ustawienia · Pomoc** (File · Edit · Insert · Arrange · View · Machine · Milling · Settings · Help)

* **Windows i Linux:** pasek menu jest w oknie, pod paskiem tytułu, gdy otwarty jest rysunek (ekran startowy nie ma paska menu). Działają tam też klawisze pozycji (zob. [Skróty klawiszowe](Skroty-klawiszowe)).
* **macOS:** menu są na pasku menu systemu u góry ekranu, także na ekranie startowym (pozycja, która potrzebuje rysunku, jest tam szara). Pierwsze menu, **MillCreator**, ma **About MillCreator** (ten napis nie jest tłumaczony), usługi, ukrywanie / pokazywanie i **Zakończ** (Quit, Cmd+Q), więc **Plik** nie ma *Zakończ*, a **Pomoc** nie ma *O programie*. Klawisze menu działają na macOS trochę inaczej: zob. [Skróty klawiszowe](Skroty-klawiszowe#macos).

Klawisze są zapisane jak w Windows i Linuksie (Ctrl); na macOS Ctrl to Cmd. Pozycja, której teraz nie można użyć, jest szara (na przykład **Zapisz**, **Widok 3D zadania…** i **Eksportuj G-code…**, gdy żaden rysunek nie jest otwarty, **Wyśrodkuj względem siebie**, gdy zaznaczono mniej niż dwa kształty, **Dostosuj obraz…** i **Wektoryzacja obrazu…** bez zaznaczonego obrazu).

## Plik (File)

| Pozycja | Klawisz | Co robi |
|---|---|---|
| **Nowy** (New) | Ctrl+N | Otwiera nowy, pusty rysunek w osobnej karcie, a potem okno **Ustawienia projektu**. Zob. [Szybki start](Szybki-start). |
| **Otwórz…** (Open…) | Ctrl+O | Otwiera projekt (`.cre`) albo rysunek SVG, PDF lub Illustratora w osobnej karcie; projekt, który jest już otwarty, zostaje pokazany, a nie otwarty drugi raz. Wybrany tu DXF, projekt UGS i model 3D (STL, OBJ, 3MF) są importowane do rysunku. Plik upuszczony na okno idzie tą samą drogą, a także obraz albo projekt LightBurn (import) i plik G-code (pokazany w analizie G-code). |
| **Otwórz ostatnie** (Open recent) | | Ostatnie pliki (do 10); **Wyczyść listę** ją opróżnia. |
| **Zapisz** (Save) | Ctrl+S | Zapisuje projekt (`.cre`). |
| **Zapisz jako…** (Save as…) | Ctrl+Shift+S | Zapisuje projekt pod inną nazwą. |
| **Ustawienia projektu…** (Project setup…) | | Rozmiar i grubość materiału, zero programu na nim, Z od spodu, obszar roboczy jako materiał. Zob. [Szybki start](Szybki-start#2-ustawienia-projektu). |
| **Importuj ▸ Import obrazu…** | | Obraz (PNG, JPG, BMP, GIF, WebP) na stronę, z oknem importu. Zob. [Narzędzia projektowe](Narzedzia-projektowe). |
| **Importuj ▸ Importuj DXF…** | | DXF w ASCII jako obrysy; jego warstwy trafiają na wolne warstwy projektu. |
| **Importuj ▸ Importuj projekt LightBurn…** | | Kształty projektu `.lbrn2` / `.lbrn`. |
| **Importuj ▸ Importuj projekt UGS…** | | Projekt `.ugsd` z UGS Designera, ze sposobami cięcia jako warstwami. |
| **Importuj ▸ Importuj model 3D jako relief…** | | Model STL, OBJ lub 3MF jako mapa głębokości na warstwie reliefu. Zob. [Warstwy i operacje](Warstwy-i-operacje#obrazy). |
| **Importuj ▸ Importuj wtyczką…** | | Plik, który czyta wtyczka. Zob. [Wtyczki](Wtyczki). |
| **Importuj ▸ Generuj kształt…** | | Okno generatorów kształtów z wtyczek (także w menu **Wstawianie**). |
| **Eksport ▸ Eksportuj G-code…** | | To samo co **Frezowanie → Eksportuj G-code…**. |
| **Eksport ▸ Eksportuj relief jako STL…** | | To samo co **Frezowanie → Eksportuj relief jako STL…**. |
| **Eksport ▸ Eksportuj zaznaczenie jako SVG…** | | Zapisuje zaznaczone kształty (wszystkie, gdy nic nie jest zaznaczone) jako SVG. |
| **Eksport ▸ Eksportuj zaznaczenie jako DXF…** | | To samo jako DXF R12, z warstwą DXF dla każdej warstwy projektu. |
| **Pokaż plik G-code…** (View G-code file…) | | Otwiera plik G-code w analizie G-code: jego ścieżkę w 2D lub 3D, długość cięcia i czas. |
| **Zakończ** (Quit) | Ctrl+Q | Kończy program; najpierw pyta o niezapisaną pracę. Nie na macOS (tam jest w menu **MillCreator**). |

## Edycja (Edit)

| Pozycja | Klawisz | Co robi |
|---|---|---|
| **Cofnij** (Undo) | Ctrl+Z | Cofa ostatnią zmianę. |
| **Ponów** (Redo) | Ctrl+Y | Wykonuje ją ponownie (także Ctrl+Shift+Z). |
| **Kopiuj** (Copy) | Ctrl+C | Kopiuje zaznaczone kształty. |
| **Wklej** (Paste) | Ctrl+V | Wstawia skopiowane kształty. |
| **Powiel** (Duplicate) | Ctrl+D | Kopiuje zaznaczenie jednym krokiem. |
| **Usuń** (Delete) | Del | Usuwa zaznaczenie (Delete albo Backspace); gdy narzędziem węzłów zaznaczono węzły, usuwa węzły. |
| **Zaznacz wszystko** (Select all) | Ctrl+A | Zaznacza wszystkie kształty. |

## Wstawianie (Insert)

Narzędzia lewej palety, które wstawiają obiekty; pozycja wybiera narzędzie (używane jest zaznaczone ptaszkiem; nie na pasku menu macOS, który zaznacza tylko **Siatka**, **Przyciągaj do siatki**, **Przyciągaj do obiektów**, język i schemat kolorów), a litera w nazwie to jego klawisz:

**Prostokąt (R)**, **Elipsa (E)**, **Trójkąt (Y)**, **Gwiazda (S)**, **Wielokąt (G)**, **Serce (K)**, **Spirala (I)**, **Linia (L)**, **Linia łamana / pióro (P)**, **Tekst (T)**.

Dalej **Generuj kształt…** (generatory z wtyczek, zob. [Wtyczki](Wtyczki)) i **Online clipart…** (ta pozycja nie jest tłumaczona; otwiera kartę **Clipart** i okno **Cliparty z internetu**, wyszukiwanie darmowych obrazków w internecie). Zob. [Narzędzia projektowe](Narzedzia-projektowe).

## Rozmieść (Arrange)

| Pozycja | Klawisz | Co robi |
|---|---|---|
| **Wyrównaj do lewej**, **Wyśrodkuj w poziomie**, **Wyrównaj do prawej**, **Wyrównaj do góry**, **Wyśrodkuj w pionie**, **Wyrównaj do dołu** | | Wyrównują zaznaczone kształty. |
| **Wyśrodkuj na stole** (Centre on bed) | | Stawia zaznaczenie na środku obszaru roboczego. |
| **Wyśrodkuj względem siebie** (Centre on each other) | | Stawia zaznaczone kształty na jednym środku (dwa lub więcej). |
| **Odbij w poziomie**, **Odbij w pionie** | | Odbijają zaznaczenie. |
| **Obróć 90° w prawo**, **Obróć 90° w lewo** | | Obracają zaznaczenie o ćwierć obrotu. |
| **Grupuj** (Group) | Ctrl+G | Robi z zaznaczonych kształtów jedną grupę. |
| **Rozgrupuj** (Ungroup) | Ctrl+Shift+G | Rozbija grupę. |
| **Zablokuj** (Lock) | Ctrl+L | Kształtów nie można już przesuwać ani zmieniać. |
| **Odblokuj** (Unlock) | Ctrl+Shift+L | Zwalnia zablokowane kształty. |
| **Przesuń na wierzch**, **Przesuń na spód** | | Kolejność kształtów. |
| **Edytuj węzły** (Edit nodes) | N | Narzędzie węzłów: przesuwanie punktów linii. |
| **Zamień na ścieżkę** (Convert to path) | | Zamienia prostokąt, elipsę, gwiazdę… na ścieżkę, której węzły można edytować. |
| **Suma**, **Część wspólna**, **Odejmij**, **Różnica symetryczna** | | Operacje logiczne na dwóch lub więcej kształtach. |
| **Przesuń obrys…**, **Zaokrąglij rogi…**, **Zamień na krzywe** | | Większy lub mniejszy obrys, zaokrąglone rogi, krzywe z linii. |
| **Połącz ścieżki automatycznie…**, **Zamknij ścieżkę z tolerancją…** | | Łączą stykające się linie w ścieżki, zamykają prawie zamknięte ścieżki. |
| **Narożniki dogbone / T-bone…** | | Wycięcia w wewnętrznych narożnikach, żeby kwadratowa część weszła w rowek wycięty okrągłym frezem. |
| **Tablica siatkowa…**, **Tablica kołowa…**, **Kopiuj wzdłuż ścieżki…**, **Tekst na ścieżce…** | | Kopie w wierszach i kolumnach, wokół środka albo wzdłuż ścieżki; tekst wygięty wzdłuż ścieżki. |
| **Przytnij kształtem…**, **Usuń duplikaty**, **Rozdziel**, **Uprość…** | | Przycinanie linii kształtem, usuwanie podwójnych kształtów, jeden kształt na obrys, mniej punktów. |
| **Tekst seryjny…**, **Kod kreskowy / QR…**, **Rozmieść kształty (nesting)…**, **Pomiar…** | | Numerowane kopie tekstu, kody jako obrysy, upakowanie kształtów na obszarze, wymiary i odległości. |
| **Dostosuj obraz…**, **Wektoryzacja obrazu…** | | Zmiana obrazu na stronie; zamiana go na obrysy. |

Wszystkie są opisane w [Narzędziach projektowych](Narzedzia-projektowe).

## Widok (View)

| Pozycja | Klawisz | Co robi |
|---|---|---|
| **Siatka**, **Przyciągaj do siatki**, **Przyciągaj do obiektów** | | Przełączniki (zaznaczone, gdy włączone). Samą siatkę ustawia się w **Ustawienia → Opcje widoku…**. |
| **Powiększ** (Zoom in) | Ctrl++ | Powiększa wokół środka obszaru roboczego (klawisz to Ctrl i `=`). |
| **Pomniejsz** (Zoom out) | Ctrl+- | Pomniejsza. |
| **Dopasuj stół do okna** (Fit bed to window) | Ctrl+Shift+0 | Cały obszar roboczy w oknie. |
| **Dopasuj do wszystkich obiektów** (Fit all objects) | Ctrl+Shift+9 | Wszystkie kształty w oknie. |
| **Automatycznie dopasuj obszar roboczy do okna** | | Przełącznik (zaznaczony, gdy włączony, poza macOS): obszar roboczy dopasowuje się do okna przy zmianie jego rozmiaru (ręczne powiększanie go wyłącza). |
| **Podziel widok: obok siebie**, **Podziel widok: jeden nad drugim** | | Dzielą środkową część okna na panele (gdy jest jeden, dochodzi drugi). |
| **Jeden widok** (Single view) | | Wszystkie karty znów w jednym panelu. |
| **Obraz w tle…** (Background picture…) | | Zdjęcie albo plan pod rysunkiem. Zob. [Narzędzia projektowe](Narzedzia-projektowe). |

Panele i karty: [Narzędzia projektowe](Narzedzia-projektowe).

## Maszyna (Machine)

Najpierw ustawienia maszyny, potem to, co na niej działa. Zob. [Sterowanie maszyną](Sterowanie-maszyna).

| Pozycja | Co robi |
|---|---|
| **Ustawienia urządzenia…** (Device settings…) | Ustawienia używanego urządzenia (strony są opisane w [Urządzeniach i kamerach](Urzadzenia-i-kamery)). |
| **Ustawienia firmware ($$)…** | Ustawienia sterownika GRBL, z kopiami zapasowymi. |
| **Kalibracja…** (Calibration…) | Kroki na milimetr i prostopadłość osi. |
| **Czujnik długości narzędzia…** (Tool length sensor…) | Kreator czujnika długości narzędzia. |
| **Diagnostyka i statystyki…** | Wejścia sterownika, zadania, godziny, alarmy i błędy, przypomnienia o konserwacji, eksport raportu. |
| **Start od wiersza…** (Start from line…) | Zaczyna zadanie od wiersza programu. |
| **Sprawdź program** (Check the program) | Sterownik przechodzi program w trybie sprawdzania (`$C`): nic się nie rusza, a wiersze, których nie rozumie, są zgłaszane. Wymaga połączonej maszyny. |

## Frezowanie (Milling)

| Pozycja | Klawisz | Co robi |
|---|---|---|
| **Biblioteka frezów…** (Bit library…) | | Gotowe frezy i twoje. Zob. [Biblioteka frezów](Biblioteka-frezow). |
| **Widok 3D zadania…** (3D view of the job…) | Ctrl+Shift+P | Otwiera kartę **Podgląd**: materiał, cięcia i ruchy w 3D, z animacją. Zob. [Widok 3D i G-code](Widok-3D-i-G-code). |
| **Eksportuj G-code…** (Export G-code…) | Ctrl+E | Pyta o nazwę pliku (`nazwa.nc`) i zapisuje G-code projektu, przez włączone postprocesory. To samo co przycisk **Zapisz G-code**. |
| **Eksportuj relief jako STL…** | | Zapisuje relief 3D obrazu jako model STL (tylko gdy na warstwie z metodą relief 3D jest obraz). |

## Ustawienia (Settings)

| Pozycja | Co robi |
|---|---|
| **Opcje widoku…** (View options…) | Siatka główna i pomocnicza, kolory obszaru roboczego, grubość linii, **Pokaż nazwy warstw**, **Podgląd G-code po zapisaniu**, **Pokaż nazwy zmiennych**. Zob. [Ustawienia i pliki](Ustawienia-i-pliki). |
| **Opcje programu…** (Program options…) | **Szukaj nowszej wersji przy uruchamianiu programu**. |
| **Wtyczki…** (Plugins…) | Instalowanie, włączanie, wyłączanie i usuwanie wtyczek, oficjalna lista. Zob. [Wtyczki](Wtyczki). |
| **Język ▸** (Language) | 30 języków programu. |
| **Schemat kolorów ▸** (Colour scheme) | 14 schematów kolorów okna oraz świąteczne w okolicy Bożego Narodzenia, Nowego Roku, Halloween i Wielkanocy. |

## Pomoc (Help)

| Pozycja | Co robi |
|---|---|
| **Strona projektu na GitHubie** | Otwiera repozytorium MillCreatora w przeglądarce. |
| **Pobierz kopię zapasową…** (Download backup…) | Zapisuje ustawienia i wspólną galerię clipartów w jednym pliku zip. Zob. [Ustawienia i pliki](Ustawienia-i-pliki). |
| **Przywróć z kopii zapasowej…** (Restore from backup…) | Wczytuje taki plik zip z powrotem. |
| **Sprawdź aktualizacje…** (Check for updates…) | Szuka nowszego wydania MillCreatora. |
| **O programie** (About) | Wersja, licencja i podziękowania. Nie na macOS (tam **MillCreator → About MillCreator**). |

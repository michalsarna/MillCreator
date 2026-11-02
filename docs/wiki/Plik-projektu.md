🌐 [English](Project-File) · **Polski**

# Plik projektu

MillCreator zapisuje **`.cre`**, plik projektu baseCreatora, który zapisuje też LightCreator: jeden plik JSON z rysunkiem, opisem i **jedną sekcją na program**. Każdy program zapisuje swoją sekcję i zostawia nietknięte sekcje pozostałych, więc projekt przechodzi między programami bez strat.

Sekcja MillCreatora nazywa się `mill`. Zawiera:
* **64 warstwy** ze wszystkimi ustawieniami (nazwa, kolor, kolejność, operacja, frez, głębokości, rozstaw przejść, posuwy, obroty, kierunek, wejście, mostki i mostki ustawione ręcznie, chłodziwo…),
* **materiał** (rodzaj, kolor, grubość, pierwsze przejście) i **ustawienia projektu** (rozmiar kawałka, zero programu na nim, Z od spodu),
* **migawkę urządzenia** (nazwa i pole robocze),
* **frezy używane przez warstwy**, żeby projekt dało się otworzyć tam, gdzie ich nie ma w bibliotece.

Otwarcie projektu z LightCreatora (albo zwykłego SVG, PDF lub pliku LightBurna) daje domyślne warstwy; jego pozostałe sekcje (`laser`) są zachowane i zapisane z powrotem przy zapisie. Projekt z nowszym formatem `mill`, niż zna program, jest odrzucany z komunikatem. Warstwa z błędnymi liczbami jest zastępowana domyślną; brakujący frez trafia do **Moich frezów**.

Samo urządzenie nie jest w projekcie, tylko jego migawka: używane jest to, które wybrano na ekranie startowym. Pasek stanu ostrzega, gdy pole robocze migawki nie jest polem tego urządzenia.

Opis projektu (karta **Opis**) jest częścią wspólnego kontenera, więc LightCreator pokazuje ten sam tekst. MillCreator wpisuje go też na początek G-code jako komentarze `(Note: …)` (zob. [Widok 3D i G-code](Widok-3D-i-G-code#eksport-g-code)).

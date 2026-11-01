🌐 [English](3D-View-and-G-code) · **Polski**

# Widok 3D i G-code

## Ścieżki
Z kształtów, warstw, frezów i urządzenia program tworzy ruchy maszyny. Współrzędne są współrzędnymi pola roboczego: X w prawo, Y od ciebie (rysunek jest odwracany, bo edytor liczy Y w dół), Z = 0 na wierzchu materiału i ujemne pod nim.

* **Profile** idą wokół kształtu w odległości promienia freza, przejściami w dół do głębokości warstwy; kolejne przejście następuje bez podnoszenia freza.
* **Kieszenie** są wybierane pierścieniami od obrysu do środka, co krok boczny.
* **Follow the line** idzie wzdłuż linii i z powrotem, przejście po przejściu.
* **Drill** wierci porcjami o głębokość przejścia i wychodzi na powierzchnię między nimi.
* Między cięciami frez wznosi się na **bezpieczną wysokość** i porusza się tam z prędkością ruchów nad materiałem, potem schodzi szybko na 1 mm nad powierzchnię i wchodzi z posuwem wejścia.

## Okno 3D
**Milling → 3D view of the job…** (Ctrl/Cmd+Shift+P). Pokazuje tylko obszar projektu, **o 10 % większy niż projekt** w każdym kierunku, jako blok wybranego materiału o jego grubości.

![Widok 3D](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/16-3d-view.png)

| Element | Co robi |
|---|---|
| przeciąganie | obraca widok |
| kółko myszy | zoom |
| przeciąganie prawym lub środkowym | przesuwa obraz |
| **Reset view** lub dwuklik | powrót do pierwszego widoku |
| Material, Cuts, Moves at the safe height | pokazuje lub ukrywa każdą część; niebieska ramka to bezpieczna wysokość |
| animacja | z powrotem na początek, odtwarzanie / pauza, suwak postępu, szybkość (×0.25 do ×25) i powtarzanie; zadanie jest pokazane do pewnej części jego długości, z zaznaczonym frezem; materiał pokazuje cięcia wykonane do tej pory |
| **3D** / **From above** | obraz 3D albo plan zadania widziany z góry (nie da się go obracać) |

**Material** to materiał tak, jak jest sfrezowany: rowki po frezach są wyżłobione w jego wierzchu, do miejsca, do którego doszedł suwak, z prawdziwym kształtem każdego freza (frez V poszerza się z głębokością). **Cuts** dokłada tylko ścieżki freza, kolorem ich warstwy, nad nim. Okno pokazuje długość cięć, szacunek czasu i ostrzeżenia, a po każdej zmianie projektu tworzy program od nowa.

![Obrócony widok](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/17-3d-view-turned.png)

Widok to rysunek liniowy ścieżek na bloku: nie usuwa materiału, a czas to szacunek z posuwów, nie symulacja sterownika.

## G-code
**Export G-code…** zapisuje plik w stylu GRBL: `G21 G90 G17`, wrzeciono `S… M3` i czekanie urządzenia, `G0` dla ruchów nad materiałem i `G1` z posuwem dla zagłębiania i cięć, na końcu `M5`, bezpieczna wysokość, zero maszyny i `M30`. Współrzędne są liczone od **zera maszyny** urządzenia. Gdy kolejna warstwa używa innego freza, plik zatrzymuje się na `M0 (Change the bit to: …)` po wyłączeniu wrzeciona. Ostrzeżenia trafiają na pasek stanu.

**Przy przystawce obrotowej** (ustawienia urządzenia → Rotary, włączona) podgląd pokazuje przedmiot jako **walec** wzdłuż X: Y programu to droga dookoła jego powierzchni, więc rysunek jest nawinięty na walec, środkiem zadania do góry, a cięcia i ruchy na bezpiecznej wysokości idą po powierzchni. Cięcia schodzą w stronę osi. Obraz obejmuje cały obwód, więc zadanie wyższe niż obwód pokazuje to samo miejsce dwa razy (i program ostrzega). Eksport sfrezowanego elementu jako STL nie jest wykonywany przy obrotnicy (byłaby to płaska płyta).

Ruchy każdej operacji są w pliku i w podglądzie dokładnie takie, jak zapisane: rampy, helisy, mostki, pierścienie V-carve na swoich głębokościach. Przy **zmieniaczu narzędzi** zmiana freza to `T<n> M6` zamiast pauzy `M0`; **chłodziwo** (`M7` / `M8`, `M9`) i linie **odkurzacza** idą za warstwami. Przy **ustawieniach projektu** program zaczyna się od `G54` i liczy od wybranego zera; przy *Z od spodu* każde Z to wysokość nad spodem materiału. **Postprocesor** z wtyczki (na przykład Mach3 / Mach4 / UCCNC albo LinuxCNC z *G-code dialects*) przepisuje plik przy zapisie.

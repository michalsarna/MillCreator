🌐 [English](Home) · **Polski**

<p align="center"><img src="https://raw.githubusercontent.com/michalsarna/MillCreator/main/assets/icons/icon-256.png" alt="MillCreator" width="128"></p>

# MillCreator

Otwartoźródłowy program do projektowania i przygotowania zadań dla frezarek CNC, napisany w Rust. Rysuj lub importuj kształty, przypisz warstwom operacje i frezy, obejrzyj zadanie w 3D i zapisz G-code, na macOS, Windows i Linuksie. To rodzeństwo [LightCreatora](https://github.com/michalsarna/LightCreator) (laser): to samo okno, ten sam edytor i te same pliki dzięki [baseCreatorowi](https://github.com/michalsarna/baseCreator).

![Warstwy i edytor](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/05-layers.png)

## Od czego zacząć

| Chcę… | Przejdź do |
|---|---|
| Zainstalować program | [Instalacja](Instalacja) |
| Przygotować pierwsze zadanie | [Szybki start](Szybki-start) |
| Rysować, edytować i importować kształty | [Narzędzia projektowe](Narzedzia-projektowe) |
| Dodać maszynę i kamerę | [Urządzenia i kamery](Urzadzenia-i-kamery) |
| Powiedzieć, co jest frezowane i jak | [Warstwy i operacje](Warstwy-i-operacje) |
| Wybrać lub zdefiniować frez | [Biblioteka frezów](Biblioteka-frezow) |
| Ustawić materiał | [Materiał](Material-obrabiany) |
| Zobaczyć zadanie w 3D, zapisać G-code | [Widok 3D i G-code](Widok-3D-i-G-code) |
| Dzielić projekt z LightCreatorem | [Plik projektu](Plik-projektu) |
| Ustawić materiał i zero nowego zadania | [Szybki start](Szybki-start) (Project setup) |
| Steruj maszyną: przesuw, zerowanie, sondowanie, firmware, kalibracja | [Sterowanie maszyną](Sterowanie-maszyna) |
| Zobaczyć, jak to wygląda | [Zrzuty ekranu](Zrzuty-ekranu) |
| Znaleźć plik ustawień | [Ustawienia i pliki](Ustawienia-i-pliki) |
| Poczytać o kodzie | [Architektura](Architektura) |
| Wiedzieć, co jest testowane | [Bezpieczeństwo i ograniczenia](Bezpieczenstwo-i-ograniczenia) |

## W skrócie

* **Urządzenia:** twoje maszyny (pole robocze, skok w Z, bezpieczna wysokość, zakres wrzeciona, kamery) na jednej liście; **Nie mam urządzenia** daje zastępcze o nazwie Dummy.
* **Warstwy:** 64 warstwy, każda z operacją (cięcie na zewnątrz / wewnątrz, podążanie za linią, kieszeń pierścieniami lub rastrem, kieszeń resztkowa, planowanie, wiercenie, rozwiercanie, fazowanie, szczelina dogbone, V-carve z inlayami, obrazy i relief 3D), frezem, głębokością, głębokością przejścia, posuwami, obrotami wrzeciona, mostkami, rampami lub helisą, współbieżnie lub przeciwbieżnie, naddatkiem, chłodziwem; ustawienia można zapisać jako presety.
* **Frezy:** biblioteka gotowych frezów i twoich własnych (wspólna dla wszystkich urządzeń), siedem rodzajów, każdy z rysunkiem pokazującym, gdzie mierzony jest każdy wymiar; import i eksport.
* **Materiał:** drewno w trzech kolorach, metal, plastik w wybranym kolorze oraz grubość.
* **Widok 3D:** blok materiału, cięcia i ruchy na bezpiecznej wysokości; obrót, zoom, reset; suwak odtwarza zadanie.
* **G-code:** w odmianie GRBL, z pauzą na wymianę frezu albo `T… M6` dla zmieniacza narzędzi, własnymi liniami (początek, koniec, zmiana freza lub operacji), układem współrzędnych i zerem na materiale; Mach3 / Mach4 / UCCNC i LinuxCNC przez wtyczki.
* **Maszyna:** GRBL (od 0.7 do 1.1 i nowszego), Smoothieware albo Duet przez HTTP; przesuw, sondowanie, środek otworu, korekty, zapisane pozycje, chłodziwo.
* **Rysowanie:** import DXF, SVG, PDF, Illustratora, LightBurna i UGS, tablice, kopie wzdłuż ścieżki, tekst na ścieżce, przycinanie, nesting, kody QR i kreskowe, tekst seryjny oraz modele STL / OBJ / 3MF jako relief.
* **Wspólne z LightCreatorem:** edytor, 310 clipartów, import SVG / PDF / Illustratora i LightBurna, plik projektu `.cre`, 30 języków (wspólne teksty) i 14 schematów kolorów.

> ⚠️ **Bezpieczeństwo:** frezarki są niebezpieczne. Nowe zadanie sprawdź najpierw w powietrzu nad materiałem, noś ochronę oczu i słuchu, zamocuj materiał, nigdy nie zostawiaj pracującej maszyny bez nadzoru. Zobacz [Bezpieczeństwo i ograniczenia](Bezpieczenstwo-i-ograniczenia).

Najnowsza wersja: **v0.5.1** · [Wydania](https://github.com/michalsarna/MillCreator/releases) · [Zgłoszenia](https://github.com/michalsarna/MillCreator/issues) · Licencja: [MIT](https://github.com/michalsarna/MillCreator/blob/main/LICENSE) lub [Apache-2.0](https://github.com/michalsarna/MillCreator/blob/main/LICENSE-APACHE), do wyboru

🌐 [English](Quick-Start) · **Polski**

# Szybki start

1. **Wybierz urządzenie.** Na ekranie startowym wskaż swoją maszynę albo naciśnij **Add device…**. Bez maszyny naciśnij **I have no device**: powstaje i jest wybrane zastępcze urządzenie *Dummy* (300 × 200 × 60 mm). Jeśli naciśniesz **New** lub **Open…** bez żadnego urządzenia, Dummy jest tworzone i używane. Zobacz [Urządzenia i kamery](Urzadzenia-i-kamery).
2. **Zacznij projekt.** **New** zaczyna pusty projekt; **Open…** wczytuje plik SVG, PDF, Illustratora lub `.cre`; **File → Import → Import LightBurn project…** wczytuje projekt LightBurna, a **Import UGS design…** plik UGS Designera (kształty są dodawane do rysunku). Albo rysuj narzędziami po lewej.
3. **Przypisz kształty do warstw.** Zaznacz kształty, otwórz kartę **Layers**, kliknij warstwę i naciśnij **Put the selected shapes on this layer**. Pierwsze warstwy są gotowe: *Cut out* (na zewnątrz linii), *Holes and windows* (wewnątrz linii), *Pocket* (kieszeń), *Engrave* (po linii) i *Drill* (wiercenie). Zobacz [Warstwy i operacje](Warstwy-i-operacje).
4. **Wybierz frez i głębokość** każdej używanej warstwy. Każdy gotowy frez przynosi swój posuw i obroty. **Bit library…** (menu Milling albo przycisk **…**) pokazuje je i pozwala edytować: [Biblioteka frezów](Biblioteka-frezow).
5. **Ustaw materiał** i jego grubość na karcie **Material**: [Materiał](Material-obrabiany).
6. **Obejrzyj zadanie:** **Milling → 3D view of the job…** pokazuje cięcia i ruchy na bezpiecznej wysokości. Ostrzeżenia (brak freza, głębiej niż materiał, poza polem roboczym) są wypisane nad obrazem. Zobacz [Widok 3D i G-code](Widok-3D-i-G-code).
7. **Zapisz G-code** z okna 3D albo z **Milling → Export G-code…** i zapisz projekt (`.cre`).

> Program zapisuje G-code i może przesłać go do podłączonej maszyny GRBL (karta Device: Connect; pasek: Frame, Start, Stop), ale nie było to sprawdzone na prawdziwej maszynie. Sprawdź zadanie najpierw w powietrzu i bądź przy maszynie. Zobacz [Bezpieczeństwo i ograniczenia](Bezpieczenstwo-i-ograniczenia).

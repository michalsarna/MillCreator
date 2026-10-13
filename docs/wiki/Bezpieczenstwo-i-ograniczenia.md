🌐 [English](Safety-and-Limitations) · **Polski**

# Bezpieczeństwo i ograniczenia

> ⚠️ Frezarka może cię zranić i zniszczyć pracę, frez i siebie. Program jest młody i **autor nie uruchamiał go na maszynie**. Używasz go na własne ryzyko.

* **Najpierw w powietrzu:** podnieś maszynę tak, żeby frez niczego nie dotknął, uruchom G-code i obserwuj ścieżkę. Dopiero potem tanim kawałkiem.
* Noś ochronę oczu i słuchu, zamocuj materiał, trzymaj rękę blisko przycisku stop, nigdy nie zostawiaj pracującej maszyny bez nadzoru.
* **Posuwy i obroty** gotowych frezów to ostrożne punkty wyjścia; nie były sprawdzane na twojej maszynie.
* **Sprawdź zero:** G-code liczy od zera maszyny ustawionego w urządzeniu (który róg) i od Z = 0 na wierzchu materiału. Ustaw to samo na maszynie.
* **Co jest testowane:** geometria ścieżek (odległość od linii, przejścia, bezpieczna wysokość), biblioteka frezów z jej sprawdzaniem i plikami, pliki urządzeń, plik projektu i tekst G-code, automatycznymi testami przy każdej zmianie. Obrazy w tej wiki generuje sam program.

## Czego nie robi
* **Nie łączy się z maszyną i nią nie steruje**; tylko zapisuje G-code.
* Brak mostków, najazdów skośnych i spiralnych (frez wchodzi prosto w dół), wyboru współbieżnie / przeciwbieżnie, V-carvingu, reliefu 3D, korekcji długości narzędzia i sprawdzania kolizji z zaciskami.
* Kieszeń zostawia cienki pasek pośrodku, gdy jej szerokość nie jest wielokrotnością kroku bocznego.
* Kształt poza polem roboczym jest zgłaszany, ale nie przesuwany.
* Obrazy nie są frezowane. Tekst jest frezowany jako obrys (linia za nim idzie).
* Widok 3D rysuje linie na bloku; nie usuwa materiału.
* Czas jest szacunkiem.

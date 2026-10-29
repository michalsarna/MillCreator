🌐 [English](Safety-and-Limitations) · **Polski**

# Bezpieczeństwo i ograniczenia

> ⚠️ Frezarka może cię zranić i zniszczyć pracę, frez i siebie. Program jest młody i **był testowany na TwoTrees TTC450Pro i na niej działa, ale nie na innych maszynach**. Używasz go na własne ryzyko.

* **Najpierw w powietrzu:** podnieś maszynę tak, żeby frez niczego nie dotknął, uruchom G-code i obserwuj ścieżkę. Dopiero potem tanim kawałkiem.
* Noś ochronę oczu i słuchu, zamocuj materiał, trzymaj rękę blisko przycisku stop, nigdy nie zostawiaj pracującej maszyny bez nadzoru.
* **Posuwy i obroty** gotowych frezów to ostrożne punkty wyjścia; nie były sprawdzane na twojej maszynie.
* **Sprawdź zero:** G-code liczy od zera maszyny ustawionego w urządzeniu (który róg) i od Z = 0 na wierzchu materiału. Ustaw to samo na maszynie.
* **Co jest testowane:** geometria ścieżek (odległość od linii, przejścia, bezpieczna wysokość), biblioteka frezów z jej sprawdzaniem i plikami, pliki urządzeń, plik projektu i tekst G-code, automatycznymi testami przy każdej zmianie. Obrazy w tej wiki generuje sam program.

## Czego nie robi
* Może **połączyć się z maszyną GRBL i przesłać zadanie** (Connect, Home, Frame, Start, Stop); sprawdzone na TwoTrees TTC450Pro, **nie na innych maszynach**. Zaczynaj z wyłączonym wrzecionem i frezem w powietrzu i bądź przy maszynie; Stop przerywa miękkim resetem.
* Frame obchodzi obrys pracy na bezpiecznej wysokości przy wyłączonym wrzecionie: sprawdź, czy ta wysokość naprawdę jest wolna.
* Brak V-carvingu, korekcji długości narzędzia i sprawdzania kolizji z zaciskami. Mostki rozmieszcza program (równomiernie) albo Ty ręcznie na obszarze roboczym.
* Rampy, helisy, mostki i wybór współbieżnie / przeciwbieżnie są sprawdzone na geometrii i na firmware GRBL w symulatorze, **nie na prawdziwej maszynie**: nowe ustawienie wypróbuj najpierw na odpadzie i z wyłączonym wrzecionem. Mostek trzyma część tylko wtedy, gdy jest dość wysoki: program ostrzega, gdy jest tak wysoki jak cięcie. Helisa potrzebuje miejsca: tam, gdzie się nie mieści, używana jest rampa po konturze.
* Kieszeń zostawia cienki pasek pośrodku, gdy jej szerokość nie jest wielokrotnością kroku bocznego.
* Kształt poza polem roboczym jest zgłaszany, ale nie przesuwany.
* Obrazy nie są frezowane. Tekst jest frezowany jako obrys (linia za nim idzie).
* Widok 3D pokazuje materiał sfrezowany do miejsca, do którego doszedł suwak (siatka, gęstsza przy małym zadaniu); to obraz do oglądania, nie pomiar.
* Czas jest szacunkiem.

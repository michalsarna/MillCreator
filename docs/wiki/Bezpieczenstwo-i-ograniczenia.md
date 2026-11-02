🌐 [English](Safety-and-Limitations) · **Polski**

# Bezpieczeństwo i ograniczenia

> ⚠️ Frezarka może cię zranić i zniszczyć pracę, frez i siebie. Program jest młody i **był testowany na TwoTrees TTC450Pro i na niej działa, ale nie na innych maszynach**. Używasz go na własne ryzyko.

* **Najpierw w powietrzu:** podnieś maszynę tak, żeby frez niczego nie dotknął, uruchom G-code i obserwuj ścieżkę. Dopiero potem tanim kawałkiem.
* Noś ochronę oczu i słuchu, zamocuj materiał, trzymaj rękę blisko przycisku stop, nigdy nie zostawiaj pracującej maszyny bez nadzoru.
* **Posuwy i obroty** gotowych frezów to ostrożne punkty wyjścia; nie były sprawdzane na twojej maszynie.
* **Sprawdź zero:** G-code liczy od zera maszyny ustawionego w urządzeniu (który róg) i od Z = 0 na wierzchu materiału. Jeśli użyłeś **Plik → Ustawienia projektu…**, liczy od zera wybranego na materiale (G54 z tym zerem: ustaw je tam na maszynie) i, jeśli tak wybrano, Z od spodu materiału. Ustaw to samo na maszynie i obejrzyj pierwsze ruchy z frezem w powietrzu.
* **Co jest testowane:** geometria ścieżek (odległość od linii, przejścia, bezpieczna wysokość), biblioteka frezów z jej sprawdzaniem i plikami, pliki urządzeń, plik projektu i tekst G-code, automatycznymi testami przy każdej zmianie. Obrazy w tej wiki generuje sam program.

## Czego nie robi
* Może **połączyć się z maszyną GRBL i przesłać zadanie** (Connect, Home, Frame, Start, Stop); sprawdzone na TwoTrees TTC450Pro, **nie na innych maszynach**. Zaczynaj z wyłączonym wrzecionem i frezem w powietrzu i bądź przy maszynie; Stop przerywa miękkim resetem.
* Frame obchodzi obrys pracy na bezpiecznej wysokości przy wyłączonym wrzecionie: sprawdź, czy ta wysokość naprawdę jest wolna.
* Brak sprawdzania kolizji z zaciskami i korekcji długości narzędzia (czujnik długości narzędzia ustawia Z dla nowego freza). Mostki rozmieszcza program (równomiernie) albo Ty ręcznie na obszarze roboczym.
* **V-carve:** ściany są dokładne niezależnie od odstępu pierścieni, ale płaskie dno szerokiego kształtu jest frezowane samym frezem V (grzbiety, długi program; program o tym ostrzega): użyj do niego freza płaskiego w warstwie Pocket. Wkładkę inlaya rzeźbi się na odwrocie kawałka: odbij jej kształty lustrzanie.
* Rampy, helisy, mostki i wybór współbieżnie / przeciwbieżnie są sprawdzone na geometrii i na firmware GRBL w symulatorze, **nie na prawdziwej maszynie**: nowe ustawienie wypróbuj najpierw na odpadzie i z wyłączonym wrzecionem. Mostek trzyma część tylko wtedy, gdy jest dość wysoki: program ostrzega, gdy jest tak wysoki jak cięcie. Helisa potrzebuje miejsca: tam, gdzie się nie mieści, używana jest rampa po konturze.
* Kształt poza polem roboczym jest zgłaszany, ale nie przesuwany.
* Obraz jest frezowany tylko na warstwie z operacją **Obraz** (Picture); na każdej innej warstwie jest tylko pokazywany (z ostrzeżeniem). Tekst jest frezowany jako obrys (linia za nim idzie).
* Widok 3D pokazuje materiał sfrezowany do miejsca, do którego doszedł suwak (siatka, gęstsza przy małym zadaniu); to obraz do oglądania, nie pomiar.
* Czas jest szacunkiem.

## Co jest nowe i nie było próbowane na prawdziwej maszynie
Wszystko poniżej sprawdzono testami jednostkowymi, renderami, a tam, gdzie zmienia G-code, na firmware GRBL w symulatorze, **nie na prawdziwych sterownikach**. Każdą z tych rzeczy wypróbuj najpierw z wyłączonym wrzecionem i frezem w powietrzu.

* **Zmieniacz narzędzi** (`T<n> M6` bez pauzy): tylko dla sterowników znających M6 (grblHAL, FluidNC); zwykły GRBL odpowiada błędem. Frez bez numeru w zmieniaczu zatrzymuje maszynę jak dawniej.
* **Chłodziwo i odkurzacz** na warstwie (M7 / M8 / M9 i własne linie urządzenia): włączane z warstwą, wyłączane przed zmianą freza i na końcu. Nie uruchamiane w symulatorze.
* **Korekty, zapisane pozycje, Find the centre, przyciski chłodziwa:** polecenia czasu rzeczywistego i ruchy jog we współrzędnych maszyny (`G53`). Zapisana pozycja jedzie najpierw w górę, jeśli jest wyżej niż narzędzie, potem w bok, potem w dół: sprawdź, czy nic nie stoi na drodze. Szukanie środka jest dla otworów i porusza sondą w X i Y; zacznij, gdy sonda wisi w otworze.
* **Duet / RepRapFirmware:** płytka wykonuje wgrany program sama; **stąd da się ją zatrzymać tylko przyciskiem Stop** (`M0` przez sieć), nie ma korekt ani jogowania. Trzymaj rękę na własnym grzybku awaryjnym maszyny. Smoothieware jest strumieniowany jak GRBL i nie był próbowany na prawdziwej płytce.
* **Postprocesory Mach3 / Mach4 / UCCNC i LinuxCNC** (wtyczka G-code dialects) powstały według instrukcji: przeczytaj plik, zanim go uruchomisz.
* **Modele 3D jako relief** są oglądane z góry: podcięcia przepadają, najwyższy punkt zostaje nietknięty, a to, co jest wokół modelu, jest wybierane do jego najniższego punktu.

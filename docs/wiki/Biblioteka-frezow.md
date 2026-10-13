🌐 [English](Bit-Library) · **Polski**

# Biblioteka frezów

**Milling → Bit library…** (albo przycisk **…** obok freza warstwy) otwiera bibliotekę. Po lewej: **gotowe frezy** programu i **My bits** (twoje). Po prawej: rysunek wybranego freza, jego wymiary, startowe prędkości i notatki.

![Biblioteka frezów, frez V](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/11-bits-vbit.png)

## Rysunek
Każdy rodzaj freza jest narysowany w przekroju z **literą każdego wymiaru tam, gdzie jest mierzony**: D w poprzek najszerszej części skrawającej, L wzdłuż części skrawającej, O wzdłuż całego freza, S w poprzek chwytu, R przy narożu, A jako kąt boków i T w poprzek końcówki. Te same litery stoją przy polach do wpisywania. Długi chwyt jest rysowany krótko, z przerwą.

| ![Prosty](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/10-bits-flat.png) | ![Z zaokrąglonym narożem](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/12-bits-bull.png) |
|---|---|
| Frez prosty | Frez z zaokrąglonym narożem |
| ![Stożkowy](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/13-bits-tapered.png) | ![Do grawerowania](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/14-bits-engraver.png) |
| Frez stożkowy kulisty | Frez do grawerowania |

## Rodzaje i wymiary
| Rodzaj | Wymiary |
|---|---|
| Frez prosty | D średnica skrawająca, L długość skrawająca, S średnica chwytu, O długość całkowita |
| Frez kulisty | D, L, S, O (promień to D/2) |
| Frez z zaokrąglonym narożem | D, R promień naroża, L, S, O |
| Frez V | D, A kąt, L, S, O |
| Frez do grawerowania | D, T średnica końcówki, A kąt, L, S, O |
| Frez stożkowy kulisty | D, T średnica końcówki (kulki), A kąt stożka, L, S, O |
| Wiertło | D, A kąt ostrza, L, S, O |

## Twoje frezy
* **New bit** tworzy frez ze zwykłymi wartościami; **Copy** robi kopię wybranego freza (także gotowego) wśród twoich; **Delete** usuwa jeden z twoich. Gotowych frezów nie można zmieniać ani usuwać; skopiuj, żeby zmienić.
* Zmiany zapisują się, gdy tylko liczby są poprawne (długość skrawająca nie może być większa niż frez, promień naroża nie większy niż połowa średnicy, końcówka nie szersza niż frez). Błędna liczba jest wyjaśniona pod frezem i nie jest zapisywana.
* **My bits są zapisywane raz, dla wszystkich urządzeń**, razem z ustawieniami programu.
* **Export…** zapisuje wszystkie twoje frezy do pliku JSON; **Export this bit…** zapisuje wybrany (także gotowy). **Import…** wczytuje taki plik: każdy frez jest sprawdzany, dostaje własny identyfikator i nazwę, a frez z błędną liczbą jest pomijany i wymieniony. Nowszy format lub plik ponad 4 MB są odrzucane.
* Projekt niesie frezy, których używają jego warstwy (zobacz [Plik projektu](Plik-projektu)). Otwarty tam, gdzie ich brakuje, dodaje je do **My bits**, a pasek stanu o tym mówi.

## Prędkości startowe
Każdy frez ma posuw, posuw wejścia, obroty, głębokość przejścia (procent średnicy), krok boczny (procent) i liczbę ostrzy. Warstwa bierze je po wyborze freza. Wartości gotowych frezów to ostrożne punkty wyjścia dla miękkiego drewna i plastiku: **dopasuj je do swojej maszyny, materiału i freza**.

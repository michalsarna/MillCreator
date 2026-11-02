🌐 [English](Quick-Start) · **Polski**

# Szybki start

Pierwsze zadanie od początku: urządzenie, materiał, kształt, warstwa z frezem, obejrzenie zadania w 3D i jako G-code oraz plik dla maszyny. Napisy **pogrubione** to napisy programu (w nawiasie angielskie, gdy to pomaga); Ctrl to Cmd na macOS.

## 1. Urządzenie

Ekran startowy wypisuje twoje maszyny pod **Urządzenie** (Device). Wybierz tę, na której będziesz pracować.

* **Dodaj urządzenie…** (Add device…) tworzy nowe: nazwa, jednostki, zero maszyny, obszar roboczy, bezpieczna wysokość, wrzeciono… (wszystkie strony są w [Urządzeniach i kamerach](Urzadzenia-i-kamery)). Nowe urządzenie może zacząć od **Start od → Zwykły rozmiar maszyny…** (Start from → A usual machine size…).
* **Edytuj…**, **Powiel** i **Importuj…** zmieniają, kopiują albo wczytują urządzenie z pliku.
* Bez maszyny naciśnij **Nie mam urządzenia** (I have no device): powstaje i zostaje wybrany zastępczy *Dummy* (obszar roboczy 300 × 200 mm i 60 mm w Z, bezpieczna wysokość 5 mm, wrzeciono 1000–24000 obr./min). **Nowy** albo **Otwórz…** bez żadnego urządzenia też go tworzą i używają.

Potem naciśnij **Nowy** (New; albo **Otwórz…** dla pliku, który masz; **Otwórz ostatnie** wypisuje ostatnie pliki).

## 2. Ustawienia projektu

Po **Nowy** okno **Ustawienia projektu** (Project setup) pyta o kawałek materiału (otwiera się też z **Plik → Ustawienia projektu…**):

| Pole | Co znaczy |
|---|---|
| **Szerokość materiału**, **Wysokość materiału (Y)** | Rozmiar kawałka (na początku obszar roboczy, najwyżej 300 × 200 mm). |
| **Grubość** (Thickness) | Grubość kawałka (od 0,1 do 500 mm). |
| **Zero programu** (Zero of the program) | Gdzie na kawałku jest X 0, Y 0: **Przód po lewej**, **Przód po prawej**, **Tył po lewej**, **Tył po prawej** albo **Środek**. |
| **Z liczone od spodu materiału** | Wyłączone: Z = 0 to wierzch materiału. Włączone: każde Z to wysokość nad jego spodem. |
| **Obszar roboczy to materiał** | Robi obszar roboczy urządzenia tak duży jak kawałek. |

Materiał leży w lewym górnym rogu strony. **Zastosuj** (Apply) pisze program dla układu współrzędnych roboczych **G54** (albo tego, który wybrano w urządzeniu) z zerem w wybranym miejscu: ustaw to zero na maszynie w tym miejscu kawałka (dojedź tam i naciśnij **X0**, **Y0** i **Z0** albo **Zeruj wszystko** na karcie **Sterowanie**, albo użyj sondy). **Pomiń** (Skip) zostawia zero urządzenia. Wybory są zapisywane z projektem.

## 3. Materiał

Karta **Materiał**: **Materiał obrabianego przedmiotu** (**Drewno, jasne**, **Drewno, średnie**, **Drewno, ciemne**, **Metal**, **Tworzywa** w wybranym **Kolorze**) i **Grubość**. Kolor służy tylko widokowi 3D; posuwy pochodzą z frezów i warstw. **Pierwsze przejście** może zaczynać każdą warstwę płytkim przejściem wzdłuż krawędzi. Zob. [Materiał](Material-obrabiany).

## 4. Kształt

* Rysuj narzędziami po lewej: **Prostokąt (R)**, **Elipsa (E)**, **Wielokąt (G)**, **Tekst (T)**, **Linia łamana / pióro (P)**… (litera to klawisz; te same narzędzia są w menu **Wstawianie**). Nowy kształt trafia na aktywną warstwę.
* Albo wczytaj: **Plik → Otwórz…** (SVG, PDF, Illustrator, DXF), **Plik → Importuj ▸** (obraz, DXF, projekt LightBurn, projekt UGS, model 3D jako relief), karta **Clipart** albo upuszczenie pliku na okno.
* Karta **Właściwości** (Properties) ustawia położenie i rozmiar zaznaczenia. Zob. [Narzędzia projektowe](Narzedzia-projektowe).

## 5. Warstwy: operacja, frez, głębokości, posuwy

Kształt jest frezowany tak, jak mówi jego warstwa. Otwórz kartę **Warstwy**. Pierwsze warstwy nowego projektu są gotowe (ich nazwy są danymi projektu, więc się nie tłumaczą):

| Warstwa | Operacja | Głębokość | Frez |
|---|---|---|---|
| *Cut out* | **Wycinanie na zewnątrz linii** (część zostaje) | 10 mm | Flat 3.175 mm (1/8") |
| *Holes and windows* | **Wycinanie wewnątrz linii** (środek wypada) | 10 mm | Flat 3.175 mm (1/8") |
| *Pocket* | **Kieszeń** (obszar jest wybierany) | 3 mm | Flat 3.175 mm (1/8") |
| *Engrave* | **Podążanie za linią** | 1 mm | V-bit 60° |
| *Drill* | **Wiertło** (Drill: otwór w środku każdego zamkniętego kształtu) | 10 mm | Drill 3 mm |

1. **Połóż kształt na warstwie.** Zaznacz go, kliknij warstwę na liście i naciśnij **Umieść zaznaczone kształty na tej warstwie**, albo kliknij kolor warstwy w rzędzie kolorów pod obszarem roboczym (dwuklik tam otwiera opcje warstwy). Lista pokazuje warstwy, na których są kształty, i aktywną; **Pokaż wszystkie 64 warstw** pokazuje pozostałe.
2. **Operacja:** **Operacja** ma ich jedenaście (profile, kieszeń, wiercenie, planowanie, rozwiercanie, fazowanie, rzeźbienie V, rowek z narożnikami dogbone, obraz…): [Warstwy i operacje](Warstwy-i-operacje).
3. **Frez:** wybierz go w polu **Frez**. Wybór freza ustawia też **Posuw**, **Posuw wcięcia**, **Obroty wrzeciona**, **Głębokość przejścia** i **Rozstaw przejść** z jego prędkości początkowych. Przycisk **…** otwiera **Bibliotekę frezów** ([Biblioteka frezów](Biblioteka-frezow)).
4. **Głębokości:** **Głębokość** to cała głębokość pod wierzchem materiału; **Głębokość przejścia** to głębokość jednego przejścia (0 = cała głębokość w jednym przejściu). Wiersz pod ustawieniami mówi, ile przejść z tego wychodzi.
5. **Posuwy:** **Posuw** i **Posuw wcięcia** w mm/min, **Obroty wrzeciona** w obr./min. Pod nimi, wyliczone ze średnicy i liczby ostrzy freza: **Prędkość skrawania (Vc)** w m/min oraz **Posuw na ząb (fz)** i **Posuw wgłębny na ząb** w mm. Wpisanie Vc zmienia obroty wrzeciona, wpisanie fz zmienia posuw; zapisywane są tylko obroty i posuwy.
6. **Mostki** (Tabs) trzymają wycinaną część na miejscu: ustaw je na warstwie z *Wycinaniem na zewnątrz linii* albo rozmieść ręcznie.

Ptaszek na liście bierze warstwę do ścieżek i G-code; oko ją pokazuje albo ukrywa. Dobre ustawienia można zachować przyciskiem **Zapisz preset…** i położyć na innej warstwie przyciskiem **Wczytaj preset**.

## 6. Obejrzyj zadanie

* **Ścieżki** (Toolpaths, zielony przycisk nad obszarem roboczym) rysują drogi freza na rysunku.
* **Podgląd…** albo **Frezowanie → Widok 3D zadania…** (Ctrl+Shift+P w Windows i Linuksie) otwiera kartę **Podgląd**: blok materiału z wyrzeźbionymi cięciami, ruchy na bezpiecznej wysokości, animację z suwakiem, czas i długość cięć oraz ostrzeżenia (warstwa bez freza, głębiej niż materiał, poza obszarem roboczym…). Zob. [Widok 3D i G-code](Widok-3D-i-G-code).
* **Podgląd G-code** pokazuje sam program w karcie (*Podgląd G-code: nazwa.nc*), pokolorowany, bez zapisywania czegokolwiek. Tekst można edytować; **Zapisz G-code** w karcie go zapisuje, **Wygeneruj ponownie** robi go od nowa z projektu, **Analizuj** pokazuje jego ścieżki.

## 7. Zapisz projekt i G-code

* **Plik → Zapisz** (Ctrl+S) zachowuje projekt jako plik `.cre`.
* **Zapisz G-code** (przycisk nad obszarem roboczym), **Frezowanie → Eksportuj G-code…** albo **Ctrl+E** (Windows i Linux; o macOS zob. [Skróty klawiszowe](Skroty-klawiszowe#macos)) pyta o plik (`nazwa.nc`) i zapisuje G-code, przez włączone postprocesory wtyczek. Z **Ustawienia → Opcje widoku… → Podgląd G-code po zapisaniu** zapisany plik otwiera się w analizie G-code.
* Tekst karty **Opis** (Description) trafia na początek G-code jako komentarze `(Note: …)`.

## 8. Na maszynie

Program może też wysłać zadanie do maszyny z GRBL (sprawdzone na TwoTrees TTC450Pro, nie na innych maszynach):

1. Ustaw port w **Maszyna → Ustawienia urządzenia… → Połączenie**, potem **Połącz** na karcie **Urządzenie**.
2. **Bazuj** (Home), potem dojedź do zera programu i ustaw je na karcie **Sterowanie** (**Zeruj X i Y**, **Z0** albo **Sonduj Z**).
3. **Obrys** (Frame) objeżdża prostokąt zadania na bezpiecznej wysokości z wyłączonym wrzecionem: sprawdź, czy zadanie mieści się na materiale.
4. **Start** wysyła zadanie. **STOP** je zatrzymuje; **Pauza** i **Wznów** wstrzymują i wznawiają. **Od wiersza…** zaczyna od wiersza programu.

Zob. [Sterowanie maszyną](Sterowanie-maszyna).

> ⚠️ Nowe zadanie najpierw wypróbuj w powietrzu nad materiałem, noś ochronę oczu i uszu, zamocuj materiał i zostań przy maszynie. Zob. [Bezpieczeństwo i ograniczenia](Bezpieczenstwo-i-ograniczenia).

🌐 [English](Machine-Control) · **Polski**

# Sterowanie maszyną

MillCreator steruje maszyną z GRBL z karty **Sterowanie** i z menu **Machine**. Wszystko to pochodzi z baseCreatora, więc MillCreator, PenCreator i LightCreator działają tak samo. **Nic z tego nie było jeszcze próbowane na prawdziwej maszynie**: próbuj z podniesionym narzędziem albo bez pisaka i miej rękę przy zatrzymaniu.

## Odczyt maszyny

Połącz się na karcie **Device**. Program odczytuje kontroler (`$I` – firmware, `$$` – ustawienia, `$G` – stan). W ustawieniach urządzenia, na stronie **Machine**, przycisk **Użyj tych wartości dla urządzenia** kopiuje zakres ruchu, zakres obrotów wrzeciona i prędkość ruchów z kontrolera do urządzenia, więc nie musisz ich wpisywać. Nowe urządzenie może też zacząć od **Zwykły rozmiar maszyny…** (tylko punkt wyjścia).

## Karta Sterowanie

* **Pozycja:** pozycja robocza i maszynowa X, Y i Z. **X0 Y0 Z0** robią z miejsca, gdzie jest narzędzie, zero pracy (`G10 L20`); kliknij liczbę, aby wpisać pozycję. **Współrzędne** wybierają G54 … G59.
* **Przesuw:** przyciski kroków (ich wielkości ustawisz w urządzeniu, na stronie **Jogging**), strzałki i **Z▲ Z▼**; kwadrat pośrodku zatrzymuje ruch. Z opcją **Przytrzymaj, aby jechać** narzędzie jedzie, dopóki przycisk jest wciśnięty, i staje po puszczeniu. **Klawiatura** (domyślnie wyłączona, bo strzałki przesuwają też kształty) używa strzałek i Page Up / Page Down.
* **Jedź do X0 Y0** najpierw podnosi na bezpieczną wysokość; **Z na bezpieczną wysokość**.

## Sondowanie (MillCreator)

W ustawieniach urządzenia, na stronie **Probe**, określ, czy masz **płytkę dotykową** (jej grubość i grubość jej boku), czy **sondę dotykową 3D** (przesunięcie Z przy dotknięciu i średnicę kulki), a także odległość błędu, dwie prędkości i odjazd. Na karcie Sterowanie:

* **Probe Z** – narzędzie jedzie w dół do dotknięcia (szybko, cofa się, potem powoli), a Z jest ustawiane na grubość płytki. Najpierw połóż płytkę pod narzędziem.
* **X+ X− Y+ Y−** – narzędzie jedzie ku krawędzi do dotknięcia; krawędź staje się zerem, z uwzględnieniem średnicy narzędzia (wpisz ją w polu **Tool diameter**) i grubości boku płytki. Najpierw ustaw narzędzie przy krawędzi.
* Gdy wejście sondy jest już zwarte, nic nie jest sondowane: narzędzie zostałoby dociśnięte do płytki. Sprawdź okablowanie, dotykając płytki: wejście pokazuje się jako *zwarte*.
* **Makra:** przyciski wysyłające napisany przez Ciebie G-code (strona **Macros** ustawień urządzenia). Zmienne w nawiasach kwadratowych są podmieniane przy wysyłaniu: `[posx] [posy] [posz]` (pozycja robocza), `[mposx]` …, `[xmin] [xmax] [ymin] [ymax]` (obrys zadania), `[safez]`, `[prbx] [prby] [prbz]` (ostatnie sondowanie). `$RST` jest w makrze zabronione. Ta sama strona ma G-code uruchamiany **przed** i **po** każdym zadaniu (także po zatrzymaniu).

## Uruchamianie zadania

* **Start** wysyła całe zadanie; **Od wiersza…** (też w menu *Machine*) zaczyna od wybranego wiersza programu: maszyna jest najpierw doprowadzana do stanu, jaki program ma w tym miejscu (jednostki, układ współrzędnych, wrzeciono i jego obroty, posuw, pozycja), na bezpiecznej wysokości, a reszta idzie dalej. Odmawia, gdy nie umie tego obliczyć (np. po `G53` lub `G92`).
* **Sprawdź program** (menu *Machine* albo *Check the program before every run* na stronie **Jogging**): kontroler przechodzi przez program w trybie sprawdzania (`$C`). Nic się nie rusza, a wiersze, których nie rozumie, są zgłaszane z numerem. Przy włączonej opcji właściwe zadanie rusza dopiero, gdy sprawdzanie nic nie znalazło.
* **Pauzy:** Gdy program dojdzie do zmiany frezu, maszyna zatrzymuje się tam (poprzedni wiersz jest już wykonany), a okno mówi, który frez założyć. **Probe Z** ustawia Z na nowo płytką; **Kontynuuj zadanie** idzie dalej, **Zatrzymaj zadanie** je kończy. Gdy zadanie czeka, możesz przesuwać maszynę, zerować i sondować na karcie Sterowanie.
* Błędy i alarmy w trakcie zadania są liczone, a pierwszy wiersz każdego jest zapamiętywany.

## Menu Machine

* **Firmware settings ($$):** ustawienia kontrolera z wyszukiwaniem i filtrem pokazującym tylko te, które różnią się od typowej wartości. Zmiana jest zapisywana po pytaniu, a ustawienia są potem czytane ponownie. **Pierwsza zmiana w sesji zapisuje wszystkie ustawienia do pliku** w folderze danych (`firmware-backups`). **Back up to a file…** i **Restore from a file…** robią to ręcznie (plik jest porównywany z kontrolerem przed zapisem i czytany z powrotem po nim); **Factory settings ($RST=$)…** wymaga wpisania słowa RESET.
* **Calibration:** *kroki na milimetr* (zleć ruch, zmierz go, dostaniesz nowe `$100` / `$101` / `$102` i zapiszesz) oraz *prostopadłość osi* (wpisz dwie przekątne prostokąta: pokazany jest kąt i przesunięcie).
* **Diagnostics and statistics:** wejścia kontrolera (wyłączniki krańcowe, sonda, osłona, hold, reset, start), firmware, zadania, czas pracy, alarmy i błędy tej maszyny oraz **przypomnienia o konserwacji** (np. „nasmaruj prowadnice co 50 godzin”; ustawisz je na stronie **Maintenance**). **Export diagnostics (zip)…** zapisuje raport, ustawienia, konsolę i program.

## Czujnik długości narzędzia (MillCreator)

Czujnik, którego dotyka końcówka każdego narzędzia, pozwala nowemu frezowi dostać dokładnie to samo Z co poprzedni, bez ponownego dotykania materiału. Prowadzi przez to menu **Machine → Tool length sensor…**:

1. **Gdzie jest czujnik:** zrób bazowanie maszyny, ustaw narzędzie nad środkiem czujnika kilka milimetrów nad nim i naciśnij **Czujnik jest tutaj**. Położenie jest zapamiętane we współrzędnych *maszyny*, więc nigdy nie przesuwaj czujnika. To samo okno ma wysokość przejazdu, odległość błędu, odjazd i dwie prędkości.
2. **Pierwsze narzędzie, wzorcowe:** włóż pierwszy frez i ustaw nim Z na materiale (sondowanie Z albo dotknięcie powierzchni i Z0). **Zmierz narzędzie wzorcowe** zawozi narzędzie do czujnika i dotyka go dwa razy (szybko, odjazd, powoli); Z w chwili dotknięcia jest zapamiętane przy urządzeniu.
3. **Każde następne narzędzie:** gdy zadanie zatrzymuje się na zmianę frezu, okno ma **Zmierz narzędzie (czujnik)**: po dotknięciu Z jest ustawiane tak, żeby to miejsce miało Z narzędzia wzorcowego, i narzędzie jedzie w górę. To samo zrobisz ręcznie w kreatorze (**Zmierz to narzędzie i ustaw Z**), a **Tylko zmierz** pokazuje, o ile narzędzie jest dłuższe lub krótsze, niczego nie zmieniając.

Wejście sondy kontrolera musi być podłączone do czujnika. Pomiar jest odrzucany, gdy to wejście jest już zwarte, gdy maszyna nie stoi bezczynnie albo zanim kontroler poda pozycję. **Nie próbowane na prawdziwej maszynie:** za pierwszym razem miej rękę przy zatrzymaniu.

## Kopie ustawień kontrolera

Każde urządzenie przechowuje kopie ustawień swojego kontrolera (`$$`). Pierwsza powstaje przy pierwszym połączeniu z maszyną, a kolejne za każdym razem, gdy ustawienia odczytane przy łączeniu różnią się od najnowszej kopii: pasek stanu i konsola mówią, ile ustawień się zmieniło i które (`$110: 3000 → 5000`), więc zauważysz zmianę zrobioną z innego programu albo przez sam kontroler. Zmiana zrobiona tutaj (zapis w **Firmware settings**) dostaje własną kopię bez alarmu. **Zrób kopię teraz** w oknie ustawień firmware i to samo w ustawieniach urządzenia, na stronie **Machine**, robi kopię ręcznie; **Przywróć…** porównuje kopię z kontrolerem i po pytaniu zapisuje różnice z powrotem; **Usuń** kasuje kopię. Przechowywanych jest osiem najnowszych. Kopie są częścią urządzenia, więc są w **kopii zapasowej programu** i w wyeksportowanym pliku urządzenia.

## Alarmy i błędy

Gdy GRBL powie `ALARM:n` albo `error:n`, konsola dostaje pod spodem wiersz z wyjaśnieniem, a pasek stanu je pokazuje. Wyjaśnienia są w 30 językach programu.

> ⚠️ Sondowanie, przesuw i pauzy wysyłają prawdziwe ruchy. Miej zatrzymanie w zasięgu ręki.

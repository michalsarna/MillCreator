🌐 [English](Machine-Control) · **Polski**

# Sterowanie maszyną

MillCreator steruje maszyną z GRBL z karty **Sterowanie** i z menu **Machine**. Wszystko to pochodzi z baseCreatora, więc MillCreator, PenCreator i LightCreator działają tak samo. **Sprawdzane tylko na TwoTrees TTC450Pro**: próbuj z podniesionym narzędziem albo bez pisaka i miej rękę przy zatrzymaniu.

## Odczyt maszyny

Połącz się na karcie **Device**. Program odczytuje kontroler (`$I` – firmware, `$$` – ustawienia, `$G` – stan). W ustawieniach urządzenia, na stronie **Machine**, przycisk **Użyj tych wartości dla urządzenia** kopiuje zakres ruchu, zakres obrotów wrzeciona i prędkość ruchów z kontrolera do urządzenia, więc nie musisz ich wpisywać. Nowe urządzenie może też zacząć od **Zwykły rozmiar maszyny…** (tylko punkt wyjścia).

## Karta Sterowanie

* **Pozycja:** pozycja robocza i maszynowa X, Y i Z, dużymi cyframi. **X0 Y0 Z0** (po lewej każdej osi) robią z miejsca, gdzie jest narzędzie, zero pracy (`G10 L20`); kliknij liczbę, aby wpisać pozycję. Pod nimi: **Zero all**, **Zero X and Y**, **Jedź do X0 Y0** (najpierw podnosi na bezpieczną wysokość) i **Z na bezpieczną wysokość**. **Współrzędne** wybierają G54 … G59.
* **Przesuw:** odległość jednego kliknięcia dla XY i dla Z to pole, w którym można pisać; przyciski obok wstawiają tam wartość (XY 0,1 … 100, Z 0,1 … 20; wartości ustawia się w urządzeniu, strona **Jogging**). **Prędkość** to prędkości XY i Z urządzenia (zmiana tutaj trwa do zmiany w urządzeniu).
* **Przyciski ruchu** (duże, pośrodku): strzałki, **Z▲ Z▼**; kwadrat zatrzymuje ruch. Jedno kliknięcie to jeden ruch. Z opcją **Przytrzymaj, aby jechać** narzędzie jedzie, dopóki przycisk jest wciśnięty, i staje po puszczeniu. **Klawiatura** (domyślnie wyłączona, bo strzałki przesuwają też kształty) używa strzałek i Page Up / Page Down o wybrany krok.

* **Opcje zadania** (pod przyciskami Frame i From line…; tylko w tej sesji, nie są zapisywane; można je włączyć razem albo osobno, zmieniają G-code, podgląd i oszacowanie): **One element at a time** (Element po elemencie) wycina każdy element, czyli grupę (także obejmującą kilka warstw) albo kształt bez grupy, w całości, ze wszystkimi jego warstwami i przejściami, zanim zacznie następny; pierwszy jest element najbliższy zera maszyny (wyłączone: warstwa po warstwie). Przeznaczone dla warstw z tym samym frezem: przy różnych frezach przy każdej zmianie następuje wymiana (pauza). **Selection only** (Tylko zaznaczenie) wycina tylko kształty zaznaczone na obszarze roboczym w chwili zapisu G-code lub startu zadania; bez zaznaczenia nie ma czego wycinać.
* **Podgląd jako karta:** ikona z kartą w linii tytułu okna podglądu (obok przycisku zamknięcia) przypina podgląd jako kartę obszaru roboczego, obok rysunków i widoku z kamery. W karcie ikona z małym oknem zamienia ją z powrotem w okno, a krzyżyk karty zamyka podgląd. Przycisk *Preview…* otwiera go znów tak, jak był ostatnio (okno albo karta).
* **Eksport widoku 3D jako STL:** ikona *download* w linii tytułu podglądu (i nad kartą) zapisuje sfrezowany element jako binarny model STL: materiał z rowkami po frezach, w stanie, do którego doszedł suwak (na końcu: całe zadanie), w milimetrach, z lewym dolnym rogiem widoku w 0, 0 i górą materiału na wysokości grubości. Frez jest przesuwany wzdłuż cięć z prawdziwym kształtem (walcowy, kulisty, V…), więc powierzchnia jest tak dokładna, jak pozwala najmniejszy frez (przy dużym zadaniu siatka jest rzadsza). Milling → *Export relief as STL…* to inny eksport: sam relief obrazu.
* **Wrzeciono** (MillCreator): prędkość i **Włącz** / **Wyłącz** (`M3 S…` i `M5`), żeby wypróbować wrzeciono. Wartość S jest rozkładana na zakres sterownika (`$30`), gdy nie jest to maksymalna prędkość urządzenia.

## Sondowanie (MillCreator)

W ustawieniach urządzenia, na stronie **Probe**, określ, czy masz **płytkę dotykową** (jej grubość i grubość jej boku), czy **sondę dotykową 3D** (przesunięcie Z przy dotknięciu i średnicę kulki), a także odległość błędu, dwie prędkości i odjazd. Na karcie Sterowanie:

* **Probe Z** – narzędzie jedzie w dół do dotknięcia (szybko, cofa się, potem powoli), a Z jest ustawiane na grubość płytki. Najpierw połóż płytkę pod narzędziem.
* Gdy wejście sondy jest już zwarte, nic nie jest sondowane: narzędzie zostałoby dociśnięte do płytki. Sprawdź okablowanie, dotykając płytki: wejście pokazuje się jako *zwarte*.
* **Makra:** przyciski wysyłające napisany przez Ciebie G-code (strona **Macros** ustawień urządzenia). Zmienne w nawiasach kwadratowych są podmieniane przy wysyłaniu: `[posx] [posy] [posz]` (pozycja robocza), `[mposx]` …, `[xmin] [xmax] [ymin] [ymax]` (obrys zadania), `[safez]`, `[prbx] [prby] [prbz]` (ostatnie sondowanie). `$RST` jest w makrze zabronione. Ta sama strona ma G-code uruchamiany **przed** i **po** każdym zadaniu (także po zatrzymaniu).

### Korekty, chłodziwo i zapisane pozycje

* **Overrides** (GRBL 1.1 i nowszy, widoczne, gdy kontroler je raportuje): podczas zadania zmieniasz **posuw** i **obroty wrzeciona** co 10 % i 1 %, wracasz do 100 % albo zwalniasz **ruchy szybkie** do 50 % lub 25 %. Widać bieżące wartości. To realtime override GRBL-a: działa od razu i nie zmienia programu.
* **Chłodziwo:** *Mist* (M7), *Flood* (M8) i *Off* (M9). Warstwa może sama prosić o chłodziwo (patrz Warstwy i operacje). W maszynach odkurzacz bywa podłączony do M8.
* **Zapisane pozycje** (GRBL 1.1 i nowszy): *Save here* zapamiętuje, gdzie jest narzędzie (pozycja maszynowa, z nazwą). Przycisk z nazwą wraca tam: najpierw w górę, jeśli miejsce jest wyżej niż narzędzie, potem w bok, potem w dół. Lista leży w `saved_positions.json` we wspólnym folderze danych, więc każdy program ma te same miejsca.
* **Find the centre** (w sekcji Probe): sonda wisi w otworze, mniej więcej pośrodku i na głębokości, jaką chcesz. Dotyka lewej i prawej strony, jedzie na środek między nimi, dotyka przodu i tyłu, jedzie na środek i ustawia go jako zero X i Y używanego układu współrzędnych; w pasku stanu jest rozmiar otworu. Nie robi Z: najpierw sondą ustaw Z. Działa dla otworów; dla występu obejdź go sondą ręcznie i sondując krawędzie. Nieudany dotyk albo alarm przerywa działanie.
* **Zmieniacz narzędzi:** maszynę, która sama zmienia frez (grblHAL, FluidNC: znają M6; zwykły GRBL nie), włączasz w ustawieniach urządzenia (*Spindle*: *Tool changer*). Każdy frez potrzebuje wtedy **numeru w zmieniaczu** w bibliotece, a program prosi o niego przez `T<numer> M6` zamiast zatrzymywać się pauzą `M0`; na czas zmiany chłodziwo i odkurzacz są wyłączane.
* **Frame round the outline:** obok przycisku Frame: ramka obiega obrys cięć (gumka, otoczka wypukła) zamiast ich prostokąta.

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

**Wersje GRBL.** Wersję płyty wybierasz w polu *GRBL version* na stronie Połączenie urządzenia: **1.1 lub nowszy** (domyślnie; także grblHAL i FluidNC), **1.0**, **0.9**, **0.8** albo **0.7 lub starszy**. GRBL 0.8, 0.9 i 1.0 podają położenie jako `<Idle,MPos:…,WPos:…>` i nie mają polecenia `$J=`, więc program przesuwa głowicę zwykłymi ruchami `G91` / `G1`; takiego ruchu nie da się przerwać w połowie, kończy się na końcu swojego kroku. GRBL 0.7 nie ma żadnych poleceń czasu rzeczywistego: nie ma raportu stanu (więc i położenia), Pauzy, Wznowienia, bazowania ani odblokowania, a Stop tylko porzuca wiersze, które jeszcze nie poszły, bo płyta dokończy to, co już ma; ten firmware ucina też wiersze dłuższe niż 49 znaków. Wrzeciono (`M3 S…`) działa tak samo w każdej wersji.

Gdy GRBL powie `ALARM:n` albo `error:n`, konsola dostaje pod spodem wiersz z wyjaśnieniem, a pasek stanu je pokazuje. Wyjaśnienia są w 30 językach programu.

> ⚠️ Sondowanie, przesuw i pauzy wysyłają prawdziwe ruchy. Miej zatrzymanie w zasięgu ręki.

## Gdy maszyna się zatrzyma

Alarm, błąd w zadaniu albo sterownik, który nie odpowiada, otwiera **okno pośrodku ekranu**: co się stało oraz dwa duże przyciski, **STOP** (czerwony: zatrzymuje zadanie i wrzeciono) i **UNLOCK** (pomarańczowy: kasuje alarm, `$X`). Zobacz, dlaczego się zatrzymało, zanim odblokujesz: narzędzie może tkwić w materiale.

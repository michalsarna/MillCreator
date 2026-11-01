🌐 [English](Devices-and-Cameras) · **Polski**

# Urządzenia i kamery

**Urządzenie** to frezarka. Lista urządzeń jest zapamiętywana między uruchomieniami i jest wspólna dla wszystkich projektów. Ekran startowy wybiera urządzenie; **New** i **Open…** go używają. Karta Device w edytorze pokazuje używane urządzenie i otwiera jego ustawienia.

![Dodawanie urządzenia](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/02-add-device.png)

## Przyciski ekranu startowego
**Add device…**, **Edit…**, **Duplicate** i **Import…** (ściśle sprawdzany: nowszy format, błędna liczba lub plik ponad 2 MB są odrzucane) w pierwszym rzędzie oraz **I have no device** w drugim. **Export…** (jedno urządzenie jako plik JSON) i **Delete** (prosi o potwierdzenie) są w oknie **Edit…**. Kopia lub importowane urządzenie o zajętej nazwie dostaje numer: `Router (2)`.

## Nie mam urządzenia
Tworzy jednorazowo zwykłe urządzenie o nazwie **Dummy**: pole robocze 300 × 200 mm, 60 mm skoku w Z, bezpieczna wysokość 5 mm, wrzeciono 1000–24000 obr/min. Zostaje na liście jak każde inne i można je edytować; projekt zapisuje się z nim jako z maszyną.

## Ustawienia urządzenia
| Strona | Co zawiera |
|---|---|
| General | nazwa, jednostki (mm lub cale; geometria jest zawsze zapisana w mm), zero maszyny (przód lewo, przód prawo, tył lewo, tył prawo), **współrzędne robocze** (brak albo G54 … G59) i **zero programu** |
| Machine | pole robocze X i Y, skok w Z, **bezpieczna wysokość** (na której wrzeciono porusza się między cięciami) i prędkość ruchów nad materiałem |
| Spindle | najniższe i najwyższe obroty oraz czekanie po włączeniu |
| G-code | **własny G-code** na początku programu, przy każdej zmianie freza, przy każdej zmianie operacji (nowej warstwy) i na końcu programu (patrz niżej) |
| Connection | port szeregowy lub gniazdo sieciowe, port lub identyfikator USB, prędkość, strumieniowanie, co zrobić po połączeniu |
| Rotary | obrotnica: rodzaj, średnice, mikrokroki na obrót |
| Camera | do ośmiu kamer: adres (strumień MJPEG, zdjęcie JPEG, http / https albo strumień RTSP) albo kamera komputera, obrót 0 / 90 / 180 / 270°, włączona / wyłączona |
| Notes | dowolny tekst |

Sterownik to GRBL (od 0.7 do 1.1 i nowszego), albo **Smoothieware** (pozycja listy *GRBL version*), albo płytka **Duet / RepRapFirmware** przez HTTP (pole wyboru na stronie Connection z adresem i hasłem, które nie jest zapisywane; patrz Sterowanie maszyną). Strona **Spindle** ma też **Tool changer** (`T<n> M6`), a strona **G-code** linie dla odkurzacza. Strona **Connection** ustawia, jak się z nim połączyć (port szeregowy albo gniazdo sieciowe, prędkość, strumieniowanie z liczeniem bufora lub proste, bazowanie albo kasowanie alarmu po połączeniu), a strona **Rotary** obrotnicę (uchwyt lub rolki, średnica przedmiotu, mikrokroki na obrót; oś Y obraca przedmiot). Karta **Device** edytora ma Connect, Home, Unlock, Pause, Resume, stan i położenie maszyny oraz konsolę.

## Kamera
Gdy urządzenie ma włączoną (i poprawną) kamerę, edytor dostaje kartę **Camera** z jej obrazem na żywo. Obrazy z adresu czyta czytnik strumienia z baseCreatora; kamery komputera wymagają kompilacji z `--features camera`. Obrót i korekty obrazu kamery są stosowane.

## W projekcie
Projekt zapisuje migawkę urządzenia, dla którego powstał (nazwa i trzy rozmiary pola roboczego). Gdy otwiera się go dla urządzenia o innym polu roboczym, pasek stanu o tym mówi.

**Własny G-code (strona G-code).** Cztery pola, jedno polecenie w wierszu (najwyżej 20 wierszy po 100 znaków; znaki sterujące są odrzucane): *Na początku programu* (po nagłówku i słowie współrzędnych roboczych), *Przy każdej zmianie freza* (także przy pierwszym frezie; po pauzie na wymianę, przed włączeniem wrzeciona, np. `T1 M6` albo polecenie chłodzenia), *Przy każdej zmianie operacji* (nowa warstwa; po jej komentarzu, przed obsługą freza) i *Na końcu programu* (po powrocie głowicy do zera, przed `M30`). Wiersze trafiają do każdego G-code, który program tworzy: plik, wysyłane zadanie, przebieg próbny. To własne polecenia maszyny, więc program ich nie sprawdza: nową linię wypróbuj najpierw na maszynie bez freza.

**Zmienne, sam początek i koniec pliku oraz format pliku (strona G-code).** W każdym polu własnego G-code słowo w nawiasach klamrowych jest zastępowane przy tworzeniu pliku: `{project_name}`, `{project_path}` (puste, dopóki projekt nie został zapisany), `{date}` i `{time}` (UTC), `{machine_name}`, pudełko wszystkiego, co jest frezowane, we współrzędnych maszyny (`{min_x}` `{max_x}` `{min_y}` `{max_y}` `{min_z}` `{max_z}`) i jego rozmiar (`{width}` `{height}` `{depth}`); przy zmianie freza i operacji także `{layer_name}`, `{layer_index}`, `{total_layers}` oraz zmienne freza warstwy: `{tool_number}` (jego miejsce w zmieniaczu, 0 gdy go nie ma), `{tool_name}`, `{tool_diameter}`, `{tool_flutes}` i prędkości freza z biblioteki `{tool_rpm}`, `{tool_feed}` i `{tool_plunge}`. Lista jest pod polami; nieznana nazwa jest czerwona i plik nie powstaje, dopóki nie zostanie poprawiona. *Very start of the file* jest zapisywany przed komentarzami i nagłówkiem (na `%`, numer programu albo nagłówek sterowania), a *Very end of the file* po `M30`. **Own file format** to ostatnie przejście po gotowym programie, dla sterowań, które czytają G-code po swojemu; wszystko jest domyślnie wyłączone i wtedy plik jest taki jak dotąd: *Decimal point after whole numbers* (`X1` staje się `X1.`, dla X, Y, Z, F, P, I, J, K), *% at the start and end*, *Dwell word* (`G4 P` w sekundach, `G4 P` w milisekundach albo `G4 X` w sekundach; wartość jest przeliczana), *Line numbers* (N10, N20, … z krokiem) i *Remove comments*. Gdy to samo jest włączone w postprocesorze wtyczki *G-code tools*, pasek stanu ostrzega, że robi się to dwa razy.

**Współrzędne robocze (G54 … G59) i zero programu (strona General).** Domyślnie program liczy od zera maszyny (narożnik wybrany jako *Machine zero*), jak dotąd. Gdy wybierzesz **G54 … G59**, program zaczyna się tym słowem (maszyna używa więc układu współrzędnych roboczych, którego zero ustawiłeś na niej, np. dotykając materiału), a każde X i Y jest liczone od **zera programu**: punktu pola roboczego podanego jako X, Y (pole robocze liczone od narożnika zera maszyny). Przy zerze 0, 0 liczby są takie same jak bez kodu, dochodzi tylko słowo. Korzystają z tego obrys (*Frame*), przebieg próbny i zadanie, a także powrót do zera na końcu i własny G-code. Z nie zmienia się: jest liczone od wierzchu materiału.

**Zero programu na obszarze roboczym.** W ustawieniach urządzenia, obok *Zero of the program* (X, Y), przycisk **Pick on the work area** pozwala ustawić zero następnym kliknięciem na obszarze roboczym; przyciąga do narożników i środków kształtów oraz do narożników obszaru roboczego (Esc anuluje). Gdy wybrany jest układ współrzędnych (G54 … G59), na obszarze roboczym w miejscu zera stoi **ikona osi** (X czerwona, Y zielona, z nazwą układu); przeciągnij ją, a pola ustawień od razu się zmieniają. Jedno wskazanie albo jedno przeciągnięcie to jeden krok *Undo* (Ctrl+Z), dopóki po nim nic innego nie zostało zmienione. Zero zapisuje się z urządzeniem, gdy zapiszesz ustawienia.

## Kamera: RTSP, zdjęcia, wideo i poklatkowe

* **Strumienie RTSP** (`rtsp://` lub `rtsps://`, z użytkownikiem i hasłem w adresie, gdy kamera ich wymaga) działają jak adres kamery. Strumień dekoduje **ffmpeg**, uruchamiany jako osobny program: program szuka go obok siebie (pakiety go zawierają), w zasobach aplikacji macOS, a potem w PATH systemu. Bez niego widok kamery o tym mówi. Gdy strumień się urwie, jest otwierany ponownie co kilka sekund. Adres z hasłem widać na liście procesów komputera: użyj konta kamery, które może tylko oglądać.
* **Inne strumienie:** `udp://` i `rtp://` (MPEG-TS lub RTP z kodera, także multicast; `udp://@:5000` nasłuchuje na porcie) oraz `rtmp://` czyta ffmpeg tak samo. Strumień, który przez 20 sekund nic nie daje, jest uznawany za martwy i otwierany ponownie. Adresy z TLS (`rtsps://`, `rtmps://`) nie są obsługiwane przez mały ffmpeg z pakietów.
* **Opcje kamery** (pod adresem strumienia, domyślnie wyłączone): **Użyj UDP dla RTSP** (mniejsze opóźnienie niż TCP, ale obraz może się psuć w słabej sieci) i **Małe opóźnienie** (najnowszy obraz szybciej, bez buforowania; obraz może się zacinać). Zmiana otwiera strumień od nowa.
* **Format wideo** (w widoku kamery): **Automatyczny** (zalecany) kopiuje strumień kamery IP do pliku **MP4** w takiej postaci, w jakiej przychodzi. Nic nie jest dekodowane ani kodowane ponownie, więc procesor ma niemal zero pracy, a jakość jest taka, jaką daje kamera. Kopia to strumień taki, jaki wysyła kamera: obrót i ustawienia obrazu z programu nie są do niej stosowane. Kamera, która daje obraz po obrazie (adres MJPEG, kamera komputera), jest nagrywana jako **AVI** (Motion-JPEG). **MP4** prosi o kopię (kamera, która nie jest strumieniem, jest nagrywana jako AVI, a linia pod przyciskami o tym mówi); **AVI** zawsze zapisuje obrazy Motion-JPEG.
* Pod obrazem widoku kamery: **Take a photo** (JPEG), **Record video** (Motion-JPEG `.avi` odtwarzany zwykłymi odtwarzaczami, z faktyczną liczbą klatek na sekundę) i **poklatkowe**: **Every** N sekund albo **After each layer** (zdjęcie, gdy zaczyna się następna warstwa zadania i gdy zadanie się kończy; wymaga zadania wysłanego z programu na maszynę przyciskiem Start). Zdjęcia i filmy są numerowane (`photo-0001.jpg`, `video-0001.avi`); poklatkowe dostaje własny folder (`timelapse-0001`) z `frame-00001.jpg` albo `layer-00001.jpg`.
* **Choose the folder…** ustawia, gdzie to wszystko trafia (pamiętane dla wszystkich programów; bez wyboru to folder `camera` wspólnego folderu danych). Nagrywanie trwa, gdy widok kamery jest zamknięty lub ukryty.

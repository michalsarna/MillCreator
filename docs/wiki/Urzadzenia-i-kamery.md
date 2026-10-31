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
| Camera | do ośmiu kamer: adres (strumień MJPEG lub zdjęcie JPEG, http / https) albo kamera komputera, obrót 0 / 90 / 180 / 270°, włączona / wyłączona |
| Notes | dowolny tekst |

Sterownik to GRBL (od 0.7 do 1.1 i nowszego), albo **Smoothieware** (pozycja listy *GRBL version*), albo płytka **Duet / RepRapFirmware** przez HTTP (pole wyboru na stronie Connection z adresem i hasłem, które nie jest zapisywane; patrz Sterowanie maszyną). Strona **Spindle** ma też **Tool changer** (`T<n> M6`), a strona **G-code** linie dla odkurzacza. Strona **Connection** ustawia, jak się z nim połączyć (port szeregowy albo gniazdo sieciowe, prędkość, strumieniowanie z liczeniem bufora lub proste, bazowanie albo kasowanie alarmu po połączeniu), a strona **Rotary** obrotnicę (uchwyt lub rolki, średnica przedmiotu, mikrokroki na obrót; oś Y obraca przedmiot). Karta **Device** edytora ma Connect, Home, Unlock, Pause, Resume, stan i położenie maszyny oraz konsolę.

## Kamera
Gdy urządzenie ma włączoną (i poprawną) kamerę, edytor dostaje kartę **Camera** z jej obrazem na żywo. Obrazy z adresu czyta czytnik strumienia z baseCreatora; kamery komputera wymagają kompilacji z `--features camera`. Obrót i korekty obrazu kamery są stosowane.

## W projekcie
Projekt zapisuje migawkę urządzenia, dla którego powstał (nazwa i trzy rozmiary pola roboczego). Gdy otwiera się go dla urządzenia o innym polu roboczym, pasek stanu o tym mówi.

**Własny G-code (strona G-code).** Cztery pola, jedno polecenie w wierszu (najwyżej 20 wierszy po 100 znaków; znaki sterujące są odrzucane): *Na początku programu* (po nagłówku i słowie współrzędnych roboczych), *Przy każdej zmianie freza* (także przy pierwszym frezie; po pauzie na wymianę, przed włączeniem wrzeciona, np. `T1 M6` albo polecenie chłodzenia), *Przy każdej zmianie operacji* (nowa warstwa; po jej komentarzu, przed obsługą freza) i *Na końcu programu* (po powrocie głowicy do zera, przed `M30`). Wiersze trafiają do każdego G-code, który program tworzy: plik, wysyłane zadanie, przebieg próbny. To własne polecenia maszyny, więc program ich nie sprawdza: nową linię wypróbuj najpierw na maszynie bez freza.

**Współrzędne robocze (G54 … G59) i zero programu (strona General).** Domyślnie program liczy od zera maszyny (narożnik wybrany jako *Machine zero*), jak dotąd. Gdy wybierzesz **G54 … G59**, program zaczyna się tym słowem (maszyna używa więc układu współrzędnych roboczych, którego zero ustawiłeś na niej, np. dotykając materiału), a każde X i Y jest liczone od **zera programu**: punktu pola roboczego podanego jako X, Y (pole robocze liczone od narożnika zera maszyny). Przy zerze 0, 0 liczby są takie same jak bez kodu, dochodzi tylko słowo. Korzystają z tego obrys (*Frame*), przebieg próbny i zadanie, a także powrót do zera na końcu i własny G-code. Z nie zmienia się: jest liczone od wierzchu materiału.

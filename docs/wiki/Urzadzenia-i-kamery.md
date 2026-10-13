🌐 [English](Devices-and-Cameras) · **Polski**

# Urządzenia i kamery

**Urządzenie** to frezarka. Lista urządzeń jest zapamiętywana między uruchomieniami i jest wspólna dla wszystkich projektów. Ekran startowy wybiera urządzenie; **New** i **Open…** go używają. Karta Device w edytorze pokazuje używane urządzenie i otwiera jego ustawienia.

![Dodawanie urządzenia](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/02-add-device.png)

## Przyciski ekranu startowego
**Add device…**, **I have no device**, **Edit…**, **Duplicate**, **Export…** (jedno urządzenie jako plik JSON), **Import…** (ściśle sprawdzany: nowszy format, błędna liczba lub plik ponad 2 MB są odrzucane) i **Delete** (prosi o potwierdzenie). Kopia lub importowane urządzenie o zajętej nazwie dostaje numer: `Router (2)`.

## Nie mam urządzenia
Tworzy jednorazowo zwykłe urządzenie o nazwie **Dummy**: pole robocze 300 × 200 mm, 60 mm skoku w Z, bezpieczna wysokość 5 mm, wrzeciono 1000–24000 obr/min. Zostaje na liście jak każde inne i można je edytować; projekt zapisuje się z nim jako z maszyną.

## Ustawienia urządzenia
| Strona | Co zawiera |
|---|---|
| General | nazwa, jednostki (mm lub cale; geometria jest zawsze zapisana w mm), zero maszyny (przód lewo, przód prawo, tył lewo, tył prawo) |
| Machine | pole robocze X i Y, skok w Z, **bezpieczna wysokość** (na której wrzeciono porusza się między cięciami) i prędkość ruchów nad materiałem |
| Spindle | najniższe i najwyższe obroty oraz czekanie po włączeniu |
| Camera | do ośmiu kamer: adres (strumień MJPEG lub zdjęcie JPEG, http / https) albo kamera komputera, obrót 0 / 90 / 180 / 270°, włączona / wyłączona |
| Notes | dowolny tekst |

Sterownik to GRBL. **MillCreator nie łączy się z maszyną**: urządzenie ją opisuje, żeby ścieżki do niej pasowały.

## Kamera
Gdy urządzenie ma włączoną (i poprawną) kamerę, edytor dostaje kartę **Camera** z jej obrazem na żywo. Obrazy z adresu czyta czytnik strumienia z baseCreatora; kamery komputera wymagają kompilacji z `--features camera`. Obrót i korekty obrazu kamery są stosowane.

## W projekcie
Projekt zapisuje migawkę urządzenia, dla którego powstał (nazwa i trzy rozmiary pola roboczego). Gdy otwiera się go dla urządzenia o innym polu roboczym, pasek stanu o tym mówi.

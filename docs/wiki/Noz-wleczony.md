🌐 [English](Drag-Knife) · **Polski**

# Nóż wleczony

Frezarka może też **ciąć** nożem wleczonym w uchwycie włożonym do tulejki wrzeciona: folię, karton, gumę, skórę, cienkie arkusze. Ostrze obraca się swobodnie w uchwycie, a jego czubek wlecze się trochę za osią, tak jak kółko krzesła za swoim sworzniem. MillCreator prowadzi oś tak, żeby po Twoich liniach szedł **czubek**, przy **stojącym wrzecionie**.

## Warstwa noża wleczonego

1. Nadaj warstwie operację **Nóż wleczony** (karta **Warstwy**). Frez nie jest potrzebny.
2. Wybierz **Nóż** z listy (albo **Biblioteka noży…**, żeby otworzyć bibliotekę). Ustawia on naraz **Przesunięcie czubka**, **Obracaj na narożach ostrzejszych niż** i **Tnij dalej za początkiem**; potem każdą z tych wartości można zmienić na warstwie.
3. Ustaw **Głębokość** warstwy: jak głęboko ostrze wchodzi w materiał. Nóż tnie **jednym przejściem** na tę głębokość: przejścia tam i z powrotem obracałyby ostrze w drugą stronę.
4. Przed warstwą zadanie zatrzymuje się z komunikatem **Put in the drag knife** (`M0` w G-code), a wrzeciono zostaje wyłączone; włóż uchwyt, wyzeruj Z na materiale, jeśli Twój uchwyt tego wymaga, i naciśnij **Kontynuuj zadanie**.

## Jak program koryguje nóż

* **Przesunięcie czubka:** jak daleko czubek jest za swoją osią (podaje je producent ostrza lub uchwytu). Oś idzie o tyle przed czubkiem, więc cięcie leży na linii.
* **Obracaj na narożach ostrzejszych niż:** na ostrzejszym narożu oś obchodzi je po małym łuku, ostrze obraca się w miejscu i naroże zostaje ostre.
* **Tnij dalej za początkiem:** zamknięta linia jest cięta o tyle dalej za swój początek, żeby się domknęła.
* Kierunek ostrza przechodzi z cięcia na cięcie, a każde cięcie zaczyna się od obrócenia go z kierunku, w którym zostało.
* **Najpierw otwory:** otwory są cięte przed częściami wokół nich, więc część trzyma się materiału, dopóki nie zostanie wycięta.

## Biblioteka noży

**Frezowanie → Biblioteka noży…** otwiera bibliotekę: **Gotowe noże** po lewej, pod nimi **Moje noże**, po prawej wartości wybranego noża.

| Gotowy nóż | Przesunięcie czubka | Obracaj na narożach ostrzejszych niż | Tnij dalej za początkiem | Do czego |
|---|---|---|---|---|
| Nóż wleczony w uchwycie maszyny CNC | 0,5 mm | 10° | 1,0 mm | uchwyt w tulejce wrzeciona, wrzeciono stoi; głębokość ustaw na warstwie |
| Ostrze 30° | 0,15 mm | 15° | 0,4 mm | cienka folia i papier, drobne szczegóły i małe litery |
| Ostrze 45° | 0,25 mm | 15° | 0,5 mm | folia, flex, papier i cienki karton |
| Ostrze 60° | 0,5 mm | 15° | 0,8 mm | gruba folia, karton, guma, pianka i szablony do piaskowania |

* Gotowego noża nie można zmienić. **Kopiuj** wstawia kopię do **Moich noży**, z **Nazwą**, wartościami i **Notatkami**; **Usuń** usuwa własny nóż.
* Twoje noże są zapisywane z ustawieniami programu i w jego kopii zapasowej.
* Warstwa **kopiuje** wartości noża: późniejsza zmiana w bibliotece jej nie zmienia. Gdy wartości warstwy nie pasują do żadnego noża, lista pokazuje **Własne wartości**.

## Pierwsze cięcie

Wytnij mały test (kwadrat z kołem w środku) w zbędnym kawałku, z materiałem przytrzymanym płasko (taśma albo stół podciśnieniowy: nóż go ciągnie). Zaokrąglone naroże znaczy, że przesunięcie jest za małe, pętelka albo haczyk na narożu, że za duże; linia, która się nie domyka, potrzebuje dłuższego **Tnij dalej za początkiem**. Gdy wszystko jest dobrze, **skopiuj** nóż do **Moich noży**, z głębokością, która się sprawdziła, w notatkach.

Zobacz też: [Warstwy i operacje](Warstwy-i-operacje), [Narzędzia projektowe](Narzedzia-projektowe#nóż-wleczony), [Bezpieczeństwo i ograniczenia](Bezpieczenstwo-i-ograniczenia).

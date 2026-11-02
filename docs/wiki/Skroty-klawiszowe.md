🌐 [English](Keyboard-Shortcuts) · **Polski**

# Skróty klawiszowe

Ctrl w Windows i Linuksie to **Cmd** na macOS. Klawisze działają w edytorze (nie na ekranie startowym) i żaden z nich nie działa, gdy klawiaturę ma pole tekstowe: gdy wpisujesz nazwę, liczbę albo G-code, Ctrl+C, Delete czy litera należą do pola.

## Klawisze menu

Klawisze pokazane obok pozycji menu (zob. [Menu](Menu)). Działają, gdy tylko otwarty jest rysunek, w Windows, Linuksie i na macOS, a pozycja, która w tej chwili jest szara, nic nie robi. O pasku menu macOS zob. [niżej](#macos).

| Klawisz | Pozycja | Menu |
|---|---|---|
| Ctrl+N | **Nowy** (New) | Plik |
| Ctrl+O | **Otwórz…** (Open…) | Plik |
| Ctrl+S | **Zapisz** (Save) | Plik |
| Ctrl+Shift+S | **Zapisz jako…** (Save as…) | Plik |
| Ctrl+Q | **Zakończ** (Quit) | Plik (macOS: Cmd+Q w menu **MillCreator**) |
| Ctrl+Z | **Cofnij** (Undo) | Edycja |
| Ctrl+Y, Ctrl+Shift+Z | **Ponów** (Redo) | Edycja |
| Ctrl+C | **Kopiuj** (Copy) | Edycja |
| Ctrl+V | **Wklej** (Paste) | Edycja |
| Ctrl+D | **Powiel** (Duplicate) | Edycja |
| Delete albo Backspace | **Usuń** (Delete; w menu *Del*) | Edycja |
| Ctrl+A | **Zaznacz wszystko** (Select all) | Edycja |
| Ctrl+G | **Grupuj** (Group) | Rozmieść |
| Ctrl+Shift+G | **Rozgrupuj** (Ungroup) | Rozmieść |
| Ctrl+L | **Zablokuj** (Lock) | Rozmieść |
| Ctrl+Shift+L | **Odblokuj** (Unlock) | Rozmieść |
| N | **Edytuj węzły** (Edit nodes) | Rozmieść (narzędzie węzłów, zob. niżej) |
| Ctrl+= (w menu Ctrl++) | **Powiększ** (Zoom in) | Widok |
| Ctrl+- | **Pomniejsz** (Zoom out) | Widok |
| Ctrl+Shift+0 | **Dopasuj stół do okna** (Fit bed to window) | Widok |
| Ctrl+Shift+9 | **Dopasuj do wszystkich obiektów** (Fit all objects) | Widok |
| Ctrl+Shift+P | **Widok 3D zadania…** (3D view of the job…) | Frezowanie |
| Ctrl+E | **Eksportuj G-code…** (Export G-code…) | Frezowanie |

Klawisz należy tylko do jednej pozycji (sprawdza to test programu): Ctrl+Shift+G to **Rozgrupuj**, a G-code eksportuje się klawiszem **Ctrl+E**.

## Inne klawisze edytora

| Klawisz | Co robi |
|---|---|
| Delete, Backspace | Usuwa zaznaczone kształty; z narzędziem węzłów i zaznaczonymi węzłami usuwa węzły. |
| Esc | Czyści zaznaczenie (i punkty linii rysowanej piórem). |
| Esc dwa razy | Drugie Esc w ciągu pół sekundy wraca do narzędzia **Zaznaczanie** (Select). |
| Ctrl+1 … Ctrl+9, Ctrl+0 | Pokazuje rysunek z karty 1 … 9; Ctrl+0 to karta 10. |
| Ctrl+W | Zamyka pokazany rysunek (pyta o niezapisaną pracę). |
| Enter | Kończy linię narzędzia pióra (to samo robi prawy przycisk albo dwuklik). |

## Narzędzia (jedna litera)

Litera bez Ctrl wybiera narzędzie lewej palety. Litery są w nazwach narzędzi (w podpowiedziach i w menu **Wstawianie**).

| Klawisz | Narzędzie | Klawisz | Narzędzie |
|---|---|---|---|
| V | **Zaznaczanie (V)** | L | **Linia (L)** |
| N | **Edycja węzłów (N)** | P | **Linia łamana / pióro (P)** |
| R | **Prostokąt (R)** | T | **Tekst (T)** |
| E | **Elipsa (E)** | H | **Przesuwanie widoku (H)** |
| Y | **Trójkąt (Y)** | Z | **Powiększ (Z)** |
| S | **Gwiazda (S)** | X | **Pomniejsz (X)** |
| G | **Wielokąt (G)** | | |
| K | **Serce (K)** | | |
| I | **Spirala (I)** | | |

## Mysz na obszarze roboczym

| Czynność | Co robi |
|---|---|
| Kółko albo szczypnięcie na touchpadzie | Powiększa wokół wskaźnika. |
| Przeciąganie środkowym przyciskiem | Przesuwa widok (z narzędziem **Przesuwanie widoku** także lewym). |
| Shift + klik, Ctrl + klik | Dodaje kształt do zaznaczenia albo go z niego wyjmuje. |
| Prawy przycisk (narzędzie Zaznaczanie) | Zaznacza kształt pod wskaźnikiem i otwiera menu kontekstowe. |
| Shift z narzędziami powiększania | Zamienia powiększanie z pomniejszaniem. |
| Klik na rysunek w innym panelu (albo na jego kartę) | Robi z tego rysunku aktywny. |
| Klik środkowym przyciskiem na kartę | Zamyka kartę. |

## Podgląd (widok 3D)

| Czynność | Co robi |
|---|---|
| Przeciąganie lewym przyciskiem | Obraca widok. |
| Przeciąganie prawym lub środkowym | Przesuwa obraz. |
| Kółko | Powiększa. |
| Dwuklik | Wraca do pierwszego widoku (to samo co **Resetuj widok**). |

W analizie G-code (Plik → Pokaż plik G-code…) przeciąganie przesuwa płaski plik i obraca plik 3D (przeciąganie prawym przyciskiem albo z Shiftem go przesuwa), kółko powiększa, a **Esc** ją zamyka.

## Okna i pola

* **Esc** anuluje pytanie *Zapisać pracę przed zamknięciem?* (jak **Anuluj**) i przerywa **Wskaż na obszarze roboczym** przy zerze programu.
* W oknie **Presety warstw** Enter zatwierdza nową nazwę presetu, a Esc ją porzuca.

## Przesuw z klawiatury

Na karcie **Sterowanie** (Control), gdy zaznaczona jest **Klawiatura** (Keyboard; domyślnie wyłączona): strzałki przesuwają X i Y, a **Page Up** / **Page Down** przesuwają Z, o krok wybrany nad nimi. Nic się nie rusza, gdy klawiaturę ma pole albo maszyna jest zajęta. Zob. [Sterowanie maszyną](Sterowanie-maszyna).

## macOS

Na macOS program używa paska menu systemu. Wszystkie klawisze z tej strony działają tam tak jak w Windows i Linuksie, z Cmd zamiast Ctrl:

* Pasek menu systemu przejmuje klawisze **Nowy** (Cmd+N), **Otwórz…** (Cmd+O), **Zapisz** (Cmd+S), **Zapisz jako…** (Cmd+Shift+S) i **Zakończ** (Cmd+Q, w menu **MillCreator**) i pokazuje je obok tych pozycji.
* Wszystkie pozostałe klawisze program przyjmuje sam, tak samo jak na innych systemach: Cmd+E, Cmd+Shift+P, Cmd+L, Cmd+Shift+L, Cmd+= / Cmd+-, Cmd+Shift+0 / Cmd+Shift+9, klawisze edycji, Cmd+1 … Cmd+0, Cmd+W i litery narzędzi. Pasek menu systemu nie pokazuje tych klawiszy obok pozycji; pokazuje je ta strona i pasek menu w Windows i Linuksie.

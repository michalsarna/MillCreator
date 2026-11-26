🌐 [English](Project-File) · **Polski** · [简体中文](Project-File-ZH) · [한국어](Project-File-KO)

# Plik projektu

MillCreator zapisuje **`.cre`**, plik projektu baseCreatora, który zapisuje też LightCreator: jeden plik JSON z rysunkiem, opisem i **jedną sekcją na program**. Każdy program zapisuje swoją sekcję i zostawia nietknięte sekcje pozostałych, więc projekt przechodzi między programami bez strat.

Sekcja MillCreatora nazywa się `mill`. Zawiera:
* **64 warstwy** ze wszystkimi ustawieniami (nazwa, kolor, kolejność, operacja, frez, głębokości, rozstaw przejść, posuwy, obroty, kierunek, wejście, mostki i mostki ustawione ręcznie, chłodziwo…),
* **materiał** (rodzaj, kolor, grubość, pierwsze przejście) i **ustawienia projektu** (rozmiar kawałka, zero programu na nim, Z od spodu),
* **migawkę urządzenia** (nazwa i pole robocze),
* **frezy używane przez warstwy**, żeby projekt dało się otworzyć tam, gdzie ich nie ma w bibliotece.

Otwarcie projektu z LightCreatora (albo zwykłego SVG, PDF lub pliku LightBurna) daje domyślne warstwy; jego pozostałe sekcje (`laser`) są zachowane i zapisane z powrotem przy zapisie. Projekt z nowszym formatem `mill`, niż zna program, jest odrzucany z komunikatem. Warstwa z błędnymi liczbami jest zastępowana domyślną; brakujący frez trafia do **Moich frezów**.

Samo urządzenie nie jest w projekcie, tylko jego migawka: używane jest to, które wybrano na ekranie startowym. Pasek stanu ostrzega, gdy pole robocze migawki nie jest polem tego urządzenia.

Opis projektu (karta **Opis**) jest częścią wspólnego kontenera, więc LightCreator pokazuje ten sam tekst. MillCreator wpisuje go też na początek G-code jako komentarze `(Note: …)` (zob. [Widok 3D i G-code](Widok-3D-i-G-code#eksport-g-code)).

## Ochrona hasłem

Projekt można chronić hasłami: w **Ustawieniach projektu…** LightCreatora, MillCreatora i PenCreatora oraz w **Plik → Ustawienia strony…** BaseCreatora, w części **Ochrona**.

* **Hasło właściciela**: wszystko (edycja, kopiowanie, eksport, zmiana lub usunięcie ochrony).
* **Hasło otwarcia** (opcjonalne): projekt można otworzyć, oglądać, zamienić w G-code i uruchomić na maszynie, można też zmienić materiał i ustawienia warstw (prędkość, moc, przejścia, głębokość, frez, pisak), ale rysunku nie da się edytować, kopiować, eksportować jako rysunku (SVG, DXF, Illustrator, LightBurn …) ani drukować. Wbudowana strona WWW nie pokazuje wtedy rysunku.
* Gdy jest tylko hasło właściciela, nikt inny nie otworzy pliku. Bez hasła pliku nie da się w ogóle odczytać, a **zapomnianego hasła nie da się odzyskać**.
* Projekt otwarty hasłem otwarcia można odblokować hasłem właściciela bez ponownego otwierania: w części **Ochrona** ustawień (**Hasło właściciela**, **Odblokuj**) albo poleceniem **Odblokuj hasłem właściciela…** z menu jego karty (prawy przycisk). To menu ma też **Informacje o projekcie…** i ustawienia projektu.

Plik pozostaje tekstem JSON z kluczem `protected_cre`; projekt w nim jest zaszyfrowany **XChaCha20-Poly1305** losowym kluczem 256-bitowym, zapisanym raz dla każdego hasła i zaszyfrowanym kluczem zrobionym z hasła przez **Argon2id** (64 MiB, 3 przebiegi, własna sól). Każdy zapis ma nowy nonce; zmieniony plik się nie otworzy. Hasła muszą mieć co najmniej 8 znaków, a hasło otwarcia musi być inne niż hasło właściciela.

To, czego zabrania hasło otwarcia, egzekwują programy, tak jak hasło właściciela w PDF: kto je zna, może w zasadzie odszyfrować projekt innymi narzędziami. Hasło właściciela zachowaj dla siebie.

🌐 [English](Design-Tools) · **Polski**

# Narzędzia projektowe

Edytor jest edytorem baseCreatora, takim samym jak w LightCreatorze.

![Właściwości](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/04-editor-properties.png)

* **Narzędzia** (po lewej): zaznaczanie, edycja węzłów, prostokąt, elipsa, trójkąt, gwiazda, wielokąt, serce, spirala, linia, pióro, tekst, przesuwanie i zoom. Klawisze literowe wybierają narzędzia jak w LightCreatorze. Nowe kształty trafiają na aktywną warstwę.
* **Uchwyty** przesuwają, zmieniają rozmiar, obracają i pochylają zaznaczenie; **pasek ułożenia** (po prawej) wyrównuje, rozmieszcza, porządkuje, grupuje, odbija i blokuje.
* Karta **Properties**: położenie i rozmiar, obrót, odbicie, wyrównanie.
* Menu **Edit**: cofnij / ponów, zaznacz wszystko, kopiuj, wklej, powiel, usuń, grupuj, rozgrupuj, operacje logiczne, odsunięcie i zaokrąglanie narożników.
* **Siatka i przyciąganie:** menu View i **Settings → View options…**
* **Import:** **File → Open…** wczytuje SVG, PDF i Illustratora (zgodnego z PDF) oraz projekty `.cre`; działa też przeciąganie pliku. **File → Import → Import LightBurn project…** wczytuje `.lbrn2` / `.lbrn`: prostokąty, elipsy, wielokąty, gwiazdy, ścieżki z krzywymi, grupy i tekst jako kontury (warstwy LightBurna nic tu nie znaczą). **Import UGS design…** wczytuje pliki `.ugsd` z UGS Designera: ścieżki, prostokąty, elipsy, tekst i obrazy, także ukryte obiekty; każdy sposób cięcia (na zewnątrz, w środku, po ścieżce, kieszeń, wiercenie) staje się ustawieniem warstwy z głębokością, posuwem i obrotami, a frez płaski o średnicy z pliku trafia do biblioteki. Import dodaje kształty do otwartego rysunku (jedno cofnięcie je usuwa) i używa warstw z pliku tylko tam, gdzie nie ma jeszcze kształtów.
* **Clipart:** galeria 310 obrazków z własnymi i wyszukiwaniem w sieci ([taka sama jak w LightCreatorze](https://github.com/michalsarna/LightCreator/wiki/Galeria-clipartow)). Obrazek jest wstawiany jako zwykłe kształty.
* **Prawy przycisk myszy** na kształcie otwiera menu kontekstowe: kopiuj, wklej, powiel, usuń, zaznacz wszystko, grupuj, rozgrupuj, zablokuj, odblokuj, odbij, obróć oraz podmenu *Arrange* (kolejność, wyrównanie, centrowanie, edycja węzłów, zamiana na ścieżkę); dla obrazu także *Adjust image…* i *Trace image…*. Pozycje, które nie pasują do zaznaczenia, są szare.
* **Obrazy (bitmapy):** **File → Import → Import image…** (albo upuszczenie pliku PNG, JPG, BMP, GIF lub WebP na okno) otwiera okno importu: obrót, odbicie, jasność, kontrast, gamma, automatyczne poziomy, negatyw i rozmiar. **Arrange → Adjust image…** zmienia obraz na stronie, a **Arrange → Trace image…** zamienia jego ciemne (lub jasne) obszary na kontury (próg, filtr plamek, uproszczenie, wygładzanie krzywych), które będą frezowane jak każdy inny kształt. Sam obraz jest tylko pokazywany, nie jest frezowany.
* **Panel boczny:** przeciągnij zakładkę, żeby ustawić ją w innym miejscu (kolejność jest zapamiętywana, także w kopii zapasowej). Mała strzałka po lewej stronie zakładek chowa panel; zostaje wąski pasek z nazwami zakładek, a kliknięcie nazwy otwiera panel na tej zakładce.
* **Dwa razy Esc** (drugi Esc w ciągu pół sekundy) wraca do narzędzia Select.
* **Pasek warstw** pod edytorem: jeden rząd 64 kolorów (kropka oznacza warstwy z kształtami); kliknięcie przenosi zaznaczone kształty na tę warstwę, podwójne kliknięcie otwiera jej opcje.
* Pole robocze to pole urządzenia; jednostki to jego jednostki.

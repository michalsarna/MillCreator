🌐 [English](Layers-and-Operations) · **Polski**

# Warstwy i operacje

Projekt ma **64 warstwy**. Każdy kształt leży na jednej warstwie, a warstwa mówi, co się z nim robi. Kliknij warstwę na karcie **Warstwy**, by uczynić ją aktywną (nowe kształty trafiają na nią) i edytować ją pod listą.

![Warstwy](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/05-layers.png)

Na liście: ptaszek bierze warstwę do ścieżek i G-code, oko pokazuje ją w polu roboczym, a liczba to ilość kształtów na niej.

### Obrazy

Obraz położony na warstwie przełącza tę warstwę na operację **Obraz** (raz; można to zmienić z powrotem). Warstwa graweruje wtedy obraz jednym przejściem na swoją głębokość (**Głębokość**; używaj małej głębokości: wierszy i kropek jest dużo), według ustawienia **Sposób rysowania obrazu**: **Rastrowane wiersze** (obraz jest rastrowany, a każdy wiersz jest frezowany tam, gdzie jest ciemny), **Kropki** (otwór dla każdej ciemnej komórki), **Głębokość według tonu** (wiersze są frezowane na głębokość zależną od tonu: im ciemniej, tym głębiej, aż do głębokości warstwy) albo **Relief 3D**. Dla dwóch pierwszych wybierasz **Rastrowanie** (próg, Floyd–Steinberg, Jarvis, Stucki, Atkinson, uporządkowane). Wiersze są oddalone o rozstaw przejść freza (**Rozstaw przejść**, procent jego średnicy) i biegną pod kątem z pola **Kąt skanowania**. 

**Relief 3D** (jak w molino): poziom szarości każdego piksela to głębokość, czerń jest najgłębsza, a biel zostaje nietknięta, aż do głębokości warstwy (**Głębokość**). Relief jest frezowany przejściami o głębokość przejścia (**Głębokość przejścia**) aż do pełnej głębokości (ostatnie przejście to sam relief), wiersze są oddalone o **Rozstaw przejść**, a **kształt freza jest uwzględniany** (frez płaski, kulisty, z zaokrąglonym narożem lub V nie wcina się w relief nigdzie pod swoim korpusem: wąskie zagłębienie, w które frez się nie mieści, zostaje płytkie; przy krawędzi obrazu frez zostaje trochę wyżej). Najpierw dopasuj obraz (jasność, kontrast, negatyw): *Rozmieść → Dostosuj obraz…*. **Frezowanie → Eksportuj relief jako STL…** zapisuje wyfrezowany element jako zamkniętą bryłę (grubość materiału z karty **Materiał**), do obejrzenia lub udostępnienia.

Relief można frezować na dwóch warstwach, tak jak robi się prawdziwą pracę: **Część reliefu** warstwy to **Zgrubnie i wykańczająco** (jak dotychczas), **Zgrubnie** (przejścia do reliefu z pozostawionym naddatkiem (*Naddatek*); użyj freza płaskiego) albo **Wykańczająco** (jedno przejście po samym reliefie, wiersz po wierszu co **Rozstaw przejść** i pod kątem z pola **Kąt skanowania**; użyj freza kulistego). Połóż ten sam obraz na dwóch warstwach (albo skopiuj warstwę): pierwsza dostaje frez płaski i część **Zgrubnie**, druga frez kulisty i część **Wykańczająco**; warstwy idą w kolejności swojej rangi.

**Plik → Importuj ▸ Importuj model 3D jako relief…** (także **Otwórz…** i przeciągnięcie pliku) wczytuje model **STL** (binarny lub tekstowy), **OBJ** albo **3MF** w milimetrach, patrzy na niego z góry i kładzie na stronie mapę głębi jako obraz: najwyższy punkt modelu jest biały (zostaje nietknięty), najniższy czarny, a tam, gdzie modelu nie ma, mapa jest czarna (materiał wokół jest wybierany). Obraz ma wielkość modelu. Program kładzie go na warstwie, która już frezuje relief, albo przełącza pierwszą pustą warstwę na operację **Obraz** z metodą **Relief 3D**, frezem kulistym z biblioteki i głębokością równą wysokości modelu (najwyżej grubość materiału). Wczytywane są modele do pięciu milionów trójkątów; komponenty i przekształcenia 3MF nie są stosowane (każdy obiekt leży tak, jak w pliku). Jeśli chcesz frezować tył modelu, odwróć go najpierw w jego programie.

Inne kształty na tej samej warstwie są prowadzone jak linie. Na warstwie z inną operacją obraz jest tylko pokazywany (ostrzeżenie mówi, że nie jest frezowany).

## Ustawienia warstwy
| Ustawienie | Znaczenie |
|---|---|
| Nazwa, Kolor (Name, Colour) | dowolne; kolor to kolor jej kształtów i jej cięć w widoku 3D |
| Operacja (Operation) | patrz niżej |
| Frez (Bit) | dowolny frez z [Biblioteki frezów](Biblioteka-frezow). Wybór freza bierze jego posuw, posuw wejścia, obroty, głębokość przejścia równą połowie średnicy i krok boczny |
| Głębokość (Depth) | całkowita głębokość pod powierzchnią materiału |
| Głębokość przejścia (Pass depth) | jak głęboko idzie jedno przejście; ostatnie zawsze kończy się na pełnej głębokości. **0 oznacza pełną głębokość w jednym przejściu** (ostrzeżenie mówi o tym, gdy to głębiej niż część tnąca freza); tak samo *Głębokość przejścia (część średnicy)* freza w bibliotece |
| Rozstaw przejść (Step over) | dla kieszeni: odstęp między pierścieniami jako część średnicy |
| Posuw, Posuw wcięcia (Feed, Plunge feed) | mm/min w materiale i przy zagłębianiu |
| Obroty wrzeciona (Spindle speed) | obr/min; wartość poza zakresem urządzenia jest zgłaszana |
| Kierunek (Direction) | dla profilu i kieszeni: **Jak narysowano** (jak program robił dotąd), **Współbieżne** (Climb) albo **Przeciwbieżne** (Conventional). Przy wrzecionie w prawo cięcie współbieżne trzyma materiał po lewej stronie freza (przeciwnie do ruchu wskazówek wokół części, zgodnie z ruchem wskazówek w kieszeni); kontury otworów części idą w drugą stronę |
| Naddatek, Przejście wykańczające (Stock to leave, Finishing pass) | przejścia do głębokości zostają tak daleko od linii; z włączonym *Przejście wykańczające* jedno dodatkowe przejście na pełnej głębokości zdejmuje resztę wzdłuż prawdziwej linii |
| Wejście (Entry) | jak frez wchodzi w dół: **Wbicie** (prosto w dół z posuwem wbicia), **Rampa zygzakiem** (tam i z powrotem wzdłuż pierwszego odcinka drogi), **Rampa po konturze** (w dół w trakcie okrążania) albo **Helisa** (w kieszeni, po okręgu, który się mieści; w innych miejscach rampa po konturze). **Kąt rampy** to nachylenie w stopniach; **Posuw rampy** to posuw na rampie (0 = *Posuw rampy* freza z biblioteki, a gdy go nie ma, posuw wbicia). Rampa stromsza niż 45° jest zgłaszana |
| Wzór kieszeni, Kąt rastra (Pocket pattern, Raster angle) | dla kieszeni i planowania: **Pierścienie** (pierścień za pierścieniem od zewnątrz; pierścień wzdłuż środka czyści pasek, który zostaje, gdy szerokość nie jest wielokrotnością kroku bocznego) albo **Raster** (równoległe linie pod kątem z pola *Kąt rastra*, tam i z powrotem, łączone tam, gdzie droga między nimi zostaje wewnątrz) |
| Zostań w dole między pierścieniami (Stay down between rings) | dla *Kieszeni* ze wzorem *Pierścienie* (domyślnie wyłączone): na każdej głębokości pierścienie idą po kolei, a frez przechodzi do następnego pierścienia cięciem w materiale zamiast się unosić, gdy cała droga leży wewnątrz obszaru, do którego frez sięga (wysp nigdy nie przecina, więc tam unosi się jak dotąd). Pierwszy pierścień głębszego przejścia zaczyna się z głębokości poprzedniego przejścia. Podgląd i szacunek czasu pokazują nowe ruchy. |
| Przejście po obrysie (Outline pass) | dla kieszeni: przejście wzdłuż ściany **Przed** albo **Po** czyszczeniu (albo *Brak*), na każdej głębokości |
| Resztki po warstwie (Rest of layer) | dla kieszeni: czyści tylko to, do czego nie sięgnął frez innej warstwy (kieszeni większym frezem), frezem tej warstwy, który musi być mniejszy |
| Głębokość startowa, Inlay (wkładka), Luz inlay (Start depth, Inlay, Inlay gap) | dla operacji **Rzeźbienie V**. *Inlay (wkładka)* robi jedną połowę inlaya: **Gniazdo** (Socket) jest rzeźbione wewnątrz kształtu (powiększonego o *Luz inlay*), **Wkładka** (Plug) to kształt pomniejszony o luz, z ukośnymi ścianami rzeźbionymi *wokół niego*, na zewnątrz linii, aż do głębokości warstwy. Wkładkę rzeźb w drugim kawałku z tymi samymi kształtami odbitymi lustrzanie, wytnij ją warstwą profilu i wklej w gniazdo |
| Część reliefu (Relief part) | dla metody **Relief 3D**: **Zgrubnie i wykańczająco**, **Zgrubnie** (z *Naddatkiem*) albo **Wykańczająco**, patrz Obrazy wyżej |
| Chłodziwo, Odkurzacz (Coolant, Dust collector) | *Chłodziwo* to Off (nie jest tłumaczone), **Mgła** (Mist, M7) albo **Zalewanie** (Flood, M8) na czas cięcia warstwy; *Odkurzacz* włącza odkurzacz własnymi liniami urządzenia (Ustawienia urządzenia → G-code: *Włączenie odkurzacza / Wyłączenie odkurzacza*; warstwa, która o niego prosi bez tych linii, jest zgłaszana). Zostają włączone między warstwami, które proszą o to samo, a wyłączają się przed zmianą freza i na końcu programu |
| Trochoidy (Trochoids) | dla „Podążania za linią”: frez idzie pętlami wzdłuż linii (*Średnica pętli trochoidy*, z posuwem *Krok trochoidy* na obrót; z *Półokręgi* frez kołysze się w poprzek linii), więc wycina rowek szeroki jak pętle plus frez, przy małym obciążeniu |
| Mostki (Tabs) | **Pełny** (frez unosi się nad materiał), **Częściowy** (do wysokości mostka) albo **Rampa** (częściowy ze skosem, *Nachylenie mostka*); *Szerokość mostka* to szerokość mostka przy części (do tego dochodzi frez), *Wysokość mostka* liczy się od dna cięcia. Rozmieszczone są równomiernie: jeden na każdy *Odstęp mostków* konturu, nie mniej i nie więcej niż podane liczby, bez mostków na otworach części przy włączonym *Bez mostków na otworach*. Unoszone są tylko przejścia poniżej wierzchu mostka. Dla profili i „Podążanie za linią” na liniach zamkniętych. **Umieść mostki ręcznie** (pod ustawieniami mostków) zamienia obszar roboczy w edytor mostków dla kształtów warstwy: kliknij obrys, by dodać mostek (żółta kropka), przeciągnij kropkę wzdłuż obrysu, by ją przesunąć, kliknij kropkę, by ją usunąć; *Usuń mostki ustawione ręcznie* usuwa wszystkie. Kontur z mostkami ustawionymi ręcznie dostaje tylko je; zapisują się w projekcie razem z warstwą i podążają za kształtem przy jego przesuwaniu i skalowaniu |

## Operacje
| Operacja | Co robi frez |
|---|---|
| **Wycinanie na zewnątrz linii** (Cut outside the line) | idzie po zewnętrznej stronie linii, więc kształt zostaje: część jest wycinana z materiału |
| **Wycinanie wewnątrz linii** (Cut inside the line) | idzie po wewnętrznej stronie linii, więc kształt wypada: otwór lub okno |
| **Podążanie za linią** (Follow the line) | środek freza idzie po linii (grawer, rowki); linie otwarte też działają; przy głębszych przejściach frez wraca wzdłuż linii |
| **Kieszeń** (Pocket) | wybiera całą powierzchnię wewnątrz linii, pierścień po pierścieniu, od zewnątrz do środka |
| **Wiertło** (Drill) | otwór w środku każdego zamkniętego kształtu, porcjami o głębokość przejścia |
| **Obraz** (Picture) | obraz na warstwie jest grawerowany (zobacz Obrazy wyżej) |
| **Planowanie** (Face) | planuje wierzch materiału na obszarze obejmującym wszystkie kształty warstwy (narysuj prostokąt, który da rozmiar), pierścieniami lub rastrem; *Wyjście poza krawędź* to wyjście krawędzi freza poza krawędź obszaru (frez wychodzi trochę poza, żeby nie została krawędź) |
| **Rozwiercanie** (Bore) | okrągły otwór większy od freza: frez schodzi dookoła spiralą (rampa po konturze, pod kątem z pola *Kąt rampy*), a potem raz okrąża otwór na pełnej głębokości; rdzeń wypada |
| **Fazowanie** (Chamfer) | frez V (albo grawerski) idzie po linii na głębokości, która daje fazę o szerokości *Szerokość fazy* na powierzchni (głębokość warstwy wynika z niej; frez płaski jest zgłaszany) |
| **Rzeźbienie V** (V-carve) | frez V (lub grawerski) schodzi tym głębiej, im dalej jest od krawędzi kształtu, więc ściany są skośne, a narożniki ostre: napisy, tablice, inlaye. **Głębokość** to najgłębsze miejsce freza; tam, gdzie kształt jest szerszy, dno jest płaskie (frezowane samym frezem V, co zostawia grzbiety i trwa długo, więc program o tym ostrzega: lepszy jest frez płaski w warstwie *Kieszeń*). *Głębokość startowa*: nic nie jest cięte płycej niż ta głębokość (płytką część ściany i tak wycinają głębsze pierścienie, więc to tylko oszczędza czas). Frez płaski jest zgłaszany jako błąd |
| **Rowek z narożnikami dogbone** (Dogbone slot) | jak *Wycinanie wewnątrz linii*, z wcięciem w każdy ostry narożnik: środek freza idzie dalej, aż jego krawędź dotrze do narożnika, więc kwadratowa część pasuje do rowka zrobionego okrągłym frezem |

Kształty jednej warstwy tworzą jeden obszar (parzysto-nieparzysty): kształt wewnątrz innego jest jego otworem. Kształty, które mają być cięte osobno, umieść na osobnych warstwach. Linie otwarte są pomijane w profilu i kieszeni (program o tym mówi). Obrazy nigdy nie są frezowane.

## Co jest sprawdzane
Warstwa bez freza, głębokość większa niż grubość materiału, głębokość większa niż skok maszyny, obroty poza zakresem, cięcia poza polem roboczym i linia za mała dla freza są zgłaszane jako ostrzeżenia w podglądzie i przy zapisie G-code.

## Prędkość skrawania i posuw na ząb

Obok obrotów i posuwów warstwa (oraz *Prędkości początkowe dla warstwy* freza w bibliotece) pokazuje **Prędkość skrawania (Vc)** (m/min, Vc = π · D · n / 1000), **Posuw na ząb (fz)** (mm, fz = F / (n · z)) i **Posuw wgłębny na ząb**, liczone ze średnicy D i liczby ostrzy z freza. Wpisanie Vc zmienia obroty, wpisanie fz zmienia posuw (albo posuw wgłębny); zapisywane są tylko obroty i posuwy w mm/min, więc pliki się nie zmieniają. Bez freza wiersze są przyciemnione (podpowiedź mówi dlaczego: *Z freza: średnica i liczba ostrzy*).

## Presety warstw

Nad ustawieniami warstwy: **Zapisz preset…** i **Wczytaj preset** otwierają okno **Presety warstw**: pole z nazwą i **Zapisz preset…** zachowują sposób frezowania warstwy (operację, frez, głębokości, posuwy, mostki, rampy…) pod tą nazwą (preset o tej samej nazwie jest zastępowany); pole wyszukiwania zawęża listę; kliknięcie presetu nakłada go na warstwę, ✎ zmienia nazwę, a ✖ zapomina preset; **Eksportuj…** zapisuje wszystkie presety do pliku, a **Importuj…** dodaje te z pliku (złe wpisy są pomijane). Nazwa, kolor i kolejność warstwy, warstwa, po której działa kieszeń resztkowa, oraz mostki ustawione ręcznie zostają jak były. Preset z frezem, którego nie ma już w bibliotece, nakłada pozostałe ustawienia i zostawia warstwę bez freza (mówi o tym pasek stanu). Presety są w `mill_layer_presets.json` we wspólnym folderze danych (najwyżej 60).

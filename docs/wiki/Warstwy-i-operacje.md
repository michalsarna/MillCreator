🌐 [English](Layers-and-Operations) · **Polski**

# Warstwy i operacje

Projekt ma **64 warstwy**. Każdy kształt leży na jednej warstwie, a warstwa mówi, co się z nim robi. Kliknij warstwę na karcie **Layers**, by uczynić ją aktywną (nowe kształty trafiają na nią) i edytować ją pod listą.

![Warstwy](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/05-layers.png)

Na liście: ptaszek bierze warstwę do ścieżek i G-code, oko pokazuje ją w polu roboczym, a liczba to ilość kształtów na niej.

### Obrazy

Obraz położony na warstwie przełącza tę warstwę na operację **Picture** (raz; można to zmienić z powrotem). Warstwa graweruje wtedy obraz jednym przejściem na swoją **Depth** (używaj małej głębokości: wierszy i kropek jest dużo), według **Picture method**: **Dithered rows** (obraz jest rastrowany, a każdy wiersz jest frezowany tam, gdzie jest ciemny), **Dots** (otwór dla każdej ciemnej komórki) **Depth by tone** (wiersze są frezowane na głębokość zależną od tonu: im ciemniej, tym głębiej, aż do głębokości warstwy) albo **3D relief**. Dla dwóch pierwszych wybierasz **Dithering** (próg, Floyd–Steinberg, Jarvis, Stucki, Atkinson, uporządkowane). Wiersze są oddalone o **Step over** freza (procent jego średnicy) i biegną pod kątem **Scan angle**. 

**3D relief** (jak w molino): poziom szarości każdego piksela to głębokość, czerń jest najgłębsza, a biel zostaje nietknięta, aż do **Depth** warstwy. Relief jest frezowany przejściami o **Pass depth** aż do pełnej głębokości (ostatnie przejście to sam relief), wiersze są oddalone o **Step over**, a **kształt freza jest uwzględniany** (frez płaski, kulisty, z zaokrąglonym narożem lub V nie wcina się w relief nigdzie pod swoim korpusem: wąskie zagłębienie, w które frez się nie mieści, zostaje płytkie; przy krawędzi obrazu frez zostaje trochę wyżej). Najpierw dopasuj obraz (jasność, kontrast, negatyw): *Arrange → Adjust image…*. **Milling → Export relief as STL…** zapisuje wyfrezowany element jako zamkniętą bryłę (grubość materiału z zakładki Material), do obejrzenia lub udostępnienia.

Inne kształty na tej samej warstwie są prowadzone jak linie. Na warstwie z inną operacją obraz jest tylko pokazywany (ostrzeżenie mówi, że nie jest frezowany).

## Ustawienia warstwy
| Ustawienie | Znaczenie |
|---|---|
| Name, Colour | dowolne; kolor to kolor jej kształtów i jej cięć w widoku 3D |
| Operation | patrz niżej |
| Bit | dowolny frez z [Biblioteki frezów](Biblioteka-frezow). Wybór freza bierze jego posuw, posuw wejścia, obroty, głębokość przejścia równą połowie średnicy i krok boczny |
| Depth | całkowita głębokość pod powierzchnią materiału |
| Pass depth | jak głęboko idzie jedno przejście; ostatnie zawsze kończy się na pełnej głębokości |
| Step over | dla kieszeni: odstęp między pierścieniami jako część średnicy |
| Feed, Plunge feed | mm/min w materiale i przy zagłębianiu |
| Spindle speed | obr/min; wartość poza zakresem urządzenia jest zgłaszana |
| Direction | dla profilu i kieszeni: **As drawn** (jak program robił dotąd), **Climb** (współbieżnie) albo **Conventional** (przeciwbieżnie). Przy wrzecionie w prawo cięcie współbieżne trzyma materiał po lewej stronie freza (przeciwnie do ruchu wskazówek wokół części, zgodnie z ruchem wskazówek w kieszeni); kontury otworów części idą w drugą stronę |
| Stock to leave, Finishing pass | przejścia do głębokości zostają tak daleko od linii; z *Finishing pass* jedno dodatkowe przejście na pełnej głębokości zdejmuje resztę wzdłuż prawdziwej linii |
| Entry | jak frez wchodzi w dół: **Plunge** (prosto w dół z posuwem wbicia), **Zigzag ramp** (tam i z powrotem wzdłuż pierwszego odcinka drogi), **Ramp along the contour** (w dół w trakcie okrążania) albo **Helix** (w kieszeni, po okręgu, który się mieści; w innych miejscach rampa po konturze). **Ramp angle** to nachylenie w stopniach; **Ramp feed** to posuw na rampie (0 = *Ramp feed* freza z biblioteki, a gdy go nie ma, posuw wbicia). Rampa stromsza niż 45° jest zgłaszana |
| Tabs | **Full** (frez unosi się nad materiał), **Partial** (do wysokości mostka) albo **Ramp** (częściowy ze skosem, *Tab slope*); *Tab width* to szerokość mostka przy części (do tego dochodzi frez), *Tab height* liczy się od dna cięcia. Rozmieszczone są równomiernie: jeden na każdy *Tab spacing* konturu, nie mniej i nie więcej niż podane liczby, bez mostków na otworach części przy włączonym *No tabs on holes*. Unoszone są tylko przejścia poniżej wierzchu mostka. Dla profili i „Follow the line” na liniach zamkniętych. **Place tabs by hand** (pod ustawieniami mostków) zamienia obszar roboczy w edytor mostków dla kształtów warstwy: kliknij obrys, by dodać mostek (żółta kropka), przeciągnij kropkę wzdłuż obrysu, by ją przesunąć, kliknij kropkę, by ją usunąć; *Clear hand-placed tabs* usuwa wszystkie. Kontur z mostkami ustawionymi ręcznie dostaje tylko je; zapisują się w projekcie razem z warstwą i podążają za kształtem przy jego przesuwaniu i skalowaniu |

## Operacje
| Operacja | Co robi frez |
|---|---|
| **Cut outside the line** | idzie po zewnętrznej stronie linii, więc kształt zostaje: część jest wycinana z materiału |
| **Cut inside the line** | idzie po wewnętrznej stronie linii, więc kształt wypada: otwór lub okno |
| **Follow the line** | środek freza idzie po linii (grawer, rowki); linie otwarte też działają; przy głębszych przejściach frez wraca wzdłuż linii |
| **Pocket** | wybiera całą powierzchnię wewnątrz linii, pierścień po pierścieniu, od zewnątrz do środka |
| **Drill** | otwór w środku każdego zamkniętego kształtu, porcjami o głębokość przejścia |
| **Picture** | obraz na warstwie jest grawerowany (zobacz Obrazy wyżej) |

Kształty jednej warstwy tworzą jeden obszar (parzysto-nieparzysty): kształt wewnątrz innego jest jego otworem. Kształty, które mają być cięte osobno, umieść na osobnych warstwach. Linie otwarte są pomijane w profilu i kieszeni (program o tym mówi). Obrazy nigdy nie są frezowane.

## Co jest sprawdzane
Warstwa bez freza, głębokość większa niż grubość materiału, głębokość większa niż skok maszyny, obroty poza zakresem, cięcia poza polem roboczym i linia za mała dla freza są zgłaszane jako ostrzeżenia w oknie 3D i przy zapisie G-code.

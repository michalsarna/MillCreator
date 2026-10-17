🌐 [English](Layers-and-Operations) · **Polski**

# Warstwy i operacje

Projekt ma **64 warstwy**. Każdy kształt leży na jednej warstwie, a warstwa mówi, co się z nim robi. Kliknij warstwę na karcie **Layers**, by uczynić ją aktywną (nowe kształty trafiają na nią) i edytować ją pod listą.

![Warstwy](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/05-layers.png)

Na liście: ptaszek bierze warstwę do ścieżek i G-code, oko pokazuje ją w polu roboczym, a liczba to ilość kształtów na niej.

### Obrazy

Obraz położony na warstwie przełącza tę warstwę na operację **Picture** (raz; można to zmienić z powrotem). Warstwa graweruje wtedy obraz jednym przejściem na swoją **Depth** (używaj małej głębokości: wierszy i kropek jest dużo), według **Picture method**: **Dithered rows** (obraz jest rastrowany, a każdy wiersz jest frezowany tam, gdzie jest ciemny), **Dots** (otwór dla każdej ciemnej komórki) albo **Depth by tone** (wiersze są frezowane na głębokość zależną od tonu: im ciemniej, tym głębiej, aż do głębokości warstwy). Dla dwóch pierwszych wybierasz **Dithering** (próg, Floyd–Steinberg, Jarvis, Stucki, Atkinson, uporządkowane). Wiersze są oddalone o **Step over** freza (procent jego średnicy) i biegną pod kątem **Scan angle**. Inne kształty na tej samej warstwie są prowadzone jak linie. Na warstwie z inną operacją obraz jest tylko pokazywany (ostrzeżenie mówi, że nie jest frezowany).

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

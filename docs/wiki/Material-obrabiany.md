🌐 [English](Material) · **Polski**

# Materiał

Karta **Materiał** (pod nagłówkiem **Materiał obrabianego przedmiotu**) ustawia obrabiany przedmiot, który pokazuje widok 3D i który zapamiętuje projekt.

![Materiał](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/06-material.png)

| Materiał | Kolor |
|---|---|
| **Drewno, jasne** (Wood, light) | piaskowy |
| **Drewno, średnie** (Wood, medium) | brązowy |
| **Drewno, ciemne** (Wood, dark) | ciemnobrązowy |
| **Metal** | szary |
| **Tworzywa** (Plastic) | domyślnie biały, **dowolny wybrany kolor** |

Tylko kolor plastiku można zmienić; pozostałe materiały mają swój kolor. **Grubość** (0,1 do 500 mm) to grubość materiału: wierzch materiału to Z = 0, a warstwa głębsza niż grubość jest zgłaszana. **Plik → Ustawienia projektu…** (otwiera się po Plik → Nowy) zapisuje też **wymiary** kawałka, **gdzie jest zero X i Y** na nim i czy **Z liczy się od spodu** (patrz Szybki start). Materiał nie wpływa na posuwy ani obroty; niosą je frezy i warstwy.

**Pierwsze przejście.** W sekcji **Pierwsze przejście** włącz *Zacznij od płytkiego przejścia wzdłuż krawędzi* i podaj **Głębokość pierwszego przejścia** (domyślnie 0,1 mm). Każda warstwa najpierw przechodzi wtedy raz wzdłuż krawędzi, które będzie ciąć, tylko na tę głębokość, co czysto przełamuje wierzch materiału (okleinę, laminat, powłokę), a potem wykonuje zwykłe przejścia. Profil i *Podążanie za linią* mają je wzdłuż swojej linii, kieszeń tylko wzdłuż krawędzi (nie po całym obszarze), wiercenie go nie ma; działa z każdym frezem. Warstwa nie głębsza niż pierwsze przejście nie dostaje dodatkowego przejścia. Ustawienie zapisuje się w projekcie.

![Plastik w kolorze niebieskim](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/18-3d-view-plastic.png)

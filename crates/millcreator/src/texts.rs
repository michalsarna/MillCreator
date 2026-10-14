//! The texts of MillCreator that the shared interface does not have: English, six languages in the table, and the other 23
//! in the files of `assets/lang`. The shared texts are in baseCreator; the application's own are asked first.
use base_ui::i18n::{self, Texts};
use std::sync::Once;

/// The texts of the other languages, in the order of `Lang::ALL` after the first seven. One line each:
/// `id|translation`, where the id is the line of the English text in `assets/lang/keys.txt`.
const EXTRA_FILES: [&str; 23] = [
    include_str!("../../../assets/lang/bg.txt"),
    include_str!("../../../assets/lang/ca.txt"),
    include_str!("../../../assets/lang/hr.txt"),
    include_str!("../../../assets/lang/cs.txt"),
    include_str!("../../../assets/lang/da.txt"),
    include_str!("../../../assets/lang/nl.txt"),
    include_str!("../../../assets/lang/et.txt"),
    include_str!("../../../assets/lang/fr.txt"),
    include_str!("../../../assets/lang/el.txt"),
    include_str!("../../../assets/lang/hu.txt"),
    include_str!("../../../assets/lang/is.txt"),
    include_str!("../../../assets/lang/ga.txt"),
    include_str!("../../../assets/lang/lv.txt"),
    include_str!("../../../assets/lang/lt.txt"),
    include_str!("../../../assets/lang/mt.txt"),
    include_str!("../../../assets/lang/nb.txt"),
    include_str!("../../../assets/lang/pt.txt"),
    include_str!("../../../assets/lang/ro.txt"),
    include_str!("../../../assets/lang/sk.txt"),
    include_str!("../../../assets/lang/sl.txt"),
    include_str!("../../../assets/lang/es.txt"),
    include_str!("../../../assets/lang/sv.txt"),
    include_str!("../../../assets/lang/uk.txt"),
];
const KEYS: &str = include_str!("../../../assets/lang/keys.txt");

static TEXTS: Texts = Texts::new(TABLE, KEYS, EXTRA_FILES);

/// Make the texts known to the translation layer (once).
pub fn register() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| i18n::register(&TEXTS));
}

// Columns: English, Polski, Deutsch, Italiano, Suomi, 中文, हिन्दी
#[rustfmt::skip]
const TABLE: &[[&str; 7]] = &[
    ["3D view of the job", "Widok 3D zadania", "3D-Ansicht des Auftrags", "Vista 3D del lavoro", "Työn 3D-näkymä", "作业 3D 视图", "कार्य का 3D दृश्य"],
    ["3D view of the job…", "Widok 3D zadania…", "3D-Ansicht des Auftrags…", "Vista 3D del lavoro…", "Työn 3D-näkymä…", "作业 3D 视图…", "कार्य का 3D दृश्य…"],
    ["A copy among your own bits", "Kopia wśród Twoich frezów", "Eine Kopie unter deinen eigenen Fräsern", "Una copia tra le tue frese", "Kopio omien terien joukossa", "在你自己的刀具中的副本", "आपके अपने बिट्स में एक प्रति"],
    ["A hole at the centre of every closed shape of the layer, in steps of the pass depth.", "Otwór pośrodku każdego zamkniętego kształtu warstwy, kolejnymi przejściami o zadanej głębokości.", "Ein Loch in der Mitte jeder geschlossenen Form der Ebene, in Schritten der Zustelltiefe.", "Un foro al centro di ogni forma chiusa del livello, a passate della profondità indicata.", "Reikä jokaisen tason suljetun muodon keskelle, syöttösyvyyden askelin.", "在图层每个闭合图形的中心钻一个孔，按切深分步进行。", "लेयर के हर बंद आकार के केंद्र में एक छेद, पास की गहराई के चरणों में।"],
    ["A ready-made bit cannot be changed. Make a copy to change it.", "Gotowego freza nie można zmienić. Zrób kopię, żeby go zmienić.", "Ein fertiger Fräser kann nicht geändert werden. Erstelle eine Kopie, um ihn zu ändern.", "Una fresa predefinita non si può modificare. Fanne una copia per cambiarla.", "Valmista terää ei voi muuttaa. Tee kopio, jos haluat muuttaa sitä.", "内置刀具不能修改。请先复制再修改。", "तैयार बिट को बदला नहीं जा सकता। बदलने के लिए उसकी प्रति बनाएँ।"],
    ["About", "O programie", "Über", "Informazioni", "Tietoja", "关于", "के बारे में"],
    ["Add a camera", "Dodaj kamerę", "Kamera hinzufügen", "Aggiungi una telecamera", "Lisää kamera", "添加摄像头", "कैमरा जोड़ें"],
    ["Add a device", "Dodaj urządzenie", "Gerät hinzufügen", "Aggiungi un dispositivo", "Lisää laite", "添加设备", "डिवाइस जोड़ें"],
    ["Add device", "Dodaj urządzenie", "Gerät hinzufügen", "Aggiungi dispositivo", "Lisää laite", "添加设备", "डिवाइस जोड़ें"],
    ["Add device…", "Dodaj urządzenie…", "Gerät hinzufügen…", "Aggiungi dispositivo…", "Lisää laite…", "添加设备…", "डिवाइस जोड़ें…"],
    ["Address", "Adres", "Adresse", "Indirizzo", "Osoite", "地址", "पता"],
    ["All of the area inside the line is cleared, ring after ring, to the depth of the layer.", "Cały obszar wewnątrz linii jest wybierany, pierścień po pierścieniu, do głębokości warstwy.", "Die gesamte Fläche innerhalb der Linie wird Ring für Ring bis zur Tiefe der Ebene ausgeräumt.", "Tutta l'area dentro la linea viene svuotata, anello dopo anello, fino alla profondità del livello.", "Koko viivan sisäpuolinen alue tyhjennetään rengas renkaalta tason syvyyteen asti.", "线内的整个区域按一圈一圈清除，直到图层的深度。", "रेखा के भीतर का पूरा क्षेत्र, छल्ले दर छल्ले, लेयर की गहराई तक साफ़ किया जाता है।"],
    ["Arrange", "Rozmieść", "Anordnen", "Disponi", "Järjestä", "排列", "व्यवस्थित करें"],
    ["Back left", "Tył po lewej", "Hinten links", "Dietro a sinistra", "Takana vasemmalla", "左后", "पीछे बाएँ"],
    ["Back right", "Tył po prawej", "Hinten rechts", "Dietro a destra", "Takana oikealla", "右后", "पीछे दाएँ"],
    ["Back to the first view. Double click on the picture does the same.", "Powrót do pierwszego widoku. Podwójne kliknięcie obrazu robi to samo.", "Zurück zur ersten Ansicht. Ein Doppelklick auf das Bild tut dasselbe.", "Torna alla prima vista. Il doppio clic sull'immagine fa lo stesso.", "Takaisin ensimmäiseen näkymään. Kuvan kaksoisnapsautus tekee saman.", "回到最初的视图。双击图片效果相同。", "पहले दृश्य पर वापस। चित्र पर डबल क्लिक भी यही करता है।"],
    ["Ball nose", "Frez kulisty", "Kugelfräser", "Fresa a testa sferica", "Pallopäinen jyrsin", "球头铣刀", "बॉल नोज़"],
    ["Bit", "Frez", "Fräser", "Fresa", "Terä", "刀具", "बिट"],
    ["Bit library", "Biblioteka frezów", "Fräserbibliothek", "Libreria di frese", "Terakirjasto", "刀具库", "बिट लाइब्रेरी"],
    ["Bit library…", "Biblioteka frezów…", "Fräserbibliothek…", "Libreria di frese…", "Terakirjasto…", "刀具库…", "बिट लाइब्रेरी…"],
    ["Bull nose (corner radius)", "Frez z zaokrąglonym narożem (promień naroża)", "Torusfräser (Eckenradius)", "Fresa torica (raggio d'angolo)", "Pyöristetty kulmajyrsin (kulmasäde)", "圆角铣刀（圆角半径）", "बुल नोज़ (कोने की त्रिज्या)"],
    ["Camera", "Kamera", "Kamera", "Fotocamera", "Kamera", "摄像头", "कैमरा"],
    ["Camera name", "Nazwa kamery", "Kameraname", "Nome della telecamera", "Kameran nimi", "摄像头名称", "कैमरे का नाम"],
    ["Camera number", "Numer kamery", "Kameranummer", "Numero della telecamera", "Kameran numero", "摄像头编号", "कैमरा संख्या"],
    ["Camera of this computer", "Kamera tego komputera", "Kamera dieses Computers", "Telecamera di questo computer", "Tämän tietokoneen kamera", "本机摄像头", "इस कंप्यूटर का कैमरा"],
    ["Camera rotation", "Obrót kamery", "Kameradrehung", "Rotazione fotocamera", "Kameran kierto", "摄像头旋转", "कैमरा रोटेशन"],
    ["Cameras", "Kamery", "Kameras", "Telecamere", "Kamerat", "摄像头", "कैमरे"],
    ["Cannot use the file", "Nie można użyć pliku", "Die Datei kann nicht verwendet werden", "Impossibile usare il file", "Tiedostoa ei voi käyttää", "无法使用该文件", "फ़ाइल का उपयोग नहीं किया जा सकता"],
    ["Cannot use the file: {}", "Nie można użyć pliku: {}", "Die Datei kann nicht verwendet werden: {}", "Impossibile usare il file: {}", "Tiedostoa ei voi käyttää: {}", "无法使用该文件：{}", "फ़ाइल का उपयोग नहीं हो सकता: {}"],
    ["Choose a bit", "Wybierz frez", "Einen Fräser wählen", "Scegli una fresa", "Valitse terä", "选择刀具", "बिट चुनें"],
    ["Choose the machine you will work with.", "Wybierz maszynę, na której będziesz pracować.", "Wähle die Maschine, mit der du arbeiten willst.", "Scegli la macchina con cui lavorare.", "Valitse kone, jolla työskentelet.", "选择你要使用的机器。", "वह मशीन चुनें जिस पर आप काम करेंगे।"],
    ["Confirm delete", "Potwierdź usunięcie", "Löschen bestätigen", "Conferma eliminazione", "Vahvista poisto", "确认删除", "हटाने की पुष्टि करें"],
    ["Controller", "Sterownik", "Steuerung", "Controller", "Ohjain", "控制器", "कंट्रोलर"],
    ["Copied to {}", "Skopiowano do {}", "Kopiert nach {}", "Copiato in {}", "Kopioitu: {}", "已复制为 {}", "{} में कॉपी किया"],
    ["Corner radius", "Promień naroża", "Eckenradius", "Raggio d'angolo", "Kulmasäde", "圆角半径", "कोने की त्रिज्या"],
    ["Could not save", "Nie udało się zapisać", "Konnte nicht speichern", "Impossibile salvare", "Tallennus ei onnistunut", "无法保存", "सहेजा नहीं जा सका"],
    ["Could not save: {}", "Nie udało się zapisać: {}", "Konnte nicht speichern: {}", "Impossibile salvare: {}", "Tallennus ei onnistunut: {}", "无法保存：{}", "सहेजा नहीं जा सका: {}"],
    ["Cut inside the line", "Wycinanie wewnątrz linii", "Innerhalb der Linie schneiden", "Taglia all'interno della linea", "Leikkaa viivan sisäpuolelta", "沿线内侧切割", "रेखा के भीतर काटें"],
    ["Cut outside the line", "Wycinanie na zewnątrz linii", "Außerhalb der Linie schneiden", "Taglia all'esterno della linea", "Leikkaa viivan ulkopuolelta", "沿线外侧切割", "रेखा के बाहर काटें"],
    ["Cuts", "Cięcia", "Schnitte", "Tagli", "Leikkaukset", "切割", "कट"],
    ["Cuts: {} m, about {} min", "Cięcia: {} m, około {} min", "Schnitte: {} m, etwa {} min", "Tagli: {} m, circa {} min", "Leikkaukset: {} m, noin {} min", "切割：{} 米，约 {} 分钟", "कट: {} मी, लगभग {} मिनट"],
    ["Cutting diameter", "Średnica skrawająca", "Schneiddurchmesser", "Diametro di taglio", "Leikkaushalkaisija", "切削直径", "काटने का व्यास"],
    ["Cutting length", "Długość skrawająca", "Schneidenlänge", "Lunghezza di taglio", "Leikkauspituus", "切削长度", "काटने की लंबाई"],
    ["Delete this camera", "Usuń tę kamerę", "Diese Kamera löschen", "Elimina questa telecamera", "Poista tämä kamera", "删除此摄像头", "इस कैमरे को हटाएँ"],
    ["Depth", "Głębokość", "Tiefe", "Profondità", "Syvyys", "深度", "गहराई"],
    ["Device", "Urządzenie", "Gerät", "Dispositivo", "Laite", "设备", "डिवाइस"],
    ["Device settings", "Ustawienia urządzenia", "Geräteeinstellungen", "Impostazioni dispositivo", "Laitteen asetukset", "设备设置", "डिवाइस सेटिंग्स"],
    ["Device settings…", "Ustawienia urządzenia…", "Geräteeinstellungen…", "Impostazioni dispositivo…", "Laitteen asetukset…", "设备设置…", "डिवाइस सेटिंग्स…"],
    ["Drill", "Wiertło", "Bohrer", "Punta da foratura", "Pora", "钻头", "ड्रिल"],
    ["Edit", "Edycja", "Bearbeiten", "Modifica", "Muokkaa", "编辑", "संपादन"],
    ["Edit…", "Edytuj…", "Bearbeiten…", "Modifica…", "Muokkaa…", "编辑…", "संपादित करें…"],
    ["Engraving bit", "Frez grawerski", "Gravierfräser", "Fresa per incisione", "Kaiverrusterä", "雕刻刀", "उत्कीर्णन बिट"],
    ["Export G-code…", "Eksportuj G-code…", "G-Code exportieren…", "Esporta G-code…", "Vie G-koodi…", "导出 G-code…", "G-code निर्यात करें…"],
    ["Export this bit…", "Eksportuj ten frez…", "Diesen Fräser exportieren…", "Esporta questa fresa…", "Vie tämä terä…", "导出此刀具…", "यह बिट निर्यात करें…"],
    ["Export…", "Eksportuj…", "Exportieren…", "Esporta…", "Vie…", "导出…", "निर्यात…"],
    ["Feed", "Posuw", "Vorschub", "Avanzamento", "Syöttö", "进给", "फ़ीड"],
    ["File", "Plik", "Datei", "File", "Tiedosto", "文件", "फ़ाइल"],
    ["Flat end mill", "Frez palcowy", "Schaftfräser", "Fresa a candela", "Sormijyrsin", "平底铣刀", "फ़्लैट एंड मिल"],
    ["Flutes", "Liczba ostrzy", "Schneiden", "Taglienti", "Uurteet", "刃数", "फ़्लूट"],
    ["Follow the line", "Podążanie za linią", "Der Linie folgen", "Segui la linea", "Seuraa viivaa", "沿线走", "रेखा का अनुसरण करें"],
    ["Front left", "Przód po lewej", "Vorne links", "Davanti a sinistra", "Edessä vasemmalla", "左前", "आगे बाएँ"],
    ["Front right", "Przód po prawej", "Vorne rechts", "Davanti a destra", "Edessä oikealla", "右前", "आगे दाएँ"],
    ["General", "Ogólne", "Allgemein", "Generale", "Yleiset", "常规", "सामान्य"],
    ["Help", "Pomoc", "Hilfe", "Aiuto", "Ohje", "帮助", "सहायता"],
    ["Highest speed", "Największa prędkość", "Höchste Drehzahl", "Velocità massima", "Suurin nopeus", "最高转速", "सबसे अधिक गति"],
    ["I have no device", "Nie mam urządzenia", "Ich habe kein Gerät", "Non ho un dispositivo", "Minulla ei ole laitetta", "我没有设备", "मेरे पास कोई डिवाइस नहीं है"],
    ["Import LightBurn project…", "Importuj projekt LightBurn…", "LightBurn-Projekt importieren…", "Importa progetto LightBurn…", "Tuo LightBurn-projekti…", "导入 LightBurn 项目…", "LightBurn प्रोजेक्ट आयात करें…"],
    ["Imported the device {}", "Zaimportowano urządzenie {}", "Gerät {} importiert", "Dispositivo {} importato", "Laite {} tuotu", "已导入设备 {}", "डिवाइस {} आयात किया गया"],
    ["Import…", "Importuj…", "Importieren…", "Importa…", "Tuo…", "导入…", "आयात…"],
    ["Layers", "Warstwy", "Ebenen", "Livelli", "Tasot", "图层", "लेयर"],
    ["Left out", "Pominięte", "Ausgelassen", "Esclusi", "Pois jätetyt", "已忽略", "छोड़े गए"],
    ["Lowest speed", "Najmniejsza prędkość", "Niedrigste Drehzahl", "Velocità minima", "Pienin nopeus", "最低转速", "सबसे कम गति"],
    ["Machine", "Maszyna", "Maschine", "Macchina", "Kone", "机器", "मशीन"],
    ["Machine zero (0,0)", "Zero maszyny (0,0)", "Maschinennullpunkt (0,0)", "Zero macchina (0,0)", "Koneen nollapiste (0,0)", "机器原点 (0,0)", "मशीन शून्य (0,0)"],
    ["Material", "Materiał", "Material", "Materiale", "Materiaali", "材料", "सामग्री"],
    ["Material of the workpiece", "Materiał obrabianego przedmiotu", "Material des Werkstücks", "Materiale del pezzo", "Työkappaleen materiaali", "工件材料", "वर्कपीस की सामग्री"],
    ["Metal", "Metal", "Metall", "Metallo", "Metalli", "金属", "धातु"],
    ["Milled: taken into the toolpaths and the G-code", "Frezowana: wchodzi do ścieżek i do G-code", "Gefräst: wird in die Werkzeugwege und den G-Code übernommen", "Fresato: incluso nei percorsi e nel G-code", "Jyrsitään: mukaan työstöratoihin ja G-koodiin", "铣削：计入刀路和 G-code", "मिलिंग होती है: टूलपाथ और G-code में शामिल"],
    ["Milling", "Frezowanie", "Fräsen", "Fresatura", "Jyrsintä", "铣削", "मिलिंग"],
    ["Moves at the safe height", "Ruchy na bezpiecznej wysokości", "Bewegungen in sicherer Höhe", "Movimenti all'altezza di sicurezza", "Liikkeet turvakorkeudella", "安全高度上的移动", "सुरक्षित ऊँचाई पर चालें"],
    ["My bits", "Moje frezy", "Meine Fräser", "Le mie frese", "Omat terät", "我的刀具", "मेरे बिट्स"],
    ["New", "Nowy", "Neu", "Nuovo", "Uusi", "新建", "नया"],
    ["New bit", "Nowy frez", "Neuer Fräser", "Nuova fresa", "Uusi terä", "新建刀具", "नया बिट"],
    ["No bit", "Brak freza", "Kein Fräser", "Nessuna fresa", "Ei terää", "无刀具", "कोई बिट नहीं"],
    ["No device yet. Add one, or work without a machine.", "Nie ma jeszcze urządzenia. Dodaj je albo pracuj bez maszyny.", "Noch kein Gerät. Füge eines hinzu oder arbeite ohne Maschine.", "Ancora nessun dispositivo. Aggiungine uno o lavora senza macchina.", "Ei vielä laitetta. Lisää sellainen tai työskentele ilman konetta.", "还没有设备。添加一个，或不用机器工作。", "अभी कोई डिवाइस नहीं। एक जोड़ें, या बिना मशीन के काम करें।"],
    ["Not an http or https address.", "To nie jest adres http ani https.", "Keine http- oder https-Adresse.", "Non è un indirizzo http o https.", "Ei ole http- tai https-osoite.", "不是 http 或 https 地址。", "यह http या https पता नहीं है।"],
    ["Notes", "Notatki", "Notizen", "Note", "Muistiinpanot", "笔记", "नोट"],
    ["Notes about this machine", "Notatki o tej maszynie", "Notizen zu dieser Maschine", "Note su questa macchina", "Muistiinpanoja tästä koneesta", "关于这台机器的备注", "इस मशीन के बारे में नोट्स"],
    ["Nothing here yet", "Jeszcze nic tu nie ma", "Noch nichts vorhanden", "Ancora niente qui", "Täällä ei ole vielä mitään", "这里还没有内容", "यहाँ अभी कुछ नहीं है"],
    ["Nothing selected", "Nic nie zaznaczono", "Nichts ausgewählt", "Nessuna selezione", "Ei valintaa", "未选择任何内容", "कुछ चयनित नहीं"],
    ["Only the colour of a plastic can be changed. The material and its thickness are shown in the 3D view and saved in the project.", "Tylko kolor plastiku można zmienić. Materiał i jego grubość widać w widoku 3D i zapisują się w projekcie.", "Nur die Farbe eines Kunststoffs kann geändert werden. Das Material und seine Dicke werden in der 3D-Ansicht gezeigt und im Projekt gespeichert.", "Si può cambiare solo il colore di una plastica. Il materiale e il suo spessore compaiono nella vista 3D e vengono salvati nel progetto.", "Vain muovin väriä voi muuttaa. Materiaali ja sen paksuus näkyvät 3D-näkymässä ja tallentuvat projektiin.", "只有塑料的颜色可以更改。材料及其厚度会显示在 3D 视图中并保存在项目里。", "केवल प्लास्टिक का रंग बदला जा सकता है। सामग्री और उसकी मोटाई 3D दृश्य में दिखती है और प्रोजेक्ट में सहेजी जाती है।"],
    ["Open a drawing or start an empty design.", "Otwórz rysunek albo zacznij pusty projekt.", "Öffne eine Zeichnung oder beginne mit einem leeren Entwurf.", "Apri un disegno o inizia un progetto vuoto.", "Avaa piirros tai aloita tyhjä suunnitelma.", "打开一幅图，或开始一个空白设计。", "कोई ड्रॉइंग खोलें या खाली डिज़ाइन शुरू करें।"],
    ["Open…", "Otwórz…", "Öffnen…", "Apri…", "Avaa…", "打开…", "खोलें…"],
    ["Operation", "Operacja", "Vorgang", "Operazione", "Toiminto", "操作", "ऑपरेशन"],
    ["Overall length", "Długość całkowita", "Gesamtlänge", "Lunghezza totale", "Kokonaispituus", "总长度", "कुल लंबाई"],
    ["Pass depth", "Głębokość przejścia", "Zustelltiefe", "Profondità di passata", "Syöttösyvyys", "每层切深", "पास की गहराई"],
    ["Pass depth (of the diameter)", "Głębokość przejścia (część średnicy)", "Zustelltiefe (vom Durchmesser)", "Profondità di passata (del diametro)", "Syöttösyvyys (halkaisijasta)", "每层切深（占直径的比例）", "पास की गहराई (व्यास का)"],
    ["Plastic", "Tworzywa", "Kunststoff", "Plastica", "Muovi", "塑料", "प्लास्टिक"],
    ["Plunge feed", "Posuw wcięcia", "Eintauchvorschub", "Avanzamento di affondo", "Pistosyöttö", "下刀进给", "प्लंज फ़ीड"],
    ["Pocket", "Kieszeń", "Tasche", "Tasca", "Tasku", "型腔", "पॉकेट"],
    ["Progress", "Postęp", "Fortschritt", "Avanzamento", "Edistyminen", "进度", "प्रगति"],
    ["Properties", "Właściwości", "Eigenschaften", "Proprietà", "Ominaisuudet", "属性", "गुण"],
    ["Put the selected shapes on this layer", "Umieść zaznaczone kształty na tej warstwie", "Die ausgewählten Formen auf diese Ebene legen", "Metti le forme selezionate su questo livello", "Siirrä valitut muodot tälle tasolle", "将所选图形放到此图层", "चुने हुए आकार इस लेयर पर रखें"],
    ["Read a device from a file", "Wczytaj urządzenie z pliku", "Ein Gerät aus einer Datei lesen", "Leggi un dispositivo da un file", "Lue laite tiedostosta", "从文件读取设备", "फ़ाइल से डिवाइस पढ़ें"],
    ["Read bits from a file", "Wczytaj frezy z pliku", "Fräser aus einer Datei lesen", "Leggi le frese da un file", "Lue terät tiedostosta", "从文件读取刀具", "फ़ाइल से बिट पढ़ें"],
    ["Ready", "Gotowe", "Bereit", "Pronto", "Valmis", "就绪", "तैयार"],
    ["Ready-made bits", "Gotowe frezy", "Fertige Fräser", "Frese predefinite", "Valmiit terät", "内置刀具", "तैयार बिट"],
    ["Reset view", "Resetuj widok", "Ansicht zurücksetzen", "Reimposta la vista", "Palauta näkymä", "重置视图", "दृश्य रीसेट करें"],
    ["Safe height", "Bezpieczna wysokość", "Sichere Höhe", "Altezza di sicurezza", "Turvakorkeus", "安全高度", "सुरक्षित ऊँचाई"],
    ["Save", "Zapisz", "Speichern", "Salva", "Tallenna", "保存", "सहेजें"],
    ["Save as…", "Zapisz jako…", "Speichern unter…", "Salva come…", "Tallenna nimellä…", "另存为…", "इस रूप में सहेजें…"],
    ["Save this device to a file", "Zapisz to urządzenie do pliku", "Dieses Gerät in einer Datei speichern", "Salva questo dispositivo in un file", "Tallenna tämä laite tiedostoon", "将此设备保存到文件", "इस डिवाइस को फ़ाइल में सहेजें"],
    ["Save your bits to a file", "Zapisz swoje frezy do pliku", "Deine Fräser in einer Datei speichern", "Salva le tue frese in un file", "Tallenna omat terät tiedostoon", "将你的刀具保存到文件", "अपने बिट्स को फ़ाइल में सहेजें"],
    ["Saved", "Zapisano", "Gespeichert", "Salvato", "Tallennettu", "已保存", "सहेजा गया"],
    ["Saved the device to {}", "Zapisano urządzenie do {}", "Gerät in {} gespeichert", "Dispositivo salvato in {}", "Laite tallennettu: {}", "设备已保存到 {}", "डिवाइस {} में सहेजा गया"],
    ["Saved {}", "Zapisano {}", "{} gespeichert", "Salvato {}", "Tallennettu {}", "已保存 {}", "{} सहेजा गया"],
    ["Settings", "Ustawienia", "Einstellungen", "Impostazioni", "Asetukset", "设置", "सेटिंग्स"],
    ["Shank diameter", "Średnica trzonka", "Schaftdurchmesser", "Diametro del gambo", "Varren halkaisija", "刀柄直径", "शैंक का व्यास"],
    ["Shapes on this layer", "Kształty na tej warstwie", "Formen auf dieser Ebene", "Forme su questo livello", "Muodot tällä tasolla", "此图层上的图形", "इस लेयर पर आकार"],
    ["Shown in the Camera tab", "Pokazywana w zakładce Kamera", "Im Reiter Kamera gezeigt", "Mostrata nella scheda Telecamera", "Näytetään Kamera-välilehdellä", "显示在“摄像头”标签页", "कैमरा टैब में दिखाया गया"],
    ["Shown in the work area", "Pokazana w obszarze roboczym", "Im Arbeitsbereich angezeigt", "Mostrato nell'area di lavoro", "Näytetään työalueella", "显示在工作区域", "कार्य क्षेत्र में दिखाया गया"],
    ["Speed of moves above the material", "Prędkość ruchów nad materiałem", "Geschwindigkeit der Bewegungen über dem Material", "Velocità dei movimenti sopra il materiale", "Liikkeiden nopeus materiaalin yläpuolella", "材料上方的移动速度", "सामग्री के ऊपर चालों की गति"],
    ["Spindle", "Wrzeciono", "Spindel", "Mandrino", "Kara", "主轴", "स्पिंडल"],
    ["Spindle speed", "Obroty wrzeciona", "Spindeldrehzahl", "Giri del mandrino", "Karan kierrosnopeus", "主轴转速", "स्पिंडल की गति"],
    ["Start", "Start", "Start", "Avvia", "Käynnistä", "开始", "शुरू"],
    ["Starting speeds for a layer", "Prędkości początkowe dla warstwy", "Startwerte für eine Ebene", "Velocità iniziali per un livello", "Tason aloitusnopeudet", "图层的起始速度", "लेयर के लिए शुरुआती गतियाँ"],
    ["Step over", "Rozstaw przejść", "Seitliche Zustellung", "Sovrapposizione laterale", "Sivuttaissyöttö", "步距", "स्टेप ओवर"],
    ["Step over (of the diameter)", "Rozstaw przejść (część średnicy)", "Seitliche Zustellung (vom Durchmesser)", "Sovrapposizione laterale (del diametro)", "Sivuttaissyöttö (halkaisijasta)", "步距（占直径的比例）", "स्टेप ओवर (व्यास का)"],
    ["Switched on", "Włączona", "Eingeschaltet", "Attiva", "Päällä", "已开启", "चालू"],
    ["Tapered ball nose", "Frez kulisty stożkowy", "Konischer Kugelfräser", "Fresa conica a testa sferica", "Kartiomainen pallopäinen jyrsin", "锥形球头铣刀", "टेपर्ड बॉल नोज़"],
    ["The bit goes round the inside of the line, so the shape falls out: a hole or a window.", "Frez idzie po wewnętrznej stronie linii, więc kształt wypada: otwór lub okno.", "Der Fräser läuft innen an der Linie entlang, die Form fällt also heraus: ein Loch oder ein Fenster.", "La fresa passa all'interno della linea, quindi la forma cade: un foro o una finestra.", "Terä kulkee viivan sisäpuolella, joten muoto putoaa pois: reikä tai ikkuna.", "刀具沿线的内侧走，所以图形会掉下来：形成孔或窗口。", "बिट रेखा के भीतर से चलता है, इसलिए आकार गिर जाता है: छेद या खिड़की।"],
    ["The bit goes round the outside of the line, so the shape stays: a part is cut out of the material.", "Frez idzie po zewnętrznej stronie linii, więc kształt zostaje: część jest wycinana z materiału.", "Der Fräser läuft außen an der Linie entlang, die Form bleibt also stehen: ein Teil wird aus dem Material geschnitten.", "La fresa passa all'esterno della linea, quindi la forma resta: un pezzo viene ritagliato dal materiale.", "Terä kulkee viivan ulkopuolella, joten muoto jää: kappale leikataan materiaalista.", "刀具沿线的外侧走，所以图形保留：从材料中切出一个零件。", "बिट रेखा के बाहर से चलता है, इसलिए आकार रहता है: सामग्री से एक पुर्ज़ा काटा जाता है।"],
    ["The centre of the bit follows the line: engraving and grooves. Open lines work too.", "Środek freza podąża za linią: grawerowanie i rowki. Otwarte linie też działają.", "Die Mitte des Fräsers folgt der Linie: Gravuren und Nuten. Offene Linien funktionieren auch.", "Il centro della fresa segue la linea: incisioni e scanalature. Funzionano anche le linee aperte.", "Terän keskipiste seuraa viivaa: kaiverrukset ja urat. Avoimet viivat toimivat myös.", "刀具中心沿线走：雕刻和开槽。开放线也可以。", "बिट का केंद्र रेखा का अनुसरण करता है: उत्कीर्णन और खाँचे। खुली रेखाएँ भी काम करती हैं।"],
    ["The cuts go outside of the work area of the device", "Cięcia wychodzą poza obszar roboczy urządzenia", "Die Schnitte liegen außerhalb des Arbeitsbereichs des Geräts", "I tagli escono dall'area di lavoro del dispositivo", "Leikkaukset ulottuvat laitteen työalueen ulkopuolelle", "切割超出了设备的工作区域", "कट डिवाइस के कार्य क्षेत्र से बाहर जाते हैं"],
    ["The device of this project", "Urządzenie tego projektu", "Das Gerät dieses Projekts", "Il dispositivo di questo progetto", "Tämän projektin laite", "此项目的设备", "इस प्रोजेक्ट का डिवाइस"],
    ["The device was added.", "Dodano urządzenie.", "Das Gerät wurde hinzugefügt.", "Il dispositivo è stato aggiunto.", "Laite lisättiin.", "设备已添加。", "डिवाइस जोड़ा गया।"],
    ["The device was saved.", "Zapisano urządzenie.", "Das Gerät wurde gespeichert.", "Il dispositivo è stato salvato.", "Laite tallennettiin.", "设备已保存。", "डिवाइस सहेजा गया।"],
    ["The file is too large", "Plik jest zbyt duży", "Die Datei ist zu groß", "Il file è troppo grande", "Tiedosto on liian suuri", "文件太大", "फ़ाइल बहुत बड़ी है"],
    ["There is nothing to cut: put shapes on layers that have a bit.", "Nie ma czego wyciąć: połóż kształty na warstwach, które mają frez.", "Es gibt nichts zu schneiden: Lege Formen auf Ebenen mit einem Fräser.", "Non c'è nulla da tagliare: metti le forme su livelli con una fresa.", "Ei ole mitään leikattavaa: laita muodot tasoille, joilla on terä.", "没有可切割的内容：请把图形放到有刀具的图层上。", "काटने के लिए कुछ नहीं है: आकार उन लेयर पर रखें जिनमें बिट हो।"],
    ["There is nothing to show yet: draw shapes and put them on layers that have a bit.", "Nie ma jeszcze czego pokazać: narysuj kształty i połóż je na warstwach, które mają frez.", "Es gibt noch nichts zu zeigen: Zeichne Formen und lege sie auf Ebenen mit einem Fräser.", "Non c'è ancora nulla da mostrare: disegna delle forme e mettile su livelli con una fresa.", "Ei ole vielä mitään näytettävää: piirrä muotoja ja laita ne tasoille, joilla on terä.", "还没有可显示的内容：请绘制图形并放到有刀具的图层上。", "अभी दिखाने के लिए कुछ नहीं है: आकार बनाएँ और उन्हें उन लेयर पर रखें जिनमें बिट हो।"],
    ["Thickness", "Grubość", "Dicke", "Spessore", "Paksuus", "厚度", "मोटाई"],
    ["This build cannot use the cameras of the computer.", "Ta wersja programu nie może używać kamer komputera.", "Diese Version kann die Kameras des Computers nicht verwenden.", "Questa versione non può usare le telecamere del computer.", "Tämä versio ei voi käyttää tietokoneen kameroita.", "此版本不能使用电脑的摄像头。", "यह संस्करण कंप्यूटर के कैमरों का उपयोग नहीं कर सकता।"],
    ["This device has no camera that is switched on. Add one in the device settings.", "To urządzenie nie ma włączonej kamery. Dodaj ją w ustawieniach urządzenia.", "Dieses Gerät hat keine eingeschaltete Kamera. Füge in den Geräteeinstellungen eine hinzu.", "Questo dispositivo non ha una telecamera attiva. Aggiungila nelle impostazioni del dispositivo.", "Tällä laitteella ei ole päällä olevaa kameraa. Lisää sellainen laitteen asetuksissa.", "此设备没有开启的摄像头。请在设备设置中添加。", "इस डिवाइस में कोई चालू कैमरा नहीं है। डिवाइस सेटिंग्स में एक जोड़ें।"],
    ["This device is not in the list: it came with the project", "Tego urządzenia nie ma na liście: przyszło z projektem", "Dieses Gerät ist nicht in der Liste: Es kam mit dem Projekt", "Questo dispositivo non è nell'elenco: è arrivato con il progetto", "Tämä laite ei ole luettelossa: se tuli projektin mukana", "此设备不在列表中：它随项目而来", "यह डिवाइस सूची में नहीं है: यह प्रोजेक्ट के साथ आया"],
    ["This is the stand-in device (Dummy): the default work area and speeds.", "To urządzenie zastępcze (Dummy): domyślny obszar roboczy i prędkości.", "Das ist das Ersatzgerät (Dummy): Standard-Arbeitsbereich und -Geschwindigkeiten.", "Questo è il dispositivo sostitutivo (Dummy): area di lavoro e velocità predefinite.", "Tämä on varalaite (Dummy): oletustyöalue ja -nopeudet.", "这是替代设备（Dummy）：默认的工作区域和速度。", "यह स्थानापन्न डिवाइस (Dummy) है: डिफ़ॉल्ट कार्य क्षेत्र और गतियाँ।"],
    ["Tip diameter", "Średnica końcówki", "Spitzendurchmesser", "Diametro della punta", "Kärjen halkaisija", "刀尖直径", "नोक का व्यास"],
    ["Too many devices", "Zbyt wiele urządzeń", "Zu viele Geräte", "Troppi dispositivi", "Liian monta laitetta", "设备太多", "बहुत अधिक डिवाइस"],
    ["Type", "Rodzaj", "Typ", "Tipo", "Tyyppi", "类型", "प्रकार"],
    ["Units", "Jednostki", "Einheiten", "Unità", "Yksiköt", "单位", "इकाइयाँ"],
    ["V-bit", "Frez V", "V-Fräser", "Fresa a V", "V-jyrsin", "V 形刀", "V-बिट"],
    ["View", "Widok", "Ansicht", "Vista", "Näytä", "视图", "दृश्य"],
    ["View options…", "Opcje widoku…", "Ansichtsoptionen…", "Opzioni di visualizzazione…", "Näkymän asetukset…", "视图选项…", "दृश्य विकल्प…"],
    ["Wait after switching on", "Czekanie po włączeniu", "Wartezeit nach dem Einschalten", "Attesa dopo l'accensione", "Odotus päällekytkennän jälkeen", "开启后的等待时间", "चालू करने के बाद प्रतीक्षा"],
    ["Waiting for the picture…", "Oczekiwanie na obraz…", "Warte auf das Bild…", "In attesa dell'immagine…", "Odotetaan kuvaa…", "正在等待画面…", "चित्र की प्रतीक्षा…"],
    ["Wood, dark", "Drewno, ciemne", "Holz, dunkel", "Legno, scuro", "Puu, tumma", "木材，深色", "लकड़ी, गहरी"],
    ["Wood, light", "Drewno, jasne", "Holz, hell", "Legno, chiaro", "Puu, vaalea", "木材，浅色", "लकड़ी, हल्की"],
    ["Wood, medium", "Drewno, średnie", "Holz, mittel", "Legno, medio", "Puu, keskisävyinen", "木材，中等", "लकड़ी, मध्यम"],
    ["Work on a project without a machine: a plain device called Dummy, with the default work area and speeds", "Pracuj nad projektem bez maszyny: zwykłe urządzenie o nazwie Dummy z domyślnym polem roboczym i prędkościami", "An einem Projekt ohne Maschine arbeiten: ein einfaches Gerät namens Dummy mit der Standard-Arbeitsfläche und den Standardgeschwindigkeiten", "Lavora a un progetto senza macchina: un dispositivo semplice chiamato Dummy, con l'area di lavoro e le velocità predefinite", "Työskentele projektin parissa ilman konetta: tavallinen laite nimeltä Dummy oletustyöalueella ja oletusnopeuksilla", "无需机器即可处理项目：一台名为 Dummy 的普通设备，使用默认工作区域和速度", "मशीन के बिना प्रोजेक्ट पर काम करें: Dummy नाम का एक सामान्य डिवाइस, डिफ़ॉल्ट कार्य क्षेत्र और गति के साथ"],
    ["Z is how far the spindle goes down from the top of the material. The safe height is where the spindle moves between cuts.", "Z to jak głęboko wrzeciono schodzi od góry materiału. Bezpieczna wysokość to miejsce, gdzie wrzeciono przemieszcza się między cięciami.", "Z ist, wie tief die Spindel von der Oberseite des Materials nach unten fährt. Die sichere Höhe ist die Höhe, auf der sich die Spindel zwischen den Schnitten bewegt.", "Z indica quanto scende il mandrino dalla superficie del materiale. L'altezza di sicurezza è quella a cui il mandrino si muove tra un taglio e l'altro.", "Z kertoo, kuinka syvälle kara menee materiaalin yläpinnasta. Turvakorkeus on korkeus, jolla kara liikkuu leikkausten välillä.", "Z 是主轴从材料顶面向下走的深度。安全高度是切割之间主轴移动时所在的高度。", "Z यह है कि स्पिंडल सामग्री के ऊपर से कितना नीचे जाता है। सुरक्षित ऊँचाई वह है जहाँ स्पिंडल कटों के बीच चलता है।"],
    ["bits added", "dodane frezy", "Fräser hinzugefügt", "frese aggiunte", "terät lisätty", "已添加刀具", "बिट जोड़े गए"],
    ["bits added to the library", "frezy dodane do biblioteki", "Fräser zur Bibliothek hinzugefügt", "frese aggiunte alla libreria", "terät lisätty kirjastoon", "刀具已添加到刀具库", "बिट लाइब्रेरी में जोड़े गए"],
    ["e.g. My CNC router", "np. Moja frezarka CNC", "z. B. Meine CNC-Fräse", "es. La mia fresatrice CNC", "esim. Oma CNC-jyrsin", "例如：我的 CNC 雕刻机", "उदा. मेरा CNC राउटर"],
    ["this project was made for a different machine: {}", "ten projekt powstał dla innej maszyny: {}", "dieses Projekt wurde für eine andere Maschine erstellt: {}", "questo progetto è stato creato per un'altra macchina: {}", "tämä projekti on tehty toiselle koneelle: {}", "此项目是为另一台机器制作的：{}", "यह प्रोजेक्ट किसी दूसरी मशीन के लिए बनाया गया था: {}"],
    ["{} passes to {} mm", "{} przejść do {} mm", "{} Durchgänge bis {} mm", "{} passate fino a {} mm", "{} kierrosta {} mm:iin", "{} 次切削，深度至 {} mm", "{} पास, {} मिमी तक"],
    ["{}: no bit chosen", "{}: nie wybrano freza", "{}: kein Fräser gewählt", "{}: nessuna fresa scelta", "{}: terää ei ole valittu", "{}：未选择刀具", "{}: कोई बिट नहीं चुना गया"],
    ["{}: nothing to cut (the line may be too small for the bit)", "{}: nie ma czego ciąć (linia może być za mała dla freza)", "{}: nichts zu schneiden (die Linie ist vielleicht zu klein für den Fräser)", "{}: niente da tagliare (la linea potrebbe essere troppo piccola per la fresa)", "{}: ei mitään leikattavaa (viiva voi olla liian pieni terälle)", "{}：没有可切割的内容（线条对刀具来说可能太小）", "{}: काटने के लिए कुछ नहीं (रेखा बिट के लिए बहुत छोटी हो सकती है)"],
    ["{}: open lines are left out of a pocket", "{}: otwarte linie są pomijane w kieszeni", "{}: Offene Linien werden bei einer Tasche ausgelassen", "{}: le linee aperte sono escluse da una tasca", "{}: avoimet viivat jätetään pois taskusta", "{}：开放线在型腔中被忽略", "{}: पॉकेट में खुली रेखाएँ छोड़ दी जाती हैं"],
    ["{}: open lines are left out of a profile (use \"Follow the line\")", "{}: otwarte linie są pomijane w obrysie (użyj „Podążanie za linią”)", "{}: Offene Linien werden bei einem Profil ausgelassen (nutze „Der Linie folgen“)", "{}: le linee aperte sono escluse da un profilo (usa «Segui la linea»)", "{}: avoimet viivat jätetään pois ääriviivasta (käytä toimintoa ”Seuraa viivaa”)", "{}：开放线在轮廓切割中被忽略（请使用“沿线走”）", "{}: प्रोफ़ाइल में खुली रेखाएँ छोड़ दी जाती हैं (\"रेखा का अनुसरण करें\" का उपयोग करें)"],
    ["{}: pictures are not milled", "{}: obrazy nie są frezowane", "{}: Bilder werden nicht gefräst", "{}: le immagini non vengono fresate", "{}: kuvia ei jyrsitä", "{}：图片不会被铣削", "{}: चित्रों की मिलिंग नहीं होती"],
    ["{}: the depth is limited to the travel of the machine ({} mm)", "{}: głębokość jest ograniczona skokiem maszyny ({} mm)", "{}: Die Tiefe ist auf den Verfahrweg der Maschine begrenzt ({} mm)", "{}: la profondità è limitata alla corsa della macchina ({} mm)", "{}: syvyys on rajoitettu koneen liikematkaan ({} mm)", "{}：深度受机器行程限制（{} mm）", "{}: गहराई मशीन की यात्रा तक सीमित है ({} मिमी)"],
    ["{}: the depth of {} mm is more than the thickness of the material ({} mm)", "{}: głębokość {} mm jest większa niż grubość materiału ({} mm)", "{}: Die Tiefe von {} mm ist größer als die Dicke des Materials ({} mm)", "{}: la profondità di {} mm supera lo spessore del materiale ({} mm)", "{}: syvyys {} mm on suurempi kuin materiaalin paksuus ({} mm)", "{}：深度 {} mm 大于材料厚度（{} mm）", "{}: {} मिमी की गहराई सामग्री की मोटाई ({} मिमी) से अधिक है"],
    ["{}: {} rpm is outside the spindle range {}…{} rpm", "{}: {} obr./min jest poza zakresem wrzeciona {}…{} obr./min", "{}: {} U/min liegen außerhalb des Spindelbereichs {}…{} U/min", "{}: {} giri/min sono fuori dall'intervallo del mandrino {}…{} giri/min", "{}: {} rpm on karan alueen {}…{} rpm ulkopuolella", "{}：{} rpm 超出主轴范围 {}…{} rpm", "{}: {} आरपीएम स्पिंडल की सीमा {}…{} आरपीएम से बाहर है"],
];

#[cfg(test)]
mod tests {
    use super::*;
    use base_ui::i18n::{Lang, TABLE_LANGS};

    #[test]
    fn every_row_is_complete_and_keeps_placeholders() {
        let mut seen = std::collections::HashSet::new();
        for row in TABLE {
            assert!(seen.insert(row[0]), "duplicate key {:?}", row[0]);
            let n = row[0].matches("{}").count();
            for (i, t) in row.iter().enumerate() {
                assert!(!t.is_empty(), "empty translation {i} for {:?}", row[0]);
                assert_eq!(t.matches("{}").count(), n, "placeholder mismatch in {:?} (column {i})", row[0]);
            }
        }
    }

    /// `assets/lang/keys.txt` lists the English texts of `TABLE` in order; the other languages refer to it by id.
    #[test]
    fn keys_file_matches_the_table() {
        let keys: Vec<&str> = KEYS.lines().filter_map(|l| l.split_once('\t')).map(|(_, t)| t).collect();
        assert_eq!(keys.len(), TABLE.len());
        for (i, (k, row)) in keys.iter().zip(TABLE).enumerate() {
            assert_eq!(*k, row[0], "line {i} of keys.txt");
        }
    }

    #[test]
    fn every_language_file_is_complete_and_keeps_placeholders() {
        let english: Vec<&str> = KEYS.lines().filter_map(|l| l.split_once('\t')).map(|(_, t)| t).collect();
        for (n, file) in EXTRA_FILES.iter().enumerate() {
            let code = Lang::ALL[TABLE_LANGS + n].code();
            let mut seen = std::collections::HashSet::new();
            for line in file.lines().filter(|l| !l.is_empty()) {
                let (id, text) = line.split_once('|').unwrap_or_else(|| panic!("{code}: no separator in {line:?}"));
                let id: usize = id.parse().unwrap_or_else(|_| panic!("{code}: bad id in {line:?}"));
                assert!(seen.insert(id), "{code}: duplicate id {id}");
                let key = english.get(id).unwrap_or_else(|| panic!("{code}: unknown id {id}"));
                assert!(!text.trim().is_empty(), "{code}: empty text for {key:?}");
                assert_eq!(text.matches("{}").count(), key.matches("{}").count(), "{code}: placeholders in {key:?}");
            }
            assert_eq!(seen.len(), english.len(), "{code}: every text needs a translation");
        }
    }

    #[test]
    fn the_texts_translate() {
        let (pl, fr) = (Lang::Pl.index() as usize, Lang::Fr.index() as usize);
        let first = TABLE[0][0];
        assert_ne!(TEXTS.lookup(pl, first), None);
        assert_ne!(TEXTS.lookup(fr, first), None);
        assert_eq!(TEXTS.lookup(0, first), None, "English needs no lookup");
    }

    /// Every text that the code of this application passes to `tr`, `trf` or `Warn::new` is in the table or in the
    /// shared texts, so no language is left with an English text by mistake.
    #[test]
    fn every_text_in_the_code_has_a_translation() {
        let own: std::collections::HashSet<&str> = TABLE.iter().map(|r| r[0]).collect();
        let mut missing = vec![];
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|e| e != "rs") || path.ends_with("texts.rs") {
                continue;
            }
            let code = std::fs::read_to_string(&path).unwrap();
            let code = code.split("#[cfg(test)]\nmod tests").next().unwrap();
            for call in ["tr(\"", "trf(\"", "Warn::new(\""] {
                for (at, _) in code.match_indices(call) {
                    if code[..at].chars().next_back().is_some_and(|c| c.is_alphanumeric() || c == '_') && !call.starts_with("Warn") {
                        continue;
                    }
                    let part = &code[at + call.len()..];
                    let end = part.find('"').unwrap_or(0);
                    let text = &part[..end];
                    if part[..end].ends_with('\\') || text.is_empty() {
                        continue;
                    }
                    let shared = base_ui::texts::TEXTS.lookup(Lang::Pl.index() as usize, Box::leak(text.to_string().into_boxed_str())).is_some();
                    if !own.contains(text) && !shared && text != "{}" {
                        missing.push(format!("{}: {text}", path.file_name().unwrap().to_string_lossy()));
                    }
                }
            }
        }
        assert!(missing.is_empty(), "texts without a translation: {missing:#?}");
    }
}

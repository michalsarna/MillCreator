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
    ["3D view of the job…", "Widok 3D zadania…", "3D-Ansicht des Auftrags…", "Vista 3D del lavoro…", "Työn 3D-näkymä…", "作业 3D 视图…", "कार्य का 3D दृश्य…"],
    ["A copy among your own bits", "Kopia wśród Twoich frezów", "Eine Kopie unter deinen eigenen Fräsern", "Una copia tra le tue frese", "Kopio omien terien joukossa", "在你自己的刀具中的副本", "आपके अपने बिट्स में एक प्रति"],
    ["A hole at the centre of every closed shape of the layer, in steps of the pass depth.", "Otwór pośrodku każdego zamkniętego kształtu warstwy, kolejnymi przejściami o zadanej głębokości.", "Ein Loch in der Mitte jeder geschlossenen Form der Ebene, in Schritten der Zustelltiefe.", "Un foro al centro di ogni forma chiusa del livello, a passate della profondità indicata.", "Reikä jokaisen tason suljetun muodon keskelle, syöttösyvyyden askelin.", "在图层每个闭合图形的中心钻一个孔，按切深分步进行。", "लेयर के हर बंद आकार के केंद्र में एक छेद, पास की गहराई के चरणों में।"],
    ["A port name, or the USB vendor and product id (VID:PID, hexadecimal): the port that has these ids is used.", "Nazwa portu albo identyfikator USB producenta i produktu (VID:PID, szesnastkowo): używany jest port z tymi identyfikatorami.", "Ein Portname oder die USB-Hersteller- und Produkt-ID (VID:PID, hexadezimal): Es wird der Port mit diesen IDs verwendet.", "Il nome di una porta, oppure l'ID USB di produttore e prodotto (VID:PID, esadecimale): si usa la porta con questi ID.", "Portin nimi tai USB-valmistajan ja tuotteen tunnus (VID:PID, heksadesimaalina): käytetään porttia, jolla on nämä tunnukset.", "端口名称，或 USB 厂商与产品 ID（VID:PID，十六进制）：将使用具有这些 ID 的端口。", "पोर्ट का नाम, या USB विक्रेता और उत्पाद आईडी (VID:PID, हेक्साडेसिमल): इन आईडी वाला पोर्ट उपयोग होता है।"],
    ["A ready-made bit cannot be changed. Make a copy to change it.", "Gotowego freza nie można zmienić. Zrób kopię, żeby go zmienić.", "Ein fertiger Fräser kann nicht geändert werden. Erstelle eine Kopie, um ihn zu ändern.", "Una fresa predefinita non si può modificare. Fanne una copia per cambiarla.", "Valmista terää ei voi muuttaa. Tee kopio, jos haluat muuttaa sitä.", "内置刀具不能修改。请先复制再修改。", "तैयार बिट को बदला नहीं जा सकता। बदलने के लिए उसकी प्रति बनाएँ।"],
    ["A rotary attachment is connected", "Podłączona jest przystawka obrotowa", "Eine Rotationsachse ist angeschlossen", "È collegato un accessorio rotativo", "Pyörityslaite on kytketty", "已连接旋转附件", "घूर्णन अटैचमेंट जुड़ा है"],
    ["A usual machine size…", "Zwykły rozmiar maszyny…", "Eine übliche Maschinengröße…", "Una dimensione di macchina comune…", "Tavallinen koneen koko…", "常见的机器尺寸…", "एक सामान्य मशीन आकार…"],
    ["Abort the running job", "Przerwij wykonywaną pracę", "Laufenden Auftrag abbrechen", "Interrompi il lavoro in corso", "Keskeytä käynnissä oleva työ", "中止正在运行的作业", "चल रहे जॉब को रोकें"],
    ["About", "O programie", "Über", "Informazioni", "Tietoja", "关于", "के बारे में"],
    ["Add a camera", "Dodaj kamerę", "Kamera hinzufügen", "Aggiungi una telecamera", "Lisää kamera", "添加摄像头", "कैमरा जोड़ें"],
    ["Add a device", "Dodaj urządzenie", "Gerät hinzufügen", "Aggiungi un dispositivo", "Lisää laite", "添加设备", "डिवाइस जोड़ें"],
    ["Add device", "Dodaj urządzenie", "Gerät hinzufügen", "Aggiungi dispositivo", "Lisää laite", "添加设备", "डिवाइस जोड़ें"],
    ["Add device…", "Dodaj urządzenie…", "Gerät hinzufügen…", "Aggiungi dispositivo…", "Lisää laite…", "添加设备…", "डिवाइस जोड़ें…"],
    ["Address", "Adres", "Adresse", "Indirizzo", "Osoite", "地址", "पता"],
    ["After connecting", "Po połączeniu", "Nach dem Verbinden", "Dopo la connessione", "Yhdistämisen jälkeen", "连接后", "कनेक्ट होने के बाद"],
    ["Alarm {}", "Alarm {}", "Alarm {}", "Allarme {}", "Hälytys {}", "报警 {}", "अलार्म {}"],
    ["All of the area inside the line is cleared, ring after ring, to the depth of the layer.", "Cały obszar wewnątrz linii jest wybierany, pierścień po pierścieniu, do głębokości warstwy.", "Die gesamte Fläche innerhalb der Linie wird Ring für Ring bis zur Tiefe der Ebene ausgeräumt.", "Tutta l'area dentro la linea viene svuotata, anello dopo anello, fino alla profondità del livello.", "Koko viivan sisäpuolinen alue tyhjennetään rengas renkaalta tason syvyyteen asti.", "线内的整个区域按一圈一圈清除，直到图层的深度。", "रेखा के भीतर का पूरा क्षेत्र, छल्ले दर छल्ले, लेयर की गहराई तक साफ़ किया जाता है।"],
    ["Arrange", "Rozmieść", "Anordnen", "Disponi", "Järjestä", "排列", "व्यवस्थित करें"],
    ["Back left", "Tył po lewej", "Hinten links", "Dietro a sinistra", "Takana vasemmalla", "左后", "पीछे बाएँ"],
    ["Back right", "Tył po prawej", "Hinten rechts", "Dietro a destra", "Takana oikealla", "右后", "पीछे दाएँ"],
    ["Back to the first view. Double click on the picture does the same.", "Powrót do pierwszego widoku. Podwójne kliknięcie obrazu robi to samo.", "Zurück zur ersten Ansicht. Ein Doppelklick auf das Bild tut dasselbe.", "Torna alla prima vista. Il doppio clic sull'immagine fa lo stesso.", "Takaisin ensimmäiseen näkymään. Kuvan kaksoisnapsautus tekee saman.", "回到最初的视图。双击图片效果相同。", "पहले दृश्य पर वापस। चित्र पर डबल क्लिक भी यही करता है।"],
    ["Ball nose", "Frez kulisty", "Kugelfräser", "Fresa a testa sferica", "Pallopäinen jyrsin", "球头铣刀", "बॉल नोज़"],
    ["Baud rate", "Szybkość transmisji", "Baudrate", "Baud rate", "Siirtonopeus (baud)", "波特率", "बॉड दर"],
    ["Bind to this port", "Powiąż z tym portem", "An diesen Port binden", "Associa a questa porta", "Sido tähän porttiin", "绑定到此端口", "इस पोर्ट से बाँधें"],
    ["Bit", "Frez", "Fräser", "Fresa", "Terä", "刀具", "बिट"],
    ["Bit library", "Biblioteka frezów", "Fräserbibliothek", "Libreria di frese", "Terakirjasto", "刀具库", "बिट लाइब्रेरी"],
    ["Bit library…", "Biblioteka frezów…", "Fräserbibliothek…", "Libreria di frese…", "Terakirjasto…", "刀具库…", "बिट लाइब्रेरी…"],
    ["Bound USB device", "Powiązane urządzenie USB", "Gebundenes USB-Gerät", "Dispositivo USB associato", "Sidottu USB-laite", "已绑定的 USB 设备", "बँधा USB डिवाइस"],
    ["Bull nose (corner radius)", "Frez z zaokrąglonym narożem (promień naroża)", "Torusfräser (Eckenradius)", "Fresa torica (raggio d'angolo)", "Pyöristetty kulmajyrsin (kulmasäde)", "圆角铣刀（圆角半径）", "बुल नोज़ (कोने की त्रिज्या)"],
    ["Camera", "Kamera", "Kamera", "Fotocamera", "Kamera", "摄像头", "कैमरा"],
    ["Camera name", "Nazwa kamery", "Kameraname", "Nome della telecamera", "Kameran nimi", "摄像头名称", "कैमरे का नाम"],
    ["Camera number", "Numer kamery", "Kameranummer", "Numero della telecamera", "Kameran numero", "摄像头编号", "कैमरा संख्या"],
    ["Camera of this computer", "Kamera tego komputera", "Kamera dieses Computers", "Telecamera di questo computer", "Tämän tietokoneen kamera", "本机摄像头", "इस कंप्यूटर का कैमरा"],
    ["Camera rotation", "Obrót kamery", "Kameradrehung", "Rotazione fotocamera", "Kameran kierto", "摄像头旋转", "कैमरा रोटेशन"],
    ["Camera view", "Widok z kamery", "Kameraansicht", "Vista fotocamera", "Kameran näkymä", "摄像头画面", "कैमरा दृश्य"],
    ["Cameras", "Kamery", "Kameras", "Telecamere", "Kamerat", "摄像头", "कैमरे"],
    ["Cannot use the file", "Nie można użyć pliku", "Die Datei kann nicht verwendet werden", "Impossibile usare il file", "Tiedostoa ei voi käyttää", "无法使用该文件", "फ़ाइल का उपयोग नहीं किया जा सकता"],
    ["Check the program", "Sprawdź program", "Programm prüfen", "Controlla il programma", "Tarkista ohjelma", "检查程序", "प्रोग्राम जाँचें"],
    ["Checking the program first…", "Najpierw sprawdzam program…", "Programm wird zuerst geprüft…", "Prima controllo il programma…", "Tarkistetaan ohjelma ensin…", "先检查程序…", "पहले प्रोग्राम जाँच रहे हैं…"],
    ["Checking the program…", "Sprawdzam program…", "Programm wird geprüft…", "Controllo del programma…", "Tarkistetaan ohjelmaa…", "正在检查程序…", "प्रोग्राम जाँच रहे हैं…"],
    ["Choose a bit", "Wybierz frez", "Einen Fräser wählen", "Scegli una fresa", "Valitse terä", "选择刀具", "बिट चुनें"],
    ["Choose a port or a host in the device settings first.", "Najpierw wybierz port lub host w ustawieniach urządzenia.", "Wähle zuerst in den Geräteeinstellungen einen Port oder Host.", "Scegli prima una porta o un host nelle impostazioni del dispositivo.", "Valitse ensin portti tai isäntä laitteen asetuksissa.", "请先在设备设置中选择端口或主机。", "पहले डिवाइस सेटिंग्स में पोर्ट या होस्ट चुनें।"],
    ["Choose the machine you will work with.", "Wybierz maszynę, na której będziesz pracować.", "Wähle die Maschine, mit der du arbeiten willst.", "Scegli la macchina con cui lavorare.", "Valitse kone, jolla työskentelet.", "选择你要使用的机器。", "वह मशीन चुनें जिस पर आप काम करेंगे।"],
    ["Chuck", "Uchwyt", "Backenfutter", "Mandrino", "Istukka", "卡盘", "चक"],
    ["Circumference", "Obwód", "Umfang", "Circonferenza", "Ympärysmitta", "周长", "परिधि"],
    ["Clear an alarm", "Wyczyść alarm", "Alarm löschen", "Cancella un allarme", "Kuittaa hälytys", "清除报警", "अलार्म साफ़ करें"],
    ["Clearing the alarm skips homing: use it only when the head is known to stand in a safe place.", "Wyczyszczenie alarmu pomija bazowanie: używaj tylko, gdy wiadomo, że głowica stoi w bezpiecznym miejscu.", "Das Löschen des Alarms überspringt das Referenzieren: nur verwenden, wenn der Kopf sicher an einem sicheren Ort steht.", "Cancellare l'allarme salta l'azzeramento: usalo solo quando la testa si trova di sicuro in un punto sicuro.", "Hälytyksen kuittaus ohittaa referoinnin: käytä vain, kun pään tiedetään olevan turvallisessa paikassa.", "清除报警会跳过回零：仅在确知机头位于安全位置时使用。", "अलार्म साफ़ करना होमिंग छोड़ देता है: केवल तब उपयोग करें जब हेड सुरक्षित जगह पर हो।"],
    ["Confirm delete", "Potwierdź usunięcie", "Löschen bestätigen", "Conferma eliminazione", "Vahvista poisto", "确认删除", "हटाने की पुष्टि करें"],
    ["Connect", "Połącz", "Verbinden", "Connetti", "Yhdistä", "连接", "कनेक्ट"],
    ["Connect again automatically", "Łącz ponownie automatycznie", "Automatisch erneut verbinden", "Riconnetti automaticamente", "Yhdistä uudelleen automaattisesti", "自动重新连接", "अपने आप दोबारा कनेक्ट करें"],
    ["Connected", "Połączono", "Verbunden", "Collegato", "Yhdistetty", "已连接", "कनेक्टेड"],
    ["Connection", "Połączenie", "Verbindung", "Connessione", "Yhteys", "连接", "कनेक्शन"],
    ["Connection type", "Typ połączenia", "Verbindungsart", "Tipo di connessione", "Yhteyden tyyppi", "连接类型", "कनेक्शन प्रकार"],
    ["Console", "Konsola", "Konsole", "Console", "Konsoli", "控制台", "कंसोल"],
    ["Control", "Sterowanie", "Steuerung", "Controllo", "Ohjaus", "控制", "नियंत्रण"],
    ["Copied to {}", "Skopiowano do {}", "Kopiert nach {}", "Copiato in {}", "Kopioitu: {}", "已复制为 {}", "{} में कॉपी किया"],
    ["Corner radius", "Promień naroża", "Eckenradius", "Raggio d'angolo", "Kulmasäde", "圆角半径", "कोने की त्रिज्या"],
    ["Could not save", "Nie udało się zapisać", "Konnte nicht speichern", "Impossibile salvare", "Tallennus ei onnistunut", "无法保存", "सहेजा नहीं जा सका"],
    ["Could not save: {}", "Nie udało się zapisać: {}", "Konnte nicht speichern: {}", "Impossibile salvare: {}", "Tallennus ei onnistunut: {}", "无法保存：{}", "सहेजा नहीं जा सका: {}"],
    ["Counting fills the controller's buffer and is fast. Simple sends one line and waits for its ok: slow, but for firmware that answers wrongly.", "Zliczanie wypełnia bufor sterownika i jest szybkie. Prosty wysyła jedną linię i czeka na ok: wolny, ale dla firmware odpowiadającego błędnie.", "Zählen füllt den Puffer der Steuerung und ist schnell. Einfach sendet eine Zeile und wartet auf ihr ok: langsam, aber für Firmware, die falsch antwortet.", "Il conteggio riempie il buffer del controller ed è veloce. Semplice invia una riga e attende il suo ok: lento, ma per firmware che risponde male.", "Laskenta täyttää ohjaimen puskurin ja on nopea. Yksinkertainen lähettää yhden rivin ja odottaa sen ok:n: hidas, mutta väärin vastaavalle laiteohjelmistolle.", "计数会填满控制器缓冲区，速度快。简单模式发送一行并等待其 ok：较慢，但适用于回答错误的固件。", "गिनती कंट्रोलर का बफ़र भरती है और तेज़ है। सरल एक पंक्ति भेजकर उसके ok की प्रतीक्षा करता है: धीमा, पर गलत जवाब देने वाले फ़र्मवेयर के लिए।"],
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
    ["Diagnostics and statistics…", "Diagnostyka i statystyki…", "Diagnose und Statistik…", "Diagnostica e statistiche…", "Diagnostiikka ja tilastot…", "诊断和统计…", "निदान और आँकड़े…"],
    ["Disconnect", "Rozłącz", "Trennen", "Disconnetti", "Katkaise yhteys", "断开连接", "डिस्कनेक्ट"],
    ["Drill", "Wiertło", "Bohrer", "Punta da foratura", "Pora", "钻头", "ड्रिल"],
    ["Edit", "Edycja", "Bearbeiten", "Modifica", "Muokkaa", "编辑", "संपादन"],
    ["Edit…", "Edytuj…", "Bearbeiten…", "Modifica…", "Muokkaa…", "编辑…", "संपादित करें…"],
    ["Engraving bit", "Frez grawerski", "Gravierfräser", "Fresa per incisione", "Kaiverrusterä", "雕刻刀", "उत्कीर्णन बिट"],
    ["Error {} in line {}: {}", "Błąd {} w wierszu {}: {}", "Fehler {} in Zeile {}: {}", "Errore {} alla riga {}: {}", "Virhe {} rivillä {}: {}", "第 {} 行出现错误 {}：{}", "पंक्ति {} में त्रुटि {}: {}"],
    ["Error {}: {}", "Błąd {}: {}", "Fehler {}: {}", "Errore {}: {}", "Virhe {}: {}", "错误 {}：{}", "त्रुटि {}: {}"],
    ["Export G-code…", "Eksportuj G-code…", "G-Code exportieren…", "Esporta G-code…", "Vie G-koodi…", "导出 G-code…", "G-code निर्यात करें…"],
    ["Export this bit…", "Eksportuj ten frez…", "Diesen Fräser exportieren…", "Esporta questa fresa…", "Vie tämä terä…", "导出此刀具…", "यह बिट निर्यात करें…"],
    ["Export…", "Eksportuj…", "Exportieren…", "Esporta…", "Vie…", "导出…", "निर्यात…"],
    ["Feed", "Posuw", "Vorschub", "Avanzamento", "Syöttö", "进给", "फ़ीड"],
    ["Firmware settings ($$)…", "Ustawienia firmware ($$)…", "Firmware-Einstellungen ($$)…", "Impostazioni del firmware ($$)…", "Laiteohjelmiston asetukset ($$)…", "固件设置（$$）…", "फ़र्मवेयर सेटिंग ($$)…"],
    ["Flat end mill", "Frez palcowy", "Schaftfräser", "Fresa a candela", "Sormijyrsin", "平底铣刀", "फ़्लैट एंड मिल"],
    ["Flutes", "Liczba ostrzy", "Schneiden", "Taglienti", "Uurteet", "刃数", "फ़्लूट"],
    ["Follow the line", "Podążanie za linią", "Der Linie folgen", "Segui la linea", "Seuraa viivaa", "沿线走", "रेखा का अनुसरण करें"],
    ["Frame", "Obrys", "Rahmen", "Cornice", "Kehys", "走边框", "फ़्रेम"],
    ["Framing the work area", "Obrys obszaru pracy", "Arbeitsbereich abfahren", "Contorno dell'area di lavoro", "Työalueen kehystys", "给工作区域画框", "कार्य क्षेत्र का फ़्रेम"],
    ["From above", "Z góry", "Von oben", "Dall'alto", "Ylhäältä", "俯视", "ऊपर से"],
    ["From line…", "Od wiersza…", "Ab Zeile…", "Dalla riga…", "Riviltä…", "从行…", "पंक्ति से…"],
    ["Front left", "Przód po lewej", "Vorne links", "Davanti a sinistra", "Edessä vasemmalla", "左前", "आगे बाएँ"],
    ["Front right", "Przód po prawej", "Vorne rechts", "Davanti a destra", "Edessä oikealla", "右前", "आगे दाएँ"],
    ["GRBL 1.1 has three axes: the A axis is not available, Y is used.", "GRBL 1.1 ma trzy osie: oś A jest niedostępna, używane jest Y.", "GRBL 1.1 hat drei Achsen: die A-Achse ist nicht verfügbar, Y wird verwendet.", "GRBL 1.1 ha tre assi: l'asse A non è disponibile, si usa Y.", "GRBL 1.1:ssä on kolme akselia: A-akseli ei ole käytettävissä, käytetään Y:tä.", "GRBL 1.1 只有三个轴：A 轴不可用，改用 Y。", "GRBL 1.1 में तीन अक्ष हैं: A अक्ष उपलब्ध नहीं, Y उपयोग होता है।"],
    ["General", "Ogólne", "Allgemein", "Generale", "Yleiset", "常规", "सामान्य"],
    ["Go round the box where the work will be done", "Obejdź obrys miejsca, gdzie będzie wykonywana praca", "Fahre den Rahmen ab, in dem gearbeitet wird", "Percorri il riquadro dove verrà eseguito il lavoro", "Kierrä laatikko, jossa työ tehdään", "沿着将要加工的区域外框走一圈", "उस बॉक्स का चक्कर लगाएँ जहाँ काम होगा"],
    ["Help", "Pomoc", "Hilfe", "Aiuto", "Ohje", "帮助", "सहायता"],
    ["Highest speed", "Największa prędkość", "Höchste Drehzahl", "Velocità massima", "Suurin nopeus", "最高转速", "सबसे अधिक गति"],
    ["Home", "Bazuj", "Referenzfahrt", "Home", "Kotiutus", "回零", "होम"],
    ["Home the machine", "Bazuj maszynę", "Maschine referenzieren", "Azzera la macchina", "Referoi kone", "机器回零", "मशीन होम करें"],
    ["Host", "Host", "Host", "Host", "Isäntä", "主机", "होस्ट"],
    ["I have no device", "Nie mam urządzenia", "Ich habe kein Gerät", "Non ho un dispositivo", "Minulla ei ole laitetta", "我没有设备", "मेरे पास कोई डिवाइस नहीं है"],
    ["Import LightBurn project…", "Importuj projekt LightBurn…", "LightBurn-Projekt importieren…", "Importa progetto LightBurn…", "Tuo LightBurn-projekti…", "导入 LightBurn 项目…", "LightBurn प्रोजेक्ट आयात करें…"],
    ["Imported the device {}", "Zaimportowano urządzenie {}", "Gerät {} importiert", "Dispositivo {} importato", "Laite {} tuotu", "已导入设备 {}", "डिवाइस {} आयात किया गया"],
    ["Import…", "Importuj…", "Importieren…", "Importa…", "Tuo…", "导入…", "आयात…"],
    ["Kind", "Rodzaj", "Art", "Tipo", "Laji", "类型", "प्रकार"],
    ["Layers", "Warstwy", "Ebenen", "Livelli", "Tasot", "图层", "लेयर"],
    ["Left out", "Pominięte", "Ausgelassen", "Esclusi", "Pois jätetyt", "已忽略", "छोड़े गए"],
    ["Lowest speed", "Najmniejsza prędkość", "Niedrigste Drehzahl", "Velocità minima", "Pienin nopeus", "最低转速", "सबसे कम गति"],
    ["Machine zero (0,0)", "Zero maszyny (0,0)", "Maschinennullpunkt (0,0)", "Zero macchina (0,0)", "Koneen nollapiste (0,0)", "机器原点 (0,0)", "मशीन शून्य (0,0)"],
    ["Material", "Materiał", "Material", "Materiale", "Materiaali", "材料", "सामग्री"],
    ["Material of the workpiece", "Materiał obrabianego przedmiotu", "Material des Werkstücks", "Materiale del pezzo", "Työkappaleen materiaali", "工件材料", "वर्कपीस की सामग्री"],
    ["Metal", "Metal", "Metall", "Metallo", "Metalli", "金属", "धातु"],
    ["Microsteps of the motor for one full turn of what it drives (the chuck, or one roller).", "Mikrokroki silnika na pełny obrót tego, co napędza (uchwytu albo jednej rolki).", "Mikroschritte des Motors für eine volle Umdrehung dessen, was er antreibt (Futter oder eine Rolle).", "Microstep del motore per un giro completo di ciò che aziona (il mandrino o un rullo).", "Moottorin mikroaskeleet yhtä täyttä kierrosta kohti sen, mitä se käyttää (istukka tai yksi tela).", "电机转动其驱动对象（卡盘或一个滚轮）一整圈所需的微步数。", "मोटर के लिए उसके चलाए भाग (चक या एक रोलर) के एक पूरे घुमाव के माइक्रोस्टेप।"],
    ["Microsteps per turn", "Mikrokroków na obrót", "Mikroschritte je Umdrehung", "Microstep per giro", "Mikroaskelta kierrosta kohti", "每圈微步数", "प्रति घुमाव माइक्रोस्टेप"],
    ["Milled: taken into the toolpaths and the G-code", "Frezowana: wchodzi do ścieżek i do G-code", "Gefräst: wird in die Werkzeugwege und den G-Code übernommen", "Fresato: incluso nei percorsi e nel G-code", "Jyrsitään: mukaan työstöratoihin ja G-koodiin", "铣削：计入刀路和 G-code", "मिलिंग होती है: टूलपाथ और G-code में शामिल"],
    ["Milling", "Frezowanie", "Fräsen", "Fresatura", "Jyrsintä", "铣削", "मिलिंग"],
    ["Moves at the safe height", "Ruchy na bezpiecznej wysokości", "Bewegungen in sicherer Höhe", "Movimenti all'altezza di sicurezza", "Liikkeet turvakorkeudella", "安全高度上的移动", "सुरक्षित ऊँचाई पर चालें"],
    ["My bits", "Moje frezy", "Meine Fräser", "Le mie frese", "Omat terät", "我的刀具", "मेरे बिट्स"],
    ["New", "Nowy", "Neu", "Nuovo", "Uusi", "新建", "नया"],
    ["New bit", "Nowy frez", "Neuer Fräser", "Nuova fresa", "Uusi terä", "新建刀具", "नया बिट"],
    ["No bit", "Brak freza", "Kein Fräser", "Nessuna fresa", "Ei terää", "无刀具", "कोई बिट नहीं"],
    ["No device yet. Add one, or work without a machine.", "Nie ma jeszcze urządzenia. Dodaj je albo pracuj bez maszyny.", "Noch kein Gerät. Füge eines hinzu oder arbeite ohne Maschine.", "Ancora nessun dispositivo. Aggiungine uno o lavora senza macchina.", "Ei vielä laitetta. Lisää sellainen tai työskentele ilman konetta.", "还没有设备。添加一个，或不用机器工作。", "अभी कोई डिवाइस नहीं। एक जोड़ें, या बिना मशीन के काम करें।"],
    ["No port", "Brak portu", "Kein Port", "Nessuna porta", "Ei porttia", "无端口", "कोई पोर्ट नहीं"],
    ["Not an http or https address.", "To nie jest adres http ani https.", "Keine http- oder https-Adresse.", "Non è un indirizzo http o https.", "Ei ole http- tai https-osoite.", "不是 http 或 https 地址。", "यह http या https पता नहीं है।"],
    ["Notes", "Notatki", "Notizen", "Note", "Muistiinpanot", "笔记", "नोट"],
    ["Notes about this machine", "Notatki o tej maszynie", "Notizen zu dieser Maschine", "Note su questa macchina", "Muistiinpanoja tästä koneesta", "关于这台机器的备注", "इस मशीन के बारे में नोट्स"],
    ["Nothing here yet", "Jeszcze nic tu nie ma", "Noch nichts vorhanden", "Ancora niente qui", "Täällä ei ole vielä mitään", "这里还没有内容", "यहाँ अभी कुछ नहीं है"],
    ["Nothing selected", "Nic nie zaznaczono", "Nichts ausgewählt", "Nessuna selezione", "Ei valintaa", "未选择任何内容", "कुछ चयनित नहीं"],
    ["Only the colour of a plastic can be changed. The material and its thickness are shown in the 3D view and saved in the project.", "Tylko kolor plastiku można zmienić. Materiał i jego grubość widać w widoku 3D i zapisują się w projekcie.", "Nur die Farbe eines Kunststoffs kann geändert werden. Das Material und seine Dicke werden in der 3D-Ansicht gezeigt und im Projekt gespeichert.", "Si può cambiare solo il colore di una plastica. Il materiale e il suo spessore compaiono nella vista 3D e vengono salvati nel progetto.", "Vain muovin väriä voi muuttaa. Materiaali ja sen paksuus näkyvät 3D-näkymässä ja tallentuvat projektiin.", "只有塑料的颜色可以更改。材料及其厚度会显示在 3D 视图中并保存在项目里。", "केवल प्लास्टिक का रंग बदला जा सकता है। सामग्री और उसकी मोटाई 3D दृश्य में दिखती है और प्रोजेक्ट में सहेजी जाती है।"],
    ["Open a drawing or start an empty design.", "Otwórz rysunek albo zacznij pusty projekt.", "Öffne eine Zeichnung oder beginne mit einem leeren Entwurf.", "Apri un disegno o inizia un progetto vuoto.", "Avaa piirros tai aloita tyhjä suunnitelma.", "打开一幅图，或开始一个空白设计。", "कोई ड्रॉइंग खोलें या खाली डिज़ाइन शुरू करें।"],
    ["Open the link to the controller set in the device settings", "Otwórz łącze do sterownika ustawionego w ustawieniach urządzenia", "Die Verbindung zur Steuerung aus den Geräteeinstellungen öffnen", "Apri il collegamento al controller impostato nelle impostazioni del dispositivo", "Avaa yhteys laitteen asetuksissa määritettyyn ohjaimeen", "打开设备设置中所设控制器的连接", "डिवाइस सेटिंग्स में तय कंट्रोलर से लिंक खोलें"],
    ["Open the preview window", "Otwórz okno podglądu", "Vorschaufenster öffnen", "Apri la finestra di anteprima", "Avaa esikatseluikkuna", "打开预览窗口", "पूर्वावलोकन विंडो खोलें"],
    ["Open…", "Otwórz…", "Öffnen…", "Apri…", "Avaa…", "打开…", "खोलें…"],
    ["Operation", "Operacja", "Vorgang", "Operazione", "Toiminto", "操作", "ऑपरेशन"],
    ["Overall length", "Długość całkowita", "Gesamtlänge", "Lunghezza totale", "Kokonaispituus", "总长度", "कुल लंबाई"],
    ["Pass depth", "Głębokość przejścia", "Zustelltiefe", "Profondità di passata", "Syöttösyvyys", "每层切深", "पास की गहराई"],
    ["Pass depth (of the diameter)", "Głębokość przejścia (część średnicy)", "Zustelltiefe (vom Durchmesser)", "Profondità di passata (del diametro)", "Syöttösyvyys (halkaisijasta)", "每层切深（占直径的比例）", "पास की गहराई (व्यास का)"],
    ["Plastic", "Tworzywa", "Kunststoff", "Plastica", "Muovi", "塑料", "प्लास्टिक"],
    ["Plunge feed", "Posuw wcięcia", "Eintauchvorschub", "Avanzamento di affondo", "Pistosyöttö", "下刀进给", "प्लंज फ़ीड"],
    ["Pocket", "Kieszeń", "Tasche", "Tasca", "Tasku", "型腔", "पॉकेट"],
    ["Port", "Port", "Port", "Porta", "Portti", "端口", "पोर्ट"],
    ["Port or USB id", "Port lub identyfikator USB", "Port oder USB-ID", "Porta o ID USB", "Portti tai USB-tunnus", "端口或 USB ID", "पोर्ट या USB आईडी"],
    ["Preview", "Podgląd", "Vorschau", "Anteprima", "Esikatselu", "预览", "पूर्वावलोकन"],
    ["Preview…", "Podgląd…", "Vorschau…", "Anteprima…", "Esikatselu…", "预览…", "पूर्वावलोकन…"],
    ["Properties", "Właściwości", "Eigenschaften", "Proprietà", "Ominaisuudet", "属性", "गुण"],
    ["Put the selected shapes on this layer", "Umieść zaznaczone kształty na tej warstwie", "Die ausgewählten Formen auf diese Ebene legen", "Metti le forme selezionate su questo livello", "Siirrä valitut muodot tälle tasolle", "将所选图形放到此图层", "चुने हुए आकार इस लेयर पर रखें"],
    ["Read a device from a file", "Wczytaj urządzenie z pliku", "Ein Gerät aus einer Datei lesen", "Leggi un dispositivo da un file", "Lue laite tiedostosta", "从文件读取设备", "फ़ाइल से डिवाइस पढ़ें"],
    ["Read bits from a file", "Wczytaj frezy z pliku", "Fräser aus einer Datei lesen", "Leggi le frese da un file", "Lue terät tiedostosta", "从文件读取刀具", "फ़ाइल से बिट पढ़ें"],
    ["Ready", "Gotowe", "Bereit", "Pronto", "Valmis", "就绪", "तैयार"],
    ["Ready-made bits", "Gotowe frezy", "Fertige Fräser", "Frese predefinite", "Valmiit terät", "内置刀具", "तैयार बिट"],
    ["Reconnect", "Ponowne łączenie", "Neu verbinden", "Riconnessione", "Uudelleenyhdistys", "重新连接", "दोबारा कनेक्ट"],
    ["Refresh", "Odśwież", "Aktualisieren", "Aggiorna", "Päivitä", "刷新", "रीफ़्रेश"],
    ["Remember the USB ids of the chosen port: the port is found again by them when its name changes.", "Zapamiętaj identyfikatory USB wybranego portu: gdy zmieni się jego nazwa, port zostanie znaleziony po nich.", "Die USB-IDs des gewählten Ports merken: Ändert sich sein Name, wird er über sie wiedergefunden.", "Ricorda gli ID USB della porta scelta: se il suo nome cambia, la porta viene ritrovata tramite essi.", "Muista valitun portin USB-tunnukset: portti löytyy niillä, kun sen nimi muuttuu.", "记住所选端口的 USB ID：端口名称改变时可凭它们重新找到。", "चुने पोर्ट की USB आईडी याद रखें: नाम बदलने पर पोर्ट उनसे फिर मिल जाता है।"],
    ["Reset view", "Resetuj widok", "Ansicht zurücksetzen", "Reimposta la vista", "Palauta näkymä", "重置视图", "दृश्य रीसेट करें"],
    ["Resume", "Wznów", "Fortsetzen", "Riprendi", "Jatka", "继续", "फिर शुरू करें"],
    ["Roller diameter", "Średnica rolki", "Rollendurchmesser", "Diametro del rullo", "Telan halkaisija", "滚轮直径", "रोलर का व्यास"],
    ["Rollers", "Rolki", "Rollen", "Rulli", "Telat", "滚轮", "रोलर"],
    ["Rotary", "Obrotowy", "Rotation", "Rotativo", "Pyörivä", "旋转", "घूर्णन"],
    ["STOP", "STOP", "STOPP", "STOP", "PYSÄYTÄ", "急停", "रुकें"],
    ["Safe height", "Bezpieczna wysokość", "Sichere Höhe", "Altezza di sicurezza", "Turvakorkeus", "安全高度", "सुरक्षित ऊँचाई"],
    ["Save", "Zapisz", "Speichern", "Salva", "Tallenna", "保存", "सहेजें"],
    ["Save G-code", "Zapisz G-code", "G-Code speichern", "Salva G-code", "Tallenna G-koodi", "保存 G-code", "G-code सहेजें"],
    ["Save as…", "Zapisz jako…", "Speichern unter…", "Salva come…", "Tallenna nimellä…", "另存为…", "इस रूप में सहेजें…"],
    ["Save this device to a file", "Zapisz to urządzenie do pliku", "Dieses Gerät in einer Datei speichern", "Salva questo dispositivo in un file", "Tallenna tämä laite tiedostoon", "将此设备保存到文件", "इस डिवाइस को फ़ाइल में सहेजें"],
    ["Save your bits to a file", "Zapisz swoje frezy do pliku", "Deine Fräser in einer Datei speichern", "Salva le tue frese in un file", "Tallenna omat terät tiedostoon", "将你的刀具保存到文件", "अपने बिट्स को फ़ाइल में सहेजें"],
    ["Saved", "Zapisano", "Gespeichert", "Salvato", "Tallennettu", "已保存", "सहेजा गया"],
    ["Saved the device to {}", "Zapisano urządzenie do {}", "Gerät in {} gespeichert", "Dispositivo salvato in {}", "Laite tallennettu: {}", "设备已保存到 {}", "डिवाइस {} में सहेजा गया"],
    ["Saved {}", "Zapisano {}", "{} gespeichert", "Salvato {}", "Tallennettu {}", "已保存 {}", "{} सहेजा गया"],
    ["Settings", "Ustawienia", "Einstellungen", "Impostazioni", "Asetukset", "设置", "सेटिंग्स"],
    ["Shank diameter", "Średnica trzonka", "Schaftdurchmesser", "Diametro del gambo", "Varren halkaisija", "刀柄直径", "शैंक का व्यास"],
    ["Shapes on this layer", "Kształty na tej warstwie", "Formen auf dieser Ebene", "Forme su questo livello", "Muodot tällä tasolla", "此图层上的图形", "इस लेयर पर आकार"],
    ["Show the live camera picture of this machine", "Pokaż obraz na żywo z kamery tej maszyny", "Live-Kamerabild dieser Maschine anzeigen", "Mostra l'immagine dal vivo della fotocamera di questa macchina", "Näytä tämän koneen kameran suora kuva", "显示此机器的实时摄像头画面", "इस मशीन का लाइव कैमरा चित्र दिखाएँ"],
    ["Show the toolpaths on the work area", "Pokaż ścieżki na obszarze roboczym", "Werkzeugwege im Arbeitsbereich anzeigen", "Mostra i percorsi sull'area di lavoro", "Näytä työstöradat työalueella", "在工作区域显示刀路", "कार्य क्षेत्र पर टूलपाथ दिखाएँ"],
    ["Shown in the Camera tab", "Pokazywana w zakładce Kamera", "Im Reiter Kamera gezeigt", "Mostrata nella scheda Telecamera", "Näytetään Kamera-välilehdellä", "显示在“摄像头”标签页", "कैमरा टैब में दिखाया गया"],
    ["Shown in the work area", "Pokazana w obszarze roboczym", "Im Arbeitsbereich angezeigt", "Mostrato nell'area di lavoro", "Näytetään työalueella", "显示在工作区域", "कार्य क्षेत्र में दिखाया गया"],
    ["Speed of moves above the material", "Prędkość ruchów nad materiałem", "Geschwindigkeit der Bewegungen über dem Material", "Velocità dei movimenti sopra il materiale", "Liikkeiden nopeus materiaalin yläpuolella", "材料上方的移动速度", "सामग्री के ऊपर चालों की गति"],
    ["Spindle", "Wrzeciono", "Spindel", "Mandrino", "Kara", "主轴", "स्पिंडल"],
    ["Start", "Start", "Start", "Avvia", "Käynnistä", "开始", "शुरू"],
    ["Start from", "Start od", "Start von", "Parti da", "Aloituspiste", "起点", "यहाँ से शुरू करें"],
    ["Start the job at a line of the program", "Zacznij zadanie od wiersza programu", "Den Auftrag bei einer Zeile des Programms starten", "Avvia il lavoro da una riga del programma", "Aloita työ ohjelman riviltä", "从程序的某一行开始作业", "प्रोग्राम की एक पंक्ति से कार्य शुरू करें"],
    ["Starting job: {} lines, ~{}", "Start zadania: {} linii, ok. {}", "Auftrag startet: {} Zeilen, ca. {}", "Avvio lavoro: {} righe, ~{}", "Työn aloitus: {} riviä, ~{}", "开始任务：{} 行，约 {}", "जॉब शुरू: {} पंक्तियाँ, ~{}"],
    ["Starting speeds for a layer", "Prędkości początkowe dla warstwy", "Startwerte für eine Ebene", "Velocità iniziali per un livello", "Tason aloitusnopeudet", "图层的起始速度", "लेयर के लिए शुरुआती गतियाँ"],
    ["Step over", "Rozstaw przejść", "Seitliche Zustellung", "Sovrapposizione laterale", "Sivuttaissyöttö", "步距", "स्टेप ओवर"],
    ["Step over (of the diameter)", "Rozstaw przejść (część średnicy)", "Seitliche Zustellung (vom Durchmesser)", "Sovrapposizione laterale (del diametro)", "Sivuttaissyöttö (halkaisijasta)", "步距（占直径的比例）", "स्टेप ओवर (व्यास का)"],
    ["Stream the job to the connected machine", "Wyślij zadanie do podłączonej maszyny", "Den Auftrag an die verbundene Maschine senden", "Invia il lavoro alla macchina collegata", "Lähetä työ yhdistettyyn koneeseen", "把作业发送到已连接的机器", "जॉब को कनेक्टेड मशीन पर भेजें"],
    ["Streaming", "Strumieniowanie", "Streaming", "Streaming", "Suoratoisto", "串流", "स्ट्रीमिंग"],
    ["Switched on", "Włączona", "Eingeschaltet", "Attiva", "Päällä", "已开启", "चालू"],
    ["TCP port", "Port TCP", "TCP-Port", "Porta TCP", "TCP-portti", "TCP 端口", "TCP पोर्ट"],
    ["Tapered ball nose", "Frez kulisty stożkowy", "Konischer Kugelfräser", "Fresa conica a testa sferica", "Kartiomainen pallopäinen jyrsin", "锥形球头铣刀", "टेपर्ड बॉल नोज़"],
    ["The G-code around the job: {}", "G-code wokół zadania: {}", "Der G-Code rund um den Auftrag: {}", "Il G-code attorno al lavoro: {}", "Työn ympärillä oleva G-koodi: {}", "作业前后的 G 代码：{}", "कार्य के आसपास का G-कोड: {}"],
    ["The bit goes round the inside of the line, so the shape falls out: a hole or a window.", "Frez idzie po wewnętrznej stronie linii, więc kształt wypada: otwór lub okno.", "Der Fräser läuft innen an der Linie entlang, die Form fällt also heraus: ein Loch oder ein Fenster.", "La fresa passa all'interno della linea, quindi la forma cade: un foro o una finestra.", "Terä kulkee viivan sisäpuolella, joten muoto putoaa pois: reikä tai ikkuna.", "刀具沿线的内侧走，所以图形会掉下来：形成孔或窗口。", "बिट रेखा के भीतर से चलता है, इसलिए आकार गिर जाता है: छेद या खिड़की।"],
    ["The bit goes round the outside of the line, so the shape stays: a part is cut out of the material.", "Frez idzie po zewnętrznej stronie linii, więc kształt zostaje: część jest wycinana z materiału.", "Der Fräser läuft außen an der Linie entlang, die Form bleibt also stehen: ein Teil wird aus dem Material geschnitten.", "La fresa passa all'esterno della linea, quindi la forma resta: un pezzo viene ritagliato dal materiale.", "Terä kulkee viivan ulkopuolella, joten muoto jää: kappale leikataan materiaalista.", "刀具沿线的外侧走，所以图形保留：从材料中切出一个零件。", "बिट रेखा के बाहर से चलता है, इसलिए आकार रहता है: सामग्री से एक पुर्ज़ा काटा जाता है।"],
    ["The centre of the bit follows the line: engraving and grooves. Open lines work too.", "Środek freza podąża za linią: grawerowanie i rowki. Otwarte linie też działają.", "Die Mitte des Fräsers folgt der Linie: Gravuren und Nuten. Offene Linien funktionieren auch.", "Il centro della fresa segue la linea: incisioni e scanalature. Funzionano anche le linee aperte.", "Terän keskipiste seuraa viivaa: kaiverrukset ja urat. Avoimet viivat toimivat myös.", "刀具中心沿线走：雕刻和开槽。开放线也可以。", "बिट का केंद्र रेखा का अनुसरण करता है: उत्कीर्णन और खाँचे। खुली रेखाएँ भी काम करती हैं।"],
    ["The check found nothing wrong.", "Sprawdzanie nie znalazło nic złego.", "Die Prüfung hat nichts Falsches gefunden.", "Il controllo non ha trovato nulla di sbagliato.", "Tarkistus ei löytänyt mitään virheellistä.", "检查没有发现问题。", "जाँच में कुछ ग़लत नहीं मिला।"],
    ["The check found nothing. The job is running.", "Sprawdzanie nic nie znalazło. Zadanie trwa.", "Die Prüfung hat nichts gefunden. Der Auftrag läuft.", "Il controllo non ha trovato nulla. Il lavoro è in esecuzione.", "Tarkistus ei löytänyt mitään. Työ on käynnissä.", "检查没有发现问题。作业正在运行。", "जाँच में कुछ नहीं मिला। कार्य चल रहा है।"],
    ["The check found {} problems, the first in line {}. The job was not started.", "Sprawdzanie znalazło {} problemów, pierwszy w wierszu {}. Zadania nie uruchomiono.", "Die Prüfung hat {} Probleme gefunden, das erste in Zeile {}. Der Auftrag wurde nicht gestartet.", "Il controllo ha trovato {} problemi, il primo alla riga {}. Il lavoro non è stato avviato.", "Tarkistus löysi {} ongelmaa, ensimmäisen riviltä {}. Työtä ei aloitettu.", "检查发现 {} 个问题，第一个在第 {} 行。作业未启动。", "जाँच में {} समस्याएँ मिलीं, पहली पंक्ति {} में। कार्य शुरू नहीं हुआ।"],
    ["The controller needs {} steps per mm on the Y axis ($101).", "Sterownik potrzebuje {} kroków na mm na osi Y ($101).", "Die Steuerung braucht {} Schritte pro mm auf der Y-Achse ($101).", "Il controller richiede {} passi per mm sull'asse Y ($101).", "Ohjain tarvitsee {} askelta millimetriä kohti Y-akselilla ($101).", "控制器需要 Y 轴每毫米 {} 步 ($101)。", "कंट्रोलर को Y अक्ष पर {} स्टेप प्रति मिमी चाहिए ($101)।"],
    ["The cuts go outside of the work area of the device", "Cięcia wychodzą poza obszar roboczy urządzenia", "Die Schnitte liegen außerhalb des Arbeitsbereichs des Geräts", "I tagli escono dall'area di lavoro del dispositivo", "Leikkaukset ulottuvat laitteen työalueen ulkopuolelle", "切割超出了设备的工作区域", "कट डिवाइस के कार्य क्षेत्र से बाहर जाते हैं"],
    ["The device of this project", "Urządzenie tego projektu", "Das Gerät dieses Projekts", "Il dispositivo di questo progetto", "Tämän projektin laite", "此项目的设备", "इस प्रोजेक्ट का डिवाइस"],
    ["The device was added.", "Dodano urządzenie.", "Das Gerät wurde hinzugefügt.", "Il dispositivo è stato aggiunto.", "Laite lisättiin.", "设备已添加。", "डिवाइस जोड़ा गया।"],
    ["The device was saved.", "Zapisano urządzenie.", "Das Gerät wurde gespeichert.", "Il dispositivo è stato salvato.", "Laite tallennettiin.", "设备已保存。", "डिवाइस सहेजा गया।"],
    ["The drawing is higher than the way round the workpiece ({} mm)", "Rysunek jest wyższy niż obwód przedmiotu ({} mm)", "Die Zeichnung ist höher als der Umfang des Werkstücks ({} mm)", "Il disegno è più alto della circonferenza del pezzo ({} mm)", "Piirros on korkeampi kuin kappaleen ympärysmitta ({} mm)", "图形高度超过工件周长（{} mm）", "ड्रॉइंग वर्कपीस की परिधि ({} मिमी) से ऊँची है"],
    ["The file is too large", "Plik jest zbyt duży", "Die Datei ist zu groß", "Il file è troppo grande", "Tiedosto on liian suuri", "文件太大", "फ़ाइल बहुत बड़ी है"],
    ["The job is finished, with {} problems (see the Console).", "Zadanie zakończone, z {} problemami (patrz Konsola).", "Der Auftrag ist beendet, mit {} Problemen (siehe Konsole).", "Il lavoro è terminato, con {} problemi (vedi la Console).", "Työ on valmis, {} ongelmalla (katso Konsoli).", "作业已完成，有 {} 个问题（见控制台）。", "कार्य पूरा हुआ, {} समस्याओं के साथ (कंसोल देखें)।"],
    ["The job is finished.", "Zadanie zakończone.", "Der Auftrag ist beendet.", "Il lavoro è terminato.", "Työ on valmis.", "作业已完成。", "जॉब पूरा हुआ।"],
    ["The job is paused: {}", "Zadanie wstrzymane: {}", "Der Auftrag ist angehalten: {}", "Il lavoro è in pausa: {}", "Työ on keskeytetty: {}", "作业已暂停：{}", "कार्य रुका हुआ है: {}"],
    ["The job is running.", "Zadanie trwa.", "Der Auftrag läuft.", "Il lavoro è in corso.", "Työ on käynnissä.", "作业正在运行。", "जॉब चल रहा है।"],
    ["The measurement at the sensor failed: no contact, or an alarm.", "Pomiar na czujniku nie powiódł się: brak kontaktu albo alarm.", "Die Messung am Sensor ist fehlgeschlagen: kein Kontakt oder ein Alarm.", "La misura al sensore non è riuscita: nessun contatto o un allarme.", "Mittaus anturilla epäonnistui: ei kosketusta tai hälytys.", "在传感器上的测量失败：没有接触，或发生了报警。", "सेंसर पर माप विफल: कोई संपर्क नहीं, या अलार्म।"],
    ["The reference tool was measured: Z {} at the sensor.", "Zmierzono narzędzie wzorcowe: Z {} przy czujniku.", "Das Referenzwerkzeug wurde gemessen: Z {} am Sensor.", "L'utensile di riferimento è stato misurato: Z {} al sensore.", "Vertailutyökalu mitattiin: Z {} anturilla.", "基准工具已测量：传感器处 Z {}。", "संदर्भ औज़ार नापा गया: सेंसर पर Z {}।"],
    ["The tool was measured; there is no reference to compare with yet.", "Narzędzie zmierzono; nie ma jeszcze wzorca do porównania.", "Das Werkzeug wurde gemessen; es gibt noch keine Referenz zum Vergleichen.", "L'utensile è stato misurato; non c'è ancora un riferimento con cui confrontarlo.", "Työkalu mitattiin; vertailua ei vielä ole.", "工具已测量；还没有可供比较的基准。", "औज़ार नापा गया; तुलना के लिए अभी कोई संदर्भ नहीं है।"],
    ["The way round the workpiece: the design may not be higher.", "Obwód przedmiotu: projekt nie może być wyższy.", "Der Umfang des Werkstücks: das Design darf nicht höher sein.", "La circonferenza del pezzo: il disegno non può essere più alto.", "Kappaleen ympärysmitta: kuva ei saa olla korkeampi.", "工件的周长：图案不能比它高。", "वस्तु की परिधि: डिज़ाइन इससे ऊँचा नहीं हो सकता।"],
    ["There is nothing to cut: put shapes on layers that have a bit.", "Nie ma czego wyciąć: połóż kształty na warstwach, które mają frez.", "Es gibt nichts zu schneiden: Lege Formen auf Ebenen mit einem Fräser.", "Non c'è nulla da tagliare: metti le forme su livelli con una fresa.", "Ei ole mitään leikattavaa: laita muodot tasoille, joilla on terä.", "没有可切割的内容：请把图形放到有刀具的图层上。", "काटने के लिए कुछ नहीं है: आकार उन लेयर पर रखें जिनमें बिट हो।"],
    ["There is nothing to frame: put shapes on layers that have a bit.", "Nie ma czego obrysować: połóż kształty na warstwach, które mają frez.", "Nichts zum Abfahren: Lege Formen auf Ebenen mit einem Fräser.", "Non c'è nulla da contornare: metti le forme su livelli con una fresa.", "Ei ole mitään kehystettävää: laita muodot tasoille, joilla on terä.", "没有可描边的内容：请把图形放到有刀具的图层上。", "फ़्रेम करने के लिए कुछ नहीं: आकार उन लेयर पर रखें जिनमें बिट हो।"],
    ["There is nothing to show yet: draw shapes and put them on layers that have a bit.", "Nie ma jeszcze czego pokazać: narysuj kształty i połóż je na warstwach, które mają frez.", "Es gibt noch nichts zu zeigen: Zeichne Formen und lege sie auf Ebenen mit einem Fräser.", "Non c'è ancora nulla da mostrare: disegna delle forme e mettile su livelli con una fresa.", "Ei ole vielä mitään näytettävää: piirrä muotoja ja laita ne tasoille, joilla on terä.", "还没有可显示的内容：请绘制图形并放到有刀具的图层上。", "अभी दिखाने के लिए कुछ नहीं है: आकार बनाएँ और उन्हें उन लेयर पर रखें जिनमें बिट हो।"],
    ["Thickness", "Grubość", "Dicke", "Spessore", "Paksuus", "厚度", "मोटाई"],
    ["This build cannot use the cameras of the computer.", "Ta wersja programu nie może używać kamer komputera.", "Diese Version kann die Kameras des Computers nicht verwenden.", "Questa versione non può usare le telecamere del computer.", "Tämä versio ei voi käyttää tietokoneen kameroita.", "此版本不能使用电脑的摄像头。", "यह संस्करण कंप्यूटर के कैमरों का उपयोग नहीं कर सकता।"],
    ["This device has no camera that is switched on. Add one in the device settings.", "To urządzenie nie ma włączonej kamery. Dodaj ją w ustawieniach urządzenia.", "Dieses Gerät hat keine eingeschaltete Kamera. Füge in den Geräteeinstellungen eine hinzu.", "Questo dispositivo non ha una telecamera attiva. Aggiungila nelle impostazioni del dispositivo.", "Tällä laitteella ei ole päällä olevaa kameraa. Lisää sellainen laitteen asetuksissa.", "此设备没有开启的摄像头。请在设备设置中添加。", "इस डिवाइस में कोई चालू कैमरा नहीं है। डिवाइस सेटिंग्स में एक जोड़ें।"],
    ["This device is not in the list: it came with the project", "Tego urządzenia nie ma na liście: przyszło z projektem", "Dieses Gerät ist nicht in der Liste: Es kam mit dem Projekt", "Questo dispositivo non è nell'elenco: è arrivato con il progetto", "Tämä laite ei ole luettelossa: se tuli projektin mukana", "此设备不在列表中：它随项目而来", "यह डिवाइस सूची में नहीं है: यह प्रोजेक्ट के साथ आया"],
    ["This is the stand-in device (Dummy): the default work area and speeds.", "To urządzenie zastępcze (Dummy): domyślny obszar roboczy i prędkości.", "Das ist das Ersatzgerät (Dummy): Standard-Arbeitsbereich und -Geschwindigkeiten.", "Questo è il dispositivo sostitutivo (Dummy): area di lavoro e velocità predefinite.", "Tämä on varalaite (Dummy): oletustyöalue ja -nopeudet.", "这是替代设备（Dummy）：默认的工作区域和速度。", "यह स्थानापन्न डिवाइस (Dummy) है: डिफ़ॉल्ट कार्य क्षेत्र और गतियाँ।"],
    ["Time {} / {}", "Czas {} / {}", "Zeit {} / {}", "Tempo {} / {}", "Aika {} / {}", "时间 {} / {}", "समय {} / {}"],
    ["Tip diameter", "Średnica końcówki", "Spitzendurchmesser", "Diametro della punta", "Kärjen halkaisija", "刀尖直径", "नोक का व्यास"],
    ["Too many devices", "Zbyt wiele urządzeń", "Zu viele Geräte", "Troppi dispositivi", "Liian monta laitetta", "设备太多", "बहुत अधिक डिवाइस"],
    ["Toolpaths", "Ścieżki", "Werkzeugwege", "Percorsi", "Työstöradat", "刀路", "टूलपाथ"],
    ["Type", "Rodzaj", "Typ", "Tipo", "Tyyppi", "类型", "प्रकार"],
    ["Unbind", "Rozłącz powiązanie", "Bindung lösen", "Dissocia", "Pura sidonta", "解除绑定", "बंधन हटाएँ"],
    ["Units", "Jednostki", "Einheiten", "Unità", "Yksiköt", "单位", "इकाइयाँ"],
    ["V-bit", "Frez V", "V-Fräser", "Fresa a V", "V-jyrsin", "V 形刀", "V-बिट"],
    ["View", "Widok", "Ansicht", "Vista", "Näytä", "视图", "दृश्य"],
    ["View options…", "Opcje widoku…", "Ansichtsoptionen…", "Opzioni di visualizzazione…", "Näkymän asetukset…", "视图选项…", "दृश्य विकल्प…"],
    ["Wait after switching on", "Czekanie po włączeniu", "Wartezeit nach dem Einschalten", "Attesa dopo l'accensione", "Odotus päällekytkennän jälkeen", "开启后的等待时间", "चालू करने के बाद प्रतीक्षा"],
    ["Waiting for the picture…", "Oczekiwanie na obraz…", "Warte auf das Bild…", "In attesa dell'immagine…", "Odotetaan kuvaa…", "正在等待画面…", "चित्र की प्रतीक्षा…"],
    ["When the link drops by itself, try again every few seconds until you press Disconnect.", "Gdy połączenie samo się zerwie, próbuj co kilka sekund, aż naciśniesz Rozłącz.", "Reißt die Verbindung von selbst ab, alle paar Sekunden erneut versuchen, bis du auf Trennen drückst.", "Se il collegamento cade da solo, riprova ogni pochi secondi finché non premi Disconnetti.", "Kun yhteys katkeaa itsestään, yritä uudelleen muutaman sekunnin välein, kunnes painat Katkaise.", "连接自行断开时，每隔几秒重试，直到你点击“断开”。", "लिंक अपने आप टूटे तो हर कुछ सेकंड में फिर कोशिश करें, जब तक आप डिस्कनेक्ट न दबाएँ।"],
    ["With a rotary attachment the Y axis turns the work: the drawing may not be higher than the way round the workpiece. The model is simple (a chuck turns by the workpiece diameter, rollers move the surface like their own); a real machine needs testing.", "Przy obrotnicy oś Y obraca przedmiot: rysunek nie może być wyższy niż obwód przedmiotu. Model jest prosty (uchwyt obraca się o średnicę przedmiotu, rolki przesuwają powierzchnię jak własną); prawdziwą maszynę trzeba przetestować.", "Mit einer Rotationsachse dreht die Y-Achse das Werkstück: Die Zeichnung darf nicht höher sein als der Umfang des Werkstücks. Das Modell ist einfach (ein Futter dreht sich um den Werkstückdurchmesser, Rollen bewegen die Oberfläche wie ihre eigene); eine echte Maschine muss getestet werden.", "Con un attacco rotativo l'asse Y fa girare il pezzo: il disegno non può essere più alto della circonferenza del pezzo. Il modello è semplice (un mandrino ruota del diametro del pezzo, i rulli muovono la superficie come la propria); una macchina reale va provata.", "Pyörityslaitteella Y-akseli pyörittää kappaletta: piirros ei saa olla korkeampi kuin kappaleen ympärysmitta. Malli on yksinkertainen (istukka pyörii kappaleen halkaisijan verran, rullat liikuttavat pintaa kuin omaansa); oikea kone on testattava.", "使用旋转附件时，Y 轴转动工件：图形高度不能超过工件周长。该模型很简单（卡盘按工件直径转动，滚轮使表面像自己的表面一样移动）；真实机器需要测试。", "रोटरी अटैचमेंट के साथ Y अक्ष वर्कपीस को घुमाता है: ड्रॉइंग वर्कपीस की परिधि से ऊँची नहीं हो सकती। मॉडल सरल है (चक एक वर्कपीस व्यास से घूमता है, रोलर सतह को अपनी सतह की तरह चलाते हैं); असली मशीन पर परीक्षण ज़रूरी है।"],
    ["Wood, dark", "Drewno, ciemne", "Holz, dunkel", "Legno, scuro", "Puu, tumma", "木材，深色", "लकड़ी, गहरी"],
    ["Wood, light", "Drewno, jasne", "Holz, hell", "Legno, chiaro", "Puu, vaalea", "木材，浅色", "लकड़ी, हल्की"],
    ["Wood, medium", "Drewno, średnie", "Holz, mittel", "Legno, medio", "Puu, keskisävyinen", "木材，中等", "लकड़ी, मध्यम"],
    ["Work on a project without a machine: a plain device called Dummy, with the default work area and speeds", "Pracuj nad projektem bez maszyny: zwykłe urządzenie o nazwie Dummy z domyślnym polem roboczym i prędkościami", "An einem Projekt ohne Maschine arbeiten: ein einfaches Gerät namens Dummy mit der Standard-Arbeitsfläche und den Standardgeschwindigkeiten", "Lavora a un progetto senza macchina: un dispositivo semplice chiamato Dummy, con l'area di lavoro e le velocità predefinite", "Työskentele projektin parissa ilman konetta: tavallinen laite nimeltä Dummy oletustyöalueella ja oletusnopeuksilla", "无需机器即可处理项目：一台名为 Dummy 的普通设备，使用默认工作区域和速度", "मशीन के बिना प्रोजेक्ट पर काम करें: Dummy नाम का एक सामान्य डिवाइस, डिफ़ॉल्ट कार्य क्षेत्र और गति के साथ"],
    ["Workpiece diameter", "Średnica przedmiotu", "Werkstückdurchmesser", "Diametro del pezzo", "Kappaleen halkaisija", "工件直径", "वस्तु का व्यास"],
    ["Z is how far the spindle goes down from the top of the material. The safe height is where the spindle moves between cuts.", "Z to jak głęboko wrzeciono schodzi od góry materiału. Bezpieczna wysokość to miejsce, gdzie wrzeciono przemieszcza się między cięciami.", "Z ist, wie tief die Spindel von der Oberseite des Materials nach unten fährt. Die sichere Höhe ist die Höhe, auf der sich die Spindel zwischen den Schnitten bewegt.", "Z indica quanto scende il mandrino dalla superficie del materiale. L'altezza di sicurezza è quella a cui il mandrino si muove tra un taglio e l'altro.", "Z kertoo, kuinka syvälle kara menee materiaalin yläpinnasta. Turvakorkeus on korkeus, jolla kara liikkuu leikkausten välillä.", "Z 是主轴从材料顶面向下走的深度。安全高度是切割之间主轴移动时所在的高度。", "Z यह है कि स्पिंडल सामग्री के ऊपर से कितना नीचे जाता है। सुरक्षित ऊँचाई वह है जहाँ स्पिंडल कटों के बीच चलता है।"],
    ["Z was set from the reference.", "Z ustawiono według wzorca.", "Z wurde aus der Referenz gesetzt.", "Z è stato impostato dal riferimento.", "Z asetettiin vertailun mukaan.", "已根据基准设置 Z。", "Z संदर्भ से सेट किया गया।"],
    ["bits added", "dodane frezy", "Fräser hinzugefügt", "frese aggiunte", "terät lisätty", "已添加刀具", "बिट जोड़े गए"],
    ["bits added to the library", "frezy dodane do biblioteki", "Fräser zur Bibliothek hinzugefügt", "frese aggiunte alla libreria", "terät lisätty kirjastoon", "刀具已添加到刀具库", "बिट लाइब्रेरी में जोड़े गए"],
    ["e.g. My CNC router", "np. Moja frezarka CNC", "z. B. Meine CNC-Fräse", "es. La mia fresatrice CNC", "esim. Oma CNC-jyrsin", "例如：我的 CNC 雕刻机", "उदा. मेरा CNC राउटर"],
    ["none", "brak", "keins", "nessuno", "ei mitään", "无", "कोई नहीं"],
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
    ["Import UGS design…", "Importuj projekt UGS…", "UGS-Entwurf importieren…", "Importa progetto UGS…", "Tuo UGS-suunnitelma…", "导入 UGS 设计…", "UGS डिज़ाइन आयात करें…"],
    ["First pass", "Pierwsze przejście", "Erster Durchgang", "Prima passata", "Ensimmäinen kierros", "首道加工", "पहला पास"],
    ["Start with a shallow pass along the edges", "Zacznij od płytkiego przejścia wzdłuż krawędzi", "Mit einem flachen Durchgang entlang der Kanten beginnen", "Inizia con una passata poco profonda lungo i bordi", "Aloita matalalla kierroksella reunoja pitkin", "先沿边缘走一道浅层加工", "किनारों के साथ एक उथले पास से शुरू करें"],
    ["Every layer first goes once along the edges it will cut, only this deep, which breaks the top of the material cleanly. Pockets get it along their edge only; drilling has none. It works with every bit.", "Każda warstwa najpierw przechodzi raz wzdłuż krawędzi, które będzie ciąć, tylko na tę głębokość, co czysto przełamuje wierzch materiału. Kieszenie dostają je tylko wzdłuż swojej krawędzi; wiercenie go nie ma. Działa z każdym frezem.", "Jede Ebene fährt zuerst einmal die Kanten ab, die sie schneiden wird, nur so tief; das durchtrennt die Oberseite des Materials sauber. Taschen bekommen ihn nur entlang ihres Rands; beim Bohren entfällt er. Er funktioniert mit jedem Fräser.", "Ogni livello percorre prima una volta i bordi che taglierà, solo a questa profondità, il che rompe nettamente la parte superiore del materiale. Le tasche la ricevono solo lungo il bordo; la foratura non ce l'ha. Funziona con qualsiasi fresa.", "Jokainen taso kulkee ensin kerran leikkaamiaan reunoja pitkin vain tämän syvyyden, mikä katkaisee materiaalin pinnan siististi. Taskut saavat sen vain reunaansa pitkin; poraus ei saa. Toimii jokaisella terällä.", "每个图层会先沿着它将要切削的边缘走一遍，只切这么深，从而干净地划开材料表面。型腔只沿其边缘进行；钻孔没有此步骤。适用于任何刀具。", "हर लेयर पहले एक बार उन किनारों के साथ चलती है जिन्हें वह काटेगी, केवल इतनी गहराई तक, जिससे सामग्री की ऊपरी परत साफ़ टूट जाती है। पॉकेट को यह केवल उनके किनारे के साथ मिलता है; ड्रिलिंग में यह नहीं होता। यह हर बिट के साथ काम करता है।"],
    ["Depth of the first pass", "Głębokość pierwszego przejścia", "Tiefe des ersten Durchgangs", "Profondità della prima passata", "Ensimmäisen kierroksen syvyys", "首道加工的深度", "पहले पास की गहराई"],
    ["Picture", "Obraz", "Bild", "Immagine", "Kuva", "图片", "चित्र"],
    ["Dithered rows", "Rastrowane wiersze", "Gerasterte Zeilen", "Righe retinate", "Rasteroidut rivit", "抖动行", "डिथर की गई पंक्तियाँ"],
    ["Dots", "Kropki", "Punkte", "Punti", "Pisteet", "点", "बिंदु"],
    ["Picture method", "Sposób rysowania obrazu", "Bildmethode", "Metodo dell'immagine", "Kuvan piirtotapa", "图片方式", "चित्र की विधि"],
    ["Dithering", "Rastrowanie", "Rasterung", "Retinatura", "Rasterointi", "抖动", "डिथरिंग"],
    ["Scan angle", "Kąt skanowania", "Scanwinkel", "Angolo di scansione", "Skannauskulma", "扫描角度", "स्कैन कोण"],
    ["A picture on the layer is engraved: rows or dots by the dithered picture, or rows whose depth follows the tone.", "Obraz na warstwie jest grawerowany: wiersze lub kropki według rastrowanego obrazu albo wiersze o głębokości zależnej od tonu.", "Ein Bild auf der Ebene wird graviert: Zeilen oder Punkte nach dem gerasterten Bild oder Zeilen, deren Tiefe dem Tonwert folgt.", "Un'immagine sul livello viene incisa: righe o punti secondo l'immagine retinata, oppure righe con profondità che segue il tono.", "Tason kuva kaiverretaan: rivit tai pisteet rasteroidun kuvan mukaan tai rivit, joiden syvyys seuraa sävyä.", "图层上的图片会被雕刻：按抖动图片的行或点，或深度随色调变化的行。", "लेयर पर चित्र उकेरा जाता है: डिथर किए चित्र के अनुसार पंक्तियाँ या बिंदु, या ऐसी पंक्तियाँ जिनकी गहराई स्वर के अनुसार बदलती है।"],
    ["Depth by tone", "Głębokość według tonu", "Tiefe nach Tonwert", "Profondità per tono", "Syvyys sävyn mukaan", "按色调定深度", "स्वर के अनुसार गहराई"],
    ["The picture is dithered and each row is cut at the depth of the layer where it is dark. The dithering method is chosen below.", "Obraz jest rastrowany, a każdy wiersz jest frezowany na głębokość warstwy tam, gdzie jest ciemny. Metodę rastrowania wybierasz niżej.", "Das Bild wird gerastert, und jede Zeile wird dort, wo es dunkel ist, auf die Tiefe der Ebene gefräst. Das Rasterverfahren wählst du unten.", "L'immagine viene retinata e ogni riga è fresata alla profondità del livello dove è scura. Il metodo di retinatura si sceglie sotto.", "Kuva rasteroidaan ja jokainen rivi jyrsitään tason syvyyteen siellä, missä kuva on tumma. Rasterointitavan valitset alla.", "图片经过抖动处理，每一行在较暗处按图层深度铣削。抖动方法在下方选择。", "चित्र को डिथर किया जाता है और हर पंक्ति वहाँ लेयर की गहराई तक काटी जाती है जहाँ वह गहरा है। डिथरिंग विधि नीचे चुनें।"],
    ["A hole to the depth of the layer for every dark cell of the dithered picture.", "Otwór na głębokość warstwy dla każdej ciemnej komórki rastrowanego obrazu.", "Ein Loch bis zur Tiefe der Ebene für jede dunkle Zelle des gerasterten Bildes.", "Un foro alla profondità del livello per ogni cella scura dell'immagine retinata.", "Reikä tason syvyyteen jokaiselle rasteroidun kuvan tummalle solulle.", "抖动后图片的每个暗单元钻一个深度为图层深度的孔。", "डिथर किए चित्र के हर गहरे सेल के लिए लेयर की गहराई तक एक छेद।"],
    ["The rows are cut with a depth that follows the tone of the picture: the darker, the deeper, down to the depth of the layer.", "Wiersze są frezowane na głębokość zależną od tonu obrazu: im ciemniej, tym głębiej, aż do głębokości warstwy.", "Die Zeilen werden mit einer Tiefe gefräst, die dem Tonwert des Bildes folgt: je dunkler, desto tiefer, bis zur Tiefe der Ebene.", "Le righe sono fresate con una profondità che segue il tono dell'immagine: più è scuro, più è profondo, fino alla profondità del livello.", "Rivit jyrsitään syvyydellä, joka seuraa kuvan sävyä: mitä tummempi, sitä syvempi, tason syvyyteen asti.", "各行的铣削深度随图片色调变化：越暗越深，最深到图层深度。", "पंक्तियाँ चित्र के स्वर के अनुसार गहराई तक काटी जाती हैं: जितना गहरा, उतनी अधिक गहराई, लेयर की गहराई तक।"],
    ["The angle of the rows of the engraving", "Kąt wierszy grawerowania", "Der Winkel der Gravurzeilen", "L'angolo delle righe dell'incisione", "Kaiverruksen rivien kulma", "雕刻行的角度", "उकेरने की पंक्तियों का कोण"],
    ["{}: the program of the picture is too long: the rest was left out (use a bigger step over)", "{}: program obrazu jest za długi: reszta została pominięta (użyj większego kroku)", "{}: Das Programm des Bildes ist zu lang: der Rest wurde weggelassen (größere Überlappung verwenden)", "{}: il programma dell'immagine è troppo lungo: il resto è stato omesso (usa un passo maggiore)", "{}: kuvan ohjelma on liian pitkä: loppu jätettiin pois (käytä suurempaa limitystä)", "{}：图片的程序过长，其余部分已省略（请使用更大的步距）", "{}: चित्र का प्रोग्राम बहुत लंबा है: बाकी छोड़ दिया गया (बड़ा स्टेप-ओवर उपयोग करें)"],
    ["3D relief", "Relief 3D", "3D-Relief", "Rilievo 3D", "3D-reliefi", "3D 浮雕", "3D उभार"],
    ["A relief: black is the deepest, white is left uncut. Passes of the pass depth down to the full depth, and the shape of the bit is taken into account. The model can be exported as STL.", "Relief: czerń jest najgłębsza, biel zostaje nietknięta. Przejścia o głębokości przejścia aż do pełnej głębokości, z uwzględnieniem kształtu freza. Model można wyeksportować jako STL.", "Ein Relief: Schwarz ist am tiefsten, Weiß bleibt unbearbeitet. Durchgänge in Zustelltiefe bis zur vollen Tiefe, die Form des Fräsers wird berücksichtigt. Das Modell kann als STL exportiert werden.", "Un rilievo: il nero è il più profondo, il bianco resta intatto. Passate della profondità di passata fino alla profondità piena, tenendo conto della forma della fresa. Il modello può essere esportato come STL.", "Reliefi: musta on syvin, valkoinen jää koskematta. Syöttösyvyyden kierrokset täyteen syvyyteen asti, ja terän muoto otetaan huomioon. Malli voidaan viedä STL-tiedostona.", "浮雕：黑色最深，白色保持不动。按每次进给深度逐层加工直至全深，并考虑刀具形状。模型可导出为 STL。", "उभार: काला सबसे गहरा, सफ़ेद अछूता रहता है। पास गहराई के पास पूरी गहराई तक, और बिट का आकार ध्यान में रखा जाता है। मॉडल STL के रूप में निर्यात किया जा सकता है।"],
    ["Export relief as STL…", "Eksportuj relief jako STL…", "Relief als STL exportieren…", "Esporta il rilievo come STL…", "Vie reliefi STL-tiedostona…", "将浮雕导出为 STL…", "उभार को STL के रूप में निर्यात करें…"],
    ["There is no relief to export: put a picture on a layer with the Picture operation and the 3D relief method.", "Nie ma reliefu do eksportu: połóż obraz na warstwie z operacją Picture i metodą 3D relief.", "Es gibt kein Relief zum Exportieren: Lege ein Bild auf eine Ebene mit der Operation Bild und der Methode 3D-Relief.", "Non c'è nessun rilievo da esportare: metti un'immagine su un livello con l'operazione Immagine e il metodo Rilievo 3D.", "Vietävää reliefiä ei ole: aseta kuva tasolle, jolla on toiminto Kuva ja menetelmä 3D-reliefi.", "没有可导出的浮雕：请把图片放到操作为“图片”、方式为“3D 浮雕”的图层上。", "निर्यात के लिए कोई उभार नहीं है: चित्र को ऐसी लेयर पर रखें जिसका ऑपरेशन चित्र और तरीका 3D उभार हो।"],
    ["Fit to the work area", "Dopasuj do obszaru roboczego", "An den Arbeitsbereich anpassen", "Adatta all'area di lavoro", "Sovita työalueeseen", "适应工作区域", "कार्य क्षेत्र में फ़िट करें"],
    ["Fit to the elements", "Dopasuj do elementów", "An die Elemente anpassen", "Adatta agli elementi", "Sovita elementteihin", "适应元素", "तत्वों में फ़िट करें"],
    ["Zoom so that the whole work area of the machine is seen", "Powiększ tak, by widać było cały obszar roboczy maszyny", "So zoomen, dass der ganze Arbeitsbereich der Maschine zu sehen ist", "Ingrandisci in modo da vedere tutta l'area di lavoro della macchina", "Zoomaa niin, että koneen koko työalue näkyy", "缩放到能看到机器的整个工作区域", "इस तरह ज़ूम करें कि मशीन का पूरा कार्य क्षेत्र दिखे"],
    ["Zoom so that all the cuts fill the window", "Powiększ tak, by wszystkie cięcia wypełniły okno", "So zoomen, dass alle Schnitte das Fenster füllen", "Ingrandisci in modo che tutti i tagli riempiano la finestra", "Zoomaa niin, että kaikki leikkaukset täyttävät ikkunan", "缩放到所有切削填满窗口", "इस तरह ज़ूम करें कि सभी कट विंडो भर दें"],
    ["One element at a time", "Element po elemencie", "Ein Element nach dem anderen", "Un elemento alla volta", "Yksi elementti kerrallaan", "逐个元素进行", "एक-एक तत्व"],
    ["Each element (a group, or a shape that is in none) is cut completely, all its layers and all their passes, before the next one; the nearest element goes first. Off: layer after layer, pass after pass.", "Każdy element (grupa albo kształt bez grupy) jest wycinany w całości, ze wszystkimi warstwami i przejściami, zanim zacznie się następny; pierwszy jest najbliższy. Wyłączone: warstwa po warstwie, przejście po przejściu.", "Jedes Element (eine Gruppe oder eine Form ohne Gruppe) wird ganz gefräst, mit allen Ebenen und allen Durchgängen, bevor das nächste kommt; das nächstgelegene zuerst. Aus: Ebene für Ebene, Durchgang für Durchgang.", "Ogni elemento (un gruppo o una forma senza gruppo) viene fresato per intero, con tutti i livelli e tutti i passaggi, prima del successivo; il più vicino per primo. Disattivato: livello dopo livello, passaggio dopo passaggio.", "Jokainen elementti (ryhmä tai muoto ilman ryhmää) jyrsitään kokonaan kaikkine tasoineen ja kierroksineen ennen seuraavaa; lähin ensin. Pois: taso kerrallaan, kierros kerrallaan.", "每个元素（一个组，或不在任何组中的图形）先完整切削其所有图层和所有道次，再进行下一个；最近的元素优先。关闭时：逐图层、逐道次进行。", "हर तत्व (एक समूह, या बिना समूह वाला आकार) अपनी सभी परतों और पासों के साथ पूरा काटा जाता है, फिर अगला; सबसे नज़दीकी पहले। बंद: परत-दर-परत, पास-दर-पास।"],
    ["Selection only", "Tylko zaznaczenie", "Nur Auswahl", "Solo selezione", "Vain valinta", "仅选中项", "केवल चयन"],
    ["Only the shapes that are selected on the work area when the G-code is saved or the job is started are cut.", "Wycinane są tylko kształty zaznaczone na obszarze roboczym w chwili zapisu G-code lub startu zadania.", "Nur die Formen, die beim Speichern des G-Codes oder beim Start des Auftrags im Arbeitsbereich ausgewählt sind, werden gefräst.", "Vengono fresate solo le forme selezionate nell'area di lavoro quando si salva il G-code o si avvia il lavoro.", "Jyrsitään vain työalueella valitut muodot, kun G-koodi tallennetaan tai työ aloitetaan.", "仅切削在保存 G 代码或开始任务时于工作区中被选中的图形。", "केवल वे आकार काटे जाते हैं जो G-कोड सहेजते या कार्य शुरू करते समय कार्य क्षेत्र में चुने हों।"],
    ["There is nothing to cut: nothing is selected", "Nie ma czego wyciąć: nic nie jest zaznaczone", "Nichts zu fräsen: nichts ist ausgewählt", "Niente da fresare: nulla è selezionato", "Ei jyrsittävää: mitään ei ole valittu", "无可切削内容：未选中任何内容", "काटने के लिए कुछ नहीं: कुछ भी चयनित नहीं है"],
    ["Export the milled piece as STL…", "Eksportuj sfrezowany element jako STL…", "Gefrästes Teil als STL exportieren…", "Esporta il pezzo fresato come STL…", "Vie jyrsitty kappale STL-tiedostona…", "将铣削后的工件导出为 STL…", "मिल किए गए टुकड़े को STL के रूप में निर्यात करें…"],
    ["Nothing to export: the job has no cuts yet.", "Nie ma czego eksportować: zadanie nie ma jeszcze żadnych cięć.", "Nichts zu exportieren: Der Auftrag hat noch keine Schnitte.", "Niente da esportare: il lavoro non ha ancora tagli.", "Ei mitään vietävää: työssä ei ole vielä leikkauksia.", "没有可导出的内容：任务尚无切削。", "निर्यात करने के लिए कुछ नहीं: कार्य में अभी कोई कट नहीं है।"],
    ["G-code", "G-code", "G-Code", "G-code", "G-koodi", "G 代码", "G-कोड"],
    ["Work coordinates", "Współrzędne robocze", "Werkstückkoordinaten", "Coordinate di lavoro", "Työkoordinaatit", "工件坐标", "कार्य निर्देशांक"],
    ["Zero of the program", "Zero programu", "Nullpunkt des Programms", "Zero del programma", "Ohjelman nollapiste", "程序零点", "प्रोग्राम का शून्य बिंदु"],
    ["The program selects this work coordinate system (G54 … G59, whose zero is set on the machine, for example by touching the material) and counts from the zero below, given in the coordinates of the work area. None: the program counts from the machine zero.", "Program wybiera ten układ współrzędnych roboczych (G54 … G59, którego zero ustawia się na maszynie, np. dotykając materiału) i liczy od podanego niżej zera, w współrzędnych obszaru roboczego. Brak: program liczy od zera maszyny.", "Das Programm wählt dieses Werkstückkoordinatensystem (G54 … G59, dessen Nullpunkt an der Maschine gesetzt wird, z. B. durch Antasten des Materials) und zählt vom unten angegebenen Nullpunkt in Koordinaten des Arbeitsbereichs. Keines: Das Programm zählt vom Maschinennullpunkt.", "Il programma sceglie questo sistema di coordinate di lavoro (G54 … G59, il cui zero si imposta sulla macchina, ad esempio toccando il materiale) e conta dallo zero indicato sotto, nelle coordinate dell'area di lavoro. Nessuno: il programma conta dallo zero macchina.", "Ohjelma valitsee tämän työkoordinaatiston (G54 … G59, jonka nolla asetetaan koneella esimerkiksi koskettamalla materiaalia) ja laskee alla olevasta nollasta työalueen koordinaateissa. Ei mitään: ohjelma laskee koneen nollasta.", "程序选择此工件坐标系（G54 … G59，其零点在机床上设定，例如通过接触材料），并从下面给出的零点开始计数，该零点以工作区坐标表示。无：程序从机床零点开始计数。", "प्रोग्राम यह कार्य निर्देशांक प्रणाली (G54 … G59, जिसका शून्य मशीन पर तय किया जाता है, जैसे सामग्री को छूकर) चुनता है और नीचे दिए शून्य से गिनता है, जो कार्य क्षेत्र के निर्देशांकों में दिया है। कोई नहीं: प्रोग्राम मशीन के शून्य से गिनता है।"],
    ["Own G-code is added to every program, one command per line (at most 20 lines).", "Własny G-code jest dodawany do każdego programu, jedno polecenie w wierszu (najwyżej 20 wierszy).", "Eigener G-Code wird jedem Programm hinzugefügt, ein Befehl pro Zeile (höchstens 20 Zeilen).", "Il G-code personale viene aggiunto a ogni programma, un comando per riga (al massimo 20 righe).", "Oma G-koodi lisätään jokaiseen ohjelmaan, yksi komento riviä kohti (enintään 20 riviä).", "自定义 G 代码会添加到每个程序中，每行一条命令（最多 20 行）。", "अपना G-कोड हर प्रोग्राम में जोड़ा जाता है, एक पंक्ति में एक कमांड (अधिकतम 20 पंक्तियाँ)।"],
    ["At the start of the program", "Na początku programu", "Am Anfang des Programms", "All'inizio del programma", "Ohjelman alussa", "程序开始处", "प्रोग्राम की शुरुआत में"],
    ["At every change of the bit", "Przy każdej zmianie freza", "Bei jedem Fräserwechsel", "A ogni cambio di fresa", "Joka terän vaihdolla", "每次更换刀具时", "हर बिट बदलने पर"],
    ["At every change of the operation", "Przy każdej zmianie operacji", "Bei jedem Wechsel der Operation", "A ogni cambio di operazione", "Joka operaation vaihdolla", "每次更换工序时", "हर ऑपरेशन बदलने पर"],
    ["At the end of the program", "Na końcu programu", "Am Ende des Programms", "Alla fine del programma", "Ohjelman lopussa", "程序结束处", "प्रोग्राम के अंत में"],
    ["Direction", "Kierunek", "Richtung", "Direzione", "Suunta", "方向", "दिशा"],
    ["As drawn", "Jak narysowano", "Wie gezeichnet", "Come disegnato", "Kuten piirretty", "按绘制方向", "जैसा बना है"],
    ["Climb", "Współbieżne", "Gleichlauf", "Concorde", "Myötäjyrsintä", "顺铣", "क्लाइम्ब"],
    ["Conventional", "Przeciwbieżne", "Gegenlauf", "Discorde", "Vastajyrsintä", "逆铣", "पारंपरिक"],
    ["Stock to leave", "Naddatek", "Aufmaß", "Sovrametallo", "Työvara", "预留余量", "छोड़ने की सामग्री"],
    ["Finishing pass", "Przejście wykańczające", "Schlichtdurchgang", "Passata di finitura", "Viimeistelykierros", "精加工走刀", "फिनिशिंग पास"],
    ["Entry", "Wejście", "Einstieg", "Ingresso", "Sisäänmeno", "进刀方式", "प्रवेश"],
    ["Plunge", "Wbicie", "Eintauchen", "Tuffo", "Upotus", "直插", "प्लंज"],
    ["Zigzag ramp", "Rampa zygzakiem", "Zickzack-Rampe", "Rampa a zigzag", "Siksak-ramppi", "锯齿斜坡", "ज़िगज़ैग रैंप"],
    ["Ramp along the contour", "Rampa po konturze", "Rampe entlang der Kontur", "Rampa lungo il contorno", "Ramppi ääriviivaa pitkin", "沿轮廓斜坡", "आकृति के साथ रैंप"],
    ["Helix", "Helisa", "Helix", "Elica", "Heliksi", "螺旋", "हेलिक्स"],
    ["Ramp angle", "Kąt rampy", "Rampenwinkel", "Angolo della rampa", "Rampin kulma", "斜坡角度", "रैंप का कोण"],
    ["Ramp feed", "Posuw rampy", "Rampenvorschub", "Avanzamento in rampa", "Rampin syöttö", "斜坡进给", "रैंप फ़ीड"],
    ["0 = the plunge feed", "0 = posuw wbicia", "0 = Vorschub des Eintauchens", "0 = avanzamento del tuffo", "0 = upotussyöttö", "0 = 直插进给", "0 = प्लंज फ़ीड"],
    ["Tabs", "Mostki", "Haltestege", "Ponticelli", "Pitokielekkeet", "连接片", "टैब"],
    ["Tab type", "Typ mostka", "Art des Stegs", "Tipo di ponticello", "Pitokielekkeen tyyppi", "连接片类型", "टैब का प्रकार"],
    ["Full", "Pełny", "Vollständig", "Completo", "Täysi", "完整", "पूर्ण"],
    ["Partial", "Częściowy", "Teilweise", "Parziale", "Osittainen", "部分", "आंशिक"],
    ["Ramp", "Rampa", "Rampe", "Rampa", "Ramppi", "斜坡", "रैंप"],
    ["Tab width", "Szerokość mostka", "Stegbreite", "Larghezza del ponticello", "Pitokielekkeen leveys", "连接片宽度", "टैब की चौड़ाई"],
    ["Tab height", "Wysokość mostka", "Steghöhe", "Altezza del ponticello", "Pitokielekkeen korkeus", "连接片高度", "टैब की ऊँचाई"],
    ["Tab slope", "Nachylenie mostka", "Neigung des Stegs", "Pendenza del ponticello", "Pitokielekkeen kaltevuus", "连接片坡度", "टैब की ढलान"],
    ["Tab spacing", "Odstęp mostków", "Stegabstand", "Distanza tra i ponticelli", "Pitokielekkeiden väli", "连接片间距", "टैब की दूरी"],
    ["Fewest and most tabs", "Najmniej i najwięcej mostków", "Wenigste und meiste Stege", "Minimo e massimo di ponticelli", "Vähiten ja eniten pitokielekkeitä", "最少和最多连接片数", "टैब की न्यूनतम और अधिकतम संख्या"],
    ["No tabs on holes", "Bez mostków na otworach", "Keine Stege an Löchern", "Niente ponticelli sui fori", "Ei pitokielekkeitä rei'issä", "孔上不设连接片", "छेदों पर टैब नहीं"],
    ["{}: a helix did not fit somewhere: the ramp along the contour was used there", "{}: gdzieś nie zmieściła się helisa: użyto tam rampy po konturze", "{}: eine Helix passte irgendwo nicht: dort wurde die Rampe entlang der Kontur verwendet", "{}: da qualche parte l'elica non ci stava: lì è stata usata la rampa lungo il contorno", "{}: heliksi ei mahtunut jossakin: siellä käytettiin ramppia ääriviivaa pitkin", "{}：某处螺旋进刀放不下，该处改用沿轮廓斜坡", "{}: कहीं हेलिक्स नहीं समाया: वहाँ आकृति के साथ रैंप इस्तेमाल हुआ"],
    ["{}: some rings are too short for all their tabs: they have fewer", "{}: niektóre kontury są za krótkie na wszystkie swoje mostki: mają ich mniej", "{}: einige Ringe sind zu kurz für alle ihre Stege: sie haben weniger", "{}: alcuni anelli sono troppo corti per tutti i loro ponticelli: ne hanno meno", "{}: jotkin renkaat ovat liian lyhyitä kaikille pitokielekkeilleen: niissä on vähemmän", "{}：有些环太短，放不下全部连接片，数量已减少", "{}: कुछ घेरे अपने सभी टैब के लिए बहुत छोटे हैं: उनमें कम टैब हैं"],
    ["{}: the ramp angle of {}° is steep (more than 45°)", "{}: kąt rampy {}° jest stromy (więcej niż 45°)", "{}: der Rampenwinkel von {}° ist steil (mehr als 45°)", "{}: l'angolo della rampa di {}° è ripido (più di 45°)", "{}: rampin kulma {}° on jyrkkä (yli 45°)", "{}：斜坡角度 {}° 过陡（超过 45°）", "{}: रैंप का कोण {}° तीखा है (45° से अधिक)"],
    ["{}: the tabs are as high as the cut: the part is not cut free", "{}: mostki są tak wysokie jak cięcie: część nie zostanie odcięta", "{}: die Stege sind so hoch wie der Schnitt: das Teil wird nicht freigeschnitten", "{}: i ponticelli sono alti quanto il taglio: il pezzo non viene staccato", "{}: pitokielekkeet ovat yhtä korkeita kuin leikkaus: kappaletta ei leikata irti", "{}：连接片与切深一样高：零件不会被切断", "{}: टैब कट जितने ऊँचे हैं: भाग अलग नहीं कटेगा"],
    ["Place tabs by hand", "Umieść mostki ręcznie", "Stege von Hand setzen", "Posiziona i ponticelli a mano", "Aseta pitokielekkeet käsin", "手动放置连接片", "टैब हाथ से रखें"],
    ["Click on the outline to add a tab, drag a tab to move it, click a tab to take it away. A ring that has tabs placed by hand gets only those.", "Kliknij na obrys, aby dodać mostek, przeciągnij mostek, aby go przesunąć, kliknij mostek, aby go usunąć. Kontur z mostkami ustawionymi ręcznie dostaje tylko je.", "Klicken Sie auf die Kontur, um einen Steg hinzuzufügen, ziehen Sie einen Steg, um ihn zu verschieben, klicken Sie einen Steg an, um ihn zu entfernen. Ein Ring mit von Hand gesetzten Stegen bekommt nur diese.", "Fai clic sul contorno per aggiungere un ponticello, trascinalo per spostarlo, fai clic su di esso per rimuoverlo. Un anello con ponticelli posizionati a mano ha solo quelli.", "Lisää pitokielekkeen napsauttamalla ääriviivaa, siirrä vetämällä, poista napsauttamalla. Rengas, jolla on käsin asetettuja pitokielekkeitä, saa vain ne.", "点击轮廓添加连接片，拖动连接片可移动，点击连接片可删除。带有手动放置连接片的环只使用这些连接片。", "आकृति की रेखा पर क्लिक करके टैब जोड़ें, टैब को खींचकर हिलाएँ, टैब पर क्लिक करके हटाएँ। जिस घेरे पर हाथ से टैब रखे हों, उसे केवल वही मिलते हैं।"],
    ["Clear hand-placed tabs", "Usuń mostki ustawione ręcznie", "Von Hand gesetzte Stege löschen", "Cancella i ponticelli posizionati a mano", "Poista käsin asetetut pitokielekkeet", "清除手动放置的连接片", "हाथ से रखे टैब हटाएँ"],
    ["Import DXF…", "Importuj DXF…", "DXF importieren…", "Importa DXF…", "Tuo DXF…", "导入 DXF…", "DXF आयात करें…"],
    ["Face", "Planowanie", "Planfräsen", "Spianatura", "Tasoitus", "面铣", "फेसिंग"],
    ["Bore", "Rozwiercanie", "Aufbohren", "Alesatura", "Avartaminen", "镗孔", "बोर"],
    ["Chamfer", "Fazowanie", "Fase", "Smusso", "Viiste", "倒角", "चैम्फर"],
    ["Dogbone slot", "Rowek z narożnikami dogbone", "Nut mit Dogbone-Ecken", "Scanalatura con angoli dogbone", "Ura koiranluu-kulmilla", "狗骨槽", "डॉगबोन स्लॉट"],
    ["The top of the material is planed over the box of the shapes of the layer; the bit goes a little over the edge.", "Wierzch materiału jest planowany na obszarze obejmującym kształty warstwy; frez wychodzi trochę poza krawędź.", "Die Oberseite des Materials wird über dem Rahmen der Formen der Ebene plan gefräst; der Fräser geht ein Stück über den Rand.", "La superficie del materiale viene spianata sul riquadro delle forme del livello; la fresa va un po' oltre il bordo.", "Materiaalin yläpinta tasoitetaan tason muotojen laatikon alueelta; terä menee hieman reunan yli.", "在图层形状的包围框范围内铣平材料表面；刀具略微越过边缘。", "सामग्री की ऊपरी सतह को परत के आकारों के बॉक्स पर समतल किया जाता है; बिट किनारे से थोड़ा बाहर जाता है।"],
    ["A round hole bigger than the bit: the bit goes down round it in a spiral and then once round at the full depth.", "Okrągły otwór większy od freza: frez schodzi dookoła spiralą, a potem raz okrąża otwór na pełnej głębokości.", "Ein rundes Loch, größer als der Fräser: Der Fräser geht spiralförmig rundherum nach unten und läuft dann einmal in voller Tiefe um.", "Un foro tondo più grande della fresa: la fresa scende in spirale lungo il foro e poi fa un giro alla profondità piena.", "Pyöreä reikä, joka on terää suurempi: terä laskeutuu sen ympäri kierteenä ja kiertää sitten kerran täydellä syvyydellä.", "比刀具大的圆孔：刀具沿孔螺旋下降，然后在全深度上走一圈。", "बिट से बड़ा गोल छेद: बिट इसके चारों ओर सर्पिल में नीचे जाता है और फिर पूरी गहराई पर एक चक्कर लगाता है।"],
    ["The edge of the shapes is chamfered: a V-bit follows the line at the depth that makes the chamfer as wide as asked.", "Krawędź kształtów jest fazowana: frez V idzie po linii na głębokości, która daje fazę o żądanej szerokości.", "Die Kante der Formen wird gefast: Ein V-Fräser folgt der Linie in der Tiefe, die die Fase so breit macht wie gewünscht.", "Il bordo delle forme viene smussato: una fresa a V segue la linea alla profondità che dà allo smusso la larghezza richiesta.", "Muotojen reuna viistetään: V-terä seuraa viivaa syvyydellä, joka tekee viisteestä halutun levyisen.", "对形状的边缘倒角：V 形刀沿线走刀，深度使倒角达到所需宽度。", "आकारों के किनारे पर चैम्फर बनता है: V-बिट रेखा पर उस गहराई पर चलता है जिससे चैम्फर माँगी चौड़ाई का बने।"],
    ["The bit goes round the inside of the line and into every sharp corner, so that a round bit leaves room for a square part.", "Frez idzie wewnątrz linii i wchodzi w każdy ostry narożnik, żeby po okrągłym frezie zostało miejsce na kwadratową część.", "Der Fräser geht innen an der Linie entlang und in jede scharfe Ecke hinein, damit er nach einem runden Fräser Platz für ein eckiges Teil lässt.", "La fresa segue l'interno della linea ed entra in ogni angolo vivo, così che una fresa tonda lasci spazio a un pezzo squadrato.", "Terä kulkee viivan sisäpuolella ja jokaiseen teräviin kulmaan, jotta pyöreä terä jättää tilaa kulmikkaalle kappaleelle.", "刀具沿线内侧走刀并进入每个尖角，使圆形刀具为方形零件留出空间。", "बिट रेखा के अंदर चलता है और हर तीखे कोने में जाता है, ताकि गोल बिट चौकोर भाग के लिए जगह छोड़े।"],
    ["Rings", "Pierścienie", "Ringe", "Anelli", "Renkaat", "环形", "घेरे"],
    ["Raster", "Raster", "Raster", "Raster", "Rasteri", "光栅", "रास्टर"],
    ["Before", "Przed", "Davor", "Prima", "Ennen", "之前", "पहले"],
    ["After", "Po", "Danach", "Dopo", "Jälkeen", "之后", "बाद"],
    ["Pocket pattern", "Wzór kieszeni", "Taschenmuster", "Schema della tasca", "Taskun kuvio", "型腔模式", "पॉकेट का पैटर्न"],
    ["Raster angle", "Kąt rastra", "Rasterwinkel", "Angolo del raster", "Rasterin kulma", "光栅角度", "रास्टर का कोण"],
    ["Outline pass", "Przejście po obrysie", "Konturdurchgang", "Passata di contorno", "Reunakierros", "轮廓走刀", "रूपरेखा पास"],
    ["Rest of layer", "Resztki po warstwie", "Rest der Ebene", "Residuo del livello", "Tason jäämät", "图层余料", "परत का शेष"],
    ["Overhang", "Wyjście poza krawędź", "Überstand", "Sbordo", "Ylitys", "超出量", "ओवरहैंग"],
    ["Chamfer width", "Szerokość fazy", "Fasenbreite", "Larghezza dello smusso", "Viisteen leveys", "倒角宽度", "चैम्फर की चौड़ाई"],
    ["Trochoids", "Trochoidy", "Trochoiden", "Trocoidi", "Trokoidit", "摆线铣削", "ट्रोकॉइड"],
    ["Trochoid diameter", "Średnica pętli trochoidy", "Trochoiden-Durchmesser", "Diametro del trocoide", "Trokoidin halkaisija", "摆线直径", "ट्रोकॉइड का व्यास"],
    ["Trochoid step", "Krok trochoidy", "Trochoiden-Schritt", "Passo del trocoide", "Trokoidin askel", "摆线步距", "ट्रोकॉइड का चरण"],
    ["Half circles", "Półokręgi", "Halbkreise", "Semicerchi", "Puoliympyrät", "半圆", "अर्धवृत्त"],
    ["{}: a chamfer needs a V-bit or an engraving bit", "{}: do fazowania potrzebny jest frez V lub grawerski", "{}: Für eine Fase wird ein V-Fräser oder Gravierstichel gebraucht", "{}: per uno smusso serve una fresa a V o da incisione", "{}: viisteeseen tarvitaan V-terä tai kaiverrusterä", "{}：倒角需要 V 形刀或雕刻刀", "{}: चैम्फर के लिए V-बिट या उत्कीर्णन बिट चाहिए"],
    ["{}: the layer of the rest pocket has no bit", "{}: warstwa, po której zbieramy resztki, nie ma freza", "{}: Die Ebene des Restausräumens hat keinen Fräser", "{}: il livello del residuo non ha una fresa", "{}: jäännöstaskun tasolla ei ole terää", "{}：余料型腔所依据的图层没有刀具", "{}: शेष पॉकेट वाली परत में बिट नहीं है"],
    ["{}: the bit of a rest pocket has to be smaller than the bit of the layer it takes over from", "{}: frez do zbierania resztek musi być mniejszy niż frez warstwy, po której przejmuje", "{}: Der Fräser einer Restausräumung muss kleiner sein als der Fräser der Ebene, von der sie übernimmt", "{}: la fresa di un residuo deve essere più piccola di quella del livello da cui riprende", "{}: jäännöstaskun terän täytyy olla pienempi kuin sen tason terä, jolta se jatkaa", "{}：余料型腔的刀具必须比所接续图层的刀具小", "{}: शेष पॉकेट का बिट उस परत के बिट से छोटा होना चाहिए जिससे वह आगे बढ़ता है"],
    ["{}: nothing to plane: draw a shape that gives the size of the surface", "{}: nie ma czego planować: narysuj kształt, który wyznaczy rozmiar powierzchni", "{}: nichts zu planen: Zeichnen Sie eine Form, die die Größe der Fläche vorgibt", "{}: niente da spianare: disegna una forma che dia la misura della superficie", "{}: ei tasoitettavaa: piirrä muoto, joka antaa pinnan koon", "{}：没有可铣平的内容：请绘制一个给出表面大小的形状", "{}: समतल करने को कुछ नहीं: सतह का आकार देने वाला आकार बनाएँ"],
    ["V-carve", "Rzeźbienie V", "V-Gravur", "Incisione a V", "V-uurto", "V 形雕刻", "V-कार्विंग"],
    ["A V-bit goes deeper the farther it is from the edge of the shape, so the walls slope and the corners are sharp. The depth is the deepest the bit goes; a wider shape gets a flat bottom.", "Frez V schodzi tym głębiej, im dalej jest od krawędzi kształtu, więc ściany są skośne, a narożniki ostre. Głębokość to najgłębsze miejsce freza; szerszy kształt dostaje płaskie dno.", "Ein V-Fräser geht umso tiefer, je weiter er vom Rand der Form entfernt ist; die Wände sind schräg, die Ecken scharf. Die Tiefe ist die größte Tiefe des Fräsers; eine breitere Form bekommt einen flachen Boden.", "Una fresa a V scende tanto più in profondità quanto più è lontana dal bordo della forma, quindi le pareti sono inclinate e gli angoli netti. La profondità è la massima raggiunta dalla fresa; una forma più larga ha il fondo piatto.", "V-terä menee sitä syvemmälle, mitä kauempana se on muodon reunasta, joten seinät ovat vinot ja kulmat teräviä. Syvyys on syvin kohta, johon terä menee; leveämpi muoto saa tasaisen pohjan.", "V 形刀离形状边缘越远切得越深，所以侧壁是斜的、拐角是尖的。深度是刀具能到的最深处；更宽的形状会得到平底。", "V-बिट आकार के किनारे से जितना दूर होता है उतना गहरा जाता है, इसलिए दीवारें ढलवाँ और कोने नुकीले होते हैं। गहराई वह अधिकतम गहराई है जहाँ बिट जाता है; चौड़े आकार का तल सपाट होता है."],
    ["Socket", "Gniazdo", "Aussparung", "Sede", "Kolo", "凹槽", "सॉकेट"],
    ["Plug", "Wkładka", "Einlage", "Inserto", "Tappi", "嵌件", "प्लग"],
    ["Rough and finish", "Zgrubnie i wykańczająco", "Schruppen und Schlichten", "Sgrossatura e finitura", "Rouhinta ja viimeistely", "粗加工与精加工", "रफ और फ़िनिश"],
    ["Rough", "Zgrubnie", "Schruppen", "Sgrossatura", "Rouhinta", "粗加工", "रफ"],
    ["Finish", "Wykańczająco", "Schlichten", "Finitura", "Viimeistely", "精加工", "फ़िनिश"],
    ["Import 3D model as relief…", "Importuj model 3D jako relief…", "3D-Modell als Relief importieren…", "Importa modello 3D come rilievo…", "Tuo 3D-malli reliefinä…", "将 3D 模型导入为浮雕…", "3D मॉडल को उभार के रूप में आयात करें…"],
    ["{}: open lines are left out of a V-carve", "{}: otwarte linie są pomijane w rzeźbieniu V", "{}: Offene Linien werden bei der V-Gravur ausgelassen", "{}: le linee aperte sono escluse dall'incisione a V", "{}: avoimet viivat jätetään V-uurrosta pois", "{}：V 形雕刻会忽略开放线条", "{}: V-कार्विंग में खुली रेखाएँ छोड़ दी जाती हैं"],
    ["{}: a V-carve needs a V-bit or an engraving bit", "{}: do rzeźbienia V potrzebny jest frez V lub grawerski", "{}: Für die V-Gravur wird ein V-Fräser oder Gravierstichel gebraucht", "{}: per l'incisione a V serve una fresa a V o da incisione", "{}: V-uurtoon tarvitaan V-terä tai kaiverrusterä", "{}：V 形雕刻需要 V 形刀或雕刻刀", "{}: V-कार्विंग के लिए V-बिट या एनग्रेविंग बिट चाहिए"],
    ["{}: the flat bottom is cut with the V-bit: it leaves ridges and takes long (a flat bit in a Pocket layer is better for it)", "{}: płaskie dno jest frezowane frezem V: zostają grzbiety i trwa to długo (lepszy jest frez płaski w warstwie Pocket)", "{}: Der flache Boden wird mit dem V-Fräser gefräst: Es bleiben Grate und es dauert lange (besser ein Flachfräser in einer Ebene Tasche)", "{}: il fondo piatto è fresato con la fresa a V: restano creste e ci vuole tempo (meglio una fresa piana in un livello Tasca)", "{}: tasainen pohja jyrsitään V-terällä: jää harjanteita ja se kestää kauan (tasapääterä Tasku-tasolla on parempi)", "{}：平底由 V 形刀铣削：会留下棱线且耗时很长（用“型腔”图层中的平底刀更好）", "{}: समतल तल V-बिट से काटा जाता है: उभार रह जाते हैं और समय लगता है (पॉकेट लेयर में फ़्लैट बिट बेहतर है)"],
    ["{}: the program of the V-carve is too long: the rest was left out", "{}: program rzeźbienia V jest za długi: resztę pominięto", "{}: Das Programm der V-Gravur ist zu lang: der Rest wurde weggelassen", "{}: il programma dell'incisione a V è troppo lungo: il resto è stato escluso", "{}: V-uurron ohjelma on liian pitkä: loput jätettiin pois", "{}：V 形雕刻的程序太长：其余部分已省略", "{}: V-कार्विंग का प्रोग्राम बहुत लंबा है: बाकी छोड़ दिया गया"],
    ["Start depth", "Głębokość startowa", "Starttiefe", "Profondità iniziale", "Aloitussyvyys", "起始深度", "आरंभिक गहराई"],
    ["Nothing is cut shallower than this", "Nic nie jest cięte płycej niż ta głębokość", "Nichts wird flacher als dies geschnitten", "Niente viene tagliato più in superficie di questo", "Mitään ei leikata tätä pinnemmalta", "不会切得比这个更浅", "इससे उथला कुछ नहीं कटता"],
    ["Inlay", "Inlay (wkładka)", "Inlay", "Intarsio", "Upotus", "镶嵌", "इनले"],
    ["Inlay gap", "Luz inlay", "Inlay-Spiel", "Gioco dell'intarsio", "Upotuksen väli", "镶嵌间隙", "इनले अंतर"],
    ["The play between the plug and the socket", "Luz między wkładką a gniazdem", "Das Spiel zwischen Einlage und Aussparung", "Il gioco tra inserto e sede", "Tapin ja kolon välinen väli", "嵌件与凹槽之间的间隙", "प्लग और सॉकेट के बीच का अंतर"],
    ["Relief part", "Część reliefu", "Teil des Reliefs", "Parte del rilievo", "Reliefin vaihe", "浮雕工序", "उभार का भाग"],
    ["the model is higher than the material: the relief is cut only as deep as the layer says", "model jest wyższy niż materiał: relief jest frezowany tylko na głębokość z warstwy", "das Modell ist höher als das Material: das Relief wird nur so tief gefräst, wie die Ebene sagt", "il modello è più alto del materiale: il rilievo viene fresato solo alla profondità del livello", "malli on korkeampi kuin materiaali: relief jyrsitään vain tason antamaan syvyyteen", "模型比材料更高：浮雕只按图层设置的深度铣削", "मॉडल सामग्री से ऊँचा है: उभार केवल लेयर की गहराई तक काटा जाता है"],
    ["Could not import the model {}: {}", "Nie udało się zaimportować modelu {}: {}", "Das Modell {} konnte nicht importiert werden: {}", "Impossibile importare il modello {}: {}", "Mallia {} ei voitu tuoda: {}", "无法导入模型 {}：{}", "मॉडल {} आयात नहीं हो सका: {}"],
    ["The model {} has no area seen from above", "Model {} nie ma powierzchni widzianej z góry", "Das Modell {} hat von oben gesehen keine Fläche", "Il modello {} non ha area vista dall'alto", "Mallilla {} ei ole pinta-alaa ylhäältä katsottuna", "从上方看，模型 {} 没有面积", "ऊपर से देखने पर मॉडल {} का कोई क्षेत्रफल नहीं है"],
    ["Imported the model {} as a relief on layer {}: {} x {} x {} mm", "Zaimportowano model {} jako relief na warstwie {}: {} x {} x {} mm", "Modell {} als Relief auf Ebene {} importiert: {} x {} x {} mm", "Modello {} importato come rilievo sul livello {}: {} x {} x {} mm", "Malli {} tuotu reliefinä tasolle {}: {} x {} x {} mm", "已将模型 {} 作为浮雕导入图层 {}：{} x {} x {} mm", "मॉडल {} लेयर {} पर उभार के रूप में आयात किया गया: {} x {} x {} mm"],
    ["Dust collector", "Odkurzacz", "Absaugung", "Aspiratore", "Pölynimuri", "吸尘器", "धूल संग्राहक"],
    ["Switches the dust collector on with the device's own lines (Device settings, G-code)", "Włącza odkurzacz własnymi liniami urządzenia (Ustawienia urządzenia, G-code)", "Schaltet die Absaugung mit den eigenen Zeilen des Geräts ein (Geräteeinstellungen, G-Code)", "Accende l'aspiratore con le righe proprie del dispositivo (Impostazioni dispositivo, G-code)", "Kytkee pölynimurin päälle laitteen omilla riveillä (Laiteasetukset, G-koodi)", "用设备自己的代码行打开吸尘器（设备设置，G 代码）", "डिवाइस की अपनी पंक्तियों से धूल संग्राहक चालू करता है (डिवाइस सेटिंग्स, G-कोड)"],
    ["Dust collector on", "Włączenie odkurzacza", "Absaugung ein", "Aspiratore acceso", "Pölynimuri päälle", "吸尘器开", "धूल संग्राहक चालू"],
    ["Dust collector off", "Wyłączenie odkurzacza", "Absaugung aus", "Aspiratore spento", "Pölynimuri pois", "吸尘器关", "धूल संग्राहक बंद"],
    ["Tool changer", "Zmieniacz narzędzi", "Werkzeugwechsler", "Cambio utensile", "Työkalunvaihtaja", "换刀器", "टूल चेंजर"],
    ["Changes the bit by itself (T… M6)", "Sam wymienia frez (T… M6)", "Wechselt den Fräser selbst (T… M6)", "Cambia la fresa da solo (T… M6)", "Vaihtaa terän itse (T… M6)", "自动更换刀具 (T… M6)", "बिट खुद बदलता है (T… M6)"],
    ["The program asks for a bit with T<number> M6 and goes on without a pause. Every bit needs its number in the library. The controller has to know M6 (grblHAL, FluidNC); plain GRBL does not.", "Program prosi o frez przez T<numer> M6 i jedzie dalej bez pauzy. Każdy frez potrzebuje numeru w bibliotece. Sterownik musi znać M6 (grblHAL, FluidNC); zwykły GRBL nie zna.", "Das Programm verlangt einen Fräser mit T<Nummer> M6 und läuft ohne Pause weiter. Jeder Fräser braucht seine Nummer in der Bibliothek. Die Steuerung muss M6 kennen (grblHAL, FluidNC); reines GRBL kennt es nicht.", "Il programma chiede una fresa con T<numero> M6 e prosegue senza pausa. Ogni fresa ha bisogno del suo numero nella libreria. Il controller deve conoscere M6 (grblHAL, FluidNC); GRBL semplice no.", "Ohjelma pyytää terän komennolla T<numero> M6 ja jatkaa ilman taukoa. Jokaisella terällä on oltava numero kirjastossa. Ohjaimen on tunnettava M6 (grblHAL, FluidNC); tavallinen GRBL ei tunne.", "程序用 T<编号> M6 请求刀具并不暂停继续运行。每把刀都需要在刀库中有编号。控制器必须支持 M6（grblHAL、FluidNC）；普通 GRBL 不支持。", "प्रोग्राम T<संख्या> M6 से बिट माँगता है और बिना रुके आगे बढ़ता है। हर बिट की लाइब्रेरी में संख्या होनी चाहिए। कंट्रोलर को M6 आना चाहिए (grblHAL, FluidNC); सादा GRBL नहीं जानता।"],
    ["Changer number", "Numer w zmieniaczu", "Wechslernummer", "Numero nel cambio", "Vaihtajan numero", "换刀器编号", "चेंजर संख्या"],
    ["The place of the bit in the tool changer: the program asks for it with T… M6. 0 = not in the changer", "Miejsce freza w zmieniaczu narzędzi: program prosi o niego przez T… M6. 0 = nie ma go w zmieniaczu", "Der Platz des Fräsers im Werkzeugwechsler: das Programm verlangt ihn mit T… M6. 0 = nicht im Wechsler", "Il posto della fresa nel cambio utensile: il programma la chiede con T… M6. 0 = non nel cambio", "Terän paikka työkalunvaihtajassa: ohjelma pyytää sen komennolla T… M6. 0 = ei vaihtajassa", "刀具在换刀器中的位置：程序用 T… M6 请求它。0 = 不在换刀器中", "टूल चेंजर में बिट की जगह: प्रोग्राम इसे T… M6 से माँगता है। 0 = चेंजर में नहीं"],
    ["{}: the bit has no changer number: the machine is asked to stop for it instead", "{}: frez nie ma numeru w zmieniaczu: zamiast tego maszyna ma się zatrzymać", "{}: Der Fräser hat keine Wechslernummer: stattdessen hält die Maschine an", "{}: la fresa non ha un numero nel cambio: la macchina viene fermata al suo posto", "{}: terällä ei ole vaihtajan numeroa: kone pysäytetään sen sijaan", "{}：此刀具没有换刀器编号：机器将改为暂停等待", "{}: बिट की चेंजर संख्या नहीं है: इसकी जगह मशीन को रुकने को कहा जाता है"],
    ["{}: the layer asks for the dust collector, but the device has no lines that switch it on", "{}: warstwa wymaga odkurzacza, ale urządzenie nie ma linii, które go włączają", "{}: Die Ebene verlangt die Absaugung, aber das Gerät hat keine Zeilen, die sie einschalten", "{}: il livello richiede l'aspiratore, ma il dispositivo non ha righe che lo accendano", "{}: taso vaatii pölynimurin, mutta laitteella ei ole rivejä, jotka kytkevät sen päälle", "{}：该图层要求使用吸尘器，但设备没有开启它的代码行", "{}: लेयर धूल संग्राहक माँगती है, पर डिवाइस में उसे चालू करने वाली पंक्तियाँ नहीं हैं"],
    ["Frame round the outline", "Ramka po obrysie", "Rahmen um den Umriss", "Cornice lungo il contorno", "Kehys ääriviivan ympäri", "沿轮廓走框", "रूपरेखा के चारों ओर फ़्रेम"],
    ["The frame goes round the outline of the cuts (a rubber band) instead of the box", "Ramka obiega obrys ciętych kształtów (jak gumka) zamiast prostokąta", "Der Rahmen läuft um den Umriss der Schnitte (wie ein Gummiband) statt um das Rechteck", "La cornice segue il contorno dei tagli (come un elastico) invece del rettangolo", "Kehys kulkee leikkausten ääriviivaa pitkin (kuin kuminauha) laatikon sijaan", "框沿切削轮廓（像橡皮筋）而不是矩形走", "फ़्रेम आयत की जगह कटाई की रूपरेखा (रबर बैंड की तरह) के चारों ओर जाता है"],
    ["The centre of the hole is the zero of X and Y now; the hole is {} x {} mm", "Środek otworu jest teraz zerem X i Y; otwór ma {} x {} mm", "Die Mitte des Lochs ist jetzt der Nullpunkt von X und Y; das Loch misst {} x {} mm", "Il centro del foro è ora lo zero di X e Y; il foro misura {} x {} mm", "Reiän keskipiste on nyt X:n ja Y:n nolla; reikä on {} x {} mm", "孔的中心现在是 X 和 Y 的零点；孔大小为 {} x {} mm", "छेद का केंद्र अब X और Y का शून्य है; छेद {} x {} mm का है"],
    ["The centre finder failed: no contact, or an alarm.", "Szukanie środka nie powiodło się: brak kontaktu albo alarm.", "Die Mittensuche ist fehlgeschlagen: kein Kontakt oder ein Alarm.", "La ricerca del centro non è riuscita: nessun contatto o un allarme.", "Keskipisteen etsintä epäonnistui: ei kosketusta tai hälytys.", "寻找中心失败：未接触或出现报警。", "केंद्र खोजना विफल रहा: संपर्क नहीं हुआ, या अलार्म।"],
    ["Project set up: material {} x {} x {} mm, zero at {}", "Projekt ustawiony: materiał {} x {} x {} mm, zero: {}", "Projekt eingerichtet: Material {} x {} x {} mm, Nullpunkt: {}", "Progetto impostato: materiale {} x {} x {} mm, zero: {}", "Projekti asetettu: materiaali {} x {} x {} mm, nolla: {}", "项目已设置：材料 {} x {} x {} mm，零点：{}", "प्रोजेक्ट सेट: सामग्री {} x {} x {} mm, शून्य: {}"],
    ["Project setup…", "Ustawienia projektu…", "Projekteinrichtung…", "Impostazione del progetto…", "Projektin asetukset…", "项目设置…", "प्रोजेक्ट सेटअप…"],
    ["Material width", "Szerokość materiału", "Materialbreite", "Larghezza materiale", "Materiaalin leveys", "材料宽度", "सामग्री की चौड़ाई"],
    ["Material height", "Wysokość materiału (Y)", "Materialhöhe (Y)", "Altezza materiale (Y)", "Materiaalin korkeus (Y)", "材料高度 (Y)", "सामग्री की ऊँचाई (Y)"],
    ["Z counts from the bottom of the material", "Z liczone od spodu materiału", "Z zählt von der Unterseite des Materials", "Z conta dal fondo del materiale", "Z lasketaan materiaalin alapinnasta", "Z 从材料底部算起", "Z सामग्री के तल से गिना जाता है"],
    ["Work area is the material", "Obszar roboczy to materiał", "Arbeitsbereich ist das Material", "L'area di lavoro è il materiale", "Työalue on materiaali", "工作区域即材料", "कार्य क्षेत्र ही सामग्री है"],
    ["The material lies at the top left of the page. The program is written for the work coordinate system G54 with its zero at the place you choose: set that zero on the machine there. Z counts from the top of the material, or from its bottom.", "Materiał leży w lewym górnym rogu strony. Program jest pisany dla układu współrzędnych G54 z zerem w wybranym miejscu: ustaw to zero na maszynie właśnie tam. Z liczy się od góry materiału albo od jego spodu.", "Das Material liegt oben links auf der Seite. Das Programm ist für das Koordinatensystem G54 geschrieben, dessen Nullpunkt an der gewählten Stelle liegt: Stelle diesen Nullpunkt an der Maschine dort ein. Z zählt von der Oberseite des Materials oder von seiner Unterseite.", "Il materiale si trova in alto a sinistra della pagina. Il programma è scritto per il sistema di coordinate G54 con lo zero nel punto scelto: imposta quello zero sulla macchina lì. Z conta dalla superficie del materiale o dal fondo.", "Materiaali on sivun vasemmassa yläkulmassa. Ohjelma on kirjoitettu koordinaatistolle G54, jonka nolla on valitsemassasi paikassa: aseta se nolla koneelle siihen. Z lasketaan materiaalin yläpinnasta tai alapinnasta.", "材料位于页面左上角。程序按工件坐标系 G54 编写，其零点在你选择的位置：请在机器上把零点设在那里。Z 从材料顶面或底面算起。", "सामग्री पृष्ठ के ऊपर-बाएँ कोने में है। प्रोग्राम कार्य निर्देशांक प्रणाली G54 के लिए लिखा गया है, जिसका शून्य आपके चुने स्थान पर है: उस शून्य को मशीन पर वहीं सेट करें। Z सामग्री के ऊपर से या उसके तल से गिना जाता है।"],
    ["Skip", "Pomiń", "Überspringen", "Salta", "Ohita", "跳过", "छोड़ें"],
    ["No presets yet", "Nie ma jeszcze presetów", "Noch keine Vorlagen", "Ancora nessun preset", "Ei vielä esiasetuksia", "还没有预设", "अभी कोई प्रीसेट नहीं"],
    ["Put these settings on the layer", "Nałóż te ustawienia na warstwę", "Diese Einstellungen auf die Ebene anwenden", "Applica queste impostazioni al livello", "Käytä näitä asetuksia tasolla", "把这些设置用到图层", "ये सेटिंग्स लेयर पर लगाएँ"],
    ["Forget this preset", "Zapomnij ten preset", "Diese Vorlage vergessen", "Dimentica questo preset", "Unohda tämä esiasetus", "删除此预设", "इस प्रीसेट को भूलें"],
    ["Save as preset", "Zapisz jako preset", "Als Vorlage speichern", "Salva come preset", "Tallenna esiasetukseksi", "保存为预设", "प्रीसेट के रूप में सहेजें"],
    ["Keep the way of cutting of this layer under this name", "Zapamiętaj sposób cięcia tej warstwy pod tą nazwą", "Die Schnittart dieser Ebene unter diesem Namen merken", "Ricorda il modo di taglio di questo livello con questo nome", "Muista tämän tason leikkaustapa tällä nimellä", "以此名称保存该图层的切削方式", "इस लेयर की कटाई का तरीका इस नाम से रखें"],
    ["Set the address of the board first (Device settings → Connection).", "Najpierw ustaw adres płytki (Ustawienia urządzenia → Połączenie).", "Stelle zuerst die Adresse der Platine ein (Geräteeinstellungen → Verbindung).", "Imposta prima l'indirizzo della scheda (Impostazioni dispositivo → Connessione).", "Aseta ensin kortin osoite (Laiteasetukset → Yhteys).", "请先设置主板地址（设备设置 → 连接）。", "पहले बोर्ड का पता सेट करें (डिवाइस सेटिंग्स → कनेक्शन)।"],
    ["Talking to the Duet…", "Rozmowa z Duet…", "Verbindung zum Duet…", "Comunicazione con il Duet…", "Puhutaan Duetille…", "正在与 Duet 通信…", "Duet से बात हो रही है…"],
    ["Sent to the Duet and started. State: {}", "Wysłano do Duet i uruchomiono. Stan: {}", "An den Duet gesendet und gestartet. Zustand: {}", "Inviato al Duet e avviato. Stato: {}", "Lähetetty Duetille ja käynnistetty. Tila: {}", "已发送到 Duet 并启动。状态：{}", "Duet को भेजा और शुरू किया। स्थिति: {}"],
    ["State of the Duet: {}", "Stan Duet: {}", "Zustand des Duet: {}", "Stato del Duet: {}", "Duetin tila: {}", "Duet 状态：{}", "Duet की स्थिति: {}"],
    ["Duet: {}", "Duet: {}", "Duet: {}", "Duet: {}", "Duet: {}", "Duet: {}", "Duet: {}"],
    ["The Duet runs the program by itself: send it and start it, read its state, stop it. Jogging and overrides are not available. The password is not saved.", "Duet sam wykonuje program: wyślij go i uruchom, odczytaj jego stan, zatrzymaj. Jogging i korekty nie są dostępne. Hasło nie jest zapisywane.", "Der Duet führt das Programm selbst aus: senden und starten, den Zustand lesen, anhalten. Handverfahren und Überschreibungen gibt es nicht. Das Passwort wird nicht gespeichert.", "Il Duet esegue il programma da solo: invialo e avvialo, leggi il suo stato, fermalo. Jogging e override non sono disponibili. La password non viene salvata.", "Duet ajaa ohjelman itse: lähetä ja käynnistä se, lue sen tila, pysäytä. Käsiajo ja ohitukset eivät ole käytettävissä. Salasanaa ei tallenneta.", "Duet 自行运行程序：发送并启动，读取状态，停止。不能点动，也没有倍率调整。密码不会保存。", "Duet प्रोग्राम खुद चलाता है: उसे भेजें और शुरू करें, उसकी स्थिति पढ़ें, रोकें। जॉगिंग और ओवरराइड उपलब्ध नहीं हैं। पासवर्ड सहेजा नहीं जाता।"],
    ["Send and start", "Wyślij i uruchom", "Senden und starten", "Invia e avvia", "Lähetä ja käynnistä", "发送并启动", "भेजें और शुरू करें"],
    ["Upload the program to the board and start it there", "Wyślij program na płytkę i uruchom go tam", "Das Programm auf die Platine laden und dort starten", "Carica il programma sulla scheda e avvialo lì", "Lähetä ohjelma kortille ja käynnistä se siellä", "把程序上传到主板并在那里启动", "प्रोग्राम को बोर्ड पर अपलोड करें और वहीं शुरू करें"],
    ["State", "Stan", "Zustand", "Stato", "Tila", "状态", "स्थिति"],
    ["Duet / RepRapFirmware (HTTP)", "Duet / RepRapFirmware (HTTP)", "Duet / RepRapFirmware (HTTP)", "Duet / RepRapFirmware (HTTP)", "Duet / RepRapFirmware (HTTP)", "Duet / RepRapFirmware (HTTP)", "Duet / RepRapFirmware (HTTP)"],
    ["A Duet board runs the program by itself: it is sent over the network and started there. Jogging and overrides are not available.", "Płytka Duet sama wykonuje program: jest wysyłany siecią i uruchamiany tam. Jogging i korekty nie są dostępne.", "Eine Duet-Platine führt das Programm selbst aus: es wird über das Netzwerk gesendet und dort gestartet. Handverfahren und Überschreibungen gibt es nicht.", "Una scheda Duet esegue il programma da sola: viene inviato in rete e avviato lì. Jogging e override non sono disponibili.", "Duet-kortti ajaa ohjelman itse: se lähetetään verkon yli ja käynnistetään siellä. Käsiajo ja ohitukset eivät ole käytettävissä.", "Duet 主板自行运行程序：通过网络发送并在那里启动。不能点动，也没有倍率调整。", "Duet बोर्ड प्रोग्राम खुद चलाता है: उसे नेटवर्क से भेजा जाता है और वहीं शुरू किया जाता है। जॉगिंग और ओवरराइड उपलब्ध नहीं हैं।"],
    ["Password", "Hasło", "Passwort", "Password", "Salasana", "密码", "पासवर्ड"],
    ["Not saved: it is asked for again when the program is opened", "Nie jest zapisywane: przy ponownym otwarciu programu trzeba je podać jeszcze raz", "Wird nicht gespeichert: beim nächsten Programmstart muss es neu eingegeben werden", "Non viene salvata: viene richiesta di nuovo all'apertura del programma", "Ei tallenneta: se kysytään uudelleen, kun ohjelma avataan", "不会保存：重新打开程序时需要再次输入", "सहेजा नहीं जाता: प्रोग्राम खुलने पर फिर से पूछा जाता है"],
    ["The milled piece cannot be exported as STL while a rotary attachment is chosen.", "Sfrezowanego elementu nie można wyeksportować jako STL, gdy wybrana jest przystawka obrotowa.", "Das gefräste Teil kann nicht als STL exportiert werden, solange eine Rotationsachse gewählt ist.", "Il pezzo fresato non può essere esportato come STL finché è scelto un asse rotativo.", "Jyrsittyä kappaletta ei voi viedä STL-tiedostona, kun pyörityslaite on valittu.", "选择旋转附件时，无法将铣削件导出为 STL。", "रोटरी अटैचमेंट चुने होने पर फ्रेज़ किया हुआ टुकड़ा STL के रूप में निर्यात नहीं किया जा सकता।"],
    ["Insert", "Wstawianie", "Einfügen", "Inserisci", "Lisää", "插入", "सम्मिलित करें"],
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

🌐 [English](Menus) · [Polski](Menu) · [简体中文](Menus-ZH) · **한국어**

> ⚠️ 이 페이지는 영어판을 기계 번역한 것으로, 부정확한 부분이 있을 수 있습니다. 의심스러우면 [영어판](Menus)을 기준으로 하십시오. 수정 제안을 환영합니다.

# 메뉴

메뉴 구성은 LightCreator, MillCreator, PenCreator에서 같으며(baseCreator에서 가져옵니다), MillCreator는 자신만의 항목을 알맞은 자리에 넣습니다.

**파일 · 편집 · 삽입 · 정렬 · 보기 · 기계 · 밀링 · 설정 · 도움말** (File · Edit · Insert · Arrange · View · Machine · Milling · Settings · Help)

* **Windows와 Linux:** 도면이 열리면 메뉴 막대가 창 안, 제목 표시줄 아래에 있습니다(시작 화면에는 메뉴 막대가 없습니다). 항목의 단축키도 그곳에서 동작합니다([키보드 단축키](Keyboard-Shortcuts-KO) 참고).
* **macOS:** 메뉴는 화면 맨 위 시스템 메뉴 막대에 있으며 시작 화면에서도 보입니다(도면이 필요한 항목은 그곳에서 회색입니다). 첫 메뉴인 **MillCreator**에는 **MillCreator에 관하여**(About MillCreator), 서비스, 가리기 / 보기, **종료**(Quit)(Cmd+Q)가 있으므로 **파일**(File)에는 *종료*가 없고 **도움말**(Help)에는 *정보*가 없습니다. 메뉴의 단축키도 그곳에서 동작하지만, 시스템 막대에는 새로 만들기, 열기, 저장, 다른 이름으로 저장, 종료의 단축키만 표시됩니다. [키보드 단축키](Keyboard-Shortcuts-KO#macos)를 참고하십시오.

단축키는 Windows와 Linux 방식(Ctrl)으로 적었습니다. macOS에서는 Ctrl이 Cmd입니다. 지금 사용할 수 없는 항목은 회색입니다(예를 들어 도면이 열려 있지 않을 때의 **저장**(Save), **작업 3D 보기…**(3D view of the job…), **G-code 내보내기…**(Export G-code…), 도형이 두 개 미만으로 선택되었을 때의 **서로 중앙 맞춤**(Centre on each other), 그림이 선택되지 않았을 때의 **이미지 조정…**(Adjust image…)과 **이미지 트레이스…**(Trace image…)).

## 파일

| 항목 | 단축키 | 기능 |
|---|---|---|
| **새로 만들기**(New) | Ctrl+N | 새 빈 도면을 별도의 탭에 열고 이어서 **프로젝트 설정**(Project setup) 창을 엽니다. [빠른 시작](Quick-Start-KO)을 참고하십시오. |
| **열기…**(Open…) | Ctrl+O | 프로젝트(`.cre`) 또는 SVG, PDF, Illustrator 도면을 별도의 탭에 엽니다. 이미 열려 있는 프로젝트는 두 번 열지 않고 그 탭을 보여 줍니다. 여기서 고른 DXF, UGS 디자인, 3D 모델(STL, OBJ, 3MF)은 도면으로 가져옵니다. 창에 끌어 놓은 파일도 같은 방식으로 처리되며, 그림이나 LightBurn 프로젝트(가져오기)와 G-code 파일(G-code 분석에서 표시)도 마찬가지입니다. |
| **최근 파일 열기**(Open recent) | | 최근 파일(최대 10개). **목록 지우기**(Clear list)로 비웁니다. |
| **저장**(Save) | Ctrl+S | 프로젝트(`.cre`)를 저장합니다. |
| **다른 이름으로 저장…**(Save as…) | Ctrl+Shift+S | 프로젝트를 다른 이름으로 저장합니다. |
| **프로젝트 정보…**(Project information…) | | 파일(이름, 폴더, 크기, 생성 및 변경 시각), 프로젝트 생성 시각, 작성자, 기여자, 편집 기록. 손으로 저장할 때마다 **설정 → 프로그램 옵션 → 작성자**(Settings → Program options → Author)에 설정한 사람(이름, 이메일, 기타; 모든 프로그램에서 동일), 시각, 프로그램과 그 버전이 기록되며, 최대 1분에 한 번입니다. 만든 사람은 세 가지를 모두, 기여자는 이름과 이메일만(이메일이 만든 사람과 같으면 이름만) 저장합니다. |
| **프로젝트 설정…**(Project setup…) | | 재료의 크기와 두께, 재료 위 프로그램 원점, 바닥 기준 Z, 작업 영역을 재료로. [빠른 시작](Quick-Start-KO#2-프로젝트-설정)을 참고하십시오. |
| **가져오기 ▸ 이미지 가져오기…**(Import ▸ Import image…) | | 페이지에 넣을 그림(PNG, JPG, BMP, GIF, WebP), 가져오기 대화 상자와 함께. [디자인 도구](Design-Tools-KO)를 참고하십시오. |
| **가져오기 ▸ SVG 가져오기…**(Import ▸ Import SVG…) | Ctrl+I | SVG의 경로를 열린 도면에 추가하며, 각 경로는 색이 가장 가까운 레이어에, 그림은 활성 레이어에 들어갑니다. **열기…** 항목은 파일을 새 탭으로 엽니다. |
| **가져오기 ▸ Adobe Illustrator / PDF / EPS / VectorCraft 가져오기…**(Import ▸ Import Adobe Illustrator / PDF / EPS / VectorCraft…) | | PDF, PDF 호환 `.ai`, 예전 PostScript 방식의 `.ai`, EPS, VectorCraft 도면(`.vectorcraft`, 그림 포함)에 대해 같은 일을 합니다. Illustrator 레이어가 있는 파일은 색 기준으로 할지 그 레이어 기준으로 할지 묻습니다. |
| **가져오기 ▸ DXF 가져오기…**(Import ▸ Import DXF…) | | ASCII DXF를 윤곽선으로 가져옵니다. 그 레이어는 프로젝트의 빈 레이어로 들어갑니다. |
| **가져오기 ▸ LightBurn / MillMage 프로젝트 가져오기…**(Import ▸ Import LightBurn / MillMage project…) | | LightBurn 프로젝트(`.lbrn2` / `.lbrn`) 또는 MillMage 프로젝트(`.mage`)의 도형. |
| **가져오기 ▸ UGS 디자인 가져오기…**(Import ▸ Import UGS design…) | | UGS Designer의 `.ugsd` 디자인. 절삭 방식은 레이어가 됩니다. |
| **가져오기 ▸ 3D 모델을 릴리프로 가져오기…**(Import ▸ Import 3D model as relief…) | | STL, OBJ 또는 3MF 모델을 릴리프 레이어의 깊이 맵으로 가져옵니다. [레이어와 작업](Layers-and-Operations-KO#그림)을 참고하십시오. |
| **가져오기 ▸ 플러그인으로 가져오기…**(Import ▸ Import with a plugin…) | | 플러그인이 읽는 파일. [플러그인](Plugins-KO)을 참고하십시오. |
| **가져오기 ▸ 도형 생성…**(Import ▸ Generate shape…) | | 플러그인의 도형 생성기 창(**삽입**(Insert)에도 있음). |
| **내보내기 ▸ G-code 내보내기…**(Export ▸ Export G-code…) | | **밀링 → G-code 내보내기…**(Milling → Export G-code…)와 같습니다. |
| **내보내기 ▸ SVG 내보내기…**(Export ▸ Export SVG…) | | 도면 전체를 작업 영역 크기의 SVG로 내보냅니다. 곡선과 그림이 유지됩니다. |
| **내보내기 ▸ PLT / DXF 내보내기…**(Export ▸ Export PLT / DXF…) | | 도면 전체의 윤곽선을 HPGL(`.plt`, 레이어마다 펜 하나) 또는 DXF R12로, 선택한 확장자에 따라 내보냅니다. 이것은 도형이지 기계의 움직임이 아닙니다. |
| **내보내기 ▸ DXF 내보내기…**(Export ▸ Export DXF…) | | 도면 전체를 현재 방식의 DXF(AutoCAD 2007)로 내보냅니다. 곡선은 0.01 mm로 평탄화됩니다. |
| **내보내기 ▸ DXF R12 (레거시) 내보내기…**(Export ▸ Export DXF R12 (legacy)…) | | 같은 내용을 오래된 프로그램과 커터용 DXF R12(*legacy*라고도 함)로 내보냅니다. |
| **내보내기 ▸ Adobe Illustrator 내보내기…**(Export ▸ Export Adobe Illustrator…) | | 윤곽선을 곡선과 함께 PostScript 방식의 Illustrator 파일로 내보내며, 프로젝트 레이어마다 Illustrator 레이어가 하나씩 생깁니다. 그림은 제외됩니다. |
| **내보내기 ▸ LightBurn 프로젝트 내보내기…**(Export ▸ Export LightBurn project…) | | LightBurn 프로젝트(`.lbrn2`, 또는 파일 이름을 그렇게 지으면 `.lbrn`): 곡선을 포함한 도형과 그림, 프로젝트 레이어마다 LightBurn 레이어 하나. |
| **내보내기 ▸ MillMage 프로젝트 내보내기…**(Export ▸ Export MillMage project…) | | 같은 내용을 MillMage 프로젝트(`.mage`)로 내보내며, 재료는 작업 영역 크기입니다. |
| **내보내기 ▸ 릴리프를 STL로 내보내기…**(Export ▸ Export relief as STL…) | | **밀링 → 릴리프를 STL로 내보내기…**(Milling → Export relief as STL…)와 같습니다. |
| **내보내기 ▸ 기술 도면 ▸**(Export ▸ Technical drawing ▸) | | 기술 도면을 **PDF**, **PNG**, **JPG**, **DXF** 또는 **DXF R12 (legacy)** 형식으로 내보냅니다. DXF는 인쇄되는 그대로의 A4 용지를 밀리미터 단위로 담으며, 윤곽선, 가는 선과 점선, 글자가 각각의 레이어에 들어갑니다. 치수는 실제 크기를 나타냅니다. |
| **내보내기 ▸ 선택 영역을 SVG로 내보내기…**(Export ▸ Export selection as SVG…) | | 선택한 도형(아무것도 선택하지 않았으면 전체)을 SVG로 씁니다. |
| **내보내기 ▸ 선택 영역을 DXF로 내보내기…**(Export ▸ Export selection as DXF…) | | 같은 내용을 현재 방식의 DXF로 내보내며, 프로젝트 레이어마다 DXF 레이어가 하나씩 생깁니다. |
| **G-code 파일 보기…**(View G-code file…) | | G-code 파일을 **G-code 분석**(G-code analysis)에서 엽니다. 2D 또는 3D 공구 경로, 절삭 길이, 시간이 표시됩니다. |
| **인쇄…**(Print…) | Ctrl+P | 디자인을 종이 프린터로 인쇄하거나 PDF로 저장합니다. |
| **종료**(Quit) | Ctrl+Q | 프로그램을 끝냅니다. 저장하지 않은 작업이 있으면 먼저 묻습니다. macOS에는 없습니다(그곳에서는 **MillCreator** 메뉴에 있습니다). |

## 편집

| 항목 | 단축키 | 기능 |
|---|---|---|
| **실행 취소**(Undo) | Ctrl+Z | 마지막 변경을 되돌립니다. |
| **다시 실행**(Redo) | Ctrl+Y | 되돌린 것을 다시 합니다(Ctrl+Shift+Z도 가능). |
| **복사**(Copy) | Ctrl+C | 선택한 도형을 복사합니다. |
| **붙여넣기**(Paste) | Ctrl+V | 복사한 도형을 넣습니다. |
| **복제**(Duplicate) | Ctrl+D | 선택 항목을 한 번에 복사합니다. |
| **삭제**(Delete) | Del | 선택 항목을 삭제합니다(Delete 또는 Backspace). 노드 도구로 노드를 선택한 상태에서는 노드를 삭제합니다. |
| **모두 선택**(Select all) | Ctrl+A | 모든 도형을 선택합니다. |

## 삽입

왼쪽 팔레트에서 개체를 삽입하는 도구들입니다. 항목을 고르면 해당 도구가 선택되며(사용 중인 도구에 체크 표시가 됩니다. 단 macOS 메뉴 막대는 예외로, 그곳에서는 **격자**(Grid), **격자에 맞추기**(Snap to grid), **개체에 맞추기**(Snap to objects), 언어, 색 구성표에만 체크 표시가 됩니다), 이름 안의 글자가 그 단축키입니다.

**사각형 (R)**(Rectangle), **타원 (E)**(Ellipse), **삼각형 (Y)**(Triangle), **별 (S)**(Star), **다각형 (G)**(Polygon), **하트 (K)**(Heart), **나선 (I)**(Spiral), **선 (L)**(Line), **폴리라인 / 펜 (P)**(Polyline / pen), **텍스트 (T)**(Text).

그다음 **도형 생성…**(Generate shape…)(플러그인의 생성기, [플러그인](Plugins-KO) 참고)과 **온라인 클립아트…**(Online clipart…)(**클립아트**(Clipart) 탭과 인터넷의 무료 그림 검색을 엽니다)가 있습니다. [디자인 도구](Design-Tools-KO)를 참고하십시오.

## 정렬

| 항목 | 단축키 | 기능 |
|---|---|---|
| **왼쪽 맞춤**(Align left), **가운데 맞춤 (H)**(Align centre (H)), **오른쪽 맞춤**(Align right), **위쪽 맞춤**(Align top), **가운데 맞춤 (V)**(Align centre (V)), **아래쪽 맞춤**(Align bottom) | | 선택한 도형을 정렬합니다. |
| **베드 중앙에 놓기**(Centre on bed) | | 선택 항목을 작업 영역 가운데에 놓습니다. |
| **서로 중앙 맞춤**(Centre on each other) | | 선택한 도형(둘 이상)의 중심을 하나로 맞춥니다. |
| **좌우 뒤집기**(Flip horizontal), **상하 뒤집기**(Flip vertical) | | 선택 항목을 대칭으로 뒤집습니다. |
| **시계 방향 90° 회전**(Rotate 90° CW), **반시계 방향 90° 회전**(Rotate 90° CCW) | | 선택 항목을 4분의 1 바퀴 돌립니다. |
| **그룹**(Group) | Ctrl+G | 선택한 도형을 하나의 그룹으로 만듭니다. |
| **그룹 해제**(Ungroup) | Ctrl+Shift+G | 그룹을 풉니다. |
| **잠금**(Lock) | Ctrl+L | 도형을 더 이상 옮기거나 바꿀 수 없게 합니다. |
| **잠금 해제**(Unlock) | Ctrl+Shift+L | 잠긴 도형을 풉니다. |
| **맨 앞으로 가져오기**(Bring to front), **맨 뒤로 보내기**(Send to back) | | 도형의 순서. |
| **노드 편집**(Edit nodes) | N | 노드 도구: 선의 점을 옮깁니다. |
| **경로로 변환**(Convert to path) | | 사각형, 타원, 별… 을 노드를 편집할 수 있는 경로로 바꿉니다. |
| **합집합**(Union), **교집합**(Intersection), **빼기**(Subtract), **배타적 합**(Exclusive or) | | 둘 이상의 도형에 대한 불리언 연산. |
| **도형 오프셋…**(Offset shape…), **모서리 둥글리기…**(Round corners…), **곡선으로 변환**(Convert to curves) | | 더 크거나 작은 윤곽선, 둥근 모서리, 선에서 곡선 만들기. |
| **경로 자동 연결…**(Auto-join paths…), **허용 오차로 경로 닫기…**(Close path with tolerance…) | | 맞닿는 선을 경로로 잇고, 거의 닫힌 경로를 닫습니다. |
| **도그본 / T-본 모서리…**(Dogbone / T-bone corners…) | | 안쪽 모서리에 홈을 내어, 둥근 비트로 자른 슬롯에 사각 부품이 들어가게 합니다. |
| **격자 배열…**(Grid array…), **원형 배열…**(Circular array…), **경로를 따라 복사…**(Copy along path…), **경로 위의 텍스트…**(Text on path…) | | 행과 열로, 중심 둘레로 또는 경로를 따라 복사합니다. 경로를 따라 휜 텍스트. |
| **도형으로 트리밍…**(Trim by shape…), **중복 제거**(Remove duplicates), **분리**(Break apart), **단순화…**(Simplify…) | | 도형으로 선을 자르기, 겹친 도형 삭제, 윤곽선마다 도형 하나, 점 줄이기. |
| **일련번호 텍스트…**(Serial text…), **바코드 / QR 코드…**(Barcode / QR code…), **도형 네스팅…**(Nest shapes…), **측정…**(Measure…) | | 번호가 붙은 텍스트 복사본, 윤곽선으로 만든 코드, 영역에 도형 배치하기, 크기와 거리. |
| **이미지 조정…**(Adjust image…), **이미지 트레이스…**(Trace image…) | | 페이지의 그림을 바꾸고, 윤곽선으로 변환합니다. |
| **이미지 편집…**(Edit image…) | | 선택한 그림의 사진 편집기: 브러시, 지우개, 복제 도장, 채우기, 회색조. |
| **이미지로 변환…**(Convert to image…) | | 선택한 도형(그림 제외)을 해상도와 크기를 지정해 하나의 그림으로 만듭니다. |
| **스타일 트레이스…**(Style trace…) | | 선택한 그림을 21가지 스타일 중 하나의 선으로 다시 그립니다. [스타일 트레이스](Style-Trace-KO)를 참고하십시오. |
| **기하학적 다각형…**(Geometric polygons…) | | 선택한 그림을 기하학적(로우 폴리) 다각형으로 만들며, 다각형 사이와 윤곽 둘레에 간격을 둡니다. 이 스타일이 선택된 스타일 트레이스 창입니다. [스타일 트레이스](Style-Trace-KO)를 참고하십시오. |
| **금지 구역으로 추가**(Add as no-go zone) | | 잠기지 않은 선택 도형 각각이 장치의 금지 구역(둥글면 원, 그렇지 않으면 둘러싼 사각형)이 되고 도면에서 빠집니다. 오른쪽 클릭 메뉴에도 있습니다. |

모든 항목은 [디자인 도구](Design-Tools-KO)에서 설명합니다.

## 보기

| 항목 | 단축키 | 기능 |
|---|---|---|
| **격자**(Grid), **격자에 맞추기**(Snap to grid), **개체에 맞추기**(Snap to objects) | | 켜고 끄는 항목(켜지면 체크 표시). 격자 자체는 **설정 → 보기 옵션…**(Settings → View options…)에서 설정합니다. |
| **확대**(Zoom in) | Ctrl++ | 작업 영역 가운데를 기준으로 확대합니다(키는 Ctrl과 `=`). |
| **축소**(Zoom out) | Ctrl+- | 축소합니다. |
| **베드를 창에 맞추기**(Fit bed to window) | Ctrl+Shift+0 | 작업 영역 전체를 창에 맞춥니다. |
| **모든 개체 맞추기**(Fit all objects) | Ctrl+Shift+9 | 모든 도형을 창에 맞춥니다. |
| **작업 영역을 창에 자동 맞춤**(Auto-fit work area to window) | | 켜고 끄는 항목(켜지면 체크 표시, macOS 제외): 창 크기를 바꾸는 동안 작업 영역이 계속 창에 맞춰집니다(손으로 확대/축소하면 꺼집니다). |
| **분할 보기: 좌우**(Split view: side by side), **분할 보기: 상하**(Split view: one above the other) | | 가운데 영역을 창 영역으로 나눕니다(창 영역이 하나뿐이면 두 번째 창 영역이 추가됩니다). |
| **단일 보기**(Single view) | | 모든 탭을 다시 한 창 영역에 둡니다. |
| **카메라 오버레이 표시**(Show camera overlay) | | 도면 아래의 그림을 표시하거나 숨깁니다. |
| **금지 구역 표시**(Show no-go zones) | | 장치의 금지 구역을 빨간색으로 그립니다. 장치에 금지 구역이 없으면 회색입니다. |
| **기술 도면**(Technical drawing) | | 기술 도면을 표시하거나 숨깁니다(예전에는 편집기 위의 버튼이었습니다). |
| **미리 보기**(Preview) | | **미리 보기…**(Preview…) 버튼처럼 미리 보기를 표시하거나 숨깁니다. |
| **G-code 미리 보기**(Preview G-code) | | **G-code 미리 보기** 버튼처럼 작업의 G-code를 탭에 보여 줍니다. |
| **오버레이 가져오기…**(Overlay from…) | | 사진 또는 카메라의 실시간 화면을 도면 아래에 두며, 네 모서리 또는 기준점으로 맞춥니다. 프로젝트와 함께 저장됩니다. |

창 영역과 탭: [창 영역과 탭](Panes-and-Tabs-KO).

## 기계

먼저 기계 설정, 그다음 기계에서 실행되는 것들입니다. [기계 제어](Machine-Control-KO)를 참고하십시오.

| 항목 | 기능 |
|---|---|
| **장치 바꾸기…**(Switch device…) | 다른 장치를 고르도록 시작 화면으로 돌아갑니다. |
| **장치 설정…**(Device settings…) | 사용 중인 장치의 설정(페이지는 [장치와 카메라](Devices-and-Cameras-KO)에 있습니다). |
| **펌웨어 설정 ($$)…**(Firmware settings ($$)…) | GRBL 컨트롤러의 설정, 백업 포함. |
| **보정…**(Calibration…) | 밀리미터당 스텝 수와 축의 직각도. |
| **공구 길이 센서…**(Tool length sensor…) | 공구 길이 센서 마법사. |
| **금지 구역…**(No-go zones…) | | 장치 창의 **금지 구역**(No-go zones) 페이지. |
| **프로그램 원점 고르기**(Pick the zero of the program) | | 작업 영역을 다음에 클릭하는 곳이 프로그램 원점이 됩니다(장치 탭의 **작업 영역에서 고르기**(Pick on the work area)와 같음). |
| **중심 찾기**(Find the centre) | | 구멍 안에 내려간 프로브가 네 벽을 건드리고 그 가운데를 X와 Y의 원점으로 삼습니다(제어 탭과 같음, 연결된 기계 필요). |
| **저장된 위치…**(Saved positions…) | | 저장된 위치가 있는 제어 탭을 보여 줍니다. |
| **절삭할 면 ▸**(Side to cut ▸) | | 양면 작업의 위(Top) 또는 아래(Bottom, 뒤집은 면)(제어 탭과 같음). |
| **제어 장치 ▸**(Control devices ▸) | 제어 장치 설정…(Control device settings…)은 키보드와 패드, 그 키의 창을 엽니다. 이 기계의 제어 장치…(Control device of this machine…)는 열린 기계에 쓸 장치를 고릅니다. [제어 장치](Control-Devices-KO)를 참고하십시오. |
| **진단과 통계…**(Diagnostics and statistics…) | 입력 핀, 작업, 사용 시간, 알람과 오류, 유지 보수 알림, 보고서 내보내기. |
| **네트워크로 공유 (ser2net)…**(Share over network (ser2net)…) | Linux 전용: 기계의 시리얼 포트를 네트워크로 공유합니다. |
| **프레임**(Frame) | 작업 범위 상자를 한 바퀴 돕니다(연결된 기계 필요). |
| **작업 시작**(Start job) | 작업을 보냅니다(연결된 기계 필요). |
| **줄부터 시작…**(Start from line…) | 프로그램의 특정 줄에서 작업을 시작합니다. |
| **프로그램 검사**(Check the program) | 컨트롤러가 검사 모드(`$C`)로 프로그램을 훑습니다. 아무것도 움직이지 않으며, 이해하지 못하는 줄이 보고됩니다. 연결된 기계가 필요합니다. |

## 밀링

| 항목 | 단축키 | 기능 |
|---|---|---|
| **비트 라이브러리…**(Bit library…) | | 기성 비트와 내 비트. [비트 라이브러리](Bit-Library-KO)를 참고하십시오. |
| **나이프 라이브러리…**(Knife library…) | | 기성 드래그 나이프와 내 나이프. [디자인 도구](Design-Tools-KO#드래그-나이프)를 참고하십시오. |
| **레이어 프리셋…**(Layer presets…) | | 레이어 프리셋 창: 활성 레이어를 이름을 붙여 저장하거나 프리셋을 적용합니다. |
| **활성 레이어의 작업 ▸**(Operation of the active layer ▸) | | 모든 작업 목록: 고른 작업이 활성 레이어의 작업이 됩니다(레이어 탭과 같음). |
| **고정 탭 손으로 배치**(Place tabs by hand) | | 마우스로 윤곽선 위에 고정 탭을 배치하는 기능을 켜고 끕니다(레이어 탭과 같음). |
| **정렬 구멍 추가**(Add alignment holes) | | 양면 작업의 핀용 구멍 두 개를 빈 레이어에 추가합니다(재료 탭과 같음). |
| **작업 3D 보기…**(3D view of the job…) | Ctrl+Shift+P | **미리 보기**(Preview) 탭을 엽니다. 재료, 절삭, 이동을 애니메이션과 함께 3D로 보여 줍니다. [3D 보기와 G-code](3D-View-and-G-code-KO)를 참고하십시오. |
| **G-code 미리 보기**(Preview G-code) | | 작업의 G-code를 저장하지 않고 탭에 텍스트로 보여 줍니다(**G-code 미리 보기** 버튼과 같음). |
| **기술 도면 보기…**(Show technical drawing…) | | 부품을 기술 도면으로 보여 줍니다. 정면도, 측면도, 평면도와 치수, 표제란이 있습니다. **보기 → 기술 도면**(View → Technical drawing)으로도 열 수 있습니다. |
| **G-code 내보내기…**(Export G-code…) | Ctrl+E | 파일 이름(`name.nc`)을 묻고, 켜져 있는 포스트프로세서를 거쳐 프로젝트의 G-code를 씁니다. **G-code 저장**(Save G-code) 버튼과 같습니다. |
| **릴리프를 STL로 내보내기…**(Export relief as STL…) | | 그림의 3D 릴리프를 STL 모델로 저장합니다(3D 릴리프 방식 레이어에 그림이 있을 때만). |

## 설정

| 항목 | 기능 |
|---|---|
| **보기 옵션…**(View options…) | 주 격자와 보조 격자, 작업 영역의 색, 선 두께, **레이어 이름 표시**(Show layer names), **저장 후 G-code 미리 보기**(Preview G-code after saving), **변수 이름 표시**(Show variable names). [설정과 파일](Settings-and-Files-KO)을 참고하십시오. **G-code 콘솔**(G-code console)의 글꼴 크기와 색. |
| **프로그램 옵션…**(Program options…) | **프로그램 시작 시 새 버전 확인**(Look for a newer version when the program starts); 3D 보기의 **렌더링 품질**(Rendering quality)(1 빠름 … 5 선명); **자동 저장**(Auto-save): 작업을 자동으로 저장하고(이름이 있는 파일은 그 자리에, 이름이 없는 파일은 복구 파일로) 저장 간격(10초~1시간, 처음에는 2분)을 정합니다. |
| **플러그인…**(Plugins…) | 플러그인 설치, 켜기와 끄기, 제거, 공식 목록. [플러그인](Plugins-KO)을 참고하십시오. |
| **언어 ▸**(Language ▸) | 프로그램의 38개 언어. |
| **색 구성표 ▸**(Colour scheme ▸) | 창의 18가지 색 구성표, 그리고 크리스마스, 새해, 핼러윈, 부활절, 설날(음력 새해) 무렵의 축제용 구성표. |

## 도움말

| 항목 | 기능 |
|---|---|
| **GitHub 프로젝트 페이지**(Project page on GitHub) | 브라우저에서 MillCreator 저장소를 엽니다. |
| **백업 다운로드…**(Download backup…) | 설정과 공유 클립아트 갤러리를 zip 하나로 저장합니다. [설정과 파일](Settings-and-Files-KO)을 참고하십시오. |
| **백업에서 복원…**(Restore from backup…) | 그런 zip을 다시 읽어 들입니다. |
| **빠른 안내**(Quick guide) (F1) | 프로그램 안의 안내서, 검색 기능 포함. |
| **진단 정보 내보내기 (zip)…**(Export diagnostics (zip)…) | 버그 보고에 첨부할 보고서, 설정, 콘솔을 담은 파일 하나. |
| **업데이트 확인…**(Check for updates…) | MillCreator의 새 릴리스를 찾습니다. |
| **정보**(About) | 버전, 라이선스, 제작진. macOS에는 없습니다(그곳에서는 **MillCreator → MillCreator에 관하여**). |

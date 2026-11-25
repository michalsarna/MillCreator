🌐 [English](Settings-and-Files) · [Polski](Ustawienia-i-pliki) · [简体中文](Settings-and-Files-ZH) · **한국어**

> ⚠️ 이 페이지는 영어판을 기계 번역한 것으로, 부정확한 부분이 있을 수 있습니다. 의심스러우면 [영어판](Settings-and-Files)을 기준으로 하십시오. 수정 제안을 환영합니다.

# 설정과 파일

언어, 색 구성표, 보기 옵션, 최근 파일, **장치 목록**, **내 비트**는 하나의 파일 `app.ron`에 저장됩니다.

| 시스템 | 위치 |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/millcreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\millcreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/millcreator/app.ron`, 기본값은 `~/.local/share/baseCreator/apps/millcreator/app.ron` |

* **공유 폴더.** LightCreator, MillCreator, PenCreator는 데이터를 하나의 폴더 `baseCreator`에 보관합니다(표는 그 안에 있는 이 프로그램 자체의 설정을 보여 줍니다). **클립아트 갤러리는 공유됩니다.** 한 프로그램에서 저장하거나 내려받은 그림이 다른 프로그램에도 나타납니다(`apps` 옆의 `clipart` 폴더, 이름, 분류, 출처, 라이선스, 작성자를 기록한 `index.json` 포함). 각 프로그램은 자체 설정, 장치 프로필, 재료, 공구를 `apps/<program>/app.ron`에 보관합니다. 다른 폴더를 쓰려면 환경 변수 `BASECREATOR_DIR`를 설정하십시오.
* 파일은 프로그램을 닫을 때(그리고 때때로) 쓰입니다. **모든 것을 초기화하려면 이 파일을 지우십시오.** 장치 목록과 내 비트도 함께 사라지므로 먼저 내보내십시오(장치 창과 비트 라이브러리의 **내보내기…**(Export…)).
* **도움말 → 백업 다운로드…**(Help → Download backup…)는 MillCreator의 설정(장치 목록, 비트, 언어, 색 구성표, 최근 파일)과 공유 클립아트 갤러리 전체를 README와 함께 zip으로 저장합니다. **도움말 → 백업에서 복원…**(Help → Restore from backup…)은 확인 후 그런 파일을 다시 읽습니다. 설정은 바뀌고 그림은 갤러리에 **추가**되므로, 그사이 다른 프로그램이 저장한 것은 잃지 않습니다. 다른 프로그램이 만든 백업은 거부됩니다. LightCreator에도 같은 두 항목이 있습니다. 레이어 프리셋도 zip에 들어 있습니다. 복원하면 현재 목록 대신 들어가며, 프리셋이 없는 오래된 백업은 목록을 그대로 둡니다. 제어 장치도 zip에 들어 있습니다. 복원하면 세 프로그램 모두에서 현재 장치 대신 들어갑니다.
* 프로젝트(`.cre`), SVG, G-code는 원하는 곳에 저장됩니다.
* 창에는 자체적인 평면 테두리가 있습니다. **설정**(Settings) 메뉴에는 다음이 있습니다.
  * **보기 옵션…**(View options…): **주 격자**(Main grid)와 **보조 격자**(Secondary grid)(표시, 선 간격, 색), **작업 영역**(Work area) 색(**배경 색**(Background colour), **작업 공간 색**(Workspace colour), 자동 또는 사용자 지정), **선 두께**(Line thickness), **레이어 이름 표시**(Show layer names)(레이어 막대의 색 옆에 이름), **저장 후 G-code 미리 보기**(Preview G-code after saving)(저장한 G-code가 G-code 분석에서 열림), **변수 이름 표시**(Show variable names)(사용자 G-code 변수의 이름을 필드 옆에; 클릭하면 복사). **격자 초기화**(Reset grid)는 격자와 색을 되돌립니다.
  * **프로그램 옵션…**(Program options…): **프로그램 시작 시 새 버전 확인**(Look for a newer version when the program starts)(기본값은 켜짐; **도움말 → 업데이트 확인…**(Help → Check for updates…)은 즉시 확인합니다).
  * **플러그인…**(Plugins…), **언어**(Language)(30개, 공유 텍스트와 MillCreator 자체 텍스트용), **색 구성표**(Colour scheme)(18가지, 그리고 명절 무렵의 축제용 구성표).
* 공유 데이터 폴더의 다른 파일: `mill_layer_presets.json`(레이어 프리셋), `saved_positions.json`(기계의 저장된 위치), `camera_recording.json`(카메라 녹화의 폴더, 동영상 형식, 타임랩스), 그리고 아직 저장하지 않은 프로젝트의 녹화를 위한 `camera` 폴더. 작업 영역과 사이드 패널의 창 영역과 탭은 설정과 함께 기억됩니다.
* **도움말 → GitHub 프로젝트 페이지**(Help → Project page on GitHub)는 저장소를 엽니다.

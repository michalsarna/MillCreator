🌐 [English](Architecture) · [Polski](Architektura) · [简体中文](Architecture-ZH) · **한국어**

> ⚠️ 이 페이지는 영어판을 기계 번역한 것으로, 부정확한 부분이 있을 수 있습니다. 의심스러우면 [영어판](Architecture)을 기준으로 하십시오. 수정 제안을 환영합니다.

# 아키텍처

MillCreator는 **baseCreator**(`base-core`: 형상, 장면, SVG / PDF / LightBurn 리더, 다각형 오프셋, 프로젝트 컨테이너; `base-ui`: 창, 편집기, 클립아트, 카메라) 위에 놓인 하나의 바이너리 크레이트 `crates/millcreator`입니다. baseCreator는 git 태그로 고정되어 있습니다.

| 모듈 | 하는 일 |
|---|---|
| `tools`, `tools_ui` | 비트 라이브러리: 종류, 치수, 기성 비트, 파일; 창과 비트 그림 |
| `materials`, `setup_ui` | 재료, 그 두께와 그 위의 원점; 프로젝트 설정 창 |
| `device`, `device_ui`, `wcs_pick` | 장치, 그 파일, 시작 화면 목록, 설정 창, 카메라 탭; 작업 영역에서 고르거나 끌어 정하는 프로그램 원점 |
| `layers`, `layers_ui`, `layer_presets`, `cutting_calc` | 64개 레이어와 그 작업; 레이어 탭과 재료 탭; 레이어 프리셋; Vc와 날당 이송 |
| `toolpath`, `passes`, `clear`, `handtabs`, `relief`, `model3d` | 도형에서 움직임과 G-code로(오프셋에는 `base_core::ops` 사용): 패스, 램프와 고정 탭, 래스터, 잔여 가공과 면 가공, 손으로 놓은 탭, 릴리프로서의 그림, 깊이 맵으로서의 3D 모델 |
| `view3d`, `preview_ui` | 작업 3D 보기; 미리 보기 탭, G-code 내보내기, G-code 미리 보기 텍스트 |
| `panes_glue`, `workspaces` | 작업 영역의 탭 창 영역과 탭으로 열린 도면 |
| `machine_ui`, `duet_glue` | 컨트롤러 연결과 그것을 쓰는 버튼; HTTP로 연결한 Duet 보드 |
| `section`, `ugs_import` | 프로젝트 파일의 `mill` 구역; UGS 디자인의 레이어 |
| `main`, `menu`, `native_menu` | 애플리케이션, 메뉴(`base_ui::menu::standard_menus`의 공통 구성), macOS 메뉴 막대, 단축키, 프로젝트 열기 / 저장 |

로직 모듈에는 UI가 없으며 단위 테스트로 검증됩니다. 창은 `egui_kittest`로 화면 없이 그려서 테스트합니다. `cargo test -p millcreator render_screenshots -- --ignored`는 이 위키의 그림을 렌더링합니다.

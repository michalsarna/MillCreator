🌐 [English](Architecture) · **Polski**

# Architektura

MillCreator to jedna skrzynka binarna, `crates/millcreator`, nad **baseCreatorem** (`base-core`: geometria, scena, czytniki SVG / PDF / LightBurna, odsunięcie wielokątów, kontener projektu; `base-ui`: okno, edytor, cliparty, kamera). baseCreator jest przypięty tagiem git.

| Moduł | Co robi |
|---|---|
| `tools`, `tools_ui` | biblioteka frezów: rodzaje, wymiary, gotowe frezy, pliki; okno i rysunki frezów |
| `materials`, `setup_ui` | materiał, jego grubość i zero na nim; okno ustawień projektu |
| `device`, `device_ui`, `wcs_pick` | urządzenia, ich pliki, lista na ekranie startowym, okno ustawień, karta kamery; zero programu wskazywane albo przeciągane na obszarze roboczym |
| `layers`, `layers_ui`, `layer_presets`, `cutting_calc` | 64 warstwy i ich operacje; karty Warstwy i Materiał; presety warstw; Vc i posuw na ząb |
| `toolpath`, `passes`, `clear`, `handtabs`, `relief`, `model3d` | od kształtów do ruchów i G-code (używa `base_core::ops` do odsunięć): przejścia, rampy i mostki, raster, resztki i planowanie, mostki ustawione ręcznie, obrazy jako reliefy, modele 3D jako mapy głębokości |
| `view3d`, `preview_ui` | widok 3D zadania; karta podglądu, eksport G-code i tekst Podglądu G-code |
| `panes_glue`, `workspaces` | panele kart obszaru roboczego i otwarte rysunki jako karty |
| `machine_ui`, `duet_glue` | połączenie ze sterownikiem i przyciski, które go używają; płytka Duet przez HTTP |
| `section`, `ugs_import` | sekcja `mill` pliku projektu; warstwy projektu UGS |
| `main`, `menu`, `native_menu` | aplikacja, menu (wspólny układ `base_ui::menu::standard_menus`), pasek menu macOS, klawisze oraz otwieranie i zapis projektu |

Moduły logiki nie mają w sobie interfejsu i są pokryte testami jednostkowymi; okna są testowane przez rysowanie bez okna z `egui_kittest`. `cargo test -p millcreator render_screenshots -- --ignored` generuje obrazy tej wiki.

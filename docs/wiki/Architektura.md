🌐 [English](Architecture) · **Polski**

# Architektura

MillCreator to jedna skrzynka binarna, `crates/millcreator`, nad **baseCreatorem** (`base-core`: geometria, scena, czytniki SVG / PDF / LightBurna, odsunięcie wielokątów, kontener projektu; `base-ui`: okno, edytor, cliparty, kamera). baseCreator jest przypięty tagiem git.

| Moduł | Co robi |
|---|---|
| `tools`, `tools_ui` | biblioteka frezów: rodzaje, wymiary, gotowe frezy, pliki; okno i rysunki frezów |
| `materials` | materiał i jego grubość |
| `device`, `device_ui` | urządzenia, ich pliki, lista na ekranie startowym, okno, karta kamery |
| `layers`, `layers_ui` | 64 warstwy, operacje, przejścia; karty Layers i Material |
| `toolpath` | od kształtów do ruchów i G-code (używa `base_core::ops` do odsunięć) |
| `view3d`, `preview_ui` | rzutowanie, rysowanie i sterowanie widokiem 3D; okno i eksport |
| `section` | sekcja `mill` pliku projektu |
| `main`, `menu` | aplikacja, menu oraz otwieranie i zapis projektu |

Moduły logiki nie mają w sobie interfejsu i są pokryte testami jednostkowymi; okna są testowane przez rysowanie bez okna z `egui_kittest`. `cargo test -p millcreator render_screenshots -- --ignored` generuje obrazy tej wiki.

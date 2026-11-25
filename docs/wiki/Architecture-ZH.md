🌐 [English](Architecture) · [Polski](Architektura) · **简体中文** · [한국어](Architecture-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Architecture)为准。欢迎提出修改建议。

# 架构

MillCreator 是一个二进制 crate，`crates/millcreator`，构建在 **baseCreator** 之上（`base-core`：几何、场景、SVG / PDF / LightBurn 读取器、多边形偏移、项目容器；`base-ui`：窗口、编辑器、剪贴画、摄像头）。baseCreator 通过 git 标签固定版本。

| 模块 | 作用 |
|---|---|
| `tools`, `tools_ui` | 刀具库：类型、尺寸、现成刀具、文件；刀具窗口和刀具图片 |
| `materials`, `setup_ui` | 材料、其厚度及其上的零点；项目设置窗口 |
| `device`, `device_ui`, `wcs_pick` | 设备、设备文件、启动界面列表、设置窗口、摄像头标签页；在工作区上拾取或拖动程序零点 |
| `layers`, `layers_ui`, `layer_presets`, `cutting_calc` | 64 个图层及其操作；图层和材料标签页；图层预设；Vc 和每齿进给 |
| `toolpath`, `passes`, `clear`, `handtabs`, `relief`, `model3d` | 从图形到运动和 G-code（偏移使用 `base_core::ops`）：走刀、斜坡和连接桥、光栅、残料和平面铣、手动放置的连接桥、作为浮雕的图片、作为深度图的 3D 模型 |
| `view3d`, `preview_ui` | 任务的 3D 视图；预览标签页、G-code 导出和预览 G-code 文本 |
| `panes_glue`, `workspaces` | 工作区的标签页窗格，以及作为标签页的打开图纸 |
| `machine_ui`, `duet_glue` | 与控制器的连接以及使用它的按钮；通过 HTTP 连接的 Duet 主板 |
| `section`, `ugs_import` | 项目文件中的 `mill` 部分；UGS 设计的图层 |
| `main`, `menu`, `native_menu` | 应用程序、菜单（`base_ui::menu::standard_menus` 的通用布局）、macOS 菜单栏、快捷键以及项目的打开 / 保存 |

逻辑模块中不包含 UI，并由单元测试覆盖；窗口通过 `egui_kittest` 以无头方式绘制来测试。`cargo test -p millcreator render_screenshots -- --ignored` 渲染本 wiki 中的图片。

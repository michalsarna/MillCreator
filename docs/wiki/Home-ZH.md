🌐 [English](Home) · [Polski](Home-PL) · **简体中文** · [한국어](Home-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Home)为准。欢迎提出修改建议。

<p align="center"><img src="https://raw.githubusercontent.com/michalsarna/MillCreator/main/assets/icons/icon-256.png" alt="MillCreator" width="128"></p>

# MillCreator

用 Rust 编写的开源软件，用于设计和准备 CNC 铣削任务。绘制或导入图形，为每个图层指定一种加工操作和一把刀具，在 3D 中查看任务并生成 G-code，支持 macOS、Windows 和 Linux。它是 [LightCreator](https://github.com/michalsarna/LightCreator)（激光）的姊妹程序，通过 [baseCreator](https://github.com/michalsarna/baseCreator) 使用相同的窗口、编辑器和文件。

![图层与编辑器](https://raw.githubusercontent.com/michalsarna/MillCreator/main/docs/screenshots/05-layers.png)

## 从哪里开始

| 我想…… | 前往 |
|---|---|
| 安装程序 | [安装](Installation-ZH) |
| 准备我的第一个任务 | [快速入门](Quick-Start-ZH) |
| 绘制、编辑和导入图形 | [设计工具](Design-Tools-ZH) |
| 把图片以某种风格的线条雕刻出来（点画、波浪线、排线……） | [风格描摹](Style-Trace-ZH) |
| 用拖刀切割，刀具库 | [拖刀](Drag-Knife-ZH) |
| 布置工作区：窗格、标签页、窗口 | [窗格与标签页](Panes-and-Tabs-ZH) |
| 添加我的机器和摄像头 | [设备与摄像头](Devices-and-Cameras-ZH) |
| 说明铣削什么以及如何铣削 | [图层与操作](Layers-and-Operations-ZH) |
| 选择或定义刀具 | [刀具库](Bit-Library-ZH) |
| 设置材料 | [材料](Material-ZH) |
| 在 3D 中查看任务，导出 G-code | [3D 视图与 G-code](3D-View-and-G-code-ZH) |
| 驱动机器：点动、归零、探测、固件、校准 | [机器控制](Machine-Control-ZH) |
| 查找某个菜单项及其作用 | [菜单](Menus-ZH) |
| 学习快捷键 | [键盘快捷键](Keyboard-Shortcuts-ZH) |
| 与 LightCreator 共享项目 | [项目文件](Project-File-ZH) |
| 设置新任务的材料和零点 | [快速入门](Quick-Start-ZH)（项目设置） |
| 看看它的样子 | [截图](Screenshots-ZH) |
| 找到设置文件 | [设置与文件](Settings-and-Files-ZH) |
| 了解代码 | [架构](Architecture-ZH) |
| 了解哪些内容经过测试 | [安全与限制](Safety-and-Limitations-ZH) |

## 已测试的设备

本程序已在以下机器上试用并能正常工作：

| 设备 | 类型 | 控制器 |
|---|---|---|
| TwoTrees TTC450Pro | CNC 铣床 | GRBL |

它在其他机器上也能运行？请在 [issue](https://github.com/michalsarna/MillCreator/issues) 中告诉我，我会把它加入列表。
列表以及如何添加机器：[TESTS_ON_DEVICES.md](https://github.com/michalsarna/MillCreator/blob/main/TESTS_ON_DEVICES.md)。

## 概览

* **设备：** 你的机器（工作区、Z 向行程、安全高度、主轴转速范围、摄像头）集中在一个列表中；**我没有设备**（I have no device）会提供一个名为 Dummy 的替代设备。
* **图层：** 64 个图层，每个图层有一种操作（外侧/内侧切割、沿线加工、环形或光栅型腔、残料型腔、平面铣、钻孔、镗孔、倒角、狗骨槽、带镶嵌的 V 雕、图片和 3D 浮雕）、一把刀具、深度、每层切深、进给、主轴转速、连接桥、斜坡或螺旋下刀、顺铣或逆铣、预留余量、冷却液；这些设置可以保存为预设。
* **刀具：** 现成刀具库和你自己的刀具（为每台设备保存），共七种类型，每种都配有图片，标明每个尺寸的测量位置；支持导入和导出。
* **材料：** 三种颜色的木材、金属、可自选颜色的塑料，以及厚度。
* **3D 视图：** 材料块、切削和在安全高度上的移动；旋转、缩放、重置；滑块可回放任务。
* **G-code：** GRBL 风格，换刀时暂停或为换刀装置使用 `T… M6`，自定义代码行（开始、结束、换刀或更换操作时），工件坐标系以及材料上的零点；通过插件支持 Mach3 / Mach4 / UCCNC 和 LinuxCNC。
* **机器：** GRBL（0.7 至 1.1 及更新版本）、Smoothieware、通过 HTTP 连接的 Duet，或 LinuxCNC（文件）；点动、探测、孔中心、倍率调整、保存的位置、冷却液。
* **绘图：** 导入 DXF、SVG、PDF、Illustrator、LightBurn、MillMage 和 UGS，导出 DXF、Illustrator、LightBurn 和 MillMage，阵列、沿路径复制、路径文字、修剪、排样、二维码和条形码、序列号文字，以及将 STL / OBJ / 3MF 模型用作浮雕。
* **与 LightCreator 共享：** 编辑器、310 张剪贴画、SVG / PDF / Illustrator 和 LightBurn 导入、`.cre` 项目文件、38 种语言（共享文本）和 18 种配色方案。

> ⚠️ **安全：** 铣床是危险的。请先在材料上方的空中测试新任务，佩戴护目镜和耳罩，夹紧工件，切勿让运行中的机器无人看管。参见[安全与限制](Safety-and-Limitations-ZH)。

最新版本：**v1.1.1** · [Releases](https://github.com/michalsarna/MillCreator/releases) · [Issues](https://github.com/michalsarna/MillCreator/issues) · 许可证：[MIT](https://github.com/michalsarna/MillCreator/blob/main/LICENSE) 或 [Apache-2.0](https://github.com/michalsarna/MillCreator/blob/main/LICENSE-APACHE)，任选其一

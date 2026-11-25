🌐 [English](Settings-and-Files) · [Polski](Ustawienia-i-pliki) · **简体中文** · [한국어](Settings-and-Files-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Settings-and-Files)为准。欢迎提出修改建议。

# 设置与文件

语言、配色方案、视图选项、最近打开的文件、**设备列表** 和 **你的刀具** 保存在一个文件 `app.ron` 中：

| 系统 | 位置 |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/millcreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\millcreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/millcreator/app.ron`，默认为 `~/.local/share/baseCreator/apps/millcreator/app.ron` |

* **共享文件夹。** LightCreator、MillCreator 和 PenCreator 把数据保存在同一个文件夹 `baseCreator` 中（上表显示的是其中该程序自己的设置）。**剪贴画库是共享的**：在一个程序中保存或下载的图片会出现在其他程序中（位于 `apps` 旁边的 `clipart` 文件夹，其中的 `index.json` 记录名称、类别、来源、许可证和作者）。每个程序在 `apps/<program>/app.ron` 中保存自己的设置、设备配置、材料和刀具。设置环境变量 `BASECREATOR_DIR` 可以使用另一个文件夹。
* 该文件在程序关闭时写入（也会不时写入）。**删除它即可重置所有内容**：设备列表和你的刀具也会随之删除，因此请先导出它们（设备窗口和刀具库中的 **导出…**（**Export…**））。
* **帮助 → 下载备份…**（**Help → Download backup…**）保存一个 zip，其中包含 MillCreator 的设置（设备列表、刀具、语言、配色方案、最近打开的文件）和整个共享剪贴画库，并附有 README。**帮助 → 从备份恢复…**（**Help → Restore from backup…**）在确认后读回这样的文件：设置会被替换，图片会被 **添加** 到图库中，因此在此期间由其他程序保存的内容不会丢失。由其他程序创建的备份会被拒绝。LightCreator 也有这两个相同的条目。图层预设也在 zip 中；恢复时会替换当前列表，不含图层预设的旧备份不会改动列表。控制设备也在 zip 中；恢复时会在所有三个程序中替换当前的控制设备。
* 项目（`.cre`）、SVG 和 G-code 保存在你选择的任何位置。
* 窗口有自己的扁平边框。**设置**（**Settings**）菜单包含：
  * **视图选项…**（**View options…**）：**主网格**（**Main grid**）和 **次网格**（**Secondary grid**，显示、线间距、颜色），**工作区**（**Work area**）颜色（**背景颜色**（**Background colour**）、**工作空间颜色**（**Workspace colour**），自动或自定义），**线条粗细**（**Line thickness**），**显示图层名称**（**Show layer names**，在图层栏颜色旁边显示名称），**保存后预览 G-code**（**Preview G-code after saving**，保存的 G-code 在 G-code 分析中打开）以及 **显示变量名**（**Show variable names**，在字段旁边显示自定义 G-code 的变量名；单击可复制）。**重置网格**（**Reset grid**）恢复网格和颜色。
  * **程序选项…**（**Program options…**）：**程序启动时检查新版本**（**Look for a newer version when the program starts**，默认开启；**帮助 → 检查更新…**（**Help → Check for updates…**）会立即检查）。
  * **插件…**（**Plugins…**）、**语言**（**Language**，30 种，用于共享文本和 MillCreator 自己的文本）和 **配色方案**（**Colour scheme**，18 种，另有节日前后的节日配色）。
* 共享数据文件夹中的其他文件：`mill_layer_presets.json`（图层预设）、`saved_positions.json`（机器的保存位置）、`camera_recording.json`（摄像头录制的文件夹、视频格式和延时摄影设置），以及用于尚未保存项目的录制内容的 `camera` 文件夹。工作区的窗格和标签页以及侧面板会随设置一起被记住。
* **帮助 → GitHub 上的项目页面**（**Help → Project page on GitHub**）打开代码仓库。

🌐 [English](Keyboard-Shortcuts) · [Polski](Skroty-klawiszowe) · **简体中文** · [한국어](Keyboard-Shortcuts-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Keyboard-Shortcuts)为准。欢迎提出修改建议。

# 键盘快捷键

Windows 和 Linux 上的 Ctrl 在 macOS 上为 **Cmd**。这些快捷键在编辑器中有效（启动界面上无效），并且当文本字段获得键盘焦点时都不起作用：在你输入名称、数字或 G-code 时，Ctrl+C、Delete 或字母键都归该字段使用。

## 菜单快捷键

菜单条目旁边显示的快捷键（参见[菜单](Menus-ZH)）。只要打开了图纸，它们就在 Windows、Linux 和 macOS 上生效；当时显示为灰色的条目不执行任何操作。关于 macOS 的菜单栏，请参见[下文](#macos)。

| 快捷键 | 条目 | 菜单 |
|---|---|---|
| Ctrl+N | **新建**（**New**） | 文件 |
| Ctrl+O | **打开…**（**Open…**） | 文件 |
| Ctrl+S | **保存**（**Save**） | 文件 |
| Ctrl+Shift+S | **另存为…**（**Save as…**） | 文件 |
| Ctrl+Q | **退出**（**Quit**） | 文件（macOS：**MillCreator** 菜单中的 Cmd+Q） |
| Ctrl+Z | **撤销**（**Undo**） | 编辑 |
| Ctrl+Y, Ctrl+Shift+Z | **重做**（**Redo**） | 编辑 |
| Ctrl+C | **复制**（**Copy**） | 编辑 |
| Ctrl+V | **粘贴**（**Paste**） | 编辑 |
| Ctrl+D | **再制**（**Duplicate**） | 编辑 |
| Delete 或 Backspace | **删除**（**Delete**，显示为 *Del*） | 编辑 |
| Ctrl+A | **全选**（**Select all**） | 编辑 |
| Ctrl+G | **组合**（**Group**） | 排列 |
| Ctrl+Shift+G | **取消组合**（**Ungroup**） | 排列 |
| Ctrl+L | **锁定**（**Lock**） | 排列 |
| Ctrl+Shift+L | **解锁**（**Unlock**） | 排列 |
| N | **编辑节点**（**Edit nodes**） | 排列（节点工具，见下文） |
| Ctrl+=（显示为 Ctrl++） | **放大**（**Zoom in**） | 视图 |
| Ctrl+- | **缩小**（**Zoom out**） | 视图 |
| Ctrl+Shift+0 | **使台面适合窗口**（**Fit bed to window**） | 视图 |
| Ctrl+Shift+9 | **显示全部对象**（**Fit all objects**） | 视图 |
| Ctrl+Shift+P | **任务的 3D 视图…**（**3D view of the job…**） | 铣削 |
| Ctrl+E | **导出 G-code…**（**Export G-code…**） | 铣削 |

一个快捷键只属于一个条目（程序的一项测试会检查这一点）：Ctrl+Shift+G 是 **取消组合**，G-code 用 **Ctrl+E** 导出。

## 编辑器的其他快捷键

| 快捷键 | 作用 |
|---|---|
| Delete, Backspace | 删除所选图形；使用节点工具并选中节点时，删除这些节点。 |
| Esc | 清除选择（以及用钢笔正在绘制的线条上的点）。 |
| 按两次 Esc | 半秒内再按一次 Esc 会返回 **选择**（**Select**）工具。 |
| Space（按住） | 使用任意工具时平移视图；松开后返回原来的工具。 |
| 方向键 | 把所选内容移动 1 mm；**Shift** + 方向键 = 10 mm（在控制标签页上勾选 **键盘**（**Keyboard**）时无效）。 |
| G, I | 多边形和螺旋：一个小窗口用于设置边数或圈数，也适用于最后绘制的图形。 |
| Ctrl+1 … Ctrl+9, Ctrl+0 | 显示第 1 … 9 个标签页中的图纸；Ctrl+0 为第 10 个标签页。 |
| Ctrl+W | 关闭当前显示的图纸（会询问未保存的工作）。 |
| Enter | 结束钢笔工具的线条（右键单击或双击也可以）。 |

## 工具（单个字母）

不带 Ctrl 的字母用于选择左侧工具板中的工具。字母包含在工具名称中（在工具提示和 **插入**（**Insert**）菜单中）。

| 快捷键 | 工具 | 快捷键 | 工具 |
|---|---|---|---|
| V | **选择 (V)**（**Select (V)**） | L | **直线 (L)**（**Line (L)**） |
| N | **节点编辑 (N)**（**Node edit (N)**） | P | **折线 / 钢笔 (P)**（**Polyline / pen (P)**） |
| R | **矩形 (R)**（**Rectangle (R)**） | T | **文本 (T)**（**Text (T)**） |
| E | **椭圆 (E)**（**Ellipse (E)**） | H | **平移 (H)**（**Pan (H)**） |
| Y | **三角形 (Y)**（**Triangle (Y)**） | Z | **放大 (Z)**（**Zoom in (Z)**） |
| S | **星形 (S)**（**Star (S)**） | X | **缩小 (X)**（**Zoom out (X)**） |
| G | **多边形 (G)**（**Polygon (G)**） | | |
| K | **心形 (K)**（**Heart (K)**） | M | **标尺 (M)**（**Ruler (M)**） |
| I | **螺旋 (I)**（**Spiral (I)**） | | |

## 工作区中的鼠标操作

| 操作 | 作用 |
|---|---|
| 滚轮，或在触控板上捏合 | 以指针为中心缩放。 |
| 按住中键或右键拖动 | 移动视图，在工作区上及其旁边均可（使用 **平移**（**Pan**）工具时左键也可以；不移动的右键单击会打开菜单）。 |
| Shift + 单击、Ctrl + 单击 | 把图形加入选择，或从选择中移除。 |
| 右键单击（选择工具） | 选中指针下的图形并打开上下文菜单。 |
| 在缩放工具下按 Shift | 互换放大和缩小。 |
| 单击另一个窗格中的图纸（或其标签页） | 使该图纸成为当前图纸。 |
| 在标签页上单击中键 | 关闭该标签页。 |

## 预览（3D 视图）

| 操作 | 作用 |
|---|---|
| 左键拖动 | 旋转视图。 |
| 右键或中键拖动 | 移动画面。 |
| 滚轮 | 缩放。 |
| 双击 | 返回初始视图（与 **重置视图**（**Reset view**）相同）。 |

在 **G-code 分析**（**G-code analysis**，文件 → 查看 G-code 文件…）中，拖动会移动平面文件、旋转 3D 文件（右键拖动或 Shift+拖动用于移动），滚轮用于缩放，**Esc** 关闭它。

## 窗口和字段

* **Esc** 会取消 *关闭前保存你的工作吗？*（*Save your work before closing?*）这一询问（与 **取消**（**Cancel**）相同），并会取消程序零点的 **在工作区上拾取**（**Pick on the work area**）。
* 在 **图层预设**（**Layer presets**）窗口中，Enter 保留预设的新名称，Esc 放弃它。

## 用键盘点动

在 **控制**（**Control**）标签页上勾选 **键盘**（**Keyboard**）后（默认关闭）：方向键移动 X 和 Y，**Page Up** / **Page Down** 移动 Z，步长为其上方选择的值。当某个字段获得键盘焦点或机器忙碌时，不会有任何运动。参见[机器控制](Machine-Control-ZH)。

## macOS

在 macOS 上，程序使用系统的菜单栏。本页上的所有快捷键在那里都与在 Windows 和 Linux 上一样有效，用 Cmd 代替 Ctrl：

* 系统菜单栏接管 **新建**（Cmd+N）、**打开…**（Cmd+O）、**保存**（Cmd+S）、**另存为…**（Cmd+Shift+S）和 **退出**（Cmd+Q，位于 **MillCreator** 菜单中）的快捷键，并在这些条目旁边显示它们。
* 其他所有快捷键都由程序自己处理，方式与其他系统相同：Cmd+E、Cmd+Shift+P、Cmd+L、Cmd+Shift+L、Cmd+= / Cmd+-、Cmd+Shift+0 / Cmd+Shift+9、编辑快捷键、Cmd+1 … Cmd+0、Cmd+W 以及工具字母。系统菜单栏不会在条目旁边显示这些快捷键；本页以及 Windows 和 Linux 的菜单栏会显示。

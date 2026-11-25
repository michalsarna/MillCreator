🌐 [English](Menus) · [Polski](Menu) · **简体中文** · [한국어](Menus-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Menus)为准。欢迎提出修改建议。

# 菜单

LightCreator、MillCreator 和 PenCreator 的菜单布局相同（来自 baseCreator），MillCreator 把它自己的条目放在相应的位置：

**文件 · 编辑 · 插入 · 排列 · 视图 · 机器 · 铣削 · 设置 · 帮助**（**File · Edit · Insert · Arrange · View · Machine · Milling · Settings · Help**）

* **Windows 和 Linux：** 打开图纸后，菜单栏位于窗口内、标题栏下方（启动界面没有菜单栏）。条目的快捷键在那里同样有效（参见[键盘快捷键](Keyboard-Shortcuts-ZH)）。
* **macOS：** 菜单位于屏幕顶部的系统菜单栏中，在启动界面上也有（需要图纸的条目在那里显示为灰色）。第一个菜单 **MillCreator** 包含 **关于 MillCreator**（**About MillCreator**）、服务、隐藏 / 显示以及 **退出**（**Quit**，Cmd+Q），因此 **文件** 中没有 *退出*，**帮助** 中没有 *关于*。菜单的快捷键在那里同样有效，但系统菜单栏只显示新建、打开、保存、另存为和退出的快捷键：参见[键盘快捷键](Keyboard-Shortcuts-ZH#macos)。

快捷键按 Windows 和 Linux 的写法（Ctrl）列出；在 macOS 上 Ctrl 即 Cmd。当前无法使用的条目显示为灰色（例如未打开图纸时的 **保存**、**任务的 3D 视图…** 和 **导出 G-code…**，所选图形少于两个时的 **相互居中**，未选中图片时的 **调整图片…** 和 **描摹图片…**）。

## 文件（File）

| 条目 | 快捷键 | 作用 |
|---|---|---|
| **新建**（**New**） | Ctrl+N | 在单独的标签页中打开一张新的空白图纸，然后打开 **项目设置**（**Project setup**）窗口。参见[快速入门](Quick-Start-ZH)。 |
| **打开…**（**Open…**） | Ctrl+O | 在单独的标签页中打开项目（`.cre`）或 SVG、PDF、Illustrator 图纸；已经打开的项目会被切换显示，而不会打开两次。在这里选择的 DXF、UGS 设计和 3D 模型（STL、OBJ、3MF）会被导入到图纸中。拖放到窗口上的文件也按同样方式处理，图片或 LightBurn 项目（导入）以及 G-code 文件（在 G-code 分析中显示）也是如此。 |
| **最近打开**（**Open recent**） | | 最近的文件（最多 10 个）；**清除列表**（**Clear list**）会清空它。 |
| **保存**（**Save**） | Ctrl+S | 保存项目（`.cre`）。 |
| **另存为…**（**Save as…**） | Ctrl+Shift+S | 以另一个名称保存项目。 |
| **项目信息…**（**Project information…**） | | 文件（名称、文件夹、大小、创建和修改时间）、项目创建时间、创建者、贡献者和编辑记录。每次手动保存都会记录 **设置 → 程序选项 → 作者**（**Settings → Program options → Author**）中设置的人员（姓名、电子邮件、其他；在所有程序中相同）、时间以及程序及其版本，每分钟最多记录一次。 创建者保存全部三项，贡献者只保存姓名和电子邮件（电子邮件与创建者相同时只保存姓名）。 |
| **项目设置…**（**Project setup…**） | | 材料的尺寸和厚度、程序在材料上的零点、Z 从底部算起、工作区即材料。参见[快速入门](Quick-Start-ZH#2-项目设置)。 |
| **导入 ▸ 导入图片…**（**Import ▸ Import image…**） | | 通过导入对话框把图片（PNG、JPG、BMP、GIF、WebP）放到页面上。参见[设计工具](Design-Tools-ZH)。 |
| **导入 ▸ 导入 SVG…**（**Import ▸ Import SVG…**） | Ctrl+I | SVG 的路径被添加到当前打开的图纸中，每条路径放到颜色最接近的图层上，其中的图片放到当前图层上。**打开…** 则会为该文件新建一个标签页。 |
| **导入 ▸ 导入 Adobe Illustrator / PDF / EPS / VectorCraft…**（**Import ▸ Import Adobe Illustrator / PDF / EPS / VectorCraft…**） | | 对 PDF、PDF 兼容的 `.ai`、旧式 PostScript 类型的 `.ai`、EPS 或 VectorCraft 图纸（`.vectorcraft`，包括其中的图片）执行同样的操作。带有 Illustrator 图层的文件会询问是按颜色还是按这些图层分配。 |
| **导入 ▸ 导入 DXF…**（**Import ▸ Import DXF…**） | | 把 ASCII DXF 作为轮廓导入；其图层放到项目的空闲图层上。 |
| **导入 ▸ 导入 LightBurn / MillMage 项目…**（**Import ▸ Import LightBurn / MillMage project…**） | | LightBurn 项目（`.lbrn2` / `.lbrn`）或 MillMage 项目（`.mage`）中的图形。 |
| **导入 ▸ 导入 UGS 设计…**（**Import ▸ Import UGS design…**） | | UGS Designer 的 `.ugsd` 设计，其切削方式作为图层导入。 |
| **导入 ▸ 将 3D 模型导入为浮雕…**（**Import ▸ Import 3D model as relief…**） | | 把 STL、OBJ 或 3MF 模型作为深度图放到浮雕图层上。参见[图层与操作](Layers-and-Operations-ZH#图片)。 |
| **导入 ▸ 用插件导入…**（**Import ▸ Import with a plugin…**） | | 由插件读取的文件。参见[插件](Plugins-ZH)。 |
| **导入 ▸ 生成图形…**（**Import ▸ Generate shape…**） | | 插件的图形生成器窗口（**插入** 菜单中也有）。 |
| **导出 ▸ 导出 G-code…**（**Export ▸ Export G-code…**） | | 与 **铣削 → 导出 G-code…** 相同。 |
| **导出 ▸ 导出 SVG…**（**Export ▸ Export SVG…**） | | 把整张图纸导出为 SVG，尺寸为工作区大小。保留曲线和图片。 |
| **导出 ▸ 导出 PLT / DXF…**（**Export ▸ Export PLT / DXF…**） | | 按你选择的扩展名，把整张图纸的轮廓导出为 HPGL（`.plt`，每个图层一支笔）或 DXF R12。这些是图形，而不是机器的运动。 |
| **导出 ▸ 导出 DXF…**（**Export ▸ Export DXF…**） | | 把整张图纸导出为当前类型的 DXF（AutoCAD 2007）；曲线以 0.01 mm 精度展平。 |
| **导出 ▸ 导出 DXF R12（旧版）…**（**Export ▸ Export DXF R12 (legacy)…**） | | 同样导出，但为 DXF R12，供较老的程序和切割机使用（也称为 *legacy*）。 |
| **导出 ▸ 导出 Adobe Illustrator…**（**Export ▸ Export Adobe Illustrator…**） | | 把轮廓导出为 PostScript 类型的 Illustrator 文件，保留曲线，项目的每个图层对应一个 Illustrator 图层。不包含图片。 |
| **导出 ▸ 导出 LightBurn 项目…**（**Export ▸ Export LightBurn project…**） | | LightBurn 项目（`.lbrn2`，若你这样命名文件则为 `.lbrn`）：包含曲线的图形和图片，项目的每个图层对应一个 LightBurn 图层。 |
| **导出 ▸ 导出 MillMage 项目…**（**Export ▸ Export MillMage project…**） | | 同样导出为 MillMage 项目（`.mage`），材料尺寸为工作区大小。 |
| **导出 ▸ 将浮雕导出为 STL…**（**Export ▸ Export relief as STL…**） | | 与 **铣削 → 将浮雕导出为 STL…** 相同。 |
| **导出 ▸ 工程图 ▸**（**Export ▸ Technical drawing ▸**） | | 把工程图导出为 **PDF**、**PNG**、**JPG**、**DXF** 或 **DXF R12（旧版）**（**DXF R12 (legacy)**）。DXF 是打印出来的 A4 图纸，单位为毫米：轮廓、细线和虚线以及文字各在单独的图层上；尺寸标注给出真实尺寸。 |
| **导出 ▸ 将所选内容导出为 SVG…**（**Export ▸ Export selection as SVG…**） | | 把所选图形（未选择任何内容时为全部图形）写为 SVG。 |
| **导出 ▸ 将所选内容导出为 DXF…**（**Export ▸ Export selection as DXF…**） | | 同样导出为当前类型的 DXF，项目的每个图层对应一个 DXF 图层。 |
| **查看 G-code 文件…**（**View G-code file…**） | | 在 **G-code 分析**（**G-code analysis**）中打开 G-code 文件：以 2D 或 3D 显示其刀具路径、切削长度和时间。 |
| **打印…**（**Print…**） | Ctrl+P | 在纸张打印机上打印设计，或将其保存为 PDF。 |
| **退出**（**Quit**） | Ctrl+Q | 结束程序；会先询问未保存的工作。macOS 上没有此项（在那里位于 **MillCreator** 菜单中）。 |

## 编辑（Edit）

| 条目 | 快捷键 | 作用 |
|---|---|---|
| **撤销**（**Undo**） | Ctrl+Z | 撤回最后一次更改。 |
| **重做**（**Redo**） | Ctrl+Y | 再做一次（也可用 Ctrl+Shift+Z）。 |
| **复制**（**Copy**） | Ctrl+C | 复制所选图形。 |
| **粘贴**（**Paste**） | Ctrl+V | 放入复制的图形。 |
| **再制**（**Duplicate**） | Ctrl+D | 一步复制所选内容。 |
| **删除**（**Delete**） | Del | 删除所选内容（Delete 或 Backspace）；用节点工具选中节点时，删除这些节点。 |
| **全选**（**Select all**） | Ctrl+A | 选中每个图形。 |

## 插入（Insert）

左侧工具板中用于插入对象的工具；选择条目即选择该工具（正在使用的工具会打勾；macOS 菜单栏中不会，那里只对 **网格**、**吸附到网格**、**吸附到对象**、语言和配色方案打勾），标签中的字母就是其快捷键：

**矩形 (R)**（**Rectangle (R)**）、**椭圆 (E)**（**Ellipse (E)**）、**三角形 (Y)**（**Triangle (Y)**）、**星形 (S)**（**Star (S)**）、**多边形 (G)**（**Polygon (G)**）、**心形 (K)**（**Heart (K)**）、**螺旋 (I)**（**Spiral (I)**）、**直线 (L)**（**Line (L)**）、**折线 / 钢笔 (P)**（**Polyline / pen (P)**）、**文本 (T)**（**Text (T)**）。

然后是 **生成图形…**（**Generate shape…**，插件的生成器，参见[插件](Plugins-ZH)）和 **在线剪贴画…**（**Online clipart…**，打开 **剪贴画**（**Clipart**）标签页并在互联网上搜索免费图片）。参见[设计工具](Design-Tools-ZH)。

## 排列（Arrange）

| 条目 | 快捷键 | 作用 |
|---|---|---|
| **左对齐**（**Align left**）、**水平居中对齐**（**Align centre (H)**）、**右对齐**（**Align right**）、**顶端对齐**（**Align top**）、**垂直居中对齐**（**Align centre (V)**）、**底端对齐**（**Align bottom**） | | 对齐所选图形。 |
| **在台面上居中**（**Centre on bed**） | | 把所选内容放在工作区中央。 |
| **相互居中**（**Centre on each other**） | | 把所选图形（两个或更多）放在同一个中心上。 |
| **水平翻转**（**Flip horizontal**）、**垂直翻转**（**Flip vertical**） | | 镜像所选内容。 |
| **顺时针旋转 90°**（**Rotate 90° CW**）、**逆时针旋转 90°**（**Rotate 90° CCW**） | | 把所选内容旋转四分之一圈。 |
| **组合**（**Group**） | Ctrl+G | 把所选图形组成一个组。 |
| **取消组合**（**Ungroup**） | Ctrl+Shift+G | 拆开组。 |
| **锁定**（**Lock**） | Ctrl+L | 图形将无法再移动或更改。 |
| **解锁**（**Unlock**） | Ctrl+Shift+L | 释放被锁定的图形。 |
| **置于顶层**（**Bring to front**）、**置于底层**（**Send to back**） | | 图形的顺序。 |
| **编辑节点**（**Edit nodes**） | N | 节点工具：移动线条上的点。 |
| **转换为路径**（**Convert to path**） | | 把矩形、椭圆、星形……转换为可以编辑节点的路径。 |
| **并集**（**Union**）、**交集**（**Intersection**）、**差集**（**Subtract**）、**异或**（**Exclusive or**） | | 对两个或更多图形进行布尔运算。 |
| **偏移图形…**（**Offset shape…**）、**圆角…**（**Round corners…**）、**转换为曲线**（**Convert to curves**） | | 更大或更小的轮廓、圆角、由直线生成曲线。 |
| **自动连接路径…**（**Auto-join paths…**）、**按容差闭合路径…**（**Close path with tolerance…**） | | 把相接的线条连成路径，闭合几乎闭合的路径。 |
| **狗骨 / T 骨角…**（**Dogbone / T-bone corners…**） | | 在内角处开槽口，使方形零件能装进由圆形刀具铣出的槽中。 |
| **网格阵列…**（**Grid array…**）、**环形阵列…**（**Circular array…**）、**沿路径复制…**（**Copy along path…**）、**路径文字…**（**Text on path…**） | | 按行列、绕中心或沿路径复制；沿路径弯曲的文字。 |
| **按图形修剪…**（**Trim by shape…**）、**删除重复项**（**Remove duplicates**）、**打散**（**Break apart**）、**简化…**（**Simplify…**） | | 用图形剪切线条、删除重复图形、每条轮廓成为一个图形、减少点数。 |
| **序列文字…**（**Serial text…**）、**条形码 / 二维码…**（**Barcode / QR code…**）、**排样…**（**Nest shapes…**）、**测量…**（**Measure…**） | | 文本的编号副本、作为轮廓的码、把图形排布到一块区域上、尺寸和距离。 |
| **调整图片…**（**Adjust image…**）、**描摹图片…**（**Trace image…**） | | 修改页面上的图片；把图片转换为轮廓。 |
| **编辑图片…**（**Edit image…**） | | 所选图片的照片编辑器：画笔、橡皮擦、仿制图章、填充，以灰度进行。 |
| **转换为图片…**（**Convert to image…**） | | 把所选图形（不含图片）转为一张图片，可设置分辨率和尺寸。 |
| **风格描摹…**（**Style trace…**） | | 用 21 种风格之一把所选图片重新绘制为线条。参见[风格描摹](Style-Trace-ZH)。 |
| **几何多边形…**（**Geometric polygons…**） | | 把所选图片转为几何（低多边形）多边形，多边形之间以及轮廓周围留有间隙：即以该风格打开的风格描摹窗口。参见[风格描摹](Style-Trace-ZH)。 |
| **添加为禁入区**（**Add as no-go zone**） | | 每个未锁定的所选图形都会成为设备的一个禁入区（圆形图形为圆，否则为其外接矩形），并从图纸中移除。右键菜单中也有此项。 |

所有这些都在[设计工具](Design-Tools-ZH)中有说明。

## 视图（View）

| 条目 | 快捷键 | 作用 |
|---|---|---|
| **网格**（**Grid**）、**吸附到网格**（**Snap to grid**）、**吸附到对象**（**Snap to objects**） | | 开关（开启时打勾）。网格本身在 **设置 → 视图选项…**（**Settings → View options…**）中设置。 |
| **放大**（**Zoom in**） | Ctrl++ | 以工作区中央为中心放大（按键为 Ctrl 和 `=`）。 |
| **缩小**（**Zoom out**） | Ctrl+- | 缩小。 |
| **使台面适合窗口**（**Fit bed to window**） | Ctrl+Shift+0 | 在窗口中显示整个工作区。 |
| **显示全部对象**（**Fit all objects**） | Ctrl+Shift+9 | 在窗口中显示所有图形。 |
| **工作区自动适合窗口**（**Auto-fit work area to window**） | | 开关（开启时打勾，macOS 除外）：调整窗口大小时，工作区始终适合窗口（手动缩放会关闭此功能）。 |
| **分屏：左右并排**（**Split view: side by side**）、**分屏：上下排列**（**Split view: one above the other**） | | 把中央区域分成窗格（只有一个窗格时会添加第二个）。 |
| **单视图**（**Single view**） | | 所有标签页重新放回一个窗格中。 |
| **显示摄像头叠加**（**Show camera overlay**） | | 显示或隐藏图纸下方的图片。 |
| **显示禁入区**（**Show no-go zones**） | | 以红色绘制设备的禁入区；设备没有禁入区时显示为灰色。 |
| **工程图**（**Technical drawing**） | | 显示或隐藏工程图（以前是编辑器上方的一个按钮）。 |
| **预览**（**Preview**） | | 显示或隐藏预览，与 **预览…** 按钮相同。 |
| **预览 G-code**（**Preview G-code**） | | 在标签页中显示任务的 G-code，与 **预览 G-code** 按钮相同。 |
| **叠加来源…**（**Overlay from…**） | | 在图纸下方放一张照片或摄像头的实时画面，按四个角或参考点对齐。随项目保存。 |

窗格和标签页：[窗格与标签页](Panes-and-Tabs-ZH)。

## 机器（Machine）

先是机器的设置，然后是在机器上运行的内容。参见[机器控制](Machine-Control-ZH)。

| 条目 | 作用 |
|---|---|
| **切换设备…**（**Switch device…**） | 返回启动界面以选择另一台设备。 |
| **设备设置…**（**Device settings…**） | 当前设备的设置（各页面见[设备与摄像头](Devices-and-Cameras-ZH)）。 |
| **固件设置 ($$)…**（**Firmware settings ($$)…**） | GRBL 控制器的设置，带备份。 |
| **校准…**（**Calibration…**） | 每毫米步数和各轴的垂直度。 |
| **对刀仪…**（**Tool length sensor…**） | 对刀仪向导。 |
| **禁入区…**（**No-go zones…**） | | 打开设备窗口的 **禁入区**（**No-go zones**）页面。 |
| **拾取程序零点**（**Pick the zero of the program**） | | 下一次在工作区上的单击会设置程序零点（与设备标签页上的 **在工作区上拾取**（**Pick on the work area**）相同）。 |
| **寻找中心**（**Find the centre**） | | 探针悬在孔中，触碰孔的四个侧面，并把中点设为 X 和 Y 的零点（与控制标签页上相同；需要已连接的机器）。 |
| **保存的位置…**（**Saved positions…**） | | 显示带有已保存位置的控制标签页。 |
| **加工面 ▸**（**Side to cut ▸**） | | 双面任务的顶面或底面（翻转后）（与控制标签页上相同）。 |
| **控制设备 ▸**（**Control devices ▸**） | **控制设备设置…**（Control device settings…）打开键盘和手柄及其按键的窗口；**本机的控制设备…**（Control device of this machine…）为当前打开的机器选择控制设备。参见[控制设备](Control-Devices-ZH)。 |
| **诊断与统计…**（**Diagnostics and statistics…**） | 输入引脚、任务、工时、报警和错误、维护提醒、导出报告。 |
| **通过网络共享 (ser2net)…**（**Share over network (ser2net)…**） | 仅限 Linux：通过网络共享机器的串口。 |
| **边框**（**Frame**） | 沿任务的外框走一圈（需要已连接的机器）。 |
| **开始任务**（**Start job**） | 发送任务（需要已连接的机器）。 |
| **从某行开始…**（**Start from line…**） | 从程序的某一行开始任务。 |
| **检查程序**（**Check the program**） | 控制器以检查模式（`$C`）运行程序：不会有任何运动，会报告它无法理解的行。需要已连接的机器。 |

## 铣削（Milling）

| 条目 | 快捷键 | 作用 |
|---|---|---|
| **刀具库…**（**Bit library…**） | | 现成刀具和你自己的刀具。参见[刀具库](Bit-Library-ZH)。 |
| **刀片库…**（**Knife library…**） | | 现成拖刀和你自己的拖刀。参见[设计工具](Design-Tools-ZH#拖刀)。 |
| **图层预设…**（**Layer presets…**） | | 图层预设窗口：以某个名称保存当前图层，或把预设应用到当前图层上。 |
| **当前图层的操作 ▸**（**Operation of the active layer ▸**） | | 所有操作：所选操作成为当前图层的操作（与图层标签页上相同）。 |
| **手动放置连接桥**（**Place tabs by hand**） | | 开启（和关闭）用鼠标在轮廓上放置连接桥（与图层标签页上相同）。 |
| **添加对位孔**（**Add alignment holes**） | | 在一个空图层上为双面任务添加两个定位销孔（与材料标签页上相同）。 |
| **任务的 3D 视图…**（**3D view of the job…**） | Ctrl+Shift+P | 打开 **预览**（**Preview**）标签页：以 3D 显示材料、切削和移动，并带动画。参见 [3D 视图与 G-code](3D-View-and-G-code-ZH)。 |
| **预览 G-code**（**Preview G-code**） | | 在标签页中以文本形式显示任务的 G-code，而不保存（与 **预览 G-code** 按钮相同）。 |
| **显示工程图…**（**Show technical drawing…**） | | 把零件显示为工程图：前视图、侧视图和俯视图，带尺寸标注和标题栏。也可以用 **视图 → 工程图**（**View → Technical drawing**）。 |
| **导出 G-code…**（**Export G-code…**） | Ctrl+E | 询问文件名（`name.nc`），并通过已启用的后处理器写入项目的 G-code。与 **保存 G-code**（**Save G-code**）按钮相同。 |
| **将浮雕导出为 STL…**（**Export relief as STL…**） | | 把图片的 3D 浮雕保存为 STL 模型（仅当某个使用 3D 浮雕方式的图层上有图片时可用）。 |

## 设置（Settings）

| 条目 | 作用 |
|---|---|
| **视图选项…**（**View options…**） | 主网格和次网格、工作区颜色、线条粗细、**显示图层名称**（**Show layer names**）、**保存后预览 G-code**（**Preview G-code after saving**）、**显示变量名**（**Show variable names**）。参见[设置与文件](Settings-and-Files-ZH)。**G-code 控制台**（**G-code console**）的字体大小和颜色。 |
| **程序选项…**（**Program options…**） | **程序启动时检查新版本**（**Look for a newer version when the program starts**）；3D 视图的 **渲染质量**（**Rendering quality**，1 快速 … 5 清晰）；**自动保存**（**Auto-save**）：自动保存工作（有名称的文件保存在原位，无名称的保存到恢复文件）以及保存频率（10 秒到一小时，初始为 2 分钟）。 |
| **插件…**（**Plugins…**） | 安装、启用和停用以及移除插件，官方列表。参见[插件](Plugins-ZH)。 |
| **语言 ▸**（**Language ▸**） | 程序的 38 种语言。 |
| **配色方案 ▸**（**Colour scheme ▸**） | 窗口的 18 种配色方案，以及圣诞节、新年、万圣节、复活节和中国农历新年前后的节日配色。 |

## 帮助（Help）

| 条目 | 作用 |
|---|---|
| **GitHub 上的项目页面**（**Project page on GitHub**） | 在浏览器中打开 MillCreator 的代码仓库。 |
| **下载备份…**（**Download backup…**） | 把设置和共享剪贴画库保存到一个 zip 中。参见[设置与文件](Settings-and-Files-ZH)。 |
| **从备份恢复…**（**Restore from backup…**） | 读回这样的 zip。 |
| **快速指南**（**Quick guide**，F1） | 程序内置的指南，带搜索功能。 |
| **导出诊断信息 (zip)…**（**Export diagnostics (zip)…**） | 一个包含报告、设置和控制台内容的文件，可附加到错误报告中。 |
| **检查更新…**（**Check for updates…**） | 查找 MillCreator 的新版本。 |
| **关于**（**About**） | 版本、许可证和致谢。macOS 上没有此项（在那里为 **MillCreator → 关于 MillCreator**（**MillCreator → About MillCreator**））。 |

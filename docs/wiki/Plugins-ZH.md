🌐 [English](Plugins) · [Polski](Wtyczki) · **简体中文** · [한국어](Plugins-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Plugins)为准。欢迎提出修改建议。

# 插件

插件在不修改程序的情况下为 MillCreator 增加功能：**剪贴画、机器预设、翻译、其他文件格式的导入器、图形生成器以及 G-code 后处理器。** 它们由 LightCreator、MillCreator 和 PenCreator 共享：插件位于共享数据文件夹（`baseCreator`）中的 `plugins` 文件夹里，每个程序使用适合它的部分。

**设置 → 插件…**（**Settings → Plugins…**）打开插件窗口：从 `.zip` 安装插件、启用和停用插件、移除插件、打开文件夹，以及设置带有设置项的插件：每个有可选内容的插件（**后处理器**（**postprocessor**））在右侧面板中都有自己的标签页，在那里为下一次 G-code 导出选择它。

* **导入器：** **文件 → 导入 ▸ 用插件导入…**（**File → Import ▸ Import with a plugin…**）、拖放，或对扩展名可由插件读取的文件使用打开。图形会被添加到图纸中（处于选中状态，一次撤销即可撤回）。
* **生成器：** **插入 → 生成图形…**（**Insert → Generate shape…**，也可用 **文件 → 导入 ▸ 生成图形…**（**File → Import ▸ Generate shape…**））显示生成器的表单（数字、开关、文本、列表），并添加它生成的图形。
* **后处理器：** 在其插件的标签页中选择某个后处理器后，**导出 G-code…**（**Export G-code…**，以及 **预览 G-code**（**Preview G-code**）标签页）会让程序经过它处理。它只能更改 G-code 行；属于控制器命令（`$…`、`~`、`!`、`?`）或包含控制字符的行会被拒绝，并且不会保存任何内容。直接把任务发送到机器时不经过后处理器。
* MillCreator 还会采用类别为 `mill` 的机器预设（添加设备时出现在 *起点*（*Start from*）列表中），以及 `mill-tools` 数据文件中的刀具（按名称添加到刀具库中，只添加一次）。

**使用 G-code tools（以及任何后处理器），分步说明。** 1. *设置 → 插件…* → *官方插件*（*Official plugins*）→ 安装 **G-code tools**（或 *从 zip 安装插件…*（*Install plugin from zip…*））。2. 右侧面板中会在其他标签页之后出现一个名为 **G-code tools** 的标签页（面板折叠时，出现在窄条上）。3. 打开它并选择 **为行编号 (N10, N20, …)**（**Number the lines (N10, N20, …)**）或 **删除注释**（**Remove comments**）；*无*（*None*）会再次关闭它。4. 现在 **铣削 → 导出 G-code…**（**Milling → Export G-code…**，Ctrl+E，或 **保存 G-code**（**Save G-code**）按钮）会写入处理后的文件；状态栏会说明使用了哪个后处理器。用 **开始**（**Start**）启动的任务按原样发送，G-code 查看器显示写入时的文件。

**安全。** 插件脚本（Rhai）在沙箱中运行，无法访问文件、网络、系统或机器，并有时间和内存限制。即便如此，插件仍然是别人编写的代码，它决定了你的机器被告知做什么：只安装你信任的插件，并在机器上运行新的后处理器之前先查看其 G-code。

## 官方插件

**设置 → 插件… → 官方插件**（**Settings → Plugins… → Official plugins**）显示官方插件列表，该列表保存在仓库 [baseCreator-plugins](https://github.com/michalsarna/baseCreator-plugins) 中。按 **检查更新**（**Check for updates**）加载列表：它列出 *所有* 插件，包括用于其他程序的插件（标记为 *适用于：…*（*For: …*））。**安装**（**Install**）从官方网站下载插件（会校验其校验和）到所有程序共享的插件文件夹中；当列表中有更新的版本时会出现 **更新**（**Update**），鼠标悬停时显示更新日志。需要比你的程序更新版本的插件会显示出来，但不会提供安装。程序启动时检查更新默认 **开启**（取消勾选 *启动时检查插件更新*（*Check for plugin updates at start*）即可关闭）：当有插件过期时，会有一个窗口列出它们，并提供 **全部更新**（**Update all**）、**插件**（**Plugins**）和 **稍后**（**Later**）。没有你的点击，不会安装任何内容。

编写你自己的插件并将其提交到列表：参见插件仓库中的[作者指南](https://github.com/michalsarna/baseCreator-plugins/blob/main/docs/GUIDE.md)，其中有模板；问题和想法请提交到该仓库的 issues。

如何编写插件（清单、表单、脚本函数、示例）：[baseCreator 中的 PLUGINS.md](https://github.com/michalsarna/baseCreator/blob/main/docs/PLUGINS.md)。

**G-code dialects**（官方插件，MillCreator 的后处理器）：**Mach3 / Mach4 / UCCNC**（开头为安全状态，换刀暂停以 `(MSG, …)` 消息加 `M0` 表示，在 `M30` 之前关闭主轴）和 **LinuxCNC**（程序位于两行 `%` 之间，`G64 P0.02`，同样的消息，结尾为 `M2`）。它们与其他后处理器一样，在右侧面板中该插件的标签页里选择。它们是根据手册编写的，尚未在这些控制器上试用。

可以同时启用多个后处理器：在各插件的标签页中勾选它们；它们 **按勾选的顺序** 运行（一个的输出行传给下一个），状态栏会用 `+` 连接列出它们的名称。**无**（**None**）会把它们全部关闭。

🌐 [English](Web-Server-and-API) · [Polski](Serwer-WWW-i-API) · **简体中文** · [한국어](Web-Server-and-API-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Web-Server-and-API)为准。欢迎提出修改建议。

# Web 服务器与 API

**设置 → 程序选项… → Web 服务器和 API**（**Settings → Program options… → Web server and API**）开启 MillCreator 内置的一个小型 Web 服务器。它 **默认关闭**。它在浏览器中（包括手机上）显示机器、项目和摄像头的状态，并为其他程序提供 API。

**每个请求都需要访问口令**（在同一选项中设置，用其旁边的 *保存*（*Save*）按钮保存；初始为 `MillCreator`，在允许其他计算机访问之前请更改它）。浏览器会询问口令（用户名任意，口令填在密码字段中）；程序以 HTTP Basic 认证发送口令，例如 `curl -u any:MillCreator http://localhost:8766/api/status`。错误的口令会在短暂等待后得到 401 响应。**没有加密**，因此任何能够监听网络的人都可以读取口令和数据，并且口令以 base64 形式保存在 `the settings of the program` 中（一眼看不出来，但 base64 是一种编码，而不是加密）。默认情况下，服务器只在本机上监听（`http://localhost:8766`）。*可从网络上的其他计算机访问*（*Reachable from other computers on the network*）使其在所有网络接口上监听：此时网络上的任何人都可以看到机器，并能停止或移动它。只在你信任的网络中使用此选项，切勿把端口开放到互联网。

同一个服务器存在于该系列的每个程序中（LightCreator、MillCreator、PenCreator、BaseCreator），各自使用自己的端口，以便它们都能在同一台计算机上运行：**8765** LightCreator、**8766** MillCreator、**8767** PenCreator、**8768** BaseCreator（BaseCreator 只显示项目）。

## 网页
`http://localhost:<port>/` 有一条栏，显示设备、其状态、位置、任务的已用工作时间与估计时间以及进度条，还有一个 **主题**（**Theme**）列表：页面初始使用程序的配色方案（提供全部 18 种方案；你的选择保存在浏览器中）。

在计算机上，页面分为两栏：左侧三分之二为 **摄像头**（一个列表用于选择当前摄像头，这会改变设备的当前摄像头；没有摄像头时显示一个被划掉的大摄像头图标），右侧三分之一为 **控制面板**（停止、回原点、取消点动以及八个点动箭头，每次走控制标签页上所选长度的一步），其下方是以图片形式显示的 **项目**。双击图片（或按全屏按钮）可将其放大到整个窗口，并带有放大、缩小、移动和重置视图的按钮；鼠标滚轮、拖动和双指捏合也可以使用，Esc 关闭。

摄像头画面的旋转与程序中相同：设备设置中的 *摄像头旋转*（*Camera rotation*）为默认值（顺时针 0°、90°、180°、270°），画面上方的列表允许你选择另一种旋转，浏览器会为每个摄像头分别记住。`GET /api/cameras` 给出每个摄像头的 `rotation` 和 `rotated`（快照中已经包含了多少旋转）。

在手机上，页面为单栏：状态、摄像头、项目，底部为控制面板。

## API
除非另有说明，所有响应均为 JSON。命令需要使用 `POST`。

| 请求 | 作用 |
|---|---|
| `GET /api/status` | 程序、设备（名称、控制器、是否已连接、状态、X、Y）、任务（是否运行中、已完成 / 总行数、已用和估计秒数）以及项目（文件名、修订版本、描述） |
| `GET /api/project.svg` | 以 SVG 形式返回项目 |
| `GET /api/cameras` | 摄像头：索引、名称、是否启用、是否当前、`rotation`、`rotated`、`url`（摄像头自身的 http / https 地址，或 `null`）和 `snapshot` |
| `GET /api/camera.jpg` | 摄像头视图的最新画面，JPEG 格式（仅当程序中的摄像头视图打开时；否则为 503） |
| `POST /api/stop` | 停止正在运行的任务（与停止按钮相同） |
| `POST /api/home` | 回原点，仅在已连接且没有任务运行时 |
| `POST /api/cancel-jog` | 取消点动 |
| `POST /api/jog?dir=…` | 点动一步：`up`、`down`、`left`、`right`、`up-left`、`up-right`、`down-left`、`down-right`；仅在已连接且没有任务运行时，并且在允许的区域内 |
| `POST /api/camera/select?index=N` | 使摄像头 N 成为当前摄像头 |

这些命令遵循它们所对应按钮的规则。这里的所有内容都尚未在真实硬件上试用；服务器本身有测试覆盖。

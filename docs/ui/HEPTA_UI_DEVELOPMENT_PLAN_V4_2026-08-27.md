# Hepta UI Development Plan v4 — 2026-08-27

## 0. 文档身份与权威边界

本文件是 Hepta 最新总开发计划的 UI 执行附录，用于把桌面端、移动端、Control UI 与 Native UI 的下一阶段开发绑定到可复核的源代码和证据链。它深化、细化并操作化 UI 开发，但不替代 Hepta vNext 总计划、qualification index、CALLERS ratchet、operator acceptance 或 production promotion receipt。

### 0.1 精确绑定

| 项目 | 绑定值 |
|---|---|
| 最新总计划快照提交 | `fe0889ecd46a5fc89de7b1ff3f28158c133a3502` |
| 总计划文件 | `docs/hepta-vnext/dropbox-current-2026-08-27/root/hepta-vnext-development-plan-final-2026-08-23.md` |
| 最新计划所指 canonical source head | `a85612afb43af722c61b54efe73570b25e9e4031` |
| canonical source tree | `71026adff61523660d953867188f094184cee2e9` |
| UI 实现基线提交 | `647e294522a3b3341b4169e3f5a85f8f0df42cbe` |
| UI 实现基线 tree | `643cba1d56e99205d0db042277dee2a498d0cbae` |
| UI 实施分支 | `codex/ui-light-glass-v4-20260827` |
| 计划日期 | `2026-08-27`，Asia/Singapore |

### 0.2 当前状态

- `plan_authority=false`
- `production_authority=false`
- `effect_authority=false`
- `live_adapter_authority=false`
- `operator_acceptance=false`
- `promotion=false`
- `release_claim=false`
- `ui_implementation_allowed=true`
- `local_static_preview_allowed=true`
- `read_only_same_origin_allowed=true`

最新 Dropbox 文档快照仍是 transport/document snapshot，E.45 binding 未形成有效 receipt。本 UI lane 因此只能推进代码、静态合同、只读预览、测试和视觉 qualification 准备，不能通过 UI 文案或启用态暗示已经获得 mutation、provider effect、production caller 或 release 权限。

---

## 1. 目标

### 1.1 产品目标

把 Hepta 从“页面覆盖较高的浅色磨砂玻璃预览”推进到以下状态：

1. 桌面端以 chat-first workspace 为第一读取面，稳定内容清晰，任务、证据、审批和运行状态具备明确的信息层级。
2. 移动端以 Chats → Thread → Room 的分层导航为主路径，支持安全区、键盘、动态字体、横屏、窄屏和低透明度模式。
3. Control UI 和 Native UI 使用同一语义设计合同，但按 Web、Windows、macOS、iOS、Android 分别映射 renderer，不强行复制单一平台视觉。
4. 浅色玻璃只用于 chrome 与 transient controls；消息、正文、表格和长时阅读区域保持稳定不透明。
5. 所有可见能力都同时表达 capability、authority、readiness 和 evidence，不再用“按钮存在”代表“业务已经完成”。
6. 每一次视觉验收都绑定 commit、tree、token digest、设备、系统、缩放、透明度偏好和截图 digest。

### 1.2 工程目标

- 建立一个可机器验证的 v4 材质合同。
- 将 Web 稳定内容 backdrop blur 收敛为 0。
- 将同一像素可见 backdrop 层数限制为不超过 2。
- 将 Web 与 Native 触控/点击目标下限统一为 48 logical pixels。
- 将消息与正文默认字号提升到 15，元数据下限提升到 12。
- 将移动浮层从固定坐标优先改为 viewport-safe bottom sheet / anchored popover。
- 保留 forced-colors、reduced-transparency、reduced-motion 和高对比降级。
- 为计划、token、CSS、Native constants、authority matrix 和 visual matrix 建立静态 gate。
- 建立独立 GitHub Actions 工作流，不能把未运行的真机验收标成通过。

### 1.3 非目标

本计划不在当前 lane 中：

- 解冻 production mutation；
- 启用外部 provider effect；
- 将 read-only Control UI 宣称为生产控制台；
- 伪造 iOS/Android/Windows/macOS 真机通过；
- 通过视觉更新绕过 authentication、authorization、operator confirmation 或 receipt；
- 直接删除 legacy CSS，而没有覆盖率和视觉基线；
- 在没有平台 renderer 的情况下宣称已使用系统 Liquid Glass、Mica 或 Material 3 动态材质。

---

## 2. 设计原则

### 2.1 内容优先，玻璃从属

- 环境层建立空间，不承担关键文本。
- 稳定内容层承载消息、表格、日志、正文和长时间阅读，`backdrop_blur=0`。
- Chrome 层只用于导航、侧栏、top bar、composer 和必要的 persistent controls。
- Transient 层只用于 menu、popover、tooltip、command palette 和短时 sheet。
- 禁止在 glass chrome 内再放置持续模糊的稳定内容卡片。
- 禁止依赖纯透明度区分关键状态。

### 2.2 平台语义共享，材质实现分离

| 平台 | 环境/窗口基底 | Chrome | Transient | 稳定内容 |
|---|---|---|---|---|
| Web | solid/gradient environment | bounded backdrop blur | bounded backdrop blur + scrim | opaque/near-opaque, no blur |
| Windows | Mica-like app backdrop | platform-aware translucent chrome | Acrylic-like transient | solid content |
| macOS | window background / system material | sidebar/titlebar material | popover material | solid content |
| iOS | system background | navigation/control Liquid Glass where supported | system menu/sheet material | system grouped/background surfaces |
| Android | Material 3 dynamic tonal surface | tonal app bar/navigation | modal/sheet tonal elevation | tonal solid content |

平台不支持或用户关闭透明度时，所有 glass role 必须降级为不透明语义色，布局和对比度不得改变。

### 2.3 状态真相优先

每个动作必须具备以下独立字段：

- `visible`
- `enabled`
- `capability_available`
- `authority_granted`
- `confirmation_required`
- `evidence_available`
- `effect_allowed`
- `production_allowed`

UI 不得从视觉样式推导 authority。即使按钮为 primary，也不能代表 production allowed。

---

## 3. 信息架构

### 3.1 桌面主结构

1. **Application rail**：Chat、Tasks、Ops、Evidence；只展示高频产品入口。
2. **Conversation rail**：搜索、过滤、会话、未读与状态；宽度可调，最小宽度由内容而非固定像素决定。
3. **Thread workspace**：消息流、任务上下文、artifact preview 和 composer；是主要阅读与操作面。
4. **Context inspector**：证据、权限、运行状态、room 信息；可折叠，不与消息正文争夺视觉权重。
5. **Transient command layer**：command palette、context menu、tools popover；关闭时不参与布局或点击树。

### 3.2 移动主结构

- Root：Chats。
- Level 2：Thread。
- Level 3：Room/Context。
- Composer 固定在安全区域之上，但由键盘/IME-aware layout 管理，不通过重复 viewport resize 修正。
- Tools、command、artifact selector 在窄屏使用 bottom sheet；宽屏可使用 anchored popover。
- 返回、标题、详情和关键动作保证 48 logical pixels hit target。
- 320 logical pixels 下不发生横向滚动，非关键状态折叠到详情而不是压缩字号。

### 3.3 平板与窄桌面

- 701–980：双层布局，conversation rail 上移或压缩，但 thread 保持稳定。
- 981–1279：双栏优先，context inspector 默认折叠。
- 1280+：三栏可用。
- 使用 container query 决定组件内部布局，viewport media query 只决定全局模式。

---

## 4. Design System v4

### 4.1 材质角色

| Role | 用途 | Alpha 建议 | Blur | 最大层数 |
|---|---|---:|---:|---:|
| `environment` | 页面/窗口背景 | 1.00 | 0 | 1 |
| `content` | 消息、正文、表格、输入内容区 | 0.96–1.00 | 0 | 1 |
| `chrome` | rail、topbar、composer | 0.86–0.94 | 10–14 | 1 |
| `transient` | menu、popover、sheet | 0.92–0.98 | 14–20 | 1 |
| `fallback` | reduced transparency / unsupported | 1.00 | 0 | 1 |

约束：

- `maxVisibleBackdropLayers <= 2`
- `maxStableContentBackdropLayers = 0`
- `content.blur = 0`
- `fallback.blur = 0`
- 同一 interaction path 中最多一个 transient glass。
- 滚动容器内部禁止大面积持续 backdrop blur。

### 4.2 字体

| Token | 下限 | 用途 |
|---|---:|---|
| `display` | 24 | 页面标题、空状态主标题 |
| `title` | 18 | 区域标题 |
| `body` | 15 | 正文、表格、设置说明 |
| `message` | 15 | 消息正文 |
| `label` | 13 | 控件标签 |
| `metadata` | 12 | 时间、路由、状态、辅助说明 |
| `micro` | 11 | 仅非关键、可放大、非长文本信息 |

- 不允许通过 9–10px 字号容纳更多状态。
- 动态字体 200% 下，关键操作不能被裁切或遮挡。
- 中文/英文混排采用系统字体回退，保持 1.4–1.6 正文行高。

### 4.3 间距与尺寸

- 基础 spacing：4、8、12、16、24、32。
- Control radius：10–12。
- Panel radius：14–16。
- Floating radius：18–22。
- Touch target：48。
- Keyboard focus ring：至少 2px，外偏移 2px；高对比模式 3px。
- 可滚动内容末端必须留出 composer/safe-area 所需空间。

### 4.4 运动

- fast：100ms。
- normal：180ms。
- layout transition：180–240ms。
- 禁止将模糊值从 0 动画到高值作为主要反馈。
- `prefers-reduced-motion` / system remove animations 下关闭非必要动画。
- 所有状态变化必须在无动画时仍可理解。

---

## 5. Capability 与 Authority 展示

### 5.1 能力等级

1. `DOCUMENTED`：仅文档/route 存在。
2. `SAMPLE`：固定样例数据。
3. `READ_ONLY_LIVE`：真实数据，只读。
4. `PREPARE_ONLY`：生成计划/草稿，不执行。
5. `CONFIRMABLE`：存在明确人类确认入口。
6. `EFFECT_ENABLED`：通过独立 authority 与 receipt 启用 effect。
7. `PRODUCTION`：获得 production caller、operator acceptance 和 promotion receipt。

当前 Control UI 默认不得超过 `READ_ONLY_LIVE`；未绑定 live adapter 时保持 `SAMPLE/LOCAL_READ_ONLY`。

### 5.2 视觉规则

- disabled 不等于 unavailable：必须显示原因。
- gated 不等于 error：使用 lock/gate 语义，不使用 danger。
- unverified 不得使用 success 色。
- sample 与 live 必须在首屏可区分。
- authority 状态必须来自结构化数据，不从文本字符串推断。
- 所有危险动作必须同时显示 scope、target、dry-run、confirmation 和 evidence digest。

---

## 6. 实施分解

### U0 — 真相源与绑定（P0）

交付：

- 本计划；
- material contract；
- state/authority matrix；
- visual qualification matrix；
- 静态 gate；
- 独立 CI。

完成条件：所有文件绑定 exact source/UI SHA；任何 production flag 为 true 时 gate 失败。

### U1 — Web 材质与排版收敛（P0）

交付：

- v4 cascade 最后覆盖层；
- stable content no-blur；
- chrome/transient role；
- 15px message/body、12px metadata；
- 48px target；
- reduced transparency/forced colors/reduced motion fallback。

完成条件：静态 gate 通过；浏览器 smoke 不出现横向滚动和不可点击遮挡。

### U2 — 移动布局重构（P0/P1）

交付：

- mobile topbar 48 target；
- tools/command/artifact bottom sheet；
- `100dvh` + safe area；
- IME-aware composer；
- 320px reflow；
- 360/390/412/600 视觉基线。

完成条件：动态字体 200%、键盘打开和横屏不遮挡关键动作。

### U3 — Native typography 与 material semantics（P0/P1）

交付：

- Native v4 constants；
- later-loaded typography overrides；
- 15/15/12 字体下限；
- 48 logical pixel control contract；
- stable/chrome/transient/fallback roles。

完成条件：Makepad 编译、登录页、rooms list、thread、settings 和 modal smoke 通过。

### U4 — 平台 renderer（P1）

交付：

- Windows Mica/Acrylic adapter；
- macOS visual effect adapter；
- iOS system glass/material adapter；
- Android Material 3 tonal/dynamic-color adapter；
- unsupported/reduced-transparency solid fallback。

完成条件：平台 renderer 不改变 semantic role 和 authority state；所有 fallback 达到对比度要求。

### U5 — 组件与 legacy CSS 清理（P1）

交付：

- component-scoped selectors；
- backdrop budget lint；
- hard-coded color lint；
- z-index registry；
- popover positioning normalization；
- legacy CSS usage manifest。

完成条件：不再新增非白名单 `!important`；legacy CSS 每次删除有视觉和功能回归证据。

### U6 — Live read-only binding（P1/P2）

交付：

- strict typed snapshot；
- same-origin GET registry；
- loading/empty/error/stale/partial/offline 状态；
- freshness、source 和 evidence digest；
- cancellation、timeout 和 bounded payload。

完成条件：未授权 mutation 请求在 UI、client 和 server 三层 fail-closed。

### U7 — Mutation/approval preparation（P2）

交付：

- prepare-only plan；
- scope/target/diff/evidence；
- explicit confirmation；
- idempotency key；
- receipt readback；
- rollback/compensation view。

完成条件：没有独立 effect authority 时只能复制/导出计划，不能发出 effect。

### U8 — Visual qualification（P1/P2）

交付：

- 桌面/移动截图；
- accessibility tree；
- contrast；
- focus order；
- 320 reflow；
- dynamic type；
- reduced transparency；
- frame time 与功耗档位。

完成条件：所有证据绑定 candidate commit/tree/token digest；`NOT_RUN` 不得被折算为 PASS。

### U9 — Canonical integration 与 release（P2）

交付：

- UI branch → canonical integration 的可追踪移植；
- CALLERS/authority 状态不变证明；
- operator acceptance；
- release artifact；
- SBOM、签名、rollback 和 release notes。

完成条件：独立 promotion receipt；否则只能保持 candidate/internal preview。

---

## 7. 当前实施批次

本分支首批直接实现：

- [x] 新建 UI v4 计划及 exact binding。
- [x] 新建 v4 material contract。
- [x] 新建 capability/authority matrix。
- [x] 新建 visual qualification matrix，默认 `REQUIRED_NOT_RUN`。
- [x] 新建 Web v4 cascade。
- [x] 将 Web v4 cascade 置于 responsive 之后、accessibility 之前。
- [x] 新建 Native v4 semantic/typography override 模块。
- [x] Native module 在 legacy styles 后加载。
- [x] 新建静态 fail-closed gate 与 CI workflow。
- [ ] 浏览器实际 screenshot capture。
- [ ] Native desktop window capture。
- [ ] Android emulator/真机 capture。
- [ ] iOS device/simulator capture。
- [ ] Windows/macOS 平台材质 adapter。
- [ ] live adapter 与 mutation authority。

勾选项代表本代码批次的源码交付，不代表 runtime、真机或 production qualification。

---

## 8. 测试与验收矩阵

### 8.1 静态门禁

- JSON/YAML 可解析；
- exact binding 存在；
- production/effect/promotion flags 为 false；
- content blur 为 0；
- backdrop layer 上限不超过 2；
- target 不低于 48；
- message/body 不低于 15；
- metadata 不低于 12；
- v4 CSS 加载顺序正确；
- reduced transparency/forced colors/reduced motion 规则存在；
- visual rows 未运行时保持 `REQUIRED_NOT_RUN`。

### 8.2 Web runtime

- Chrome、Firefox、Safari、Edge；
- 320、360、390、412、600、768、980、1280、1440、1920；
- keyboard only；
- 200% zoom；
- forced colors；
- reduced transparency；
- reduced motion；
- popover open/close；
- no-JS anchor fallback；
- same-origin request audit。

### 8.3 Native runtime

- macOS desktop；
- Windows desktop；
- Linux reference；
- Android phone/tablet；
- iOS phone/tablet；
- login、rooms list、thread、composer、settings、modal、image viewer；
- desktop ↔ mobile adaptive transition；
- keyboard/IME；
- window restore；
- dynamic type/accessibility scale；
- low transparency/high contrast。

### 8.4 性能预算

- Stable scroll 不触发大面积 backdrop blur。
- 60Hz 目标设备滚动 P95 frame time ≤ 16.7ms；高刷新设备按刷新周期评估。
- 打开 transient layer 不造成持续全屏重绘。
- Mobile composer 输入 P95 interaction latency ≤ 100ms。
- 页面首次可交互使用真实设备/浏览器记录，不用静态 grep 代替。

---

## 9. 风险与回滚

| 风险 | 防护 | 回滚 |
|---|---|---|
| v4 overlay 与 legacy 冲突 | v4 独立 cascade、静态 selectors gate | 删除单一 import |
| Native duplicate token 不兼容 | later-loaded isolated module、compile gate | 移除 module registration |
| 过度实色化损失层级 | role-based chrome/transient | 回滚 v4 CSS，不改基础 token |
| 字体增大导致布局溢出 | reflow、container query、动态字体矩阵 | 局部修复布局，禁止降低字体逃避 |
| mobile sheet 遮挡 composer | safe-area/IME matrix | 恢复原 popover，记录 blocker |
| UI 暗示未授权能力 | authority matrix + copy gate | fail-closed 隐藏/禁用并显示原因 |
| UI 分支与 canonical 漂移 | exact SHA binding + compare gate | 停止 promotion，重新 rebase/rebind |

---

## 10. Definition of Done

一个 UI 阶段只有同时满足以下条件才可完成：

1. 源码、计划、token、截图和测试绑定同一 candidate。
2. 静态 gate、浏览器 smoke、Native compile 和目标平台测试均有结果。
3. `NOT_RUN`、`BLOCKED`、`UNVERIFIED` 没有被包装成 PASS。
4. 桌面和移动都覆盖 loading、empty、error、offline、stale、partial、gated 和 success。
5. 320 reflow、200% zoom、dynamic type、keyboard、reduced transparency 和 forced colors 通过。
6. Stable content 无 backdrop blur；最大可见 backdrop 层数不超过 2。
7. 触控目标、字号、焦点和对比度达到合同下限。
8. capability 与 authority 结构化展示，不通过颜色或按钮存在推断权限。
9. 没有 operator acceptance 和 promotion receipt 时，不声明 production/release。
10. 变更具备明确回滚路径，且不修改后端安全边界。

---

## 11. 下一批执行顺序

1. 运行本批静态 gate 与 token check。
2. 启动 Control UI，采集 320/390/768/1280/1440 Chrome 基线。
3. 修复 v4 overlay 与 legacy cascade 冲突。
4. 编译 Native 并采集 macOS/Android 登录、rooms list、thread、settings 基线。
5. 完成 mobile bottom sheet 的焦点管理、escape/back 和 scroll lock。
6. 引入 backdrop-budget 与 hard-coded-style lint。
7. 实现 Windows/macOS/iOS/Android renderer adapter 草案。
8. 建立 source-bound visual receipt。
9. 将 UI candidate 以非 force、可审查方式移植到 canonical integration lane。
10. 在独立 operator acceptance 前保持所有 production/effect flags 为 false。

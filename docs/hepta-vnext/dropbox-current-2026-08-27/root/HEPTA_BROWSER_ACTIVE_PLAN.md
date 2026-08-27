# Hepta Browser active-plan index

**Index status:** `CANONICAL_ACTIVE / PLANNING_ONLY`  
**Last updated:** 2026-08-27  
**唯一活动阶段命名：** `WEB-C0` … `WEB-C7`

任何实现、CI、路由或发布脚本只允许读取下列文件：

1. `hepta-vnext-development-plan-final-2026-08-23.md` 的末尾
   `WEB-PLAN-2026-08-27D` successor；
2. `hepta-browser-servo-development-plan-2026-08-27.md`（规范详细计划）；
3. `hepta-browser-runtime-plan-2026-08-27.md`（`WEB-R-PLAN-2026-08-27D`）；
4. `HEPTA_BROWSER_STAGE_MATRIX_v1_3.yaml`（唯一活动矩阵）；
5. `hepta.browser_receipt.v2.schema.json`（唯一活动浏览器 receipt schema）；
6. `hepta.browser_plan_receipt.v1.schema.json`（活动计划绑定 receipt 的 schema）；
7. `HEPTA_BROWSER_SERVO_PLAN_RECEIPT-2026-08-27.json`（当前绑定 receipt）；
8. `hepta-browser-engine-selection-2026-08-27.md`（仅作 Servo-only 决策备忘录）。

根目录的 `hepta.browser_receipt.v1.schema.json` 只为旧历史记录保留，不能用于新的
browserd receipt；新实现必须使用 v2。

## 硬约束

- 唯一 live engine：Servo；每个 session 一个 WebView/DOM/JS/storage/event loop；
- Agent semantic API 和真人 headed UI 进入同一个 BrowserActor/arbiter；
- Obscura 只提供合同、ref/snapshot 规则和 fixture 参考，不进入 Servo runtime dependency graph；
- 不实现 Obscura+Servo 混合内核、live page migration、第二 page owner 或当前 Chromium backend；
- Hosted `web_search` 是 Broker 的并列可选后端，不是 browserd 依赖；
- 当前不启用生产 listener、持久凭证、验证码/反爬绕过或外部网页写操作。

旧的 A/B/C append sections 和附件仍可能在主计划的历史 append-only 文本中出现，但它们不再
是实现来源；旧附件已移至：
`archive/2026-08-27-browser-plan-superseded/README-SUPERSEDED.md`。

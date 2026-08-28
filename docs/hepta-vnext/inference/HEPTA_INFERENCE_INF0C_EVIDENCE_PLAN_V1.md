# Hepta 本地推理 INF-0C 取消与受控重启证据计划 v1

> Plan ID: `HEPTA-INFERENCE-INF0C-EVIDENCE-V1`  
> Date: `2026-08-28`  
> Parent plan: `HEPTA-INFERENCE-RUNTIME-V2`  
> Parent branch: `codex/hepta-inference-runtime-v2-20260828`  
> Parent receipt head: `68c97b7d1211c8e319df3b850182401ab541eea4`  
> Parent tree: `3a0d8795db033f780f2e9715b2bc20a2cafa627a`  
> Status: `SOURCE_BOUND_QUALIFICATION_ONLY`  
> Authority: production, effect, Memory/KG, route/fleet, model/NPU, remote inference and promotion remain closed.

## 1. 目标

本 tranche 只补齐 INF-0C 尚缺的真实软件证据工具：

1. 对固定、预安装的 Ollama 与 LM Studio 模型执行流式请求；
2. 收到首批流字节后主动关闭客户端连接；
3. 明确区分“客户端传输断开”与“后端已确认取消”；
4. 通过受信、摘要锁定的本机 helper 执行服务重启；
5. 必须观察服务先不可用、后恢复；
6. 重启后重新完成 exact model discovery 与最小推理；
7. receipt 只记录 hash、长度、时延、状态和布尔事实，不记录 prompt 或模型原文；
8. 大型 harness 拆为受 source gate 覆盖的有界 `.inc.py` 分片，loader 只做确定性组合。

本 tranche 不创建 `hepta-inferd`，不实现 native worker，不下载模型，也不授予任何产品运行权限。

## 2. 取消证据分级

### 2.1 本 tranche 可以证明

```text
TRANSPORT_DISCONNECT_WITH_POST_HEALTH_V1
```

含义：

- 流式 Responses 请求已获得 2xx；
- 已收到非空前缀；
- 客户端主动关闭连接；
- 关闭后 provider 仍能完成模型发现和最小推理。

### 2.2 本 tranche不能证明

```text
backend_acknowledged=true
```

Ollama 与 LM Studio 的兼容接口当前没有统一、可验证的 request cancellation acknowledgement。
因此 receipt 必须固定记录：

```json
{
  "backend_acknowledged": false,
  "qualified": false
}
```

后续只有 `hepta-inferd` request/cancel generation fence 与 worker acknowledgement 才能提升该证据等级。

## 3. 受控重启 helper 合同

真实重启只能通过 self-hosted runner 预置的 helper：

```text
HEPTA_INF0C_CONTROL_HELPER=/absolute/path/to/helper
HEPTA_INF0C_CONTROL_HELPER_SHA256=sha256:<64 lowercase hex>
```

执行前必须验证：

- 绝对路径；
- 非 symlink；
- 普通文件；
- 可执行；
- Unix 下不可 group/world writable；
- 文件 SHA-256 与批准值一致。

调用固定为：

```text
<helper> restart ollama
<helper> restart lmstudio
```

禁止 shell、禁止用户提供任意命令字符串。子进程只继承受控环境 allowlist，stdout/stderr 默认丢弃。

## 4. 重启验收

每个 provider 必须依次证明：

```text
pre-restart health
→ helper starts
→ service unavailable observed
→ helper exits 0
→ service recovered observed
→ exact model still present
→ post-restart minimal inference passes
```

若没有观察到 down transition，即使 helper 返回 0 也失败关闭。

## 5. 隐私和供应链

Receipt 允许：

- source commit/tree；
-模型 ID；
- helper basename 与 SHA-256；
- HTTP status；
- response/prefix SHA-256 与 byte length；
- timing；
- down/up transition；
- negative authority snapshot。

Receipt 禁止：

- raw prompt；
- raw model output；
- helper stdout/stderr；
-完整 helper 路径；
- secrets、cookies、tokens 或 API keys。

## 6. 测试层级

### L0 source gate

- exact parent commit/tree；
- final receipt 绑定自己的父 source candidate；
- final receipt commit 只能改 receipt；
-所有 authority 关闭；
- INF-1 仍为 `NOT_STARTED`。

### L1 hermetic self-test

Hosted runner 启动两个 loopback fake providers 和 digest-pinned fake helper，证明：

- cancellation connection close；
- post-cancel health；
- restart down/up transition；
- helper digest 和 fixed argv；
- receipt owner-only；
- raw prompt 不落盘。

### L2 real software evidence

Self-hosted `hepta-inference-e2e` runner 使用固定、预安装模型执行。模型安装和下载不属于该 workflow。

## 7. Exit gate

本 tranche 只有在以下证据全部存在时才能称为 real evidence executed：

- exact-head source gate 与 hermetic self-test 真实执行；
-固定 Ollama 与 LM Studio 模型均完成 transport-disconnect probe；
-若要求 controlled restart，两个 provider 均观察到 down/up；
- post-cancel 和 post-restart model discovery/inference 均通过；
- receipt 无原文泄漏；
-所有负权限保持关闭。

即使上述通过，`backend_acknowledged=false` 仍禁止把它宣称为完整 daemon cancellation qualification，
也不能自动激活 INF-1、合并、promotion 或生产 listener。

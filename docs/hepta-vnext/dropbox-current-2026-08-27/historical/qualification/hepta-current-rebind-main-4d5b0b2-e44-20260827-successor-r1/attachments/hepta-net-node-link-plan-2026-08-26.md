# Hepta 开发计划增量：hepta-net / hepta-node-link

> 状态：Bound planning append；可写入 Dropbox，但仍为 qualification-only，不是 implementation authorization
> 日期：2026-08-26（Asia/Shanghai）
> 计划类型：独立 qualification lane；不覆盖、不改写现有 Codex execution-substrate 计划

## -1. 执行绑定（2026-08-26 写入前快照）

这份文档是可审计的计划增量，不是可直接开工的 implementation authorization。写入前已重新读取 Dropbox canonical 主计划；外部 effective-index/manifest 仍指向旧镜像 digest，因此本增量明确保持 `STALE_SOURCE_BINDING` 和 `qualification_only`，只能用于 RFC、schema、test-vector 与 shadow 设计，不能授权实现、合并、部署或 promotion。禁止沿用昨日的 E57 或旧 receipt：

```yaml
plan_id: HNL-hepta-net-node-link
plan_version: 0.1.0
parent_plan_id: hepta-vnext-development-plan-final-2026-08-23
parent_plan_digest: 07dbbd0fb36a9d82d039275a9819e6c18b133248a2864470f7ffa17ef6cda718
parent_source_status: STALE_SOURCE_BINDING
insert_after: "E.40.3 / EOF line 3346 (pre-append snapshot)"
canonical_repo_head: 7ed9c9a85fa65aa3cb26cf440a55028ce0b35079
canonical_repo_tree: not-revalidated-in-this-environment
implementation_branch: not-created
worktree: not-created
working_tree: unknown_not_mounted
mode: qualification_only
production_listener: false
fleet/shared_kg/routing_write: false
CALLERS_touched: false
promotion: false
```

若 `parent_source_status` 不是 `SEALED`，本计划只能做 RFC、schema、test-vector 和 shadow design；不得实现、合并、部署或宣称 qualification。HNL 必须从 canonical main-integration 的 exact head 派生，不能从 detached/upstream-sync 实验树直接开工。当前 `canonical_repo_head` 是主计划中记录的 unchanged main-integration head，因本环境未挂载 `/Volumes/T5`，实现前必须重新验证 head/tree/dirty，并生成新的 receipt。

执行矩阵与 receipt schema 不应只存在于 Markdown：

- `drafts/HNL_STAGE_MATRIX_v1.yaml`：阶段依赖、owner/RACI、entry/exit、命令、资源预算、receipt kind、authority flags 和 rollback。
- `drafts/hepta.hnl_receipt.v1.schema.json`：每阶段 exact-head/source binding、测试结果、artifact digest、资源读数和 negative-authority 证明。

Dropbox 追加时应把这两个文件作为同一变更集的附件或等价 machine-readable plan；Markdown 只作为渲染和决策说明。
矩阵中的命令路径是 planned commands；在对应 crate/fixture 尚未创建前，receipt 必须标记 `NOT_TESTED`，不能伪造通过结果。

## -0. 依赖方向与权威边界

```text
hepta-net-core（纯 Rust；不依赖 TRNM、agentd 或 fleet state）
        ↑
hepta-node-linkd（P0 sidecar）/ optional in-process adapter（qualification 后）
        ↑
agentd / fleet supervisor / hepta-market-adapter
```

`hepta-net-core` 只提供 transport、identity verification、policy/capability enforcement 和 immutable network events；它不授予权限、不保存链上状态、不作 workflow/fleet 决策。`PolicyVerifier/IssuerRegistry` 以 trait 注入，TRNM 只提交版本化签名对象。`EventSink` 输出带 `seq/hash` 的事实事件，权限与结算决定仍回到现有 authority。

## 0. 决策摘要

Hepta 需要的是一个与主系统同版本演进的常驻 Rust 节点连接子系统，而不是把 Tailscale daemon 原样嵌入。该子系统负责 Hepta node 之间的身份、加密会话、节点发现、NAT/relay、路径切换、Multi-Agent 服务路由、ACL、撤销和网络事件。

TRNM chain 暂不进入网络热路径。后续由 `hepta-market-adapter` 把 TRNM 状态转换为 Hepta 自己的短期签名 `Lease/Capability/Revoke/UsageReceipt`；`hepta-net` 只验证并执行这些对象，不持有链上私钥，不负责撮合、余额、托管、结算或争议。

核心原则：

- 业务只依赖 Hepta 自有 facade/ABI，不暴露任何上游项目类型。
- 一台物理设备一个 `NodeId`；本地多个 Agent 使用受限委托 `AgentId/ServiceId`，不各自启动 VPN 节点。
- 当前 Hepta 的 `agentd`、workflow、fleet lifecycle 保持唯一执行和调度权；网络模块不得新造第二套 runtime、session kernel 或 fleet bus。
- Headscale、DERP、公共 relay 只是可替换 bootstrap，不是全局信任根。

## 1. 目标与非目标

### 目标

1. 两个 Hepta node 能在不同网络/国家之间完成相互认证和端到端加密连接。
2. 一个 node 上的多个 Hepta agent 能通过同一常驻 endpoint 安全复用连接，并按 Agent/Service 做隔离。
3. 支持直连优先、NAT 穿透、可替换 relay、断线重连和 relay 迁移。
4. 会话具备短 TTL、epoch 撤销、密钥轮换和 fail-closed 行为。
5. 为未来 TRNM 提供稳定、链无关的签名租约/事件适配点。
6. P0 只以同版本 Rust sidecar 运行，由现有 agentd/fleet supervisor 管理生命周期；通过 qualification 后，才评估把同一 facade 的 backend 编译进 Hepta 主进程。两种部署不是 P0 的双重 DoD。

### 非目标

- P0 不实现 TRNM 钱包、撮合、链上结算、双花防护或争议仲裁。
- 不把 `tailscale-rs` 宣传为完整系统 VPN或去中心化信任根。
- 不让网络模块成为 Agent workflow/fleet scheduler。
- P0 只提供受控 service-stream；不开放任意默认路由、裸住宅代理、全局 TUN、主机路由写入或跨租户网络。L3/TUN 单独作为未来 `HNL-L3` feature/epic。

## 2. 目标架构

```text
TRNM chain（未来）
        │ signed Lease / Capability / Revoke / Receipt
hepta-market-adapter（链无关、异步适配）
        │
hepta-net / hepta-node-linkd（每台物理安装/受管主机一个；多 workspace 通过 tenant/generation 隔离）
  ├─ IdentityVault       host/agent 委托身份、轮换、epoch
  ├─ NodeRegistry        signed Node/Agent/Service descriptors
  ├─ Discovery            P0: signed invite/peer records；DHT 仅 P2
  ├─ SessionBroker        握手、流复用、会话生命周期
  ├─ AgentRouter          Agent/Service ACL、租约范围、配额
  ├─ PathManager          direct/NAT/relay、健康、迁移
  ├─ RouteManager         HNL-L3 future feature；P0 不编译/不启用
  ├─ Revocation           TTL/epoch/密钥撤销，fail-closed
  └─ EventSink            typed 网络/用量事件，不做结算
        │ backend trait
  tailscale-rs | native QUIC/iroh | boringtun/WireGuard | EasyTier
        │
  Hepta agents / services
```

## 3. 身份与 Multi-Agent 模型

### 身份层

- Host 根身份：Ed25519，`NodeId = hash(public_key)`，优先放 OS Keychain/TPM/Secure Enclave。
- Agent 委托身份：由 Host 根签发 `AgentId/AgentKey`，带 scope、audience、expiry、epoch、nonce。
- Service 身份：绑定到 Agent，不能自动获得 Host 的全部权限。
- 会话密钥：X25519/Noise 或 backend 的等价临时密钥；每次会话绑定新 nonce。
- 100.x 地址、Headscale node ID、EasyTier network secret 只作路径/网络标识，不作长期账户身份。

### 握手绑定

远端必须同时验证：

`local_node_id + remote_node_id + transport_endpoint_id + tenant_id + workspace_id + agent_id + service_id + intent/capability_hash + expiry + epoch + nonce`

只认证 Host 的 WireGuard/Tailscale 公钥是不够的；否则一个被攻破的本地 Agent 可以横向冒充同机其他 Agent。

### 密钥层级与生命周期（HNL-0 必须冻结）

密钥不能只有一个“Node key”。建议明确分层：

```text
HostRootSigningKey（稳定、仅签发）
        ↓ continuity certificate / alias
HostTransportKey（可轮换）
        ↓ constrained delegation
AgentDelegationKey（scope/audience/expiry/epoch）
        ↓ optional
ServiceKey
        ↓ ephemeral
SessionKey（只存在于会话内）
```

- 每把 key 都有 `key_id、parent_fingerprint、not_before、not_after、epoch`；epoch 必须持久化单调递增，防止回滚。
- Host 根轮换要有签名 continuity/alias；旧根进入有限 grace 后撤销。被盗 Agent key 不能升级 scope 或签发子根。
- 私钥不能通过 Agent IPC 导出；备份/恢复、设备迁移、硬件密钥不可用时的降级策略要写成状态机。
- `NodeId/AgentId/ServiceId/TenantId/WorkspaceId/agent_generation/EndpointId` 的 namespace、版本和 canonical encoding 必须冻结。不要同时使用含义重叠的 `PeerId` 和 `NodeId`；facade 可把 `remote_node_id` 暴露为目标 peer，但 wire schema 只保留明确字段。
- 同一物理设备的多 workspace/租户必须有独立 `workspace_id + agent_generation`；generation 变化后旧 Agent delegation、session 和 descriptor 自动失效。

### 入网与信任注册

P0 不采用“拿到 network secret 就信任”或无策略 TOFU。入网使用短期、带 audience/nonce/expiry 的签名 invite；本地 trust registry 记录允许的组织/发行者/relay，descriptor 更新必须验证签名、`record_seq` 和 freshness。控制提供方、relay 提供方和资源/市场发行方的密钥域分离。

## 4. 稳定 facade（第一版 API 草案）

```text
start(NodeConfig) -> NodeHandle
register_agent(AgentRegistration) -> AgentLease
advertise(agent_id, ServiceAdvertisement) -> ServiceId
discover_peer(query) -> Vec<PeerDescriptor>
connect(ConnectIntent) -> SessionHandle
open_stream(session, agent_id, service_id, StreamPolicy) -> Stream
revoke(subject, epoch)
rotate_key(subject)
stats() -> NetStats
events() -> EventStream
```

`ConnectIntent` 预留但不依赖 TRNM：

```text
ConnectIntent {
  remote_node_id,
  transport_endpoint_id?,
  tenant_id,
  workspace_id,
  agent_generation,
  agent_id,
  service_id,
  purpose,
  capability_hash?,
  issuer_key_id?,
  order_ref?,
  policy_digest,
  expiry,
  epoch,
  nonce,
}
```

`Session` 只暴露 Hepta 类型：remote node/agent/service ID、stream/datagram、path state、network events、cancel；不得暴露 `tailscale::Device`、EasyTier 内部结构体或底层私钥。

### Wire contract 与版本协商

- 控制面和数据面分开；签名对象使用一种固定的 canonical 编码（建议 COSE/CBOR），本地 UDS 可用带 schema version 的 Protobuf/CBOR，但不能让 JSON 成为安全协议格式。
- 每个对象带 `version、domain_tag、key_id、issued_at、expires_at、seq/epoch、nonce、policy_digest、signature`；签名采用 domain separation，防止同一对象跨用途重放。
- 握手将这些字段绑定到真正的 transcript hash/channel exporter，而不是只放在首个应用包里。路径迁移必须重新做 endpoint challenge-response，不能由 relay 单方面切换。
- 固定 protocol/ALPN、feature bits、最大帧/流/连接、clock-skew 上限和 downgrade refusal；未知强制安全 feature 必须 fail-closed。
- 所有请求带 `request_id` 并定义幂等语义；事件流有 cursor/重复去重；每个 Agent 有连接、流、队列、带宽和 CPU 背压上限。

最小对象先冻结为：

```text
NodeDescriptor{version,node_id,key_id,endpoint_candidates,allowed_relays,
  capabilities,record_seq,issued_at,expires_at,policy_digest,signature}
AgentDelegation{delegation_id,parent_node_id,parent_fingerprint,tenant_id,
  workspace_id,agent_id,agent_generation,service_ids,scope,audience,max_depth,
  issued_at,expires_at,epoch,nonce,key_id,signature}
ConnectIntent{version,tenant_id,workspace_id,agent_generation,
  initiator_node/agent/service,remote_node/agent/service,
  purpose_digest,capability_hash,session_nonce,expiry,epoch,policy_digest}
RevocationRecord{subject,parent,tenant_id,workspace_id,agent_generation,
  epoch,revocation_seq,issued_at,expires_at,reason,key_id,signature}
NetworkEvent{version,event_id,subject,seq,monotonic_counter,issued_at,
  event_type,payload_digest,policy_digest,signature}
```

`NetworkEvent` 是不可变事实事件，采用 at-least-once + `event_id/seq` 去重；网络层只写自己的 outbox/event log，不能与 agentd 或 TRNM 共享可变账本。`UsageEvent` 是其受限子类型，不等同于结算完成。

`hepta-net-core` 只依赖抽象接口：

```text
PolicyVerifier::verify(intent, delegation, descriptor) -> Decision
IssuerRegistry::resolve(key_id, domain_tag) -> TrustedIssuer
ControlProvider::publish/resolve_signed_record(...)
RelayProvider::allocate/probe/migrate(...)
```

这些接口不能反向依赖 TRNM SDK、agentd state 或某个具体链；所有拒绝都返回可分类的 Hepta error（`unauthorized、expired、revoked、replay、downgrade、unsupported、quota、path_unavailable、backpressure`），并带 request/event ID。

## 5. 后端策略

| 后端 | P0/P1 定位 | 使用边界 |
|---|---|---|
| 官方 `tailscale-rs` | P0 compatibility backend | 锁 commit，验证 Headscale/Go Tailscale/DERP/用户态 TCP-UDP；WIP、直连/TUN/DNS/路由不完整，不能做核心信任根 |
| native QUIC/iroh | Hepta 原生服务通道 | Agent RPC、事件、文件/模型分片；直连失败走可替换 relay；不提供 L3 |
| `boringtun`/WireGuard | P1 透明 L3 backend | 仅按 RouteLease/订单启用，临时 key、AllowedIPs、端口最小权限；控制/发现/ACL 由 Hepta 提供 |
| EasyTier | 隔离实验 backend | 去中心化 L3/NAT/relay 对照；LGPL/custom protocol/密码学审计和兼容性风险，类型不得进入业务层 |
| RustScale/GeiserX/sunbeam 等 | 研究/benchmark | 不作为生产基线，需单独互操作和安全审计 |

P0 的核心验证不应被 `tailscale-rs` 的 WIP 互操作结果阻塞：HNL-1 先用 deterministic loopback backend 验证 facade/UDS/Multi-Agent 语义；HNL-2A 与 HNL-2B 分别验证 native QUIC 和 Tailscale compatibility，可并行但各自独立出 receipt。若硬要求是接入现有 Tailscale 网络，保留 `tailscale-rs`；若只是 Hepta 原生 Agent 间 RPC，native QUIC 是默认服务 backend。长期核心是 `hepta-net`，不是某个上游实现。

### Backend security contract

每个 backend 在接入前必须声明能力矩阵：是否提供端到端认证、前向保密、直连、relay、路径迁移、TUN/L3、per-stream ACL、密钥轮换和撤销。facade 只承诺所有 backend 的最低共同保证；不能因为某 backend 有 TUN 就自动向业务开放 default route。

P0 只做一条完整 vertical slice：先完成 facade + Multi-Agent 语义 + 一个选定 backend 的互操作，第二个 backend 只在第一条通过 Gate 0/1 后接入，避免同时维护两套未验证数据面。

## 6. 会话流程与离线语义

```text
Agent request
 → local Agent ACL/policy check
 → optional signed Lease/Capability verification
 → direct path probe
 → relay fallback if needed
 → handshake binds IDs + intent hash + nonce + expiry/epoch
 → multiplexed session
 → path health/migration events
 → revoke/TTL/close
```

- Relay 只转发端到端密文，不能签发租约或修改路由/计量。
- 协调器离线时，未过期会话可继续，但必须受 `max_offline_grace` 和 `max_session_lifetime` 双上限约束；P0 默认建议 capability TTL 5–15 分钟，超过上限必须重新认证。禁止静默无限续租。
- TTL/epoch 到期后禁止新流；高风险/跨租户流立即 drain/close，普通流也不得超过配置上限。
- 路由广告必须签名、带 prefix/port/expiry/epoch；默认拒绝任意 default route。P0 service-stream 不触碰主机路由表。

### 威胁模型与硬不变量

威胁主体至少包括：恶意/被攻陷 Agent、恶意 peer、恶意 relay/bootstrap、网络观察者、重放/降级攻击者、恶意 descriptor/route 发布者和资源耗尽攻击者。以下不变量必须能被测试证明：

- relay 看不到应用明文，不能签发 capability、伪造 Agent 或改变 route/计量。
- Agent 不能扩大委托 scope、冒用同机其他 Agent/Service 或借用其 session。
- 过期/撤销后不能开新流；已有流按风险等级在配置上限内 drain/close。
- 路径变更不改变 Node/Agent/Service 身份；旧 epoch、旧 descriptor、旧 lease 永不恢复。
- 协调器离线不放大权限；重启、时钟回拨、重复 request/session resume 都不能绕过重新认证。

### 持久化与故障语义

持久化 Host 根/当前 transport key、Agent epoch、revocation sequence、已确认 descriptor 的 freshness 边界；临时 session key、NAT candidate 和 relay cache 丢失后必须重新握手。明确 node-linkd 崩溃、agentd 暂停/恢复、主机睡眠、网络切换、key rotation during active session、协调器/relay 分区时的 drain、恢复和 fail-closed 行为。Session resume 不能只凭旧 session ID。

建议状态机：

```text
Host:    Booting → Joining → Active → Degraded → Quarantined/Revoked → Shutdown
Agent:   Pending → Bound → Running → Suspended → Revoked
Session: Negotiating → Established → PathMigrating → Draining → Closed
```

### Gate 0 前必须做出的 7 个决策

以下不是继续扩展范围，而是必须由维护者签字冻结的选择；在这些值为空时，HNL 只能停留在 RFC：

1. `node-linkd` 的确切安装 scope（每台物理主机/每个 OS 用户/每个 Hepta 安装）及多 workspace 的 UDS 授权映射。
2. 每个 backend 的具体密码套件、ALPN、transcript/channel-exporter 绑定方式，以及 native QUIC 与 tailscale Noise/WireGuard 的最低共同保证。
3. canonical CBOR/COSE 的版本、domain tags、签名 key registry、clock-skew、TTL、replay window 和 downgrade policy。
4. HostRoot/transport/Agent delegation/Service/session key 的持久化 writer、备份恢复和 root rotation continuity 方案。
5. P0 默认 backend（native service channel 与 `tailscale-rs` compatibility 的启用顺序）及 tailscale-rs 失败时的明确 `KNOWN_GAP` 处理。
6. Pocket4 实机的 idle/peak RSS、CPU、功耗、FD/stream/peer 上限和建链/重连 p95 目标。
7. agentd/fleet 与 node-linkd 的 RACI、启动/暂停/升级/回滚责任，以及 exact source binding 的最终锚点。

## 7. 计划分期与交付物

<a id="hnl-gate0-architecture"></a>
### 7.0 进入/退出 Gate

- **Gate 0—架构**：facade、对象 schema、authority 边界、单 backend vertical slice 和 golden handshake vectors 冻结。
- **Gate 1—安全**：协议/密钥/UDS/relay threat review 通过；未完成第三方密码审计前，只允许 shadow/qualification，不得开放 production listener、TUN、L3/default route 或跨租户连接。
- **Gate 2—生命周期**：node-linkd 与现有 agentd 的启动、暂停、恢复、优雅关闭、崩溃恢复和升级 drain 有可复现实验；不得出现第二 supervisor 或悬挂会话。
- **Gate 3—硬件**：Pocket4 ARM、macOS/Linux 的 CPU/RAM/功耗/后台/建链/迁移/重连预算已量化并有 receipt。

任何阶段遇到 stale source binding、无可复现 receipt、未量化阈值或安全不变量失败，都只能标记 `known-gap/blocked`，不得宣称 qualification/promotion。

### HNL-0：设计冻结与威胁模型（P0）

- 冻结 `hepta-net` facade、对象 schema、错误码和事件类型。
- 写明 Host/Agent/Service 身份边界、密钥存储、撤销和离线行为。
- 固定上游 commit、许可证、SBOM/cargo-deny 清单。
- 增加 canonical 编码、domain separation、版本/feature negotiation、trust registry 和 golden handshake vectors。
- 交付：设计 RFC、威胁模型、API/schema、source manifest、key lifecycle/rotation spec。

### HNL-1：Rust resident core + local bridge（P0）

- 实现 `hepta-net-core`、backend trait、IdentityVault、AgentRegistry、SessionBroker、Revocation。
- 实现 `hepta-node-linkd` sidecar/UDS typed RPC；不改变现有 agentd/fleet authority。
- 实现 UDS peer credential/SID/audit-token 校验、socket 权限/防 symlink、request_id 幂等、per-Agent quota/backpressure。
- P0 的 `hepta-node-linkd` scope 固定为每台受管物理安装；由 agentd/fleet supervisor 启停、暂停、恢复、drain 和升级，sidecar 不得自行创建全局 state bus。
- 交付：两台本地虚拟 node、每台两个 agent 的注册/ACL/服务发现 demo，以及 crash/restart/revoke receipt。

### HNL-2A：Native QUIC service backend（P0）

- 在 deterministic loopback 通过后接入 native QUIC/iroh，完成两个 Hepta node 的服务级 E2E 通道、relay fallback 和 path migration。
- 明确 QUIC TLS/endpoint identity 与 Hepta transcript binding 的关系，不重复叠加未经审计的加密协议。
- 交付：native backend receipt、relay fixture、路径迁移和离线语义报告。

### HNL-2B：Tailscale compatibility（P0 compatibility track）

- 接入锁定 commit 的 `tailscale-rs` backend。
- Headscale/Go Tailscale ↔ Rust node 互操作。
- 验证 DERP、TCP/UDP、持久化、重连和控制面短时离线；C/C++ FFI 不列入 P0，只有出现真实 caller 后另开 gate。
- 增加恶意 relay、强制降级、旧 descriptor/lease、NAT rebinding 和路径 challenge 测试。
- 交付：互操作 receipt、性能/资源报告、capability matrix、已知缺口清单；明确哪些能力仍禁止进入业务。

### HNL-3：Cross-backend Multi-Agent session qualification（P0）

- 同一 Host endpoint 的多 Agent stream multiplex。
- Agent delegated key、跨租户 ACL、单 Agent 撤销、epoch/key rotation。
- 直连→relay fallback、relay migration、背压/配额、异常 Agent 隔离。
- 增加 cross-Agent/tenant spoof、重复 connect、session resume、时钟回拨、key rotation during active stream 测试。
- 交付：qualification matrix、golden vectors、fuzz/property/model tests、故障注入报告和 signed test receipt。

### HNL-4：L3/WireGuard/EasyTier 对照（future feature，P1）

- 需要透明 IP 时接 boringtun/WireGuard；严格测 MTU、CPU、内存、功耗和路由冲突，使用独立 privileged helper/netns。
- EasyTier 仅以隔离 adapter 做 PoC，不改变业务 API；DHT、联邦发现和多区域治理不进入 P0 DoD。
- 交付：L3 backend comparison receipt；P0 service-stream facade 不因 L3 未完成而阻塞。

### HNL-5：TRNM adapter hook（P1，网络 qualification 后）

- 定义 `SignedLease / Capability / Revoke / UsageEvent / Receipt` 的版本化 CBOR/Protobuf schema；基础 `NetworkEvent` envelope 在 HNL-0 已冻结，HNL-5 只扩展市场字段。
- `hepta-market-adapter` 只把 TRNM finality/撤销转换成短期 Hepta 对象。
- 链上交易号仅作 `settlement_ref`，不进入实时握手密钥或路由条件。
- 交付：TRNM simulator + adapter tests；真实链路另立计划，不阻塞网络核心。

### HNL-6：多国部署与治理（P2）

- 多 relay/operator、可替换 control provider、签名 peer records、DHT/目录 federation。
- 在 P0 已完成单机/Agent key rotation 的基础上，再做多运营方密钥治理、离线加入、升级回滚、阈值治理和多区域故障演练。

## 8. 验收门槛

1. **安全**：握手身份、前向保密、Agent 隔离、重放/降级/伪造 route 失败；relay 看不到明文。
2. **可靠性**：全锥/对称 NAT、CGNAT、IPv6、relay fallback、迁移、断线重连。
3. **权限**：TTL/epoch/revoke fail-closed；协调器离线不静默放权；默认无跨租户访问。
4. **Multi-Agent**：两个以上 Agent 并发 stream 无地址/路由/密钥冲突；单 Agent 撤销不影响其他合法 Agent。
5. **性能**：建链时延、吞吐、MTU、CPU、内存、功耗和后台稳定性在 Pocket4 实机上有 receipt。
6. **治理**：每个阶段有 source binding、commit/manifest、测试结果和 append-only receipt；不能仅凭代码存在宣称 qualification/promotion。

### 8.1 可复现 Qualification Matrix

每次 qualification 固定以下变量并记录在 receipt：

- 拓扑：provider/consumer/relay 三节点；每节点至少两个 Agent；loopback、受控 relay、Headscale/DERP fixture 分开计分。
- 网络：IPv4、IPv6、全锥 NAT、对称 NAT、CGNAT、NAT rebinding、丢包/延迟/时钟偏移、relay kill/migration。
- 软件：repo/head/tree/dirty、Cargo.lock、backend exact commit、Rust/MSRV/target、配置和 feature flags。
- 对抗矩阵：恶意 Agent/peer/relay、旧 descriptor/lease、重放、降级、伪造 route、重复 request、session resume、key rotation、crash/restart。
- 结果：每项 `pass/known_gap/not_tested/blocked`，附命令、trace/test-vector digest、signed receipt；第三方密码审计和 production promotion 单独作为 gate，不能用单元测试替代。

最低硬指标先给出可测的 provisional target，并在 HNL-0 用 Pocket4 实机基线批准最终值：

| 指标 | provisional target | 采样方式 |
|---|---:|---|
| cross-Agent/tenant 明文或权限泄漏 | 0 | 对抗测试 + packet/stream labeling |
| relay 抓包应用明文 | 0 | relay-side capture + golden payload |
| 旧 epoch/重复 nonce/旧 lease 接受率 | 0 | replay/property test |
| 撤销收敛时间 | ≤ 配置 TTL/clock-skew 上限 | 双端时间戳 + signed event |
| direct/relay 建链 p95 | HNL-0 设定 | 固定拓扑、冷/热启动分开 |
| node-linkd idle RSS / CPU | HNL-0 设定 Pocket4 budget | 长时 soak + peak workload |

确定性单元/模糊测试必须 `network_access=false`；互操作、relay 和性能测试使用独立 receipt，不得把公网状态混入 deterministic claim。

### 8.2 运行、资源与升级约束

- HNL-0 之前定义 Pocket4 ARM、macOS/Linux 的 MSRV、target、idle/peak RSS、CPU、线程、FD、socket、peer/agent/stream 上限；预算超限自动标记 `KNOWN_GAP`。
- `hepta-node-linkd` 的 launchd/systemd/移动端启动、暂停、恢复、崩溃重启、优雅 drain、升级回滚和配置/relay 轮换写 runbook；网络 daemon 不自有 fleet 生命周期。
- 日志默认脱敏 Node/Agent/Service 和 endpoint 元数据，relay/IP/时序信息按隐私策略分级；metrics 不记录私钥、token 或明文 payload。
- `cargo fmt/clippy/test/deny/audit/SBOM/fuzz`、可复现构建、unsafe/FFI review 和 Cargo.lock digest 进入每阶段 receipt。C/C++ FFI 只有在有真实 caller 时才开启，不作为 P0 硬门。
- TUN/L3 必须是独立 feature gate/privileged helper；没有 RouteLease 时 RouteManager 不接触主机路由表。

## 9. 与现有 Hepta 主计划的插入规则

- 新增独立 Epic：`HNL / hepta-net-node-link`。
- 依赖：现有 Codex execution substrate 的 authority/receipt 边界；不反向修改其核心状态机。
- P0 先在 shadow/qualification lane；不部署 production listener，不写 shared KG/routing/fleet，不解冻模型/NPU，不做 promotion。
- 当前 E57/Suspend blocker 及其证据封存完成后，才把 HNL-1/HNL-2A/HNL-2B 排入 active implementation；其中 HNL-2B 失败只能产生 compatibility `KNOWN_GAP`，不能阻断 backend-neutral core/native service qualification。
- 每个 HNL 阶段独立 receipt；不得复用 stale source binding。

## 10. 一句话定案

`tailscale-rs` 负责兼容现有网络，native QUIC 负责 Hepta 原生服务连接，WireGuard 负责必要的透明 L3；真正的常驻核心是 Hepta 自己的 `hepta-net/hepta-node-linkd`，TRNM 未来只通过签名租约适配接入，现有 agentd/fleet supervisor 仍是唯一的多智能体控制权。

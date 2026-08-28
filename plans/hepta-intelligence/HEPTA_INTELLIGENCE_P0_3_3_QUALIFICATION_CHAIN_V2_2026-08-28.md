# P0.3.3 Qualification Chain v2

本 tranche 不改变模型协议与 resolver 语义，只修复资格和重栈链：

1. source verifier 根据状态文件动态输出 P0.3.2 dependency；
2. pre-restack 明确归类为 dependency blocked，而不是 source failure 或 PASS；
3. post-restack 执行统一 exact-head v5 receipt；
4. extension/core 全目标编译和完整测试保留；
5. Clippy 只把 P0.3.3 governed production/test files 的新诊断归入 tranche；
6. restack 在 push 前完成完整 preflight，并严格绑定 P0.3.2 executable evidence；
7. 所有 runtime、projection、recall 与 production authority 标志继续为 false。

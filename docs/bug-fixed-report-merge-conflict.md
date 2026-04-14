# Bug Fixed: 合并主干后 leaderboard/report 冲突导致构建与数据链路不一致

日期：2026-04-14

## 问题

本地分支在合并 `main` 后，leaderboard 和 report 相关文件同时出现冲突状态。

这不只是 Git 层面的未完成合并，还会直接影响构建结果和运行行为：

1. `src/report.rs` 的报告主题和提交流程分叉。
2. `web/src/lib/db.ts` 的 leaderboard 排序能力和分享页数据能力分叉。
3. `web/src/components/LeaderboardTable.astro` 的主干排序 UI 与分支分享入口落在两套实现上。

## 根因

这次冲突本质上是两条功能线同时修改了同一批文件：

1. 主干引入了 leaderboard 排序能力、3 位小数的 SBAI 展示，以及新的浅色报告主题。
2. 分支引入了分享页、`reportPayload` 持久化、以及从 leaderboard 跳转个人详情/分享页的链路。
3. 两边都改了同名文件，合并时没有完成最终取舍。

## 修复

这次修复按“主干优先，分支能兼容就兼容”的原则处理：

1. `src/report.rs`
   - 保留主干的浅色主题和 `sbai:.3` 精度。
   - 同时保留分支的 `reportPayload` 隐藏字段，避免 leaderboard 提交时丢失完整报告数据。
2. `web/src/lib/db.ts`
   - 保留主干的排序 SQL 和 `listLeaderboard(sort, dir)` 能力。
   - 同时保留分支的 profile/share 数据读取、pending submission、`report_payload_json` 写入逻辑。
3. `web/src/components/LeaderboardTable.astro`
   - 保留主干的排序筛选 UI。
   - 同时并回分支的个人详情链接和分享按钮，但不引入整行可点击这种额外行为改动。
4. `web/src/components/Podium.astro`
   - 直接回到主干版本，避免引入不必要的表现差异。

## 验证

执行：

```bash
cargo test
npm --prefix web run build
```

确认：

1. Rust 测试 24 项全部通过。
2. Astro 前端构建完成，leaderboard 页面和 share/profile 相关模块可以通过编译。
3. 合并冲突相关文件已经恢复为可继续提交的状态。

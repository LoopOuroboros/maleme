# bug fixed: leaderboard 分享页支持按用户 ID 提取最新提交并生成 OG 面板

## 问题

Leaderboard 之前只有个人详情页 `/u/[login]`，但这个入口更偏浏览，不适合直接拿去分享：

1. 分享链接依赖 `login`，用户改名后稳定性不足。
2. 排行榜没有明确的“分享”动作，用户无法一键拿到可传播的链接。
3. 即使手动复制详情页链接，也没有专门为社交平台准备的 Open Graph 面板。

结果就是，排行榜里已经有数据，但分享链路不完整。

## 这次调整

本次补了一条完整的分享链路，核心点如下：

1. 新增按 `github_id` 读取用户最新提交的分享页路由：`/share/[githubId]`
2. 新增对应的 OG 图片路由：`/share/[githubId].svg`
3. 分享页会读取该用户最近一次提交记录，并展示：
   - 排名
   - SBAI 指数与状态
   - 消息数
   - 脏话次数
   - Tokens
   - 报告区间
   - 高频词条
4. Leaderboard 表格增加“分享”按钮：
   - 支持原生 `navigator.share`
   - 不支持时回退为复制分享链接
5. 分享页视觉不再单独发明一套样式，而是直接向 `src/report.rs` 的报告页靠拢：
   - 复用同一组字体和主题变量
   - 复用 hero / panel / SBAI 卡片语言
   - 带入数字滚动和烟花背景氛围

## 结果

现在从 Leaderboard 里点击“分享”，就能直接拿到一个稳定的、带 Open Graph 的用户分享链接。

社交平台抓取这个链接时，会拿到专门生成的 SBAI 分享面板，而不是普通详情页。

另外补了一次按钮样式修正：

1. `ReportScreen` 里的按钮样式改成 `:global(.submit-form .submit-button)`，确保 slot 进去的按钮也能吃到报告页同款样式。
2. 分享页上的次级按钮同步使用和详情页一致的玻璃态暗色按钮视觉，不再回退成浏览器默认控件。
3. “复制分享链接”按钮改为始终直接写入剪贴板，不再调用系统分享面板。
4. 详情页和分享页都新增了 “分享到 Twitter” 按钮，直接跳转到 Twitter Web Intent 并携带当前分享链接。
5. “分享到 Twitter” 按钮单独提升为高亮主按钮，和普通导航按钮区分开。
6. 详情页里的 GitHub 按钮补成 GitHub 主题配色，并加入 logo，和其他导航按钮明确区分。
7. 首页里的裸 `<img>` 全部替换成 Astro `Image` 组件，并补上 GitHub 远程图片白名单，去掉性能告警。

## 涉及文件

- `web/src/lib/db.ts`
- `web/src/lib/share.ts`
- `web/src/components/LeaderboardTable.astro`
- `web/src/components/ReportScreen.astro`
- `web/src/pages/share/[githubId].astro`
- `web/src/pages/share/[githubId].svg.ts`

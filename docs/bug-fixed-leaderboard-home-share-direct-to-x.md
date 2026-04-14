# bug fixed: leaderboard 首页分享按钮改为直接跳转到 X

## 问题

排行榜首页的“分享”按钮之前走的是浏览器原生 `navigator.share()`：

1. 在支持 Web Share API 的环境里，会先弹出系统自带分享面板。
2. 用户还要在系统分享面板里继续选目标，而不是直接去 X。
3. 这和 leaderboard 现有的分享目标不一致，动作路径也更绕。

结果就是，首页点“分享”并不会直接进入 X 发帖，而是先被系统分享层截走。

## 修复

这次把首页排行榜的分享按钮改成直接打开 X/Twitter intent：

1. `web/src/components/LeaderboardTable.astro`
   - 去掉 `navigator.share()` 和复制链接兜底逻辑
   - 点击后直接拼出 `https://twitter.com/intent/tweet`
   - 携带原有分享文案和用户分享页链接
   - 按钮文案改成“分享到 X”，让行为更明确

## 结果

现在从 leaderboard 首页点击分享，会直接打开 X 的分享页，不再弹系统原生分享面板。

## 影响文件

- `web/src/components/LeaderboardTable.astro`

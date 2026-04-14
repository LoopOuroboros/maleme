import { getSbaiStatus } from "./report";
import type { LeaderboardEntry, LeaderboardProfile } from "./types";

type ShareProfile = Pick<LeaderboardProfile, "displayName" | "githubId" | "login" | "rank" | "report">;

export function getSharePath(githubId: number | string) {
  return `/share/${encodeURIComponent(String(githubId))}`;
}

export function getShareImagePath(githubId: number | string) {
  return `/share/${encodeURIComponent(String(githubId))}.svg`;
}

export function buildShareTitle(profile: ShareProfile) {
  return `${profile.displayName} 的 SBAI 分享卡`;
}

export function buildShareDescription(profile: ShareProfile) {
  const sbaiStatus = getSbaiStatus(profile.report.sbai);

  return [
    `${profile.displayName} · #${profile.rank}`,
    `SBAI ${profile.report.sbai.toFixed(2)}`,
    `输入 ${profile.report.messageCount.toLocaleString("zh-CN")}`,
    `脏话 ${profile.report.profanityCount.toLocaleString("zh-CN")}`,
    `Tokens ${profile.report.tokens.toLocaleString("zh-CN")}`,
    sbaiStatus.state,
  ].join(" · ");
}

export function buildShareHeadline(profile: ShareProfile) {
  return `${profile.displayName} 最近一次提交，把 AI 骂了 ${profile.report.profanityCount.toLocaleString("zh-CN")} 次。`;
}

export function buildShareButtonText(entry: LeaderboardEntry) {
  return [
    `${entry.displayName} 在 maleme leaderboard 排名 #${entry.rank}`,
    `SBAI ${entry.sbai.toFixed(2)}`,
    `脏话 ${entry.profanityCount.toLocaleString("zh-CN")} 次`,
  ].join("，");
}

export function buildTwitterIntentUrl(text: string, url: string) {
  const intentUrl = new URL("https://twitter.com/intent/tweet");
  intentUrl.searchParams.set("text", text);
  intentUrl.searchParams.set("url", url);
  return intentUrl.toString();
}

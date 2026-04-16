import { getSbaiStatus } from "./report";
import { getNumberLocale, type Locale } from "./i18n";
import type { LeaderboardEntry, LeaderboardProfile } from "./types";

type ShareProfile = Pick<LeaderboardProfile, "displayName" | "githubId" | "login" | "rank" | "report">;

export function getSharePath(githubId: number | string) {
  return `/share/${encodeURIComponent(String(githubId))}`;
}

export function getShareImagePath(githubId: number | string) {
  return `/share/${encodeURIComponent(String(githubId))}.svg`;
}

export function buildShareTitle(profile: ShareProfile, locale: Locale = "en") {
  if (locale === "zh-CN") {
    return `${profile.displayName} 的 SBAI 分享卡`;
  }

  return `${profile.displayName}'s SBAI Share Card`;
}

export function buildShareDescription(profile: ShareProfile, locale: Locale = "en") {
  const numberLocale = getNumberLocale(locale);
  const sbaiStatus = getSbaiStatus(profile.report.sbai, locale);

  if (locale === "zh-CN") {
    return [
      `${profile.displayName} · 第 ${profile.rank} 名`,
      `SBAI ${profile.report.sbai.toFixed(3)}`,
      `${profile.report.messageCount.toLocaleString(numberLocale)} 条输入`,
      `${profile.report.profanityCount.toLocaleString(numberLocale)} 次脏话`,
      `${profile.report.tokens.toLocaleString(numberLocale)} tokens`,
      sbaiStatus.state,
    ].join(" · ");
  }

  return [
    `${profile.displayName} · #${profile.rank}`,
    `SBAI ${profile.report.sbai.toFixed(2)}`,
    `${profile.report.messageCount.toLocaleString(numberLocale)} messages`,
    `${profile.report.profanityCount.toLocaleString(numberLocale)} profanities`,
    `${profile.report.tokens.toLocaleString(numberLocale)} tokens`,
    sbaiStatus.state,
  ].join(" · ");
}

export function buildShareHeadline(profile: ShareProfile, locale: Locale = "en") {
  const numberLocale = getNumberLocale(locale);

  if (locale === "zh-CN") {
    return `${profile.displayName} 最近一次提交一共骂了 AI ${profile.report.profanityCount.toLocaleString(numberLocale)} 次。`;
  }

  return `${profile.displayName}'s latest submission cursed at AI ${profile.report.profanityCount.toLocaleString(numberLocale)} times.`;
}

export function buildShareButtonText(entry: LeaderboardEntry, locale: Locale = "en") {
  const numberLocale = getNumberLocale(locale);

  if (locale === "zh-CN") {
    return [
      `${entry.displayName} 在 maleme 排行榜排第 ${entry.rank} 名`,
      `SBAI ${entry.sbai.toFixed(3)}`,
      `${entry.profanityCount.toLocaleString(numberLocale)} 次脏话`,
    ].join(" • ");
  }

  return [
    `${entry.displayName} is ranked #${entry.rank} on the maleme leaderboard`,
    `SBAI ${entry.sbai.toFixed(2)}`,
    `${entry.profanityCount.toLocaleString(numberLocale)} profanities`,
  ].join(" • ");
}

export function buildTwitterIntentUrl(text: string, url: string) {
  const intentUrl = new URL("https://twitter.com/intent/tweet");
  intentUrl.searchParams.set("text", text);
  intentUrl.searchParams.set("url", url);
  return intentUrl.toString();
}

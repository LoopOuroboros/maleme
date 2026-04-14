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
  return `${profile.displayName}'s SBAI Share Card`;
}

export function buildShareDescription(profile: ShareProfile) {
  const sbaiStatus = getSbaiStatus(profile.report.sbai);

  return [
    `${profile.displayName} · #${profile.rank}`,
    `SBAI ${profile.report.sbai.toFixed(2)}`,
    `${profile.report.messageCount.toLocaleString("en-US")} messages`,
    `${profile.report.profanityCount.toLocaleString("en-US")} profanities`,
    `${profile.report.tokens.toLocaleString("en-US")} tokens`,
    sbaiStatus.state,
  ].join(" · ");
}

export function buildShareHeadline(profile: ShareProfile) {
  return `${profile.displayName}'s latest submission cursed at AI ${profile.report.profanityCount.toLocaleString("en-US")} times.`;
}

export function buildShareButtonText(entry: LeaderboardEntry) {
  return [
    `${entry.displayName} is ranked #${entry.rank} on the maleme leaderboard`,
    `SBAI ${entry.sbai.toFixed(2)}`,
    `${entry.profanityCount.toLocaleString("en-US")} profanities`,
  ].join(" • ");
}

export function buildTwitterIntentUrl(text: string, url: string) {
  const intentUrl = new URL("https://twitter.com/intent/tweet");
  intentUrl.searchParams.set("text", text);
  intentUrl.searchParams.set("url", url);
  return intentUrl.toString();
}

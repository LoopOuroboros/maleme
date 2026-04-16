import type { APIRoute } from "astro";
import { getLeaderboardProfileByGithubId, hasDatabaseBinding } from "../../lib/db";
import { getI18n, getNumberLocale, resolveLocale } from "../../lib/i18n";
import { getReportTheme, getSbaiStatus } from "../../lib/report";

export const prerender = false;

function escapeXml(value: string) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&apos;");
}

function metricTile(x: number, label: string, value: string) {
  return `
    <g transform="translate(${x} 442)">
      <rect width="188" height="112" rx="16" fill="rgba(255,255,255,0.05)" stroke="rgba(255,255,255,0.08)" />
      <text x="20" y="36" fill="rgba(255,255,255,0.62)" font-size="20">${escapeXml(label)}</text>
      <text x="20" y="76" fill="#fff4ef" font-size="30" font-weight="700">${escapeXml(value)}</text>
    </g>
  `;
}

export const GET: APIRoute = async (context) => {
  const { params } = context;
  const githubId = Number(params.githubId || "");
  if (!hasDatabaseBinding() || !Number.isInteger(githubId) || githubId <= 0) {
    return new Response("Not found", { status: 404 });
  }

  const profile = await getLeaderboardProfileByGithubId(githubId);
  if (!profile) {
    return new Response("Not found", { status: 404 });
  }

  const report = profile.report;
  const locale = resolveLocale(context);
  const t = getI18n(locale);
  const numberLocale = getNumberLocale(locale);
  const theme = getReportTheme(report.sbai);
  const sbaiStatus = getSbaiStatus(report.sbai, locale);
  const words = report.wordCounts.slice(0, 3).map((item) => `${item.term} ${item.count}`).join(" / ") || (locale === "zh-CN" ? "暂无高频词条" : "No high-frequency terms yet");
  const svg = `
    <svg width="1200" height="630" viewBox="0 0 1200 630" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="1200" height="630" fill="${escapeXml(theme.bg)}" />
      <circle cx="1040" cy="82" r="188" fill="${escapeXml(theme.heroGlowB)}" />
      <circle cx="168" cy="70" r="172" fill="${escapeXml(theme.heroGlowA)}" />
      <rect x="36" y="36" width="1128" height="558" rx="28" fill="${escapeXml(theme.panelStrong)}" stroke="${escapeXml(theme.border)}" />
      <rect x="56" y="56" width="654" height="238" rx="24" fill="${escapeXml(theme.heroFrom)}" />
      <rect x="56" y="56" width="654" height="238" rx="24" fill="url(#heroFade)" />

      <text x="82" y="102" fill="${escapeXml(theme.muted)}" font-size="22" letter-spacing="1.5">MALEME LEADERBOARD / SHARE</text>
      <text x="82" y="176" fill="${escapeXml(theme.text)}" font-size="54" font-weight="800">${escapeXml(profile.displayName)}</text>
      <text x="82" y="218" fill="${escapeXml(theme.muted)}" font-size="26">@${escapeXml(profile.login)} · #${profile.rank}</text>
      <text x="82" y="258" fill="${escapeXml(theme.accentWarm)}" font-size="24">${escapeXml(report.rangeStart)} - ${escapeXml(report.rangeEnd)}</text>

      <rect x="736" y="56" width="392" height="420" rx="24" fill="${escapeXml(theme.sbaiSurfaceTop)}" stroke="${escapeXml(theme.sbaiBorder)}" />
      <circle cx="836" cy="138" r="124" fill="${escapeXml(theme.sbaiGlow)}" />
      <text x="772" y="116" fill="${escapeXml(theme.sbaiText)}" font-size="22">${escapeXml(t.report.sbaiLabel)}</text>
      <text x="772" y="158" fill="${escapeXml(theme.sbaiMuted)}" font-size="24">${escapeXml(sbaiStatus.state)}</text>
      <text x="772" y="292" fill="${escapeXml(theme.sbaiText)}" font-size="106" font-weight="800">${report.sbai.toFixed(2)}</text>
      <text x="772" y="336" fill="${escapeXml(theme.sbaiText)}" font-size="28" font-weight="700">${escapeXml(t.report.sbaiKicker)}</text>
      <text x="772" y="374" fill="${escapeXml(theme.sbaiText)}" font-size="42" font-weight="800">${escapeXml(t.report.sbaiMantra)}</text>
      <text x="772" y="426" fill="${escapeXml(theme.sbaiMuted)}" font-size="20">${escapeXml(sbaiStatus.copy)}</text>

      ${metricTile(72, t.report.messageCount, report.messageCount.toLocaleString(numberLocale))}
      ${metricTile(276, t.report.profanityCount, report.profanityCount.toLocaleString(numberLocale))}
      ${metricTile(480, t.report.tokens, report.tokens.toLocaleString(numberLocale))}

      <text x="72" y="584" fill="${escapeXml(theme.muted)}" font-size="20">${escapeXml(locale === "zh-CN" ? `高频词条: ${words}` : `Top terms: ${words}`)}</text>
      <text x="1114" y="584" text-anchor="end" fill="${escapeXml(theme.muted)}" font-size="20">${escapeXml(locale === "zh-CN" ? `提交时间 ${new Date(profile.submittedAt).toLocaleString(numberLocale)}` : `Submitted ${new Date(profile.submittedAt).toLocaleString(numberLocale)}`)}</text>
      <defs>
        <linearGradient id="heroFade" x1="56" y1="56" x2="710" y2="294" gradientUnits="userSpaceOnUse">
          <stop stop-color="${escapeXml(theme.heroFrom)}" />
          <stop offset="1" stop-color="${escapeXml(theme.heroTo)}" />
        </linearGradient>
      </defs>
    </svg>
  `.trim();

  return new Response(svg, {
    headers: {
      "Content-Type": "image/svg+xml; charset=utf-8",
      "Cache-Control": "public, max-age=300",
    },
  });
};

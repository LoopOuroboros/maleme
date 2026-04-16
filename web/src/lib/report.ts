import type {
  LeaderboardReportPayload,
  ReportDailyCount,
  ReportDailyModelSeries,
  ReportModelSbai,
  ReportWordCount,
} from "./types";
import type { Locale } from "./i18n";

type ReportTheme = {
  bg: string;
  bgAccent: string;
  panel: string;
  panelStrong: string;
  panelTint: string;
  muted: string;
  text: string;
  line: string;
  lineSoft: string;
  areaTop: string;
  areaBottom: string;
  accent: string;
  accentSoft: string;
  accentWarm: string;
  accentPink: string;
  border: string;
  grid: string;
  shadow: string;
  tooltipBg: string;
  heroFrom: string;
  heroTo: string;
  heroGlowA: string;
  heroGlowB: string;
  cloudGlow: string;
  cloudTo: string;
  sbaiGlow: string;
  sbaiSurfaceTop: string;
  sbaiSurfaceBottom: string;
  sbaiBorder: string;
  sbaiText: string;
  sbaiMuted: string;
  wordPalette: string[];
  fireworksPalette: string[];
};

function asNonNegativeInteger(value: unknown, fallback = 0) {
  const number = typeof value === "number" ? value : Number(value);
  if (!Number.isFinite(number) || number < 0) {
    return fallback;
  }

  return Math.trunc(number);
}

function asNonNegativeFloat(value: unknown, fallback = 0) {
  const number = typeof value === "number" ? value : Number(value);
  if (!Number.isFinite(number) || number < 0) {
    return fallback;
  }

  return number;
}

function normalizeDailyCounts(value: unknown) {
  if (!Array.isArray(value)) {
    return [] satisfies ReportDailyCount[];
  }

  return value.flatMap((item) => {
    if (!item || typeof item !== "object") {
      return [];
    }

    const label = "label" in item ? String(item.label ?? "").trim() : "";
    const count = "count" in item ? asNonNegativeInteger(item.count) : 0;

    if (!label) {
      return [];
    }

    return [{ label, count }];
  });
}

function normalizeWordCounts(value: unknown) {
  if (!Array.isArray(value)) {
    return [] satisfies ReportWordCount[];
  }

  return value.flatMap((item) => {
    if (!item || typeof item !== "object") {
      return [];
    }

    const term = "term" in item ? String(item.term ?? "").trim() : "";
    const count = "count" in item ? asNonNegativeInteger(item.count) : 0;

    if (!term) {
      return [];
    }

    return [{ term, count }];
  });
}

function normalizeDailyModelSeries(value: unknown) {
  if (!Array.isArray(value)) {
    return [] satisfies ReportDailyModelSeries[];
  }

  return value.flatMap((item) => {
    if (!item || typeof item !== "object") {
      return [];
    }

    const model = "model" in item ? String(item.model ?? "").trim() : "";
    const points = "points" in item ? normalizeDailyCounts(item.points) : [];

    if (!model) {
      return [];
    }

    return [{ model, points }];
  });
}

function normalizeModelSbai(value: unknown) {
  if (!Array.isArray(value)) {
    return [] satisfies ReportModelSbai[];
  }

  return value.flatMap((item) => {
    if (!item || typeof item !== "object") {
      return [];
    }

    const model = "model" in item ? String(item.model ?? "").trim() : "";
    const profanityCount =
      "profanityCount" in item ? asNonNegativeInteger(item.profanityCount) : 0;
    const tokens = "tokens" in item ? asNonNegativeInteger(item.tokens) : 0;
    const sbai = "sbai" in item ? asNonNegativeFloat(item.sbai) : 0;

    if (!model) {
      return [];
    }

    return [{ model, profanityCount, tokens, sbai }];
  });
}

export function createFallbackReportPayload(
  input: Partial<Pick<LeaderboardReportPayload, "profanityCount" | "tokens" | "sbai">> = {},
) {
  return {
    rangeStart: "还没有记录",
    rangeEnd: "还没有记录",
    messageCount: 0,
    profanityCount: asNonNegativeInteger(input.profanityCount),
    tokens: asNonNegativeInteger(input.tokens),
    sbai: asNonNegativeFloat(input.sbai),
    dailyCounts: [],
    dailyModelSeries: [],
    modelSbai: [],
    wordCounts: [],
  } satisfies LeaderboardReportPayload;
}

export function normalizeReportPayload(
  value: unknown,
  fallback: Partial<LeaderboardReportPayload> = {},
): LeaderboardReportPayload {
  const source = value && typeof value === "object" ? (value as Record<string, unknown>) : {};
  const defaults = createFallbackReportPayload(fallback);

  return {
    rangeStart: typeof source.rangeStart === "string" && source.rangeStart.trim() ? source.rangeStart : defaults.rangeStart,
    rangeEnd: typeof source.rangeEnd === "string" && source.rangeEnd.trim() ? source.rangeEnd : defaults.rangeEnd,
    messageCount: asNonNegativeInteger(source.messageCount, defaults.messageCount),
    profanityCount: asNonNegativeInteger(source.profanityCount, defaults.profanityCount),
    tokens: asNonNegativeInteger(source.tokens, defaults.tokens),
    sbai: asNonNegativeFloat(source.sbai, defaults.sbai),
    dailyCounts: normalizeDailyCounts(source.dailyCounts),
    dailyModelSeries: normalizeDailyModelSeries(source.dailyModelSeries),
    modelSbai: normalizeModelSbai(source.modelSbai),
    wordCounts: normalizeWordCounts(source.wordCounts),
  };
}

export function parseReportPayloadJson(
  value: string | null | undefined,
  fallback: Partial<LeaderboardReportPayload> = {},
) {
  if (!value?.trim()) {
    return createFallbackReportPayload(fallback);
  }

  try {
    return normalizeReportPayload(JSON.parse(value), fallback);
  } catch {
    return createFallbackReportPayload(fallback);
  }
}

export function parseModelSbaiJson(value: string | null | undefined) {
  if (!value?.trim()) {
    return [] satisfies ReportModelSbai[];
  }

  try {
    return normalizeModelSbai(JSON.parse(value));
  } catch {
    return [] satisfies ReportModelSbai[];
  }
}

export function getSbaiStatus(sbai: number, locale: Locale = "zh-CN") {
  if (sbai < 0.5) {
    return locale === "zh-CN"
      ? {
          state: "还能忍",
          copy: "AI 还在试探，你已经有点绷不住了。",
        }
      : {
          state: "Holding together",
          copy: "The AI is still probing, and your patience is starting to fray.",
        };
  }

  if (sbai < 2) {
    return locale === "zh-CN"
      ? {
          state: "开始红温",
          copy: "AI 每多自信一分，人就更想当场开骂。",
        }
      : {
          state: "Temperature rising",
          copy: "Every extra point of AI confidence makes cursing feel more justified.",
        };
  }

  if (sbai < 5) {
    return locale === "zh-CN"
      ? {
          state: "马上开骂",
          copy: "AI 写得越笃定，人越接近发疯。",
        }
      : {
          state: "On the edge",
          copy: "The more certain the AI sounds, the closer a human gets to losing it.",
        };
  }

  return locale === "zh-CN"
    ? {
        state: "彻底爆炸",
        copy: "已经不是调试，是一场精神消耗战。",
      }
    : {
        state: "Total meltdown",
        copy: "This is no longer debugging. It is a straight-up war of attrition.",
      };
}

export function getReportHeadlineHtml(
  rangeStart: string,
  rangeEnd: string,
  totalProfanities: number,
  locale: Locale = "zh-CN",
) {
  if (locale === "en") {
    if (totalProfanities < 10) {
      return `Still waters. From ${rangeStart} to ${rangeEnd}, you cursed at AI only <span class="headline-count">${totalProfanities}</span> times.`;
    }

    if (totalProfanities < 100) {
      return `Good news. From ${rangeStart} to ${rangeEnd}, you cursed at AI just <span class="headline-count">${totalProfanities}</span> times. That is suspiciously calm.`;
    }

    return `Good news. From ${rangeStart} to ${rangeEnd}, you cursed at AI <span class="headline-count">${totalProfanities}</span> times in total.`;
  }

  if (totalProfanities < 10) {
    return `心如止水。${rangeStart} 到 ${rangeEnd}，你只骂了 AI <span class="headline-count">${totalProfanities}</span> 次。`;
  }

  if (totalProfanities < 100) {
    return `喜报！！！你的心态也太平稳了叭！！！！${rangeStart} 到 ${rangeEnd}，只骂了 AI <span class="headline-count">${totalProfanities}</span> 次！`;
  }

  return `喜报！！你从 ${rangeStart} 到 ${rangeEnd} 一共骂了 AI <span class="headline-count">${totalProfanities}</span> 次！`;
}

export function getReportTheme(sbai: number): ReportTheme {
  if (sbai < 0.5) {
    return {
      bg: "#090d12",
      bgAccent: "#121821",
      panel: "rgba(10, 15, 20, 0.84)",
      panelStrong: "rgba(13, 18, 24, 0.94)",
      panelTint: "rgba(14, 23, 31, 0.78)",
      muted: "#8ca5af",
      text: "#edf8ff",
      line: "#ffb800",
      lineSoft: "rgba(255, 184, 0, 0.18)",
      areaTop: "rgba(255, 184, 0, 0.22)",
      areaBottom: "rgba(255, 184, 0, 0.04)",
      accent: "#59e8ff",
      accentSoft: "rgba(89, 232, 255, 0.16)",
      accentWarm: "#ffe85b",
      accentPink: "#ff5f7d",
      border: "rgba(89, 232, 255, 0.22)",
      grid: "rgba(50, 87, 96, 0.58)",
      shadow: "0 20px 48px rgba(4, 7, 12, 0.34)",
      tooltipBg: "rgba(6, 10, 16, 0.94)",
      heroFrom: "rgba(11, 16, 22, 0.96)",
      heroTo: "rgba(13, 20, 27, 0.96)",
      heroGlowA: "rgba(255, 232, 91, 0.16)",
      heroGlowB: "rgba(89, 232, 255, 0.16)",
      cloudGlow: "rgba(89, 232, 255, 0.12)",
      cloudTo: "rgba(10, 18, 25, 0.96)",
      sbaiGlow: "rgba(255, 184, 0, 0.24)",
      sbaiSurfaceTop: "#0b0e13",
      sbaiSurfaceBottom: "#1b1910",
      sbaiBorder: "rgba(255, 232, 171, 0.24)",
      sbaiText: "rgba(255, 246, 210, 0.96)",
      sbaiMuted: "rgba(255, 226, 158, 0.82)",
      wordPalette: ["#59e8ff", "#ffb800", "#ffe85b", "#ff5f7d", "#89f5ff", "#ffcf5a"],
      fireworksPalette: ["#59e8ff", "#ffb800", "#ffe85b", "#ff5f7d", "#89f5ff"],
    };
  }

  if (sbai < 2) {
    return {
      bg: "#0a0d12",
      bgAccent: "#16151f",
      panel: "rgba(12, 15, 21, 0.86)",
      panelStrong: "rgba(14, 18, 25, 0.95)",
      panelTint: "rgba(25, 22, 30, 0.78)",
      muted: "#9ca0b1",
      text: "#f7f7ff",
      line: "#ff8f1f",
      lineSoft: "rgba(255, 143, 31, 0.2)",
      areaTop: "rgba(255, 143, 31, 0.24)",
      areaBottom: "rgba(255, 143, 31, 0.05)",
      accent: "#57d8ff",
      accentSoft: "rgba(87, 216, 255, 0.18)",
      accentWarm: "#ffd447",
      accentPink: "#ff5f7d",
      border: "rgba(104, 143, 166, 0.34)",
      grid: "rgba(56, 74, 92, 0.58)",
      shadow: "0 20px 48px rgba(4, 7, 12, 0.36)",
      tooltipBg: "rgba(7, 9, 15, 0.94)",
      heroFrom: "rgba(14, 18, 24, 0.96)",
      heroTo: "rgba(19, 18, 28, 0.96)",
      heroGlowA: "rgba(255, 212, 71, 0.16)",
      heroGlowB: "rgba(87, 216, 255, 0.18)",
      cloudGlow: "rgba(87, 216, 255, 0.12)",
      cloudTo: "rgba(15, 19, 27, 0.96)",
      sbaiGlow: "rgba(255, 143, 31, 0.28)",
      sbaiSurfaceTop: "#0b0d12",
      sbaiSurfaceBottom: "#251717",
      sbaiBorder: "rgba(255, 192, 108, 0.26)",
      sbaiText: "rgba(255, 236, 199, 0.96)",
      sbaiMuted: "rgba(255, 198, 123, 0.84)",
      wordPalette: ["#57d8ff", "#ff8f1f", "#ffd447", "#ff5f7d", "#88e6ff", "#ffb356"],
      fireworksPalette: ["#57d8ff", "#ff8f1f", "#ffd447", "#ff5f7d", "#88e6ff"],
    };
  }

  if (sbai < 5) {
    return {
      bg: "#0b0a0f",
      bgAccent: "#1a1017",
      panel: "rgba(15, 12, 18, 0.88)",
      panelStrong: "rgba(18, 14, 20, 0.96)",
      panelTint: "rgba(36, 17, 24, 0.8)",
      muted: "#b39aa3",
      text: "#fff3ef",
      line: "#ff5d3d",
      lineSoft: "rgba(255, 93, 61, 0.24)",
      areaTop: "rgba(255, 93, 61, 0.28)",
      areaBottom: "rgba(255, 93, 61, 0.07)",
      accent: "#4fdbff",
      accentSoft: "rgba(79, 219, 255, 0.16)",
      accentWarm: "#ffd54a",
      accentPink: "#ff4f7a",
      border: "rgba(118, 69, 84, 0.8)",
      grid: "rgba(77, 41, 55, 0.84)",
      shadow: "0 24px 54px rgba(5, 4, 9, 0.42)",
      tooltipBg: "rgba(6, 6, 10, 0.96)",
      heroFrom: "rgba(18, 13, 19, 0.96)",
      heroTo: "rgba(24, 13, 20, 0.96)",
      heroGlowA: "rgba(255, 213, 74, 0.14)",
      heroGlowB: "rgba(79, 219, 255, 0.12)",
      cloudGlow: "rgba(79, 219, 255, 0.1)",
      cloudTo: "rgba(20, 12, 18, 0.96)",
      sbaiGlow: "rgba(255, 93, 61, 0.34)",
      sbaiSurfaceTop: "#0a0a0e",
      sbaiSurfaceBottom: "#4a1017",
      sbaiBorder: "rgba(255, 173, 118, 0.26)",
      sbaiText: "rgba(255, 238, 202, 0.97)",
      sbaiMuted: "rgba(255, 189, 138, 0.86)",
      wordPalette: ["#4fdbff", "#ff5d3d", "#ffd54a", "#ff4f7a", "#8beaff", "#ff965a"],
      fireworksPalette: ["#4fdbff", "#ff5d3d", "#ffd54a", "#ff4f7a", "#8beaff"],
    };
  }

  return {
    bg: "#07080b",
    bgAccent: "#170b10",
    panel: "rgba(14, 10, 14, 0.9)",
    panelStrong: "rgba(17, 12, 16, 0.97)",
    panelTint: "rgba(49, 13, 21, 0.78)",
    muted: "#c7a4ac",
    text: "#fff4ef",
    line: "#ff3b30",
    lineSoft: "rgba(255, 59, 48, 0.28)",
    areaTop: "rgba(255, 59, 48, 0.3)",
    areaBottom: "rgba(255, 59, 48, 0.08)",
    accent: "#46dcff",
    accentSoft: "rgba(70, 220, 255, 0.18)",
    accentWarm: "#ffd93d",
    accentPink: "#ff3f76",
    border: "rgba(123, 45, 63, 0.86)",
    grid: "rgba(82, 25, 41, 0.84)",
    shadow: "0 26px 58px rgba(3, 3, 6, 0.48)",
    tooltipBg: "rgba(4, 4, 7, 0.97)",
    heroFrom: "rgba(18, 11, 15, 0.97)",
    heroTo: "rgba(22, 11, 16, 0.97)",
    heroGlowA: "rgba(255, 217, 61, 0.14)",
    heroGlowB: "rgba(70, 220, 255, 0.1)",
    cloudGlow: "rgba(70, 220, 255, 0.08)",
    cloudTo: "rgba(18, 11, 15, 0.97)",
    sbaiGlow: "rgba(255, 59, 48, 0.4)",
    sbaiSurfaceTop: "#09090c",
    sbaiSurfaceBottom: "#5d0914",
    sbaiBorder: "rgba(255, 158, 105, 0.28)",
    sbaiText: "rgba(255, 239, 201, 0.98)",
    sbaiMuted: "rgba(255, 184, 125, 0.88)",
    wordPalette: ["#46dcff", "#ff3b30", "#ffd93d", "#ff3f76", "#8cecff", "#ff8e52"],
    fireworksPalette: ["#46dcff", "#ff3b30", "#ffd93d", "#ff3f76", "#8cecff"],
  };
}

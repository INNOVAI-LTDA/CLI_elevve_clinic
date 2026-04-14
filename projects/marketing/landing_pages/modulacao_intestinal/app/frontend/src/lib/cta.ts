import {
  parseLandingPageFromPathname,
  resolveTrackingStatus,
} from "@/lib/tracking";

const DEFAULT_CTA_PROXY_URL = "https://dracristal.com.br/mod-whatsapp";

const viteEnv = import.meta.env as Record<string, string | undefined>;

const baseCtaProxyUrl =
  viteEnv.VITE_CTA_PROXY_URL?.trim() ||
  viteEnv.VITE_WHATSAPP_PROXY_URL?.trim() ||
  DEFAULT_CTA_PROXY_URL;

const status = resolveTrackingStatus(viteEnv.VITE_TRACKING_STATUS);
const landingPage =
  typeof window === "undefined"
    ? parseLandingPageFromPathname(viteEnv.VITE_BASE_PATH || "/")
    : parseLandingPageFromPathname(window.location.pathname);

function buildTrackedCtaUrl(url: string): string {
  try {
    const parsed = new URL(url, typeof window !== "undefined" ? window.location.origin : "http://localhost");
    parsed.searchParams.set("landing_page", landingPage);
    parsed.searchParams.set("status", status);
    return parsed.toString();
  } catch {
    return url;
  }
}

export const CTA_URL = buildTrackedCtaUrl(baseCtaProxyUrl);

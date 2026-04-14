const DEFAULT_WHATSAPP_PROXY_URL = "/mod-whatsapp";
const DEFAULT_LANDING_PAGE = "modulacao_intestinal";
const DEFAULT_STATUS = "click";
const DEFAULT_CHANNEL = "whatsapp";

const viteEnv = import.meta.env as Record<string, string | undefined>;

const baseCtaUrl =
  viteEnv.VITE_WHATSAPP_PROXY_URL?.trim() || DEFAULT_WHATSAPP_PROXY_URL;

export const CTA_URL = baseCtaUrl;

export type TrackingPayload = {
  component: string;
  landing_page: string;
  status: string;
  channel: string;
  timestamp: string;
};

export const buildTrackingPayload = (component: string): TrackingPayload => ({
  component,
  landing_page: viteEnv.VITE_TRACKING_LANDING_PAGE?.trim() || DEFAULT_LANDING_PAGE,
  status: viteEnv.VITE_TRACKING_STATUS?.trim() || DEFAULT_STATUS,
  channel: viteEnv.VITE_TRACKING_CHANNEL?.trim() || DEFAULT_CHANNEL,
  timestamp: new Date().toISOString(),
});

export const buildTrackedCtaUrl = (component: string): string => {
  const payload = buildTrackingPayload(component);
  const url = new URL(baseCtaUrl, window.location.origin);

  for (const [key, value] of Object.entries(payload)) {
    url.searchParams.set(key, value);
  }

  if (/^https?:\/\//i.test(baseCtaUrl)) {
    return url.toString();
  }

  return `${url.pathname}${url.search}`;
};

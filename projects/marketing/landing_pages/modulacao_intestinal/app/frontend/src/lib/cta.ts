const DEFAULT_WHATSAPP_PROXY_URL = "https://dracristal.com.br/mod-whatsapp";

export const resolveCtaUrl = (ctaProxyUrl = import.meta.env.VITE_CTA_PROXY_URL) =>
  ctaProxyUrl?.trim() || DEFAULT_WHATSAPP_PROXY_URL;

export const CTA_URL = resolveCtaUrl();

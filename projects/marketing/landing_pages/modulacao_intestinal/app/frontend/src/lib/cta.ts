const DEFAULT_WHATSAPP_NUMBER = "5511944885013";

const sanitizeWhatsappNumber = (number?: string) => number?.replace(/\D/g, "");

const normalizeProxyUrl = (url?: string) => {
  const normalized = url?.trim();
  return normalized ? normalized : undefined;
};

const viteEnv = import.meta.env as Record<string, string | undefined>;

const ctaProxyUrl = normalizeProxyUrl(viteEnv.VITE_CTA_PROXY_URL);
const whatsappNumber = sanitizeWhatsappNumber(viteEnv.VITE_WHATSAPP_NUMBER);

const fallbackWhatsappNumber = whatsappNumber || DEFAULT_WHATSAPP_NUMBER;

export const CTA_URL = ctaProxyUrl || `https://wa.me/${fallbackWhatsappNumber}`;

const DEFAULT_WHATSAPP_PROXY_URL = "https://dracristal.com.br/mod-whatsapp";

const viteEnv = import.meta.env as Record<string, string | undefined>;

export const CTA_URL =
  viteEnv.VITE_WHATSAPP_PROXY_URL?.trim() || DEFAULT_WHATSAPP_PROXY_URL;

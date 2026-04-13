const DEFAULT_WHATSAPP_NUMBER = "5511944885013";

const sanitizeWhatsappNumber = (number?: string) =>
  number?.replace(/\D/g, "") || DEFAULT_WHATSAPP_NUMBER;

const viteEnv = import.meta.env as Record<string, string | undefined>;

export const CTA_URL = `https://wa.me/${sanitizeWhatsappNumber(viteEnv.VITE_WHATSAPP_NUMBER)}`;

export const CTA_URL = `https://wa.me/${sanitizeWhatsappNumber(viteEnv.VITE_WHATSAPP_NUMBER)}`;

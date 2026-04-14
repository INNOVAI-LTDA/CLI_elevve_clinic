/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_CTA_PROXY_URL?: string;
  readonly VITE_WHATSAPP_NUMBER?: string;
  readonly VITE_WHATSAPP_PROXY_URL?: string;
  readonly VITE_TRACKING_LANDING_PAGE?: string;
  readonly VITE_TRACKING_STATUS?: string;
  readonly VITE_TRACKING_CHANNEL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

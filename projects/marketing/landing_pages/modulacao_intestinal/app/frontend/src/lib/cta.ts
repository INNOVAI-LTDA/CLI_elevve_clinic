import type { MouseEvent } from "react";

const DEFAULT_WHATSAPP_PROXY_URL = "https://dracristal.com.br/mod-whatsapp";

const viteEnv = import.meta.env as Record<string, string | undefined>;

export const CTA_URL =
  viteEnv.VITE_WHATSAPP_PROXY_URL?.trim() || DEFAULT_WHATSAPP_PROXY_URL;

export type CTAComponent = "hero" | "consultation" | "for_you" | "doctor" | "faq";

type CTAEventPayload = {
  component: CTAComponent;
  destination: string;
  timestamp: string;
};

declare global {
  interface Window {
    dataLayer?: Array<Record<string, unknown>>;
    gtag?: (...args: unknown[]) => void;
    fbq?: (...args: unknown[]) => void;
  }
}

const trackCtaClick = ({ component, destination, timestamp }: CTAEventPayload) => {
  window.dataLayer?.push({
    event: "cta_click",
    component,
    destination,
    timestamp,
  });

  window.gtag?.("event", "cta_click", {
    component,
    destination,
  });

  window.fbq?.("trackCustom", "cta_click", {
    component,
    destination,
  });
};

export const handleCTAClick = (
  event: MouseEvent<HTMLAnchorElement>,
  component: CTAComponent,
) => {
  event.preventDefault();

  const destination = event.currentTarget.href;
  const timestamp = new Date().toISOString();

  trackCtaClick({ component, destination, timestamp });

  const newTab = window.open(destination, "_blank", "noopener,noreferrer");

  if (!newTab) {
    window.location.assign(destination);
  }
};

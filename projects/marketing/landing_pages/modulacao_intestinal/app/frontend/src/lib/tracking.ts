export const ALLOWED_TRACKING_STATUS = ["val", "tst", "prod"] as const;

export type TrackingStatus = (typeof ALLOWED_TRACKING_STATUS)[number];

const TRACKING_STATUS_SET = new Set<string>(ALLOWED_TRACKING_STATUS);

export const DEFAULT_TRACKING_STATUS: TrackingStatus = "tst";

export function parseLandingPageFromPathname(pathname: string): string {
  const cleanPath = pathname.trim().replace(/^\/+|\/+$/g, "");
  return cleanPath || "root";
}

export function isTrackingStatus(value: string | undefined | null): value is TrackingStatus {
  return typeof value === "string" && TRACKING_STATUS_SET.has(value);
}

export function resolveTrackingStatus(rawStatus: string | undefined): TrackingStatus {
  const normalized = rawStatus?.trim().toLowerCase();

  if (isTrackingStatus(normalized)) {
    return normalized;
  }

  return DEFAULT_TRACKING_STATUS;
}

export function assertTrackingStatus(rawStatus: string | undefined, source = "status"): TrackingStatus {
  const normalized = rawStatus?.trim().toLowerCase();

  if (!isTrackingStatus(normalized)) {
    throw new Error(
      `Invalid ${source}: \"${rawStatus ?? ""}\". Allowed values: ${ALLOWED_TRACKING_STATUS.join(", ")}.`,
    );
  }

  return normalized;
}

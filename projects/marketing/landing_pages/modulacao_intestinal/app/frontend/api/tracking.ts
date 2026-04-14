import { assertTrackingStatus } from "../src/lib/tracking";

type Req = {
  method?: string;
  query?: Record<string, string | string[] | undefined>;
};

type Res = {
  status: (statusCode: number) => Res;
  json: (body: unknown) => void;
};

function readQueryStringValue(value: string | string[] | undefined): string | undefined {
  return Array.isArray(value) ? value[0] : value;
}

function isValidLandingPage(value: string | undefined): boolean {
  return typeof value === "string" && /^[a-z0-9-]+$/.test(value);
}

export default function handler(req: Req, res: Res): void {
  if (req.method && req.method !== "GET") {
    res.status(405).json({ error: "Method not allowed" });
    return;
  }

  const status = readQueryStringValue(req.query?.status);
  const landingPage = readQueryStringValue(req.query?.landing_page);

  try {
    assertTrackingStatus(status, "status");
  } catch (error) {
    res.status(400).json({ error: (error as Error).message });
    return;
  }

  if (!isValidLandingPage(landingPage)) {
    res.status(400).json({
      error:
        'Invalid landing_page. Use lowercase letters, numbers, and hyphen only (example: "intestino").',
    });
    return;
  }

  res.status(200).json({ ok: true, status, landing_page: landingPage });
}

const DEFAULT_TARGET_URL = "https://dracristal.com.br/mod-whatsapp";
const REQUIRED_FIELDS = ["component", "landing_page", "status", "channel", "timestamp"];

function readBody(req) {
  if (req.body && typeof req.body === "object") {
    return req.body;
  }

  if (typeof req.body === "string") {
    try {
      return JSON.parse(req.body);
    } catch {
      return Object.fromEntries(new URLSearchParams(req.body));
    }
  }

  return {};
}

export default function handler(req, res) {
  const method = req.method || "GET";

  if (!["GET", "POST"].includes(method)) {
    res.setHeader("Allow", "GET, POST");
    return res.status(405).json({ error: "method_not_allowed" });
  }

  const query = req.query && typeof req.query === "object" ? req.query : {};
  const body = method === "POST" ? readBody(req) : {};

  const normalized = {
    component: (query.component || body.component || "unknown").toString(),
    landing_page: (query.landing_page || body.landing_page || "modulacao_intestinal").toString(),
    status: (query.status || body.status || "click").toString(),
    channel: (query.channel || body.channel || "whatsapp").toString(),
    timestamp: (query.timestamp || body.timestamp || new Date().toISOString()).toString(),
  };

  const missing = REQUIRED_FIELDS.filter((field) => !normalized[field]);
  if (missing.length > 0) {
    return res.status(400).json({ error: "missing_required_fields", missing });
  }

  const targetBase = process.env.WHATSAPP_FORWARD_URL || DEFAULT_TARGET_URL;
  const targetUrl = new URL(targetBase);

  for (const [key, value] of Object.entries(normalized)) {
    targetUrl.searchParams.set(key, value);
  }

  console.info("[mod-whatsapp-tracking]", normalized);

  res.setHeader("Cache-Control", "no-store");
  return res.redirect(302, targetUrl.toString());
}

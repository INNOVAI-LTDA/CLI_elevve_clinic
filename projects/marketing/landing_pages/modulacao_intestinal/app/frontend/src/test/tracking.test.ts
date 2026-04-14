import { describe, expect, it } from "vitest";

import handler from "../../api/tracking";
import {
  assertTrackingStatus,
  parseLandingPageFromPathname,
  resolveTrackingStatus,
} from "@/lib/tracking";

describe("tracking helpers", () => {
  it("derives landing_page from pathname", () => {
    expect(parseLandingPageFromPathname("/intestino")).toBe("intestino");
    expect(parseLandingPageFromPathname("/intestino/")).toBe("intestino");
    expect(parseLandingPageFromPathname("/")).toBe("root");
  });

  it("accepts only val, tst and prod", () => {
    expect(resolveTrackingStatus("val")).toBe("val");
    expect(resolveTrackingStatus("tst")).toBe("tst");
    expect(resolveTrackingStatus("prod")).toBe("prod");
    expect(resolveTrackingStatus("invalid")).toBe("tst");

    expect(() => assertTrackingStatus("invalid")).toThrow(/Allowed values: val, tst, prod/);
  });
});

describe("tracking backend validation", () => {
  const createResponse = () => {
    const output = {
      statusCode: 200,
      body: undefined as unknown,
      status(code: number) {
        output.statusCode = code;
        return output;
      },
      json(payload: unknown) {
        output.body = payload;
      },
    };

    return output;
  };

  it("rejects invalid status", () => {
    const res = createResponse();

    handler(
      { method: "GET", query: { landing_page: "intestino", status: "stage" } },
      res,
    );

    expect(res.statusCode).toBe(400);
    expect(res.body).toMatchObject({ error: expect.stringContaining("Allowed values: val, tst, prod") });
  });

  it("rejects invalid landing_page", () => {
    const res = createResponse();

    handler(
      { method: "GET", query: { landing_page: "Intestino Premium", status: "tst" } },
      res,
    );

    expect(res.statusCode).toBe(400);
    expect(res.body).toMatchObject({ error: expect.stringContaining("Invalid landing_page") });
  });

  it("accepts valid payload", () => {
    const res = createResponse();

    handler(
      { method: "GET", query: { landing_page: "intestino", status: "prod" } },
      res,
    );

    expect(res.statusCode).toBe(200);
    expect(res.body).toEqual({ ok: true, landing_page: "intestino", status: "prod" });
  });
});

import { describe, expect, it } from "vitest";

import { resolveCtaUrl } from "./cta";

describe("resolveCtaUrl", () => {
  it("should fallback to default URL when value is empty", () => {
    expect(resolveCtaUrl("")).toBe("https://dracristal.com.br/mod-whatsapp");
  });
});

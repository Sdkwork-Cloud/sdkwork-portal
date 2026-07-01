import { describe, expect, it } from "vitest";

import { formatPortalHeadline, normalizePortalPreferencesInput } from "../src/index.ts";

describe("portal service", () => {
  it("normalizes pinned app keys with utils helpers", () => {
    const result = normalizePortalPreferencesInput({
      pinnedAppKeys: [" Docs ", "docs", "Drive"],
      theme: " dark ",
    });

    expect(result.pinnedAppKeys).toEqual(["docs", "drive"]);
    expect(result.theme).toBe("dark");
  });

  it("formats portal headline", () => {
    expect(
      formatPortalHeadline({ pinnedAppKeys: ["docs"], theme: "system" }),
    ).toBe("Portal (system, 1 pinned apps)");
  });
});

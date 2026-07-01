import { isBlank, slugify, trim, unique } from "@sdkwork/utils";

import type {
  PortalPreferences,
  UpdatePortalPreferencesInput,
} from "@sdkwork/portal-contracts";

export function normalizePortalPreferencesInput(
  input: UpdatePortalPreferencesInput,
): PortalPreferences {
  const theme = trim(input.theme) || "system";
  if (isBlank(theme)) {
    throw new Error("portal theme is required");
  }

  const pinnedAppKeys = unique(
    input.pinnedAppKeys
      .map((key) => slugify(trim(key)))
      .filter((key) => !isBlank(key)),
  );

  return { pinnedAppKeys, theme };
}

export function formatPortalHeadline(preferences: PortalPreferences): string {
  const count = preferences.pinnedAppKeys.length;
  return `Portal (${preferences.theme}, ${count} pinned apps)`;
}

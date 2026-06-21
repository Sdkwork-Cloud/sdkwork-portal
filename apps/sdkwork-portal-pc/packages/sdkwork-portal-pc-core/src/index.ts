import {
  formatPortalHeadline,
  normalizePortalPreferencesInput,
} from "@sdkwork/portal-service";

import type { PortalPreferences } from "@sdkwork/portal-contracts";

export function buildPortalPreferencesDraft(
  input: PortalPreferences,
): PortalPreferences {
  return normalizePortalPreferencesInput(input);
}

export function describePortal(preferences: PortalPreferences): string {
  return formatPortalHeadline(preferences);
}

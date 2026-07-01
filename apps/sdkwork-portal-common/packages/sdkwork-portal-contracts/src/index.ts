export type PortalTheme = "system" | "light" | "dark";

export interface PortalPreferences {
  pinnedAppKeys: string[];
  theme: PortalTheme | string;
}

export interface UpdatePortalPreferencesInput {
  pinnedAppKeys: string[];
  theme: string;
}

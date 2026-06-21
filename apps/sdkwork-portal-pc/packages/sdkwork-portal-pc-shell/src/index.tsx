import { buildPortalPreferencesDraft, describePortal } from "@sdkwork/portal-pc-core";

const demoPreferences = buildPortalPreferencesDraft({
  pinnedAppKeys: ["documents", "drive"],
  theme: "system",
});

export function PortalAppShell() {
  const headline = describePortal(demoPreferences);

  return (
    <main className="portal-shell">
      <section className="portal-card">
        <h1>SDKWork Portal</h1>
        <p>{headline}</p>
        <p>Unified PC portal shell aligned with sdkwork-specs.</p>
      </section>
    </main>
  );
}

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { PortalAppShell } from "@sdkwork/portal-pc-shell";

import "./styles.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <PortalAppShell />
  </StrictMode>,
);

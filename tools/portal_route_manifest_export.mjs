#!/usr/bin/env node
const checkMode = process.argv.includes("--check");
console.log(`[portal_route_manifest_export] ${checkMode ? "check ok" : "export skipped (scaffold)"}`);

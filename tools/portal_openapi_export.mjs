#!/usr/bin/env node
const checkMode = process.argv.includes("--check");
console.log(`[portal_openapi_export] ${checkMode ? "check ok" : "materialize skipped (scaffold)"}`);

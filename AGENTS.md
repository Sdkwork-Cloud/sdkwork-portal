# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v2 -->

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/PNPM_SCRIPT_SPEC.md`
- `../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md`
- `../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

Read `apps/sdkwork-portal-pc/sdkwork.app.config.json` when changing portal application behavior, runtime config, SDK wiring, release metadata, or owned capabilities.

## Local Dictionary Structure

- `AGENTS.md`: repository agent entrypoint and relative SDKWork spec index.
- `sdkwork.app.config.json`: not at repo root; PC app manifest lives under `apps/sdkwork-portal-pc/`.
- `.sdkwork/`: local skills, plugins, manifests, and AI workspace metadata.
- `specs/`: local application/component contracts and narrowing rules.
- `apis/`: authored API contracts for platform portal capability.
- `apps/sdkwork-portal-pc/`: PC browser/desktop portal application root.
- `crates/`: Rust crates including `sdkwork-router-portal-*` route crates.
- `database/`: `sdkwork-database` lifecycle assets.
- `sdks/`: SDK family workspaces and generated artifacts.
- `packages/common/portal/`: shared TypeScript portal contracts and services.
- `configs/`, `deployments/`, `scripts/`, `tools/`, `docs/`, `tests/`: config templates, deployment descriptors, validators, documentation, and verification assets.
- `package.json`, `Cargo.toml`: language/build manifests.

## Project Rules

- Canonical domain: `platform`; capability: `portal` (`DOMAIN_SPEC.md`).
- Database table prefix: `platform_` for portal-owned tables.
- App API prefix: `/app/v3/api/portal`.
- Backend API prefix: `/backend/v3/api/portal`.
- Rust HTTP runtimes integrate `sdkwork-web-framework`; database lifecycle uses `sdkwork-database`.
- TypeScript packages consume `@sdkwork/utils` for shared helpers — no local duplicates.
- `sdkwork-discovery` is deferred until RPC/cloud-split deployment exists.
- Generated SDK output under `sdks/**/generated/**` is generator-owned.

## Verification

```bash
pnpm verify
pnpm db:validate
```

# Portal Technical Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-06-29
Specs: `sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`, `WEB_FRAMEWORK_SPEC.md`, `DATABASE_FRAMEWORK_SPEC.md`

## 1. Architecture Overview

`sdkwork-portal` is the platform portal capability application. It exposes user preferences through Rust HTTP route crates, persists state in PostgreSQL via `sdkwork-database`, and delivers a PC React shell under `apps/sdkwork-portal-pc/`.

## 2. Technology Choices

| Layer | Technology |
| --- | --- |
| HTTP runtime | Rust + Axum + `sdkwork-web-framework` |
| Persistence | PostgreSQL + `sdkwork-database` lifecycle |
| PC client | React + Vite |
| Contracts | OpenAPI 3.1.2 + generated TypeScript SDKs |
| Shared helpers | `@sdkwork/utils` / `sdkwork-utils-rust` |

## 3. System Boundaries And Modules

- `crates/sdkwork-routes-portal-*`: HTTP route boundaries (app-api, backend-api)
- `crates/sdkwork-platform-portal-service`: domain service
- `crates/sdkwork-portal-standalone-gateway`: unified process entrypoint
- `packages/common/portal/`: shared TypeScript contracts and normalization
- `apps/sdkwork-portal-pc/`: PC browser shell and future IAM/bootstrap wiring

## 4. API, SDK, And Data Ownership

- App API prefix: `/app/v3/api/portal`
- Backend API prefix: `/backend/v3/api/portal`
- Success envelope: `SdkWorkApiResponse` with numeric `code: 0`, `data`, `traceId`
- Errors: `application/problem+json` (`ProblemDetail`)
- Table prefix: `platform_`
- File upload: not implemented; future work must use `sdkwork-drive` per `DRIVE_SPEC.md`
- Service discovery: deferred until RPC/cloud-split deployment exists

## 5. Deployment And Runtime Topology

- Topology authority: `specs/topology.spec.json` (`appId: sdkwork-portal`)
- Deploy authority: `deployments/deploy.yaml`
- Profiles: standalone unified-process (dev) and cloud split-services (prod)
- Gateway configs: `configs/sdkwork-api-cloud-gateway.portal.{profile}.toml`

## 6. Verification

```bash
pnpm verify
pnpm deploy:validate
pnpm topology:validate
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace ..
```

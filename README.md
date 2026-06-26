# sdkwork-portal

SDKWork **platform portal** capability application: unified PC portal shell, user preferences, and operator console.

- Standards: `../sdkwork-specs/README.md`
- Domain: `platform` / capability: `portal`
- PC app: `apps/sdkwork-portal-pc/`
- HTTP API: `crates/sdkwork-portal-api-server/`
- Database: `database/` via `sdkwork-database`

## Quick start

```bash
pnpm install
pnpm verify
pnpm --dir apps/sdkwork-portal-pc dev
```

## Framework integration

| Framework | Status | Notes |
| --- | --- | --- |
| `sdkwork-web-framework` | integrated | Rust app-api and backend-api route crates |
| `sdkwork-database` | integrated | `database/` lifecycle via `sdkwork-portal-database-host` |
| `sdkwork-utils` | integrated | `@sdkwork/utils` in portal TS packages |
| `sdkwork-discovery` | deferred | No RPC services yet |

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Application Roots

- [apps directory index](apps/README.md)

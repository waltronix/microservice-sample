# microservice-sample

A Rust workspace demonstrating microservice architecture patterns for a library system. Use this as a reference for authentication, authorization, API design, and deployment — these topics will be added incrementally.

## Workspace layout

```
microservice-sample/
├── flake.nix           crane builds, OCI images, devShell
├── libraries/          shared crates (no Axum, no Diesel — pure domain primitives)
│   └── library-core/  UserId, BookId, ReviewId ID types
├── services/
│   ├── user-service/   port 3001
│   ├── book-service/   port 3002
│   └── review-service/ port 3003
├── scripts/
│   └── init-db.sql     creates users, books, reviews databases on first start
└── docker-compose.yml  single Postgres 17 instance (port 5432, 3 databases)
```

## Architecture

Each service follows strict DDD layering:

- `domain/` — pure entities, value objects, repository traits. **No external dependencies.**
- `application/` — use cases, DTO conversions (domain ↔ DTO).
- `infrastructure/` — Diesel repository implementations, DB schema, pool, embedded migrations.
- `api/` — Axum handlers, `AppState`, `ApiError` → `IntoResponse`.

See the global DDD rules in `~/.claude/rules/ddd-architecture.md` for the full layering contract.

## Tech stack

| Concern | Choice |
|---------|--------|
| HTTP | Axum |
| ORM | Diesel (sync, compile-time checked) |
| Async DB pool | deadpool-diesel (`interact` for blocking calls) |
| Migrations | `diesel_migrations::embed_migrations!` — run on startup |
| Database | PostgreSQL 17 (one per service) |
| IDs | `uuid` v4, wrapped in newtypes from `library-core` |
| OpenAPI | utoipa + utoipa-axum — spec at `/api-docs/openapi.json`, Swagger UI at `/swagger-ui` |
| Images | Nix + crane — `nix build .#<service>-image`, loaded via `podman load` |
| Dev shell | `nix develop` — provides rustc, cargo, diesel_cli, cargo-watch (uses host `podman`/`podman-compose`) |

## Running locally

```bash
# First time: allow direnv to load the flake devShell automatically
direnv allow
# After that, the shell is activated whenever you enter the directory.
# To enter manually without direnv:
# nix develop

# Start the Postgres instance (creates users/books/reviews databases on first run)
podman compose up -d

# Copy env and fill in values (or use defaults from .env.example)
cp .env.example .env

# Run a service (migrations run automatically on startup)
cargo run -p user-service
cargo run -p book-service
cargo run -p review-service
```

## Building OCI images

```bash
# Build an image (produces a tarball at ./result)
nix build .#user-service-image

# Load into podman and run
podman load < result
podman run --env-file .env user-service
```

## Migrations

Migrations are embedded in each service binary and run automatically on startup. To work on them manually:

```bash
# From the service directory
cd services/user-service
diesel migration run      # apply pending
diesel migration redo     # rollback + reapply latest
diesel print-schema       # regenerate src/infrastructure/database/schema.rs
```

## Implementation plan

See `.plan/PLAN.md` for the phase index and `.plan/` for the full detail (35 todos across 6 phases). Each todo declares its dependencies. Note: `.plan/` is gitignored — it is local only.

## Out of scope (for now)

- Authentication / authorization
- Inter-service HTTP calls
- Production deployment

# Library System — Microservice Sample

A reference Rust workspace demonstrating microservice patterns. Three independent services manage users, books, and reviews for a library system. Each service owns its database, follows Domain-Driven Design layering, and is independently deployable.

This project is intentionally incremental — authentication, inter-service communication, and deployment will be added in later phases.

## Services

| Service | Port | Responsibility |
|---------|------|----------------|
| `user-service` | 3001 | Create and look up library members |
| `book-service` | 3002 | Manage the books catalog |
| `review-service` | 3003 | Submit and read book reviews |

## Tech stack

- **Axum** — async HTTP framework
- **Diesel** — compile-time checked SQL ORM
- **PostgreSQL 17** — single instance, three databases (`users`, `books`, `reviews`)
- **deadpool-diesel** — async connection pool
- **Embedded migrations** — applied automatically on startup
- **utoipa + utoipa-axum + utoipa-swagger-ui** — OpenAPI 3 spec auto-generated from handler annotations (served at `/api-docs/openapi.json`) with an interactive Swagger UI at `/swagger-ui`

## Prerequisites

- [Nix](https://nixos.org/download) with flakes enabled — provides Rust, `diesel_cli`, and `cargo-watch`
- [direnv](https://direnv.net) — auto-loads the dev shell on `cd` into the project
- `podman` and `podman-compose` — installed on the host (the flake intentionally does **not** ship them; on a NixOS-free distro use your package manager, e.g. `dnf install podman podman-compose` on Fedora)

Without Nix, you'll need Rust stable and `diesel_cli` (`cargo install diesel_cli --no-default-features --features postgres`) installed manually, in addition to podman.

## Quickstart

```bash
# 1. Allow direnv to activate the dev shell automatically
direnv allow
# The shell is now loaded whenever you enter this directory.
# Nix-provided tools: cargo, rustfmt, clippy, diesel_cli, cargo-watch, cargo-nextest.
# podman / podman-compose come from your host system.

# 2. Start the Postgres instance (creates users/books/reviews databases on first run)
podman compose up -d

# 3. Configure environment variables
cp .env.example .env
# Edit .env if your local setup differs from the defaults

# 4. Start a service (migrations run automatically on startup)
cargo run -p user-service
cargo run -p book-service
cargo run -p review-service
```

## Building OCI images

Images are built reproducibly with Nix and crane — no Dockerfile required.

```bash
# Build all images and load them into podman in one step
./scripts/build-images.sh

# Or build a single image manually
nix build .#user-service-image
podman load < result
```

Images use a distroless base — no shell, minimal attack surface.

After building, start the full stack with:

```bash
podman compose up -d
```

## Authorization

Each service enforces authorization via the `authz` library crate (`libraries/authz`). Two backends are supported and selected at runtime via the `AUTHZ_BACKEND` environment variable.

### Backends

| Backend | `AUTHZ_BACKEND` | Policy location |
|---------|-----------------|-----------------|
| [OpenFGA](https://openfga.dev) | `openfga` (default) | `libraries/authz/fga/model.fga` |
| [OPA](https://www.openpolicyagent.org) | `opa` | `libraries/authz/opa/library.rego` |

### Additional environment variables

**OpenFGA** (default):

| Variable | Description |
|----------|-------------|
| `OPENFGA_URL` | OpenFGA server URL, e.g. `http://localhost:8080` |
| `OPENFGA_STORE_ID` | Store ID created during provisioning |
| `OPENFGA_MODEL_ID` | Authorization model ID written to the store |

**OPA**:

| Variable | Description |
|----------|-------------|
| `OPA_URL` | OPA server URL, e.g. `http://localhost:8181` |

### Running locally with OpenFGA

```bash
# Start OpenFGA (and Postgres) via the dev compose file
podman compose -f tests/authz-bdd/docker-compose.test.yml up -d postgres openfga-migrate openfga

# Provision a store and write the model
fga store create --api-url http://localhost:8080 --name dev
fga model write --api-url http://localhost:8080 --store-id <store-id> \
    --file libraries/authz/fga/model.fga

# Set env vars and run a service
AUTHZ_BACKEND=openfga \
OPENFGA_URL=http://localhost:8080 \
OPENFGA_STORE_ID=<store-id> \
OPENFGA_MODEL_ID=<model-id> \
cargo run -p user-service
```

### Running locally with OPA

```bash
# Start OPA (and Postgres) via the OPA compose file
podman compose -f tests/authz-bdd/docker-compose.opa.yml up -d postgres opa

# Load the Rego policy
curl -X PUT http://localhost:8181/v1/policies/library \
    -H 'Content-Type: text/plain' \
    --data-binary @libraries/authz/opa/library.rego

# Run a service
AUTHZ_BACKEND=opa \
OPA_URL=http://localhost:8181 \
cargo run -p user-service
```

### Behaviour-driven tests

End-to-end BDD tests live in `tests/authz-bdd`. They spin up the full stack (services + backend) via Docker Compose, run Cucumber scenarios, and tear down on exit.

```bash
# Run against OpenFGA (default)
cargo test -p authz-bdd

# Run against OPA
AUTHZ_BACKEND=opa cargo test -p authz-bdd
```

The tests require pre-built service images. Build them first:

```bash
./scripts/build-images.sh
```

## API Reference

Every service exposes:

- `GET /api-docs/openapi.json` — raw OpenAPI 3 spec
- `GET /swagger-ui` — interactive Swagger UI to try each endpoint from the browser

### user-service — `localhost:3001`

```bash
# Create a user
curl -s -X POST localhost:3001/users \
  -H 'Content-Type: application/json' \
  -d '{"email":"alice@example.com","display_name":"Alice"}' | jq

# Get a user by ID
curl -s localhost:3001/users/<id> | jq

# List all users
curl -s localhost:3001/users | jq
```

### book-service — `localhost:3002`

```bash
# Create a book
curl -s -X POST localhost:3002/books \
  -H 'Content-Type: application/json' \
  -d '{"isbn":"9780132350884","title":"Clean Code","author":"Robert C. Martin"}' | jq

# Get a book by ID
curl -s localhost:3002/books/<id> | jq

# List all books
curl -s localhost:3002/books | jq

# Delete a book
curl -s -X DELETE localhost:3002/books/<id>
```

### review-service — `localhost:3003`

```bash
# Submit a review (rating: 1–5)
curl -s -X POST localhost:3003/reviews \
  -H 'Content-Type: application/json' \
  -d '{"book_id":"<book-uuid>","user_id":"<user-uuid>","rating":5,"body":"Excellent book!"}' | jq

# List reviews for a book
curl -s localhost:3003/books/<book-id>/reviews | jq
```

## Project structure

```
microservice-sample/
├── Cargo.toml              workspace root
├── scripts/
│   └── init-db.sql         creates users/books/reviews databases on first start
├── docker-compose.yml      Postgres 17 (port 5432, 3 databases)
├── .env.example            DATABASE_URL defaults
├── libraries/
│   └── library-core/      UserId, BookId, ReviewId ID types
└── services/
    ├── user-service/
    ├── book-service/
    └── review-service/
```

Each service is structured as:

```
<service>/src/
├── domain/          pure entities, value objects, repository traits
├── application/     use cases, DTO conversions
├── infrastructure/  Diesel schema, models, pool, repository implementations
└── api/             Axum handlers, request/response DTOs
```

## Working with migrations

Migrations are embedded in each binary and run on startup. For manual use:

```bash
cd services/user-service   # or book-service / review-service

diesel migration run        # apply pending migrations
diesel migration redo       # rollback + reapply latest
diesel print-schema         # regenerate src/infrastructure/database/schema.rs
```

## Environment variables

Each service reads its own service-prefixed variables from the environment (or a `.env` file in the working directory). Defaults are in [`.env.example`](.env.example).

| Service | Database URL | Bind address |
|---------|--------------|--------------|
| `user-service` | `USERS_DATABASE_URL` | `USERS_BIND_ADDR` |
| `book-service` | `BOOKS_DATABASE_URL` | `BOOKS_BIND_ADDR` |
| `review-service` | `REVIEWS_DATABASE_URL` | `REVIEWS_BIND_ADDR` |

`RUST_LOG` can be set to control tracing output (defaults to `info`).

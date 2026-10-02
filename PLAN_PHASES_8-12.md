# HireBridge — Detailed Phase Plans

## Phase 8: CI/CD Pipeline (GitHub Actions)

### Goal
Automated build, test, audit, and release on every push/PR.

### Tasks

**8.1 — GitHub Actions Workflow** (`.github/workflows/ci.yml`)
```yaml
name: CI
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release
      - run: cargo clippy -- -D warnings
      - run: cargo test
      - run: cargo audit
  release:
    needs: build
    if: startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release
      - uses: softprops/action-gh-release@v1
        with:
          files: target/release/hirebridge
```

**8.2 — Rust Toolchain Pinning**
- `rust-toolchain.toml` with stable version
- `rustfmt.toml` for consistent formatting
- `clippy.toml` for lint settings

**8.3 — Cache Configuration**
- Cache cargo registry and target/ directory
- Use `actions/cache@v4`

### Verification
- Push triggers CI pipeline
- All checks pass before merge
- Release tag auto-publishes binary

---

## Phase 9: Docker Deployment

### Goal
One-command deploy: `docker compose up` → dashboard at :8080

### Tasks

**9.1 — Dockerfile** (multi-stage build)
```dockerfile
FROM rust:1.96-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.20
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/hirebridge /usr/local/bin/
EXPOSE 8080
CMD ["hirebridge", "serve"]
```

**9.2 — docker-compose.yml**
```yaml
version: '3.8'
services:
  hirebridge:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - hirebridge-data:/app/data
    environment:
      - HIREBRIDGE_DB_PATH=/app/data/hirebridge.db
      - HIREBRIDGE_SERVER_ADDR=0.0.0.0:8080
volumes:
  hirebridge-data:
```

**9.3 — .dockerignore**
```
target/
.git/
*.db
*.sqlite
```

**9.4 — Health Check**
- Add `/health` endpoint returning `{"status":"ok"}`
- Docker HEALTHCHECK instruction

### Verification
- `docker build .` succeeds
- `docker compose up` → dashboard accessible at localhost:8080
- SQLite persists across container restarts

---

## Phase 10: Monitoring & Observability

### Goal
Production visibility: metrics, health checks, structured logging

### Tasks

**10.1 — Structured Logging**
- Replace `println!` with `tracing` crate
- JSON output to stdout
- Log levels: error/warn/info/debug/trace

**10.2 — Metrics Endpoint** (`/metrics`)
- Prometheus format
- Counters: claims_total, verification_pass_total, verification_fail_total
- Gauges: active_candidates, db_size_bytes
- Histograms: verification_duration_seconds

**10.3 — Health Check** (`/health`)
```json
{"status":"ok","version":"0.1.0","uptime_seconds":1234}
```

**10.4 — Error Tracking**
- `tracing-error` for error spans
- Contextual error messages with `anyhow`

### Verification
- `curl localhost:8080/metrics` returns Prometheus-format metrics
- `curl localhost:8080/health` returns `{"status":"ok"}`
- Logs output valid JSON to stdout

---

## Phase 11: Documentation

### Goal
Complete docs for users and contributors

### Tasks

**11.1 — README.md**
- Project overview
- Quick start (install, serve, dashboard)
- CLI command reference
- Configuration env vars
- Contributing guide

**11.2 — ARCHITECTURE.md**
- 5-layer architecture diagram
- Data flow between layers
- Database schema
- API endpoint reference

**11.3 — API.md**
- Dashboard API endpoints
- Request/response examples
- Error codes

**11.4 — Configuration Reference**
- All env vars documented with defaults
- Examples for each use case

**11.5 — Inline Documentation**
- Doc comments on all public APIs
- `cargo doc` generation

### Verification
- `cargo doc --open` generates clean documentation
- README has working quick-start steps
- All public APIs documented

---

## Phase 12: Performance Optimization

### Goal
Sub-100ms API responses, efficient resource usage

### Tasks

**12.1 — Connection Pooling**
- Replace `rusqlite::Connection` with `r2d2` or `deadpool`
- Pool size: 5-10 connections
- Connection timeout: 5 seconds

**12.2 — Async Database Access**
- `sqlx` with SQLite or `deadpool-rusqlite`
- Non-blocking DB queries

**12.3 — Benchmarking**
- `criterion` benchmarks for:
  - Claim verification throughput
  - Mission evaluation latency
  - Database query performance

**12.4 — Profiling**
- `cargo flamegraph` for CPU hotspots
- `cargo cache` for dependency analysis
- Target: <50ms per API call

### Verification
- Benchmark results in `benches/`
- All API calls <100ms p95
- Connection pool configured and tested

---

## Execution Order

```
Phase 8 (CI/CD)     → Automated quality gates
Phase 9 (Docker)    → Consistent deployment
Phase 10 (Monitoring) → Production visibility
Phase 11 (Docs)     → User/contributor docs
Phase 12 (Perf)     → Optimization last (measure first)
```

## Tools Required

| Tool | Install | Purpose |
|------|---------|---------|
| `cargo-audit` | pre-installed | Security scanning |
| `cargo-deny` | `cargo install cargo-deny` | License/audit |
| `cargo-outdated` | `cargo install cargo-outdated` | Dependency freshness |
| `cargo-flamegraph` | `cargo install flamegraph` | CPU profiling |
| `criterion` | dev-dependency | Benchmarking |
| `cargo-doc` | built-in | Documentation |

# HireBridge Remaining Phases — Research-Backed Plan

## Research Findings (cargo audit, available tools, skills)

### Security Audit Results
- `cargo audit` found 2 issues:
  - **RUSTSEC-2026-0258**: h2 0.3.27 — unbound empty DATA frames → fix: upgrade to >=0.4.16
  - **RUSTSEC-2024-0370**: proc-macro-error 1.0.4 unmaintained → check if still needed

### Available Tools
- `cargo-audit` — installed, ready for dependency scanning
- `docker` — available for containerization
- `gh` — GitHub CLI for CI/CD setup
- `cargo` — Rust toolchain (v1.96.0)

### Relevant Skills
- `security` — security audit checklist (OWASP, secrets, dependencies)
- `security-audit` — deeper security review
- `dependency-audit` — dependency vulnerability scanning
- `ci-cd-and-automation` — CI/CD pipeline setup
- `performance-profiling` — performance optimization
- `shipping-and-launch` — production launch readiness

---

## Phase 7: Security Hardening

**Goal:** Pass `cargo audit` with zero vulnerabilities.

1. Fix h2 vulnerability (upgrade warp dependency or pin h2 >= 0.4.16)
2. Remove unused dependencies (proc-macro-error if unmaintained)
3. Run `cargo-deny` for license compliance
4. Security review using `security` skill checklist
5. Add `.cargo/config.toml` with security settings

**Deliverables:**
- Clean `cargo audit` output
- `cargo-deny` license check passing
- Security audit report

---

## Phase 8: CI/CD Pipeline (GitHub Actions)

**Goal:** Automated build, test, and release on every push.

1. `.github/workflows/ci.yml` — run `cargo build`, `cargo test`, `cargo clippy`, `cargo audit`
2. `.github/workflows/release.yml` — auto-tag and release on semver bumps
3. Rust toolchain pinning (stable version)
4. Cache cargo registry for faster builds

**Deliverables:**
- CI runs on every push/PR
- Automated releases on version tag
- Build artifacts attached to releases

---

## Phase 9: Docker Deployment

**Goal:** One-command deployment via Docker.

1. `Dockerfile` — multi-stage build, minimal image
2. `docker-compose.yml` — HireBridge + SQLite volume
3. `.dockerignore` — exclude target/, .git
4. Health check endpoint for Docker
5. Docker Hub / GHCR image publishing (via CI)

**Deliverables:**
- `docker build .` produces working image
- `docker compose up` starts dashboard at :8080
- Image published to registry on release

---

## Phase 10: Monitoring & Observability

**Goal:** Production visibility into HireBridge operations.

1. Structured JSON logging (replace println!)
2. Metrics endpoint `/metrics` (Prometheus format)
3. Health check endpoint `/health`
4. Error tracking (log levels: error/warn/info/debug)
5. SQLite performance metrics (query times, DB size)

**Deliverables:**
- Logs output to stdout in JSON format
- Prometheus metrics at `/metrics`
- Health check at `/health`

---

## Phase 11: Documentation

**Goal:** Complete documentation for users and contributors.

1. `README.md` — overview, quick start, commands
2. `ARCHITECTURE.md` — system design, layer diagram
3. `API.md` — dashboard API endpoints
4. `CONFIGURATION.md` — all env vars documented
5. `CONTRIBUTING.md` — development setup, PR process
6. Inline docs for all public APIs

**Deliverables:**
- README with examples
- Architecture decision records
- Complete API reference

---

## Phase 12: Performance Optimization

**Goal:** Sub-second response times, efficient resource usage.

1. Profile with `cargo flamegraph`
2. Connection pooling for SQLite (r2d2 or deadpool)
3. Async database access (sqlx or deadpool + rusqlite)
4. Cache frequently accessed data
5. Benchmark critical paths (claim verification, mission eval)

**Deliverables:**
- Performance benchmarks in `benches/`
- Connection pooling configured
- Sub-100ms API response times

---

## Execution Order

```
Phase 7 (Security) → Phase 8 (CI/CD) → Phase 9 (Docker)
    → Phase 10 (Monitoring) → Phase 11 (Docs) → Phase 12 (Perf)
```

Security first (fix vulnerabilities before shipping), then automation,
then deployment, then observability, then docs, then optimization.

## Tools to Install

- `cargo-deny` — license/audit checks
- `cargo-outdated` — dependency freshness
- `cargo-flamegraph` — profiling
- `criterion` — benchmarking framework
- `sqlx-cli` — database migrations (if switching to SQLx)

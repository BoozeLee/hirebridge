# Next Phase: GitHub Integration + Web Dashboard + Analytics

## Context

HireBridge is a Rust CLI hiring verification tool with 5 completed layers:
1. Gate (claim verification with SHA-256 checksum pinning)
2. Harness (task execution sandbox)
3. Team Formation (collaboration assessment)
4. Mission-Based Evaluation (real-world scenarios)
5. Professional Development Integration (competency gap analysis, micro-learning, mentorship matching)

**Current state:**
- CLI-only, SQLite database, all data is mock/static
- `web/src/` directory exists but empty — web dashboard stubbed
- No tests exist
- All 5 layers have hardcoded/demo data, no real GitHub integration

## What to Build

### Layer 6: GitHub Integration (`src/github.rs`)
Real verification data from GitHub APIs instead of mock data.

- `GitHubClient` struct using `reqwest` (already in Cargo.toml)
- Methods:
  - `fetch_commits(candidate_id, repo_url)` — get commit history, count, recency
  - `fetch_contributions(candidate_id)` — contribution stats
  - `verify_repo_activity(claim)` — cross-reference claim against actual repo commits
  - `fetch_pr_history(candidate_id, repo_url)` — PR count, merge rate, review activity
- Use `tokio` for async (already in Cargo.toml)
- Cache responses in SQLite via existing `Database` layer

### Layer 7: Web Dashboard (`web/`)
React or plain HTML/JS dashboard to visualize candidate verification results.

- `web/index.html` — single-page app
- Shows: candidate list, claim status, verification results, mission scores, learning plans
- Fetches from a new `hirebridge serve` CLI subcommand (REST API on port 8080)
- Display: tables, charts (score distribution), candidate comparison view
- Use `warp` or `axum` for the HTTP server (add to Cargo.toml)

### Layer 8: Analytics & Reporting (`src/analytics.rs`)
Aggregate insights from the SQLite database.

- `AnalyticsEngine` struct
- Methods:
  - `candidate_scorecard(candidate_id)` — composite score from all layers
  - `aggregate_pass_rate()` — overall pass/fail stats
  - `trend_analysis(days)` — verification trends over time
  - `top_skills()` — most verified skills across all candidates
- Output: JSON reports, CSV export

### Tests (`tests/`)
- Unit tests for each new module
- Integration tests for the database layer
- Test the new CLI commands

## Engineering the Prompt

When you run this prompt, follow these rules:

1. **DO NOT** change existing working code unless necessary for integration
2. **DO** add `pub mod github;`, `pub mod analytics;` to lib.rs
3. **DO** wire new commands into main.rs: `github-sync`, `serve`, `scorecard`, `analytics`
4. **DO** add new dependencies to Cargo.toml (warp/axum for web, keep reqwest)
5. **DO** write tests in `tests/` directory
6. **DO NOT** add comments to code — code should be self-documenting
7. **DO** keep all structs serializable (serde) for the dashboard
8. **DO** follow existing patterns from claim.rs, harness.rs, mission.rs, profdev.rs
9. **DO** build and test before finishing
10. **DO NOT** push until user asks

## Priority Order
1. GitHub Integration (Layer 6) — most valuable, unlocks real verification
2. Analytics & Reporting (Layer 8) — leverages existing SQLite data
3. Web Dashboard (Layer 7) — depends on API from Layer 6 & 8
4. Tests — parallel with implementation

## Success Criteria
- `cargo build` passes with no errors (warnings ok)
- `cargo test` passes
- New CLI commands work: `github-sync`, `serve`, `scorecard`, `analytics`
- Web dashboard serves at localhost:8080
- Real GitHub data is fetched and verified

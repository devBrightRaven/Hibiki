## Context

Hibiki has 0% automated test coverage. The CI pipeline uses a file-swap hack: `cp src/managers/transcription_mock.rs src/managers/transcription.rs` and `sed` to delete the `transcribe-rs` dependency from `Cargo.toml`. This avoids compiling heavy native dependencies (Whisper/Vulkan/ONNX) but means the real transcription code is never tested in CI.

The frontend has no unit test tooling — no vitest, no jest, no testing-library. The only frontend tests are 2 Playwright smoke tests that verify the Vite dev server returns HTML (no Tauri backend involved). The Zustand store imports `@/bindings` which calls `@tauri-apps/api/core` `invoke()`, requiring a Tauri webview context — untestable without mocking.

Rust side has `tempfile` as the only dev-dependency. No feature flags for test isolation. Some `#[cfg(test)]` modules exist inline but no dedicated test files.

## Goals / Non-Goals

**Goals:**

- Establish a test pyramid: unit (large volume) → integration (medium) → E2E (small)
- Remove the CI mock-swap hack; test real code paths with proper isolation
- Enable frontend unit testing with mocked Tauri bindings
- Reach 80%+ coverage for new code, with a path to 80% overall
- Make tests fast enough to run on every PR without blocking development
- Create reusable test fixtures (WAV clips, settings snapshots) for integration tests

**Non-Goals:**

- Testing platform-specific native code (NSPanel overlay, GTK layer shell, Wayland input tools) — these require real OS environments
- Testing actual model inference quality (accuracy benchmarks are a separate concern)
- Achieving 80% total coverage immediately — focus on coverage gates for new code first
- Load testing or performance benchmarks (future work)
- Changing the application architecture to be more testable (minimal refactoring only)

## Decisions

### D1: Frontend test framework — Vitest (not Jest)

**Decision:** Use Vitest with jsdom environment.

**Rationale:** Vitest shares Vite's config and module resolution. Hibiki already uses Vite for development (`bunx vite dev`). Vitest inherits path aliases (`@/*`), TypeScript settings, and plugin configuration with zero duplication. Jest would require separate transform and alias configuration.

**Alternatives considered:**

- Jest: Requires separate babel/ts-jest transform, duplicate path alias config, slower startup. No advantage for a Vite project.
- Bun test: Viable since Hibiki uses Bun, but ecosystem support for React Testing Library and Tauri mocking is weaker.

### D2: Tauri binding mock strategy — Module-level mock of `@/bindings`

**Decision:** Create a `src/__mocks__/bindings.ts` that exports mock versions of all commands. Use `vi.mock('@/bindings')` in test setup.

**Rationale:** The auto-generated `bindings.ts` calls `@tauri-apps/api/core` `invoke()` which requires `window.__TAURI_INTERNALS__`. Rather than stubbing the Tauri runtime, mock at the module boundary — this is the natural seam between frontend logic and backend. Each mock command returns sensible defaults (e.g., `getAppSettings` returns a default `AppSettings` object).

**Alternatives considered:**

- Stubbing `window.__TAURI_INTERNALS__`: Fragile, tightly coupled to Tauri internals, breaks on Tauri updates.
- `@tauri-apps/api/mocks` (official): Limited, doesn't cover specta-generated bindings well.
- Testing only with Playwright (no unit tests): Too slow for feedback loops, can't test store logic in isolation.

### D3: Rust test isolation — Feature flags (not mock swap)

**Decision:** Introduce a Cargo feature `test-mock-transcription` that conditionally compiles a stub `TranscriptionManager`. Remove the CI file-swap hack.

```toml
[features]
test-mock-transcription = []

[dependencies]
transcribe-rs = { version = "...", optional = true }

[features]
default = ["transcribe-rs"]
```

In CI, run two test passes:

1. `cargo test` (default features, excluding tests that need a real model)
2. `cargo test --features test-mock-transcription --no-default-features` (fast, for integration tests that don't need real inference)

**Rationale:** Feature flags are the Rust-idiomatic way to handle conditional compilation. The file-swap hack is fragile (breaks if `transcription.rs` struct signature changes) and invisible to the type system.

**Alternatives considered:**

- Keep mock swap: Works today, but any signature change to `TranscriptionManager` silently breaks CI. No compiler help.
- Trait-based abstraction (`dyn TranscriptionEngine`): Ideal long-term, but requires significant refactoring of the manager pattern. Non-goal for this change.
- Compile everything always: `transcribe-rs` pulls in Whisper/Vulkan/ONNX — CI build time would increase dramatically. Not practical for PR feedback loops.

### D4: Test fixture strategy — Committed WAV files + generated settings

**Decision:** Commit small WAV fixtures (5-10s, ~100KB each) to `tests/fixtures/audio/`. Generate settings fixtures programmatically in test setup.

**Rationale:** WAV files are small, deterministic, and rarely change. Settings fixtures should be generated from `get_default_settings()` to stay in sync with schema changes. Caching models in CI is a separate concern (see D6).

**Alternatives considered:**

- Git LFS for fixtures: Overkill for <1MB of WAV files.
- Generate audio programmatically: Sine waves don't test real speech paths. Pre-recorded clips with known transcription output are more valuable.

### D5: Coverage tracking — llvm-cov (Rust) + v8 (Vitest)

**Decision:** Use `cargo-llvm-cov` for Rust coverage and Vitest's built-in v8 coverage provider for frontend.

**Rationale:** `llvm-cov` is the standard for Rust, produces lcov output for CI integration. Vitest v8 provider is fast and accurate for TypeScript. Both produce lcov format, enabling unified coverage reporting.

**Gate:** 80% coverage required for new/modified files in PR. Overall project coverage tracked but not gated initially.

**Alternatives considered:**

- `cargo-tarpaulin`: Slower, less accurate on complex Rust code, weaker macOS support.
- Istanbul (frontend): Slower than v8, no advantage for this setup.

### D6: CI pipeline structure — Parallel jobs with model caching

**Decision:** Structure CI as parallel jobs:

```
PR opened
  ├── lint (eslint + clippy + fmt)      ~2min
  ├── test-frontend (vitest)            ~1min
  ├── test-rust-unit (cargo test)       ~3min
  ├── test-rust-integration (cargo test ~5min
  │   --features test-mock-transcription)
  └── test-e2e (playwright)             ~5min
       └── coverage report (merge + gate)
```

Cache `src-tauri/resources/models/silero_vad_v4.onnx` across runs. Do NOT cache transcription models in CI (too large, not needed for mock tests).

**Rationale:** Parallel jobs maximize feedback speed. Separating Rust unit from integration tests allows fast failure. VAD model is small (~2MB) and always needed.

**Alternatives considered:**

- Single sequential job: Simpler config, but 15+ minutes per PR. Too slow.
- Cache full transcription models: 500MB-1.6GB per model. CI cache limits would be hit quickly.

## Risks / Trade-offs

**[Mock drift]** → The `@/bindings` mock may fall out of sync when `bindings.ts` is regenerated.
→ _Mitigation:_ Add a CI step that type-checks the mock against the real bindings. If `bindings.ts` adds a new command, the mock file fails typecheck.

**[Feature flag complexity]** → Adding `test-mock-transcription` feature introduces conditional compilation paths.
→ _Mitigation:_ Limit the feature to `managers/transcription.rs` only. Keep the mock minimal (same struct, stub methods). Document the pattern.

**[Flaky E2E tests]** → Playwright tests against Vite dev server (no Tauri backend) limits what E2E can cover.
→ _Mitigation:_ Accept this limitation for now. True end-to-end (with Tauri) requires `tauri-driver` or similar — scope for a future change.

**[CI time increase]** → Adding 4 parallel test jobs increases total CI resource usage.
→ _Mitigation:_ Each job is fast (<5min). Wall-clock time stays under 5min with parallelism. Monitor and optimize if needed.

**[Coverage gaming]** → 80% gate on new files only could be gamed by putting logic in existing files.
→ _Mitigation:_ Code review catches this. Increase gate to overall coverage once baseline is established.

## Open Questions

- **Q1:** Should Playwright E2E tests run against `bunx vite dev` (current, frontend-only) or `bun run tauri dev` (full app, slower, platform-dependent)? Recommend keeping frontend-only for CI speed, adding occasional full-app tests as a separate manual step.
- **Q2:** Is `bun test` (Bun's built-in test runner) mature enough to replace vitest? It would simplify the toolchain but has less ecosystem support for React Testing Library.
- **Q3:** Should we add snapshot tests for the settings schema to catch accidental breaking changes in `AppSettings`?

## ADDED Requirements

### Requirement: Parallel CI Jobs

The system SHALL run test suites as parallel CI jobs to minimize wall-clock time.

#### Scenario: PR triggers all jobs

- **WHEN** a pull request is opened or updated
- **THEN** the following jobs run in parallel:
  - lint (eslint + clippy + cargo fmt check)
  - test-frontend (vitest)
  - test-rust-unit (cargo test with default features)
  - test-rust-integration (cargo test with test-mock-transcription feature)
  - test-e2e (playwright)

#### Scenario: Job independence

- **WHEN** one test job fails
- **THEN** other jobs continue to completion
- **AND** all results are reported

#### Scenario: Total CI time

- **WHEN** all jobs run in parallel
- **THEN** total wall-clock time SHALL be under 10 minutes

---

### Requirement: Coverage Enforcement

The system SHALL track and enforce test coverage thresholds.

#### Scenario: Rust coverage collection

- **WHEN** Rust tests complete
- **THEN** `cargo-llvm-cov` generates an lcov report

#### Scenario: Frontend coverage collection

- **WHEN** Vitest tests complete
- **THEN** v8 coverage provider generates an lcov report

#### Scenario: Coverage gate on new code

- **WHEN** a PR introduces new or modified files
- **THEN** those files SHALL have at least 80% line coverage
- **AND** the PR is blocked if the gate fails

#### Scenario: Overall coverage tracking

- **WHEN** a CI run completes
- **THEN** overall project coverage is reported (not gated initially)

---

### Requirement: CI Caching

The system SHALL cache build artifacts and fixtures to speed up CI runs.

#### Scenario: VAD model caching

- **WHEN** a CI job needs `silero_vad_v4.onnx`
- **THEN** the model is restored from cache if available
- **AND** downloaded only on cache miss

#### Scenario: Rust build caching

- **WHEN** a Rust test job runs
- **THEN** `target/` and Cargo registry are cached between runs

#### Scenario: Node modules caching

- **WHEN** a frontend test job runs
- **THEN** `node_modules/` is cached based on `bun.lock` hash

---

### Requirement: CI Mock Swap Removal

The system SHALL NOT use file-swap hacks for test isolation in CI.

#### Scenario: No file copying in CI

- **WHEN** CI workflows execute
- **THEN** no `cp` commands overwrite source files
- **AND** no `sed` commands modify `Cargo.toml`

#### Scenario: Feature flags for isolation

- **WHEN** CI needs to skip heavy native dependencies
- **THEN** Cargo feature flags (`--features test-mock-transcription --no-default-features`) are used instead

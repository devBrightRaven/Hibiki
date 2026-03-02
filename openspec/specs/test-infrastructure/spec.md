## ADDED Requirements

### Requirement: Frontend Test Framework

The system SHALL provide a Vitest-based test framework for frontend unit and component testing.

#### Scenario: Vitest configuration

- **WHEN** a developer runs `bun run test`
- **THEN** Vitest executes all `*.test.ts` and `*.test.tsx` files using jsdom environment
- **AND** path aliases (`@/*` → `./src/`) resolve correctly

#### Scenario: React component testing

- **WHEN** a developer writes a component test
- **THEN** `@testing-library/react` is available for rendering and querying components

#### Scenario: Watch mode

- **WHEN** a developer runs `bun run test:watch`
- **THEN** Vitest re-runs affected tests on file change

---

### Requirement: Tauri Binding Mock Layer

The system SHALL provide a mock layer for Tauri command bindings to enable frontend testing without a Tauri runtime.

#### Scenario: Module-level mock

- **WHEN** a test imports from `@/bindings`
- **THEN** the mock layer intercepts the import via `vi.mock('@/bindings')`
- **AND** all commands return sensible defaults (e.g., `getAppSettings` returns default `AppSettings`)

#### Scenario: Mock type safety

- **WHEN** `bindings.ts` is regenerated with new commands
- **AND** the mock file does not export the new command
- **THEN** TypeScript compilation fails, alerting the developer

#### Scenario: Per-test override

- **WHEN** a test needs a specific command to return custom data
- **THEN** the test can override individual mock commands using `vi.mocked(commands.foo).mockResolvedValue(...)`

---

### Requirement: Rust Test Isolation via Feature Flags

The system SHALL use Cargo feature flags to conditionally compile a stub TranscriptionManager for tests that do not require real model inference.

#### Scenario: Default features include transcribe-rs

- **WHEN** `cargo build` runs without flags
- **THEN** the real `TranscriptionManager` with `transcribe-rs` is compiled

#### Scenario: Mock transcription feature

- **WHEN** `cargo test --features test-mock-transcription --no-default-features` runs
- **THEN** a stub `TranscriptionManager` is compiled (same public API, no-op methods)
- **AND** `transcribe-rs` and its native dependencies are NOT compiled

#### Scenario: CI file-swap removal

- **WHEN** CI runs tests
- **THEN** the `cp transcription_mock.rs` and `sed` hack is no longer used
- **AND** feature flags control compilation instead

---

### Requirement: Test Fixtures

The system SHALL provide reusable test fixtures for audio and settings data.

#### Scenario: WAV audio fixtures

- **WHEN** an integration test needs audio input
- **THEN** pre-recorded WAV files (5-10s, ~100KB each) are available in `tests/fixtures/audio/`

#### Scenario: Settings fixtures

- **WHEN** a test needs an `AppSettings` instance
- **THEN** it is generated programmatically from `get_default_settings()` to stay in sync with schema changes

#### Scenario: Fixture discovery

- **WHEN** a developer looks for available test fixtures
- **THEN** fixtures are organized by type: `tests/fixtures/audio/`, `tests/fixtures/settings/`

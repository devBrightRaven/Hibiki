## Why

Hibiki currently has 0% test coverage. The CI pipeline swaps the real TranscriptionManager with a mock, meaning the core transcription path is never automatically tested. There are no frontend unit tests, no integration tests, and only 2 smoke-level Playwright E2E tests. This makes every change a regression risk and blocks quality improvements (e.g., Chinese fluency, gamification) that depend on measurable, repeatable validation.

## What Changes

- Add frontend unit test infrastructure (vitest) with Tauri command mocking
- Add meaningful Rust unit tests for managers, settings, and audio toolkit — without mock-swapping in CI
- Add integration tests for the settings round-trip (frontend store ↔ Tauri command ↔ Rust persistence)
- Add pipeline integration tests using pre-recorded WAV fixtures for audio → VAD → transcription
- Expand Playwright E2E tests to cover the primary user journey (launch → select model → record → transcribe → paste)
- Add coverage tracking and enforcement (80%+ gate on PR merge)
- Establish CI workflow that runs all test layers on every PR

## Capabilities

### New Capabilities

- `test-infrastructure`: Vitest setup for frontend, Rust test harness improvements, Tauri mock layer, test fixtures and utilities
- `test-unit`: Unit test suites for Rust managers (settings, model, history, transcription) and frontend stores/hooks/components
- `test-integration`: Integration tests for settings consistency, transcription pipeline with WAV fixtures, and frontend ↔ backend contract validation
- `test-e2e`: Expanded Playwright E2E tests covering full user journeys across onboarding, recording, transcription, and history
- `test-ci`: CI pipeline configuration with coverage gates, test parallelization, and artifact caching (models, fixtures)

### Modified Capabilities

- `history`: Add requirement for transcription metadata (word count, duration, language) to support future quality benchmarking

## Impact

- **CI/CD**: `.github/workflows/` — new or modified workflows for test, coverage, and gating
- **Frontend**: New `vitest.config.ts`, `src/__tests__/` or co-located test files, Tauri mock utilities
- **Backend**: New test modules in `src-tauri/src/`, removal of mock transcription swap in CI, test fixtures in `src-tauri/tests/` or `tests/fixtures/`
- **Dependencies**: vitest, @testing-library/react, possibly @tauri-apps/api mock — frontend; no new Rust deps expected (uses built-in `#[cfg(test)]`)
- **Repo size**: Pre-recorded WAV test fixtures (small, ~5-10s clips) will add to repo or be cached in CI

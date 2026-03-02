## ADDED Requirements

### Requirement: Settings Consistency Tests

The system SHALL have integration tests that verify frontend and backend settings stay in sync.

#### Scenario: Settings round-trip

- **WHEN** a setting is written via the Rust `write_settings` function
- **AND** then read back via `get_settings`
- **THEN** the values match exactly

#### Scenario: Default settings consistency

- **WHEN** `get_default_settings()` is called in Rust
- **AND** compared to the frontend's expected defaults
- **THEN** all field names, types, and default values match

#### Scenario: Settings schema snapshot

- **WHEN** `AppSettings` struct fields change
- **THEN** a snapshot test detects the change
- **AND** the developer must explicitly update the snapshot

---

### Requirement: Transcription Pipeline Tests

The system SHALL have integration tests for the audio → VAD → transcription pipeline using pre-recorded fixtures.

#### Scenario: VAD filters silence

- **WHEN** a WAV fixture containing only silence is processed through VAD
- **THEN** no speech segments are detected
- **AND** no transcription is attempted

#### Scenario: VAD detects speech

- **WHEN** a WAV fixture containing speech is processed through VAD
- **THEN** speech segments are detected with start/end timestamps
- **AND** detected segments contain the speech audio

#### Scenario: Pipeline end-to-end (mock engine)

- **WHEN** a WAV fixture is processed through the full pipeline with a mock transcription engine
- **THEN** audio is captured → VAD filters → segments passed to engine → text output returned
- **AND** the pipeline completes without errors

---

### Requirement: Frontend-Backend Contract Tests

The system SHALL have tests that validate the Tauri command interface contract.

#### Scenario: Command signature validation

- **WHEN** the auto-generated `bindings.ts` is compared against the mock `__mocks__/bindings.ts`
- **THEN** all exported command names and type signatures match

#### Scenario: Settings type alignment

- **WHEN** `AppSettings` TypeScript type is compared to Rust `AppSettings` struct
- **THEN** all field names and types are compatible (validated via tauri-specta generation)

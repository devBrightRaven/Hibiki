## ADDED Requirements

### Requirement: Rust Settings Unit Tests

The system SHALL have unit tests for settings serialization, defaults, and persistence.

#### Scenario: Default settings roundtrip

- **WHEN** `get_default_settings()` is called
- **AND** the result is serialized to JSON and deserialized back
- **THEN** the output equals the original

#### Scenario: Settings migration

- **WHEN** a settings file from a previous schema version is loaded
- **THEN** missing fields are populated with defaults
- **AND** no data is lost

#### Scenario: Invalid settings handling

- **WHEN** a settings file contains invalid JSON or unknown fields
- **THEN** the system falls back to defaults without crashing

---

### Requirement: Rust Model Manager Unit Tests

The system SHALL have unit tests for model catalog, metadata, and file operations.

#### Scenario: Model catalog completeness

- **WHEN** the model catalog is loaded
- **THEN** every model has: name, file size, engine type, and language list

#### Scenario: Model path resolution

- **WHEN** a model ID is provided
- **THEN** the correct file path within `{AppDataDir}/models/` is returned

#### Scenario: Custom model discovery

- **WHEN** a `.bin` file exists in the models directory
- **THEN** it appears in the model list as a custom model

---

### Requirement: Rust History Manager Unit Tests

The system SHALL have unit tests for history database operations.

#### Scenario: Insert and retrieve entry

- **WHEN** a transcription entry is inserted
- **AND** then retrieved by ID
- **THEN** all fields match the original (text, timestamp, prompt ID, saved flag)

#### Scenario: History limit enforcement

- **WHEN** entries exceed the configured limit
- **THEN** the oldest non-saved entries are removed
- **AND** saved entries are preserved

#### Scenario: Entry deletion

- **WHEN** an entry is deleted
- **THEN** it is no longer retrievable
- **AND** its associated recording file path is returned for cleanup

---

### Requirement: Rust Audio Toolkit Unit Tests

The system SHALL have unit tests for audio processing utilities.

#### Scenario: Audio resampling

- **WHEN** a 44100Hz audio buffer is resampled to 16000Hz
- **THEN** the output has the correct number of samples
- **AND** signal integrity is maintained (no clipping or silence)

#### Scenario: Word correction matching

- **WHEN** a transcribed word is compared against a custom word list
- **THEN** matches above the configured threshold are returned
- **AND** Levenshtein distance and Soundex are both considered

---

### Requirement: Frontend Store Unit Tests

The system SHALL have unit tests for Zustand store actions and selectors.

#### Scenario: Settings store initialization

- **WHEN** the settings store initializes
- **THEN** it calls `getAppSettings` from the mock bindings
- **AND** populates the store with the returned settings

#### Scenario: Settings store update

- **WHEN** a setting is updated via a store action
- **THEN** the corresponding Tauri command is called with the new value
- **AND** the store state reflects the change

#### Scenario: Model store state transitions

- **WHEN** a model download starts
- **THEN** the model store tracks: downloading → extracting → ready
- **AND** progress updates are reflected in state

---

### Requirement: Frontend Component Unit Tests

The system SHALL have unit tests for key UI components.

#### Scenario: Settings component rendering

- **WHEN** a settings section component is rendered with mock store data
- **THEN** it displays the current setting values
- **AND** interactive elements (toggles, selects) are accessible

#### Scenario: Model selector rendering

- **WHEN** the model selector component is rendered
- **THEN** downloaded models show as selectable
- **AND** undownloaded models show a download action

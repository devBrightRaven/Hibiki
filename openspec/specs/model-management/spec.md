# Model Management

## Purpose

Manage transcription model lifecycle including discovery, download, storage, and deletion, providing users with a curated catalog of models and support for custom models.

## Requirements

### Requirement: Model Catalog

The system SHALL provide a built-in catalog of transcription models with metadata.

#### Scenario: List available models

- **WHEN** the user opens the Models settings section
- **THEN** the system displays all available models with name, size, language support, and engine type

#### Scenario: Model metadata

- **WHEN** a model is listed
- **THEN** it includes: display name, file size, supported languages, engine type, and a brief description

#### Scenario: Recommended models

- **WHEN** the user is in onboarding or model selection
- **THEN** recommended models are highlighted based on platform and use case

---

### Requirement: Model Download

The system SHALL download models from the official distribution server.

#### Scenario: Start download

- **WHEN** the user selects a model to download
- **THEN** the system streams the model file from blob.handy.computer
- **AND** reports download progress in real-time

#### Scenario: Download progress

- **WHEN** a download is in progress
- **THEN** the UI shows percentage complete and download speed

#### Scenario: Compressed models

- **WHEN** a downloaded model is compressed (e.g., tar.gz for Parakeet)
- **THEN** the system automatically extracts it after download

#### Scenario: Download failure

- **WHEN** a download fails (network error, disk full)
- **THEN** the user is informed of the failure reason
- **AND** partial files are cleaned up

---

### Requirement: Model Storage

The system SHALL store downloaded models in the application data directory.

#### Scenario: Storage location

- **WHEN** a model is downloaded
- **THEN** it is stored in `{AppDataDir}/models/`

#### Scenario: Model activation

- **WHEN** the user selects a downloaded model as active
- **THEN** subsequent transcriptions use that model

---

### Requirement: Model Deletion

The system SHALL allow users to delete downloaded models to free disk space.

#### Scenario: Delete model

- **WHEN** the user deletes a model
- **THEN** the model file is removed from disk
- **AND** the model appears as downloadable again in the catalog

#### Scenario: Delete active model

- **WHEN** the user deletes the currently active model
- **THEN** the active model selection is cleared

---

### Requirement: Custom Model Support

The system SHALL auto-discover user-provided GGML Whisper models.

#### Scenario: Auto-discovery

- **WHEN** the user places a `.bin` GGML Whisper model file in the models directory
- **THEN** the system detects and lists it as an available custom model

#### Scenario: Custom model usage

- **WHEN** the user selects a custom model
- **THEN** it is loaded and used via the Whisper engine

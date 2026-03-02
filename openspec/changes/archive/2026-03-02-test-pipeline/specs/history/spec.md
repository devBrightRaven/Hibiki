## MODIFIED Requirements

### Requirement: Transcription History Storage

The system SHALL persist transcription history in a local SQLite database.

#### Scenario: Save transcription

- **WHEN** a transcription completes
- **THEN** the system stores: ID, filename, timestamp, original text, post-processed text (if applicable), prompt ID, and saved flag

#### Scenario: Save transcription metadata

- **WHEN** a transcription completes
- **THEN** the system additionally stores: word count, audio duration in seconds, and detected language
- **AND** these fields are nullable for backward compatibility with existing entries

#### Scenario: History limit

- **WHEN** the number of history entries exceeds the configured limit (default 100)
- **THEN** the oldest non-saved entries are removed

#### Scenario: Database location

- **WHEN** the application initializes
- **THEN** the history database is at `{AppDataDir}/history.db`

#### Scenario: Schema migrations

- **WHEN** the application updates
- **THEN** database schema migrations are applied automatically

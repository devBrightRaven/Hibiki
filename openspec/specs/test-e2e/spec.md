## ADDED Requirements

### Requirement: Onboarding E2E Tests

The system SHALL have Playwright tests covering the first-launch onboarding flow.

#### Scenario: Onboarding renders

- **WHEN** the app launches for the first time (no stored settings)
- **THEN** the onboarding screen is displayed
- **AND** model selection options are visible

#### Scenario: Model selection step

- **WHEN** the user selects a model during onboarding
- **THEN** the model is marked as selected
- **AND** the onboarding advances to the next step

---

### Requirement: Settings Navigation E2E Tests

The system SHALL have Playwright tests covering settings UI navigation.

#### Scenario: Sidebar navigation

- **WHEN** the user clicks a sidebar section (General, Models, Sound, Advanced, etc.)
- **THEN** the corresponding settings panel is displayed

#### Scenario: Settings persistence

- **WHEN** the user changes a toggle setting
- **AND** refreshes the page
- **THEN** the setting retains the changed value

---

### Requirement: History E2E Tests

The system SHALL have Playwright tests covering the history view.

#### Scenario: History list rendering

- **WHEN** the user navigates to the History section
- **THEN** transcription entries are displayed with timestamps

#### Scenario: Copy from history

- **WHEN** the user clicks copy on a history entry
- **THEN** the transcription text is placed on the clipboard

---

### Requirement: Model Management E2E Tests

The system SHALL have Playwright tests covering model management interactions.

#### Scenario: Model list display

- **WHEN** the user navigates to the Models section
- **THEN** available models are listed with download status and metadata

#### Scenario: Model selection

- **WHEN** a model is already downloaded
- **AND** the user selects it
- **THEN** it becomes the active model

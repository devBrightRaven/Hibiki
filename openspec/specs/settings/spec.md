# Settings

## Purpose

Provide a comprehensive, persistent settings system that covers all application behavior, with a user-friendly interface organized by domain.

## Requirements

### Requirement: Settings Persistence

The system SHALL persist all user settings across application restarts.

#### Scenario: Auto-save

- **WHEN** the user changes a setting
- **THEN** it is immediately persisted via Tauri store plugin

#### Scenario: Platform-specific storage

- **WHEN** settings are saved
- **THEN** they are stored at the platform-specific path:
  - macOS: `~/Library/Application Support/com.pais.handy/`
  - Windows: `C:\Users\{user}\AppData\Roaming\com.pais.handy\`
  - Linux: `~/.config/com.pais.handy/`

#### Scenario: Default values

- **WHEN** a setting has no saved value
- **THEN** the system uses the documented default

---

### Requirement: Settings UI

The system SHALL organize settings into logical sections with a sidebar navigation.

#### Scenario: Settings sections

- **WHEN** the user opens the application
- **THEN** the following sections are available:
  - General (shortcuts, language, push-to-talk)
  - Models (download, select, delete)
  - Sound (microphone, feedback, volume, output device)
  - Advanced (paste method, overlay, text behavior, keyboard implementation)
  - Post-Processing (provider, model, prompts)
  - History (transcription list, recordings)
  - Debug (log level, paths, advanced options)
  - About (version, links, donations)

---

### Requirement: Onboarding

The system SHALL guide first-time users through essential setup.

#### Scenario: First launch

- **WHEN** the user launches the application for the first time
- **THEN** the onboarding flow starts

#### Scenario: Model selection step

- **WHEN** the onboarding reaches model selection
- **THEN** recommended models are shown for download

#### Scenario: Permission grants (macOS)

- **WHEN** onboarding runs on macOS
- **THEN** the user is prompted for microphone and accessibility permissions

#### Scenario: Initialization step

- **WHEN** permissions are granted
- **THEN** Enigo (input simulation) and global shortcuts are initialized

---

### Requirement: Type-Safe Bindings

The system SHALL generate TypeScript bindings from Rust types for frontend type safety.

#### Scenario: Auto-generated bindings

- **WHEN** Rust command or settings types change
- **THEN** tauri-specta generates updated TypeScript bindings in `src/bindings.ts`

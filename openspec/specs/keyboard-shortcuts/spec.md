# Keyboard Shortcuts

## Purpose

Provide global keyboard shortcuts for controlling transcription from any application, supporting multiple input modes and platforms.

## Requirements

### Requirement: Global Shortcut Registration

The system SHALL register global keyboard shortcuts that work across all applications.

#### Scenario: Transcribe shortcut

- **WHEN** the user presses the configured transcription shortcut
- **THEN** the system toggles recording on/off (toggle mode) or starts/stops recording (push-to-talk mode)

#### Scenario: Cancel shortcut

- **WHEN** the user presses the configured cancel shortcut
- **THEN** the current transcription operation is cancelled

#### Scenario: Transcribe with post-process shortcut

- **WHEN** the user presses the configured post-process shortcut
- **THEN** recording toggles with post-processing always applied

---

### Requirement: Input Modes

The system SHALL support different recording trigger modes.

#### Scenario: Toggle mode (default)

- **WHEN** push-to-talk is disabled
- **THEN** pressing the shortcut starts recording, pressing again stops

#### Scenario: Push-to-talk mode

- **WHEN** push-to-talk is enabled
- **THEN** holding the shortcut records, releasing stops and transcribes

---

### Requirement: Shortcut Customization

The system SHALL allow users to customize all keyboard shortcuts.

#### Scenario: Rebind shortcut

- **WHEN** the user configures a new key combination for a shortcut
- **THEN** the system validates for conflicts and applies the new binding

#### Scenario: Conflict detection

- **WHEN** a new shortcut conflicts with an existing one
- **THEN** the user is warned before applying

---

### Requirement: Debug Shortcut

The system SHALL provide a debug mode toggle shortcut.

#### Scenario: Toggle debug mode

- **WHEN** the user presses Cmd+Shift+D (macOS) or Ctrl+Shift+D (Windows/Linux)
- **THEN** debug mode is toggled, showing advanced settings and logging

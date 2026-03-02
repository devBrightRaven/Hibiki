# CLI

## Purpose

Enable external control of the application through command-line flags, supporting integration with scripts, window managers, and automation tools.

## Requirements

### Requirement: Remote Control Flags

The system SHALL support CLI flags that send commands to a running instance.

#### Scenario: Toggle transcription

- **WHEN** `handy --toggle-transcription` is run
- **THEN** the running instance toggles recording on/off

#### Scenario: Toggle with post-processing

- **WHEN** `handy --toggle-post-process` is run
- **THEN** the running instance toggles recording with post-processing

#### Scenario: Cancel operation

- **WHEN** `handy --cancel` is run
- **THEN** the running instance cancels the current operation

---

### Requirement: Launch Flags

The system SHALL support CLI flags that control startup behavior.

#### Scenario: Start hidden

- **WHEN** `handy --start-hidden` is run
- **THEN** the application launches without showing the main window

#### Scenario: No tray

- **WHEN** `handy --no-tray` is run
- **THEN** the application launches without the system tray icon
- **AND** closing the window quits the application

#### Scenario: Debug mode

- **WHEN** `handy --debug` is run
- **THEN** verbose (Trace level) logging is enabled

---

### Requirement: Unix Signal Handling

The system SHALL respond to Unix signals on Linux for window-manager integration.

#### Scenario: SIGUSR2

- **WHEN** the process receives SIGUSR2
- **THEN** transcription is toggled

#### Scenario: SIGUSR1

- **WHEN** the process receives SIGUSR1
- **THEN** transcription with post-processing is toggled

---

### Requirement: CLI Design Principles

CLI flags SHALL be runtime-only overrides.

#### Scenario: No persistent side effects

- **WHEN** any CLI flag is used
- **THEN** it does NOT modify persisted settings
- **AND** the effect lasts only for the current session

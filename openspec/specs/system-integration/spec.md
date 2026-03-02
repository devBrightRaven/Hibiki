# System Integration

## Purpose

Integrate with the host operating system for tray icon, autostart, updates, single-instance enforcement, and platform permissions.

## Requirements

### Requirement: System Tray

The system SHALL provide a system tray icon with quick-access actions.

#### Scenario: Tray menu

- **WHEN** the user interacts with the tray icon
- **THEN** the following actions are available:
  - Open settings
  - Check for updates
  - Copy last transcript
  - Unload model
  - Cancel operation
  - Quit application

#### Scenario: Tray icon themes

- **WHEN** the system theme changes (dark/light)
- **THEN** the tray icon updates to match
- **AND** Linux additionally supports a colored variant

#### Scenario: Hide tray icon

- **WHEN** the user disables "show tray icon"
- **THEN** the tray icon is hidden
- **AND** closing the window quits the application

---

### Requirement: Autostart

The system SHALL optionally launch at system login.

#### Scenario: Enable autostart

- **WHEN** the user enables autostart in settings
- **THEN** the application registers to start on login

#### Scenario: Start hidden

- **WHEN** start-hidden is enabled (via settings or --start-hidden flag)
- **THEN** the application launches to the tray without showing the main window

---

### Requirement: Application Updates

The system SHALL check for and install updates from GitHub releases.

#### Scenario: Automatic update check

- **WHEN** update checks are enabled and the application starts
- **THEN** the system checks GitHub for newer versions

#### Scenario: Update installation

- **WHEN** an update is available
- **THEN** the user is notified and can install with one click

---

### Requirement: Single Instance

The system SHALL enforce single-instance execution.

#### Scenario: Second instance launched

- **WHEN** a second instance is launched (e.g., with CLI flags)
- **THEN** the CLI arguments are forwarded to the running instance via tauri_plugin_single_instance
- **AND** the second instance exits

---

### Requirement: Platform Permissions (macOS)

The system SHALL handle macOS-specific permission requirements.

#### Scenario: Microphone permission

- **WHEN** the user first attempts to record on macOS
- **THEN** the system requests microphone access

#### Scenario: Accessibility permission

- **WHEN** the user needs text input simulation on macOS
- **THEN** the system prompts for accessibility access
- **AND** checks permission status before initialization

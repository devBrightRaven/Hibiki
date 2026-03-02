# Recording Overlay

## Purpose

Provide a visual indicator of recording state that floats above all other windows, giving users real-time feedback on transcription progress.

## Requirements

### Requirement: Overlay Display

The system SHALL show a floating overlay during transcription operations.

#### Scenario: Overlay positions

- **WHEN** the user configures overlay position
- **THEN** available positions are: Top, Bottom, or None (hidden)

#### Scenario: Overlay states

- **WHEN** the transcription state changes
- **THEN** the overlay reflects: Idle, Recording, Transcribing, or Processing

#### Scenario: Overlay disabled

- **WHEN** overlay position is set to None
- **THEN** no overlay window is shown

---

### Requirement: Platform-Specific Overlay

The system SHALL use platform-appropriate overlay implementations.

#### Scenario: macOS overlay

- **WHEN** running on macOS
- **THEN** the overlay uses NSPanel (floating, always-on-top, non-key window)
- **AND** does not steal focus from the active application

#### Scenario: Windows overlay

- **WHEN** running on Windows
- **THEN** the overlay uses a separate WebviewWindow with always-on-top

#### Scenario: Linux overlay

- **WHEN** running on Linux
- **THEN** the overlay uses GTK Layer Shell for Wayland compositors
- **AND** is disabled by default due to potential focus-stealing issues

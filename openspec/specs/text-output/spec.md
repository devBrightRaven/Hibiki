# Text Output

## Purpose

Deliver transcription results to the user's active application through configurable paste methods, clipboard handling, and text adjustments.

## Requirements

### Requirement: Paste Methods

The system SHALL support multiple methods for inserting transcribed text into applications.

#### Scenario: Clipboard + Ctrl/Cmd+V (default)

- **WHEN** the paste method is CtrlV
- **THEN** the system saves the current clipboard, writes transcription to clipboard, simulates Ctrl+V (Cmd+V on macOS), and restores the original clipboard

#### Scenario: Direct input simulation

- **WHEN** the paste method is DirectInput
- **THEN** the system simulates typing via the OS input system

#### Scenario: Shift+Insert

- **WHEN** the paste method is ShiftInsert
- **THEN** the system simulates Shift+Insert for pasting

#### Scenario: Ctrl+Shift+V

- **WHEN** the paste method is CtrlShiftV
- **THEN** the system simulates Ctrl+Shift+V (paste without formatting)

#### Scenario: External script

- **WHEN** the paste method is ExternalScript
- **THEN** the system executes a user-configured external script with the transcription as input

#### Scenario: None (clipboard only)

- **WHEN** the paste method is None
- **THEN** the system updates the clipboard and history but does not simulate any paste action

---

### Requirement: Clipboard Handling

The system SHALL manage clipboard state around paste operations.

#### Scenario: Don't modify clipboard

- **WHEN** clipboard handling is set to DontModify
- **THEN** the original clipboard contents are restored after pasting

#### Scenario: Copy to clipboard

- **WHEN** clipboard handling is set to CopyToClipboard
- **THEN** the transcription remains in the clipboard after pasting

---

### Requirement: Auto-Submit

The system SHALL optionally submit forms after pasting.

#### Scenario: Auto-submit enabled

- **WHEN** auto-submit is enabled
- **THEN** the system sends the configured submit key (Enter, Ctrl+Enter, Cmd+Enter, Super+Enter) after pasting

#### Scenario: Auto-submit disabled

- **WHEN** auto-submit is disabled
- **THEN** the system only pastes, without pressing any submit key

---

### Requirement: Text Adjustments

The system SHALL apply configurable text transformations before output.

#### Scenario: Trailing space

- **WHEN** "append trailing space" is enabled
- **THEN** a space character is appended after the transcription text

---

### Requirement: Platform-Specific Input

The system SHALL use platform-appropriate input methods.

#### Scenario: macOS/Windows

- **WHEN** running on macOS or Windows
- **THEN** keyboard simulation uses Enigo or HandyKeys (macOS native)

#### Scenario: Linux X11

- **WHEN** running on Linux with X11
- **THEN** xdotool is preferred for key simulation

#### Scenario: Linux Wayland

- **WHEN** running on Linux with Wayland
- **THEN** the system selects from: wtype, dotool, ydotool, or Enigo fallback
- **AND** the user can configure the preferred typing tool in settings

---

### Requirement: Keyboard Implementation

The system SHALL support multiple keyboard hook implementations.

#### Scenario: Tauri implementation

- **WHEN** keyboard implementation is set to Tauri
- **THEN** the system uses tauri-plugin-global-shortcut

#### Scenario: HandyKeys implementation (macOS)

- **WHEN** keyboard implementation is set to HandyKeys on macOS
- **THEN** the system uses the native HandyKeys implementation

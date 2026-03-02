# Audio Recording

## Purpose

Capture audio from the user's microphone with voice activity detection, providing real-time feedback and producing clean audio segments ready for transcription.

## Requirements

### Requirement: Device Selection

The system SHALL allow users to select an audio input device from all available system microphones.

#### Scenario: List available devices

- **WHEN** the user opens sound settings
- **THEN** the system lists all available audio input devices
- **AND** highlights the currently selected device

#### Scenario: Change input device

- **WHEN** the user selects a different microphone
- **THEN** the system switches to the new device for subsequent recordings

#### Scenario: Clamshell microphone (macOS)

- **WHEN** the user is on macOS with a laptop lid closed and external display connected
- **THEN** the system uses the configured clamshell microphone if set

#### Scenario: Default device

- **WHEN** no microphone is explicitly selected
- **THEN** the system uses the system default input device

---

### Requirement: Audio Capture

The system SHALL record audio in real-time from the selected input device.

#### Scenario: Start recording

- **WHEN** the user triggers a transcription shortcut
- **THEN** the system begins capturing audio from the selected microphone
- **AND** provides audio level feedback to the UI

#### Scenario: Stop recording

- **WHEN** the user releases the shortcut (push-to-talk) or toggles off (toggle mode)
- **THEN** the system stops capturing audio
- **AND** passes the recorded audio to the transcription pipeline

#### Scenario: Audio resampling

- **WHEN** the microphone sample rate differs from the transcription model's expected rate
- **THEN** the system resamples the audio to match (via Rubato)

---

### Requirement: Voice Activity Detection

The system SHALL filter audio through Silero VAD v4 to remove silence and non-speech segments.

#### Scenario: Silence filtering

- **WHEN** the user records audio containing silence gaps
- **THEN** the VAD removes silent segments before transcription
- **AND** reduces overall processing time

#### Scenario: VAD model loading

- **WHEN** the application starts
- **THEN** the Silero VAD v4 ONNX model is available from bundled resources

---

### Requirement: Audio Feedback

The system SHALL play audio cues to indicate recording state changes.

#### Scenario: Recording start sound

- **WHEN** audio recording begins
- **THEN** the system plays a start sound from the selected sound theme

#### Scenario: Recording stop sound

- **WHEN** audio recording ends
- **THEN** the system plays a stop sound from the selected sound theme

#### Scenario: Sound themes

- **WHEN** the user selects a sound theme (Marimba, Pop, or Custom)
- **THEN** start/stop sounds use that theme

#### Scenario: Disable audio feedback

- **WHEN** the user disables audio feedback in settings
- **THEN** no start/stop sounds are played

#### Scenario: Volume control

- **WHEN** the user adjusts audio feedback volume (0-1 range)
- **THEN** subsequent feedback sounds play at the configured volume

#### Scenario: Output device selection

- **WHEN** the user selects a specific audio output device
- **THEN** feedback sounds play through that device

---

### Requirement: Always-On Microphone

The system SHALL optionally keep the microphone active between recordings for faster startup.

#### Scenario: Enable always-on

- **WHEN** the user enables "always on microphone"
- **THEN** the microphone stream stays open between transcriptions
- **AND** subsequent recordings start with lower latency

#### Scenario: Mute while recording

- **WHEN** the user enables "mute while recording"
- **THEN** system audio output is muted during active recording

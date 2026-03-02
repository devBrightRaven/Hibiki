# Transcription

## Purpose

Convert recorded audio to text using local speech-to-text models, supporting multiple engines and languages without sending data to the cloud.

## Requirements

### Requirement: Multi-Engine Support

The system SHALL support multiple transcription engines, each with distinct capabilities.

#### Scenario: Whisper engine

- **WHEN** the user selects a Whisper model (Small, Medium, Turbo, Large)
- **THEN** the system uses whisper-rs/ggml for transcription
- **AND** supports multi-language transcription
- **AND** supports optional English translation mode

#### Scenario: Parakeet engine

- **WHEN** the user selects a Parakeet model (V2, V3)
- **THEN** the system uses the CPU-optimized Parakeet engine
- **AND** Parakeet V3 supports automatic language detection

#### Scenario: Moonshine engine

- **WHEN** the user selects a Moonshine model (Base, V2 Tiny, V2 Small, V2 Medium)
- **THEN** the system uses the ultra-fast Moonshine engine
- **AND** supports English only

#### Scenario: SenseVoice engine

- **WHEN** the user selects the SenseVoice model
- **THEN** the system supports Chinese, English, Japanese, Korean, and Cantonese

#### Scenario: Breeze ASR engine

- **WHEN** the user selects the Breeze ASR model
- **THEN** the system supports Taiwanese Mandarin with code-switching

#### Scenario: Custom GGML models

- **WHEN** the user places a .bin GGML Whisper model in the models directory
- **THEN** the system auto-discovers and offers it as a selectable model

---

### Requirement: GPU Acceleration

The system SHALL use hardware acceleration when available.

#### Scenario: macOS Metal

- **WHEN** running on macOS with Apple Silicon or compatible GPU
- **THEN** Whisper models use Metal acceleration

#### Scenario: Windows/Linux Vulkan

- **WHEN** running on Windows or Linux with a Vulkan-capable GPU
- **THEN** Whisper models use Vulkan acceleration

#### Scenario: CPU fallback

- **WHEN** no GPU acceleration is available
- **THEN** the system falls back to CPU-only transcription

---

### Requirement: Language Selection

The system SHALL allow users to select a transcription language or use automatic detection.

#### Scenario: Auto-detect language

- **WHEN** language is set to "auto"
- **THEN** the model detects the spoken language automatically

#### Scenario: Specific language

- **WHEN** the user selects a specific language
- **THEN** the model constrains detection to that language

#### Scenario: English translation

- **WHEN** the user enables "translate to English" with a Whisper model
- **THEN** non-English speech is translated to English during transcription

---

### Requirement: Model Loading and Unloading

The system SHALL manage model lifecycle for memory efficiency.

#### Scenario: Lazy loading

- **WHEN** the user triggers their first transcription
- **THEN** the selected model loads into memory

#### Scenario: Auto-unload

- **WHEN** the model unload timeout is set (Immediately, 2-60 minutes)
- **AND** no transcription occurs within the timeout
- **THEN** the model is unloaded from memory

#### Scenario: Never unload

- **WHEN** the model unload timeout is set to "Never"
- **THEN** the model stays loaded until application exit

#### Scenario: Manual unload

- **WHEN** the user triggers "unload model" from the tray menu
- **THEN** the model is immediately unloaded from memory

---

### Requirement: Transcription Pipeline

The system SHALL process audio through a pipeline: audio capture → VAD → engine inference → text output.

#### Scenario: Successful transcription

- **WHEN** the user completes a recording
- **THEN** audio passes through VAD, then to the selected engine
- **AND** resulting text is emitted for output handling

#### Scenario: Empty audio

- **WHEN** the recorded audio contains no detected speech (VAD filters all)
- **THEN** no transcription is produced
- **AND** the user is not shown an error

---

### Requirement: Word Correction

The system SHALL apply custom word corrections to transcription output.

#### Scenario: Custom word list

- **WHEN** the user configures custom words in settings
- **THEN** transcription output is checked against the word list using Levenshtein distance and Soundex matching

#### Scenario: Correction threshold

- **WHEN** a transcribed word matches a custom word above the configured threshold (0-1)
- **THEN** the transcribed word is replaced with the custom word

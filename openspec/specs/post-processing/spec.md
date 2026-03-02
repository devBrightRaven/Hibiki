# Post-Processing

## Purpose

Optionally refine transcription output using AI language models for grammar correction, clarity, formatting, or custom transformations.

## Requirements

### Requirement: LLM Provider Configuration

The system SHALL support configurable LLM providers for post-processing.

#### Scenario: OpenAI-compatible provider

- **WHEN** the user configures an OpenAI-compatible API provider
- **THEN** the system sends transcription text to that endpoint for refinement
- **AND** supports custom base URLs for self-hosted or alternative APIs

#### Scenario: Apple Intelligence provider

- **WHEN** the user selects Apple Intelligence on macOS Tahoe+ with Apple Silicon
- **THEN** the system uses on-device Apple Intelligence for refinement
- **AND** no data leaves the device

#### Scenario: Multiple providers

- **WHEN** the user configures multiple providers
- **THEN** each provider has independent API key, base URL, and model settings

#### Scenario: Provider model selection

- **WHEN** the user selects a provider
- **THEN** they can configure which LLM model to use for that provider

---

### Requirement: Prompt System

The system SHALL support customizable prompts for controlling refinement behavior.

#### Scenario: Built-in prompts

- **WHEN** post-processing is enabled
- **THEN** pre-built prompt templates are available (Grammar, Clarity, Format, etc.)

#### Scenario: Custom prompts

- **WHEN** the user creates a custom prompt
- **THEN** it uses a `${output}` placeholder for the transcription text
- **AND** is available alongside built-in prompts

#### Scenario: Prompt selection

- **WHEN** the user selects a prompt
- **THEN** all subsequent post-processing uses that prompt

#### Scenario: Structured output

- **WHEN** a prompt specifies a JSON schema for structured output
- **THEN** the LLM response conforms to that schema

---

### Requirement: Post-Processing Triggers

The system SHALL support multiple ways to trigger post-processing.

#### Scenario: Always-on post-processing

- **WHEN** the user uses the "transcribe with post-process" shortcut
- **THEN** transcription is always followed by post-processing

#### Scenario: Opt-in post-processing

- **WHEN** the user uses the normal transcription shortcut with post-processing enabled
- **THEN** transcription output goes through the selected prompt

#### Scenario: Post-processing disabled

- **WHEN** post-processing is disabled in settings
- **THEN** transcription output is used as-is

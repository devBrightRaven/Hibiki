# Internationalization (i18n)

## Purpose

Support multiple UI languages, enabling users worldwide to use the application in their preferred language.

## Requirements

### Requirement: Multi-Language UI

The system SHALL display all user-facing strings in the selected language.

#### Scenario: Supported languages

- **WHEN** the user selects a language
- **THEN** the UI is available in: English, Spanish, French, German, Italian, Portuguese, Polish, Russian, Ukrainian, Czech, Arabic, Turkish, Japanese, Korean, Vietnamese, Chinese Simplified, and Chinese Traditional

#### Scenario: Default language

- **WHEN** no language is selected
- **THEN** English is used as the default

#### Scenario: RTL support

- **WHEN** the selected language is Arabic
- **THEN** the UI layout adjusts for right-to-left text

---

### Requirement: Translation Infrastructure

The system SHALL enforce translation coverage for all UI strings.

#### Scenario: i18next integration

- **WHEN** a component renders text
- **THEN** it uses `t('key.path')` from react-i18next

#### Scenario: ESLint enforcement

- **WHEN** a developer adds hardcoded strings in JSX
- **THEN** ESLint flags the violation

#### Scenario: Translation files

- **WHEN** a new UI string is needed
- **THEN** it is added to `src/i18n/locales/en/translation.json` first
- **AND** other locale files are updated separately

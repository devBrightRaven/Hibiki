# Hibiki Modular Pipeline — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refactor Hibiki from a monolithic Handy fork into a modular voice pipeline with swappable STT engines, configurable audio retention, and a rule-based refinement module.

**Architecture:** Extract an `SttEngine` trait from the existing `TranscriptionManager`, route engine selection through settings, and add a post-transcription refinement module (filler removal + punctuation) as a separate pipeline stage.

**Tech Stack:** Rust / Tauri 2.x / React / TypeScript / SQLite / whisper-rs / transcribe-rs

---

## Phase 1: SttEngine Trait Extraction

### Task 1: Define the SttEngine trait

**Files:**
- Create: `src-tauri/src/engine/mod.rs`
- Create: `src-tauri/src/engine/types.rs`
- Modify: `src-tauri/src/lib.rs` (add `mod engine;`)

**Step 1: Create the engine types module**

```rust
// src-tauri/src/engine/types.rs
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EngineId {
    Whisper,
    Parakeet,
    Moonshine,
    SenseVoice,
}

impl std::fmt::Display for EngineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineId::Whisper => write!(f, "whisper"),
            EngineId::Parakeet => write!(f, "parakeet"),
            EngineId::Moonshine => write!(f, "moonshine"),
            EngineId::SenseVoice => write!(f, "sense_voice"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TranscriptSegment {
    pub raw_text: String,
    pub language: Option<String>,
    pub engine_id: EngineId,
    pub duration_ms: u64,
}
```

**Step 2: Create the engine trait module**

```rust
// src-tauri/src/engine/mod.rs
pub mod types;

use anyhow::Result;
pub use types::{EngineId, TranscriptSegment};

/// Trait that all STT engines must implement.
/// This allows swapping engines at runtime without changing the pipeline.
pub trait SttEngine: Send + Sync {
    fn id(&self) -> EngineId;
    fn display_name(&self) -> &str;
    fn supported_languages(&self) -> &[&str];
    fn transcribe(&self, audio_samples: &[f32], sample_rate: u32) -> Result<TranscriptSegment>;
}
```

**Step 3: Add `mod engine;` to lib.rs**

In `src-tauri/src/lib.rs`, add `mod engine;` alongside the existing module declarations.

**Step 4: Verify it compiles**

Run: `cd src-tauri && cargo check`
Expected: compiles with no errors (trait is defined but not yet used)

**Step 5: Commit**

```bash
git add src-tauri/src/engine/
git commit -m "feat: define SttEngine trait and TranscriptSegment types"
```

---

### Task 2: Wrap existing Whisper engine behind SttEngine trait

**Files:**
- Create: `src-tauri/src/engine/whisper_engine.rs`
- Modify: `src-tauri/src/engine/mod.rs` (add `pub mod whisper_engine;`)

**Step 1: Create whisper_engine.rs**

Wrap the existing `WhisperEngine` from `transcribe_rs` behind our `SttEngine` trait. Extract the engine creation and transcription logic from `TranscriptionManager::transcribe_audio()` (currently in `managers/transcription.rs:~200-350`).

The wrapper should:
- Accept model path and inference params at construction
- Implement `SttEngine::transcribe()` by delegating to `transcribe_rs::WhisperEngine`
- Return a `TranscriptSegment` with the raw text

Do NOT remove the existing code in `TranscriptionManager` yet — this task only creates the wrapper. Migration happens in Task 5.

**Step 2: Add to engine/mod.rs**

```rust
#[cfg(feature = "transcribe-rs")]
pub mod whisper_engine;
```

**Step 3: Verify it compiles**

Run: `cd src-tauri && cargo check`

**Step 4: Commit**

```bash
git commit -m "feat: wrap whisper behind SttEngine trait"
```

---

### Task 3: Wrap SenseVoice engine behind SttEngine trait

**Files:**
- Create: `src-tauri/src/engine/sensevoice_engine.rs`
- Modify: `src-tauri/src/engine/mod.rs`

Same pattern as Task 2 but for `SenseVoiceEngine` from `transcribe_rs`. Extract from `TranscriptionManager`'s SenseVoice handling code (~lines 400-500 in transcription.rs).

**Step 1: Create sensevoice_engine.rs**

**Step 2: Add to engine/mod.rs**

**Step 3: Verify it compiles**

**Step 4: Commit**

```bash
git commit -m "feat: wrap sensevoice behind SttEngine trait"
```

---

### Task 4: Wrap Parakeet engine behind SttEngine trait

**Files:**
- Create: `src-tauri/src/engine/parakeet_engine.rs`
- Modify: `src-tauri/src/engine/mod.rs`

Same pattern as Tasks 2-3.

**Step 1-4:** Same structure.

```bash
git commit -m "feat: wrap parakeet behind SttEngine trait"
```

---

### Task 5: Create EngineRouter that selects engine by settings

**Files:**
- Create: `src-tauri/src/engine/router.rs`
- Modify: `src-tauri/src/engine/mod.rs`

**Step 1: Create router.rs**

```rust
// src-tauri/src/engine/router.rs
use super::{EngineId, SttEngine};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

pub struct EngineRouter {
    engines: HashMap<EngineId, Arc<dyn SttEngine>>,
    default_engine: EngineId,
}

impl EngineRouter {
    pub fn new(default_engine: EngineId) -> Self {
        Self {
            engines: HashMap::new(),
            default_engine,
        }
    }

    pub fn register(&mut self, engine: Arc<dyn SttEngine>) {
        self.engines.insert(engine.id(), engine);
    }

    pub fn get(&self, id: &EngineId) -> Option<Arc<dyn SttEngine>> {
        self.engines.get(id).cloned()
    }

    pub fn get_default(&self) -> Option<Arc<dyn SttEngine>> {
        self.get(&self.default_engine)
    }

    pub fn available_engines(&self) -> Vec<EngineId> {
        self.engines.keys().cloned().collect()
    }
}
```

**Step 2: Verify it compiles**

**Step 3: Commit**

```bash
git commit -m "feat: add EngineRouter for runtime engine selection"
```

---

## Phase 2: Refinement Module

### Task 6: Create refinement module with rule-based filler removal

**Files:**
- Create: `src-tauri/src/refinement/mod.rs`
- Create: `src-tauri/src/refinement/rules.rs`
- Create: `src-tauri/src/refinement/types.rs`
- Modify: `src-tauri/src/lib.rs` (add `mod refinement;`)

**Step 1: Define refinement types**

```rust
// src-tauri/src/refinement/types.rs
#[derive(Debug, Clone)]
pub struct RefinedTranscript {
    pub original_text: String,
    pub refined_text: String,
    pub refinement_applied: bool,
}
```

**Step 2: Create rule-based refiner**

```rust
// src-tauri/src/refinement/rules.rs
use super::types::RefinedTranscript;

/// Language-specific filler words to remove
struct FillerRules {
    /// e.g. ["um", "uh", "like", "you know"] for English
    /// e.g. ["那個", "就是", "然後", "嗯", "啊", "對"] for zh-TW
    fillers: Vec<String>,
}

pub struct RuleBasedRefiner {
    rules_by_language: std::collections::HashMap<String, FillerRules>,
}

impl RuleBasedRefiner {
    pub fn new() -> Self {
        let mut rules_by_language = std::collections::HashMap::new();

        // English fillers
        rules_by_language.insert("en".to_string(), FillerRules {
            fillers: vec![
                "um", "uh", "uhh", "umm", "hmm", "like,", "you know,",
                "I mean,", "so,", "basically,", "actually,", "literally,"
            ].into_iter().map(String::from).collect(),
        });

        // Traditional Chinese / Mandarin fillers
        rules_by_language.insert("zh".to_string(), FillerRules {
            fillers: vec![
                "那個", "就是", "然後", "嗯", "啊", "對", "就是說",
                "怎麼說", "反正", "所以說",
            ].into_iter().map(String::from).collect(),
        });

        Self { rules_by_language }
    }

    pub fn refine(&self, text: &str, language: &str) -> RefinedTranscript {
        let lang_key = if language.starts_with("zh") { "zh" } else { language };

        let refined = match self.rules_by_language.get(lang_key) {
            Some(rules) => {
                let mut result = text.to_string();
                for filler in &rules.fillers {
                    // Case-insensitive replacement, preserving sentence structure
                    result = result.replace(filler, "");
                }
                // Clean up double spaces left by removal
                while result.contains("  ") {
                    result = result.replace("  ", " ");
                }
                result.trim().to_string()
            }
            None => text.to_string(),
        };

        let refinement_applied = refined != text;

        RefinedTranscript {
            original_text: text.to_string(),
            refined_text: refined,
            refinement_applied,
        }
    }
}
```

**Step 3: Create mod.rs**

```rust
// src-tauri/src/refinement/mod.rs
pub mod rules;
pub mod types;

pub use rules::RuleBasedRefiner;
pub use types::RefinedTranscript;
```

**Step 4: Add `mod refinement;` to lib.rs**

**Step 5: Write unit tests**

Create: `src-tauri/src/refinement/rules.rs` — add `#[cfg(test)]` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_filler_removal() {
        let refiner = RuleBasedRefiner::new();
        let result = refiner.refine("um so I think uh we should do this", "en");
        assert!(result.refinement_applied);
        assert!(!result.refined_text.contains("um"));
        assert!(!result.refined_text.contains("uh"));
        assert!(result.refined_text.contains("I think"));
    }

    #[test]
    fn test_chinese_filler_removal() {
        let refiner = RuleBasedRefiner::new();
        let result = refiner.refine("嗯就是我覺得那個應該這樣做", "zh-TW");
        assert!(result.refinement_applied);
        assert!(!result.refined_text.contains("嗯"));
        assert!(!result.refined_text.contains("那個"));
        assert!(result.refined_text.contains("我覺得"));
    }

    #[test]
    fn test_no_refinement_needed() {
        let refiner = RuleBasedRefiner::new();
        let result = refiner.refine("This is a clean sentence.", "en");
        assert!(!result.refinement_applied);
        assert_eq!(result.refined_text, result.original_text);
    }

    #[test]
    fn test_unknown_language_passthrough() {
        let refiner = RuleBasedRefiner::new();
        let result = refiner.refine("um hello", "xx");
        assert!(!result.refinement_applied);
    }
}
```

**Step 6: Run tests**

Run: `cd src-tauri && cargo test refinement`
Expected: all 4 tests pass

**Step 7: Commit**

```bash
git commit -m "feat: add rule-based refinement module with filler removal"
```

---

### Task 7: Add refinement toggle to settings

**Files:**
- Modify: `src-tauri/src/settings.rs` (add `refinement_enabled` field)
- Modify: `src/stores/settingsStore.ts` (add frontend setting)

**Step 1: Add to AppSettings struct** (~line 354 in settings.rs)

```rust
#[serde(default = "default_refinement_enabled")]
pub refinement_enabled: bool,
```

Add default function:

```rust
fn default_refinement_enabled() -> bool {
    true // on by default per design doc
}
```

Add getter:

```rust
pub fn get_refinement_enabled(app: &AppHandle) -> bool {
    get_settings(app).refinement_enabled
}
```

**Step 2: Add to frontend settings store**

In `src/stores/settingsStore.ts`, add `refinementEnabled: boolean` to the settings type and default.

**Step 3: Verify it compiles**

Run: `cd src-tauri && cargo check`

**Step 4: Commit**

```bash
git commit -m "feat: add refinement_enabled toggle to settings"
```

---

## Phase 3: Pipeline Integration

### Task 8: Wire refinement into transcription output path

**Files:**
- Modify: `src-tauri/src/managers/transcription.rs`
- Modify: `src-tauri/src/managers/history.rs` (store both original and refined text)

**Step 1: Import and instantiate refiner in TranscriptionManager**

In the `transcribe_audio()` method, after getting the raw transcription text, apply refinement if enabled:

```rust
use crate::refinement::RuleBasedRefiner;
use crate::settings::get_refinement_enabled;

// After transcription result is obtained:
let final_text = if get_refinement_enabled(&self.app_handle) {
    let refiner = RuleBasedRefiner::new();
    let detected_lang = detected_language.as_deref().unwrap_or("en");
    let refined = refiner.refine(&transcription_text, detected_lang);
    refined.refined_text
} else {
    transcription_text.clone()
};
```

**Step 2: Store original text in history**

Add a DB migration in `managers/history.rs` to add `original_text` column:

```rust
M::up("ALTER TABLE transcription_history ADD COLUMN original_text TEXT;"),
```

When saving history, store original in `original_text` and refined (or original if refinement off) in `transcription_text`.

**Step 3: Verify it compiles**

**Step 4: Commit**

```bash
git commit -m "feat: wire refinement into transcription pipeline"
```

---

### Task 9: Add engine preference to settings UI

**Files:**
- Modify: `src-tauri/src/settings.rs` (add `preferred_engine` field)
- Modify: frontend settings component (add engine selector)

**Step 1: Add preferred_engine to settings**

```rust
#[serde(default = "default_preferred_engine")]
pub preferred_engine: String, // "auto", "whisper", "sense_voice", "parakeet"
```

Default: `"auto"` (uses model's native engine, same as current behavior).

When set to a specific engine, the router will prefer that engine if compatible with the selected model.

**Step 2: Add to frontend**

Add a segmented control (NOT dropdown — per CLAUDE.md rules) in the settings UI for engine selection: Auto | Whisper | SenseVoice | Parakeet.

**Step 3: Commit**

```bash
git commit -m "feat: add preferred engine setting with segmented control"
```

---

## Phase 4: Rebranding

### Task 10: Replace Handy references with Hibiki

**Files:**
- Modify: `package.json` (name: "handy-app" → "hibiki")
- Modify: `src-tauri/tauri.conf.json` (app name, identifiers)
- Modify: `src-tauri/Cargo.toml` (package name)
- Modify: `CLAUDE.md` (replace all "Handy" references)
- Modify: `README.md` (complete rewrite)

**Step 1: Update all identifiers**

Search and replace:
- `handy-app` → `hibiki`
- `Handy` → `Hibiki` (display name)
- `com.handy.app` or similar → `com.brightraven.hibiki`
- `blob.handy.computer` URLs — keep as-is for now (model downloads still from upstream CDN)

**Step 2: Update CLAUDE.md**

Rewrite to reflect Hibiki's architecture:
- Remove "Handy is a cross-platform desktop speech-to-text app"
- Replace with "Hibiki is the voice foundation layer for BR-OS"
- Add module descriptions for `engine/`, `refinement/`

**Step 3: Verify it builds**

Run: `cd src-tauri && cargo check && cd .. && bun run build`

**Step 4: Commit**

```bash
git commit -m "chore: rebrand from Handy to Hibiki"
```

---

## Phase 5: Prosodic Interface (Stub Only)

### Task 11: Define ProsodicAnalyzer trait (no implementation)

**Files:**
- Create: `src-tauri/src/prosodic/mod.rs`
- Create: `src-tauri/src/prosodic/types.rs`
- Modify: `src-tauri/src/lib.rs`

**Step 1: Define types and trait**

```rust
// src-tauri/src/prosodic/types.rs
#[derive(Debug, Clone, Default)]
pub struct ProsodicFeatures {
    pub speech_rate_wpm: Option<f32>,
    pub pause_count: Option<u32>,
    pub pause_durations_ms: Vec<u64>,
    pub avg_volume: Option<f32>,
}

// src-tauri/src/prosodic/mod.rs
pub mod types;
pub use types::ProsodicFeatures;

/// Trait for prosodic analysis. NOT IMPLEMENTED in v1.
/// This exists only to define the interface for future modules.
pub trait ProsodicAnalyzer: Send + Sync {
    fn analyze(&self, audio_samples: &[f32], sample_rate: u32) -> ProsodicFeatures;
}
```

**Step 2: Verify it compiles**

**Step 3: Commit**

```bash
git commit -m "feat: define ProsodicAnalyzer trait interface (stub, no impl)"
```

---

## Summary

| Phase | Tasks | What it does |
|-------|-------|-------------|
| 1 | Tasks 1-5 | Extract SttEngine trait, wrap engines, build router |
| 2 | Tasks 6-7 | Rule-based refinement module + settings toggle |
| 3 | Tasks 8-9 | Wire pipeline together, engine preference UI |
| 4 | Task 10 | Rebrand Handy → Hibiki |
| 5 | Task 11 | Prosodic trait stub |

**Dependencies:** Phase 1 → Phase 3 (pipeline needs engines). Phase 2 is independent of Phase 1. Phase 4-5 are independent.

**Parallelizable:** Phase 1 + Phase 2 can run in parallel. Phase 4 + Phase 5 can run in parallel.

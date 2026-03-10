# Hibiki — BR-OS Voice Layer Design

**Date**: 2026-03-10
**Status**: Draft
**Scope**: Architecture redesign — from Handy fork to BR-OS voice foundation

---

## 1. Context

Hibiki was forked from [Handy](https://github.com/cjpais/Handy), a cross-platform offline STT desktop app (Tauri 2.x / Rust / React). Of 560 commits, 420 are from the original author. Our 8 commits are specs and test infrastructure — no core logic changes.

This design redefines Hibiki as the **voice foundation layer for BR-OS**, not just a standalone STT tool.

### Related Projects

- **VoiceType4TW** (Python, Faster-Whisper + LLM refinement) — will be archived. Design learnings (light refinement, per-context adjustment) are carried forward conceptually, not as code.
- **BR-OS Philosophy** — Anti-Interpretation Protocol, Lightweight Tagging, user sovereignty. See `I. Philosophy/` in the vault.

---

## 2. Positioning

### What Hibiki is NOT

- Not a general-purpose dictation app
- Not an AI writing assistant that reshapes your words
- Not a Soul System with personality templates

### What Hibiki IS

> BR-OS's voice foundation layer — responsible for turning human speech into system-usable material, without deciding how that material is used.

Voice in BR-OS has a deeper role than "another input method":

1. **Voice is more raw than text** — less self-censorship, closer to unfiltered thought
2. **Voice carries non-verbal signals** — pace, pauses, hesitation (future: whether to preserve these is an open philosophical question)
3. **Voice lowers the barrier** — some people or some states of mind cannot organize text, but can speak
4. **Multimodal accessibility** — ensuring BR-OS is reachable through multiple channels

### The Refinement Question

Pure raw transcription is not a faithful mirror — speech disfluencies (fillers, false starts, broken grammar) are noise from the medium, not the user's intent. Light refinement (filler removal, punctuation, sentence boundary repair) is **cleaning the mirror**, not replacing it with a painting.

The line:

| Level | What it does | BR-OS verdict |
|-------|-------------|---------------|
| Noise reduction | Remove "um", "uh", repeated words | Cleaning the mirror — compliant |
| Sentence repair | Fix word order, restore omitted subjects | Focusing the mirror — compliant |
| Rewriting | Change word choices, alter tone | **Overstepping** — replacing the user's voice |
| Persona | Apply personality templates | **Violation** — agent replacing subject |

Hibiki implements the first two levels. The latter two are out of scope.

---

## 3. Architecture: Modular Pipeline

Each module communicates through well-defined data structures. Any module can be swapped without affecting others.

```
Voice Input
  |
  v
+-- Module 1: Capture ----------------------+
|  Recording + VAD (silence detection)       |
|  Output: audio segments                    |
|  Audio retention: configurable             |
|    - retain in local app data folder       |
|    - discard after transcription           |
|    - retain N days then auto-purge         |
+--------------------------------------------+
  |
  v
+-- Module 2: Transcription -----------------+
|  STT engine (switchable at runtime)        |
|    - Whisper (via whisper-rs) — general     |
|    - SenseVoice (via ONNX) — Chinese/CJK   |
|    - Future engines via same trait          |
|  Output: raw transcript segments           |
+--------------------------------------------+
  |
  v
+-- Module 3: Refinement --------------------+
|  Light cleanup (opt-in, on/off in settings)|
|  v1: rule-based                            |
|    - filler removal                        |
|    - punctuation restoration               |
|    - sentence boundary detection           |
|  Future: local small LLM                   |
|  Output: refined transcript                |
+--------------------------------------------+
  |
  v
+-- Module 4: Prosodic Features (reserved) --+
|  Speech rate, pauses, volume changes       |
|  NOT IMPLEMENTED in v1                     |
|  Only: define the trait/interface           |
|  Output: metadata (TBD)                    |
+--------------------------------------------+
  |
  v
+-- Module 5: Output ------------------------+
|  System clipboard (current)                |
|  Future: BR-OS API, Obsidian, other        |
+--------------------------------------------+
```

### Key Data Structures (conceptual)

```rust
/// A segment of captured audio
struct AudioSegment {
    id: Uuid,
    audio_path: Option<PathBuf>,  // None if retention disabled
    duration_ms: u64,
    timestamp: DateTime<Utc>,
}

/// Raw transcription output
struct TranscriptSegment {
    id: Uuid,
    audio_id: Uuid,
    raw_text: String,
    language: String,
    engine: EngineType,           // Whisper, SenseVoice, etc.
    confidence: Option<f32>,
}

/// After refinement
struct RefinedTranscript {
    id: Uuid,
    transcript_id: Uuid,
    text: String,
    refinement_applied: bool,
}

/// Prosodic features (future)
trait ProsodicAnalyzer {
    fn analyze(&self, audio: &AudioSegment) -> ProsodicFeatures;
}

struct ProsodicFeatures {
    speech_rate_wpm: Option<f32>,
    pause_count: Option<u32>,
    pause_durations_ms: Vec<u64>,
    // ... TBD based on philosophical decisions
}
```

### Engine Trait

```rust
trait SttEngine: Send + Sync {
    fn name(&self) -> &str;
    fn supported_languages(&self) -> Vec<&str>;
    fn transcribe(&self, audio: &AudioSegment) -> Result<TranscriptSegment>;
}
```

All engines implement this trait. Switching engines is a settings change, not a code change.

---

## 4. v1 Decisions

| Decision | v1 Implementation | Swappable later |
|----------|-------------------|-----------------|
| Audio retention | Configurable: retain / discard / auto-purge after N days | Yes |
| Storage location | Local app data folder (same directory as app data) | Yes |
| STT engine | Whisper + SenseVoice, switchable in settings | Yes, via SttEngine trait |
| SenseVoice concern | Chinese tech company origin — disclosed in UI, user chooses | N/A |
| Refinement | Rule-based (filler removal + punctuation) | Yes, can swap to local LLM |
| Refinement toggle | On/off in settings, default on | N/A |
| Prosodic features | Trait defined, not implemented | Plug in later |
| Output | System clipboard | Extensible |
| Side-by-side display | Not in v1 — adds friction every time | Revisit if needed |

---

## 5. Repo Strategy

### Current state

- 560 commits, 420 from Handy original author (CJ)
- Our changes: openspec specs, test infra, formatting — no core logic
- Remote: `devBrightRaven/Hibiki` (fork of `cjpais/Handy`)

### Action

1. **Detach fork** on GitHub (manual: Settings or GitHub Support)
2. **Keep the existing codebase** as working reference
3. **Incrementally refactor** toward the modular pipeline above
4. No need to nuke history — detaching the fork relationship is sufficient

### What to carry forward from Handy

- Audio pipeline (cpal, resampling, device management)
- Whisper-rs integration
- Silero VAD integration
- Tauri command/event architecture
- Settings persistence pattern

### What to replace

- Hardcoded engine assumptions — replace with SttEngine trait
- Monolithic transcription manager — split into pipeline modules
- All "Handy" branding and references

### What to add

- SenseVoice engine (via sherpa-onnx / ONNX Runtime)
- Refinement module (rule-based v1)
- ProsodicAnalyzer trait (interface only)
- Audio retention configuration

---

## 6. Open Questions (deferred to implementation)

1. **Prosodic data and Anti-Interpretation Protocol** — if we capture speech rate and pauses, does presenting them to the user constitute interpretation? Needs experimentation.
2. **Refinement quality for zh-TW** — rule-based filler removal may need language-specific rules for Mandarin fillers (那個、就是、然後).
3. **SenseVoice integration path** — sherpa-onnx vs direct ONNX Runtime in Rust. Need to evaluate Rust bindings maturity.
4. **BR-OS API integration** — what does the output contract look like when Hibiki feeds into Curator/Journal? Deferred until those systems are ready.

---

## 7. Relationship to BR-OS Philosophy

| Principle | How Hibiki respects it |
|-----------|----------------------|
| Zero Interpretation | Transcribes and cleans, never interprets meaning |
| Mirror-Only | Shows what user said, not what they "meant" |
| Non-Prescriptive | Never suggests what to say or how to say it |
| User-First Narrative | User's words are preserved; refinement only removes noise |
| Agency-Return | User controls: engine choice, retention, refinement on/off |
| Lightweight | Voice input should feel lighter than typing, never heavier |

pub mod router;
pub mod types;

#[cfg(feature = "transcribe-rs")]
pub mod parakeet_engine;
#[cfg(feature = "transcribe-rs")]
pub mod sensevoice_engine;
// whisper_engine requires both transcribe-rs and its whisper-cpp sub-feature
#[cfg(all(feature = "transcribe-rs", feature = "whisper-cpp"))]
pub mod whisper_engine;

use anyhow::Result;
pub use types::{EngineId, TranscriptSegment};

/// Trait that all STT engines must implement.
/// This allows swapping engines at runtime without changing the pipeline.
pub trait SttEngine: Send + Sync {
    fn id(&self) -> EngineId;
    fn display_name(&self) -> &str;
    fn supported_languages(&self) -> &[&str];
    /// Transcribe audio samples to text.
    ///
    /// Language and translate are passed per-call so users can switch settings
    /// without reloading the model.
    fn transcribe(
        &self,
        audio_samples: &[f32],
        sample_rate: u32,
        language: Option<&str>,
        translate_to_english: bool,
    ) -> Result<TranscriptSegment>;
}

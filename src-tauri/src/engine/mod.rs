pub mod types;

#[cfg(feature = "transcribe-rs")]
pub mod parakeet_engine;
#[cfg(feature = "transcribe-rs")]
pub mod sensevoice_engine;
#[cfg(feature = "transcribe-rs")]
pub mod whisper_engine;

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

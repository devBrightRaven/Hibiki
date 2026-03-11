use super::{EngineId, SttEngine, TranscriptSegment};
use anyhow::Result;
use std::sync::Mutex;
use transcribe_rs::{
    engines::parakeet::{ParakeetEngine as Inner, ParakeetInferenceParams, TimestampGranularity},
    TranscriptionEngine,
};

/// Wrapper around `transcribe_rs::ParakeetEngine` implementing our `SttEngine` trait.
pub struct ParakeetSttEngine {
    inner: Mutex<Inner>,
}

impl ParakeetSttEngine {
    pub fn new(engine: Inner) -> Self {
        Self {
            inner: Mutex::new(engine),
        }
    }
}

impl SttEngine for ParakeetSttEngine {
    fn id(&self) -> EngineId {
        EngineId::Parakeet
    }

    fn display_name(&self) -> &str {
        "Parakeet"
    }

    fn supported_languages(&self) -> &[&str] {
        &["en"]
    }

    fn transcribe(
        &self,
        audio_samples: &[f32],
        _sample_rate: u32,
        _language: Option<&str>,
        _translate_to_english: bool,
    ) -> Result<TranscriptSegment> {
        let start = std::time::Instant::now();

        let params = ParakeetInferenceParams {
            timestamp_granularity: TimestampGranularity::Segment,
            ..Default::default()
        };

        let mut engine = self
            .inner
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock Parakeet engine: {}", e))?;

        let result = engine
            .transcribe_samples(audio_samples.to_vec(), Some(params))
            .map_err(|e| anyhow::anyhow!("Parakeet transcription failed: {}", e))?;

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(TranscriptSegment {
            raw_text: result.text,
            language: Some("en".to_string()),
            engine_id: EngineId::Parakeet,
            duration_ms,
        })
    }
}

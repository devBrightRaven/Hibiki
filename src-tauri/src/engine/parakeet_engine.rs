use super::{EngineId, SttEngine, TranscriptSegment};
use anyhow::Result;
use std::sync::Mutex;
use transcribe_rs::{onnx::parakeet::ParakeetModel, SpeechModel, TranscribeOptions};

/// Wrapper around `transcribe_rs::onnx::parakeet::ParakeetModel` implementing our `SttEngine` trait.
pub struct ParakeetSttEngine {
    inner: Mutex<ParakeetModel>,
}

impl ParakeetSttEngine {
    pub fn new(model: ParakeetModel) -> Self {
        Self {
            inner: Mutex::new(model),
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

        let options = TranscribeOptions::default();

        let mut model = self
            .inner
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock Parakeet model: {}", e))?;

        let result = model
            .transcribe(audio_samples, &options)
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

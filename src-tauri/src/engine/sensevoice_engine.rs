use super::{EngineId, SttEngine, TranscriptSegment};
use anyhow::Result;
use std::sync::Mutex;
use transcribe_rs::{onnx::sense_voice::SenseVoiceModel, SpeechModel, TranscribeOptions};

/// Wrapper around `transcribe_rs::onnx::sense_voice::SenseVoiceModel` implementing our `SttEngine` trait.
pub struct SenseVoiceSttEngine {
    inner: Mutex<SenseVoiceModel>,
}

impl SenseVoiceSttEngine {
    pub fn new(model: SenseVoiceModel) -> Self {
        Self {
            inner: Mutex::new(model),
        }
    }
}

impl SttEngine for SenseVoiceSttEngine {
    fn id(&self) -> EngineId {
        EngineId::SenseVoice
    }

    fn display_name(&self) -> &str {
        "SenseVoice"
    }

    fn supported_languages(&self) -> &[&str] {
        &["auto", "en", "zh", "ja", "ko", "yue"]
    }

    fn transcribe(
        &self,
        audio_samples: &[f32],
        _sample_rate: u32,
        language: Option<&str>,
        _translate_to_english: bool,
    ) -> Result<TranscriptSegment> {
        let start = std::time::Instant::now();

        let options = TranscribeOptions {
            language: language.map(String::from),
            translate: false,
        };

        let mut model = self
            .inner
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock SenseVoice model: {}", e))?;

        let result = model
            .transcribe(audio_samples, &options)
            .map_err(|e| anyhow::anyhow!("SenseVoice transcription failed: {}", e))?;

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(TranscriptSegment {
            raw_text: result.text,
            language: language.map(String::from),
            engine_id: EngineId::SenseVoice,
            duration_ms,
        })
    }
}

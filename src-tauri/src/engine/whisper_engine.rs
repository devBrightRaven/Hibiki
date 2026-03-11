use super::{EngineId, SttEngine, TranscriptSegment};
use anyhow::Result;
use std::sync::Mutex;
use transcribe_rs::{
    engines::whisper::{WhisperEngine as Inner, WhisperInferenceParams},
    TranscriptionEngine,
};

/// Wrapper around `transcribe_rs::WhisperEngine` implementing our `SttEngine` trait.
pub struct WhisperSttEngine {
    inner: Mutex<Inner>,
}

impl WhisperSttEngine {
    pub fn new(engine: Inner) -> Self {
        Self {
            inner: Mutex::new(engine),
        }
    }
}

impl SttEngine for WhisperSttEngine {
    fn id(&self) -> EngineId {
        EngineId::Whisper
    }

    fn display_name(&self) -> &str {
        "Whisper"
    }

    fn supported_languages(&self) -> &[&str] {
        &["auto", "en", "zh", "ja", "ko", "es", "fr", "de", "it", "pt", "nl", "ru"]
    }

    fn transcribe(
        &self,
        audio_samples: &[f32],
        _sample_rate: u32,
        language: Option<&str>,
        translate_to_english: bool,
    ) -> Result<TranscriptSegment> {
        let start = std::time::Instant::now();

        let whisper_language = language.map(|lang| match lang {
            "zh-Hans" | "zh-Hant" => "zh".to_string(),
            "auto" => return None,
            other => other.to_string(),
        });

        // Flatten Option<Option<String>> → Option<String>
        let whisper_language = whisper_language.flatten();

        let params = WhisperInferenceParams {
            language: whisper_language.clone(),
            translate: translate_to_english,
            ..Default::default()
        };

        let mut engine = self
            .inner
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock whisper engine: {}", e))?;

        let result = engine
            .transcribe_samples(audio_samples.to_vec(), Some(params))
            .map_err(|e| anyhow::anyhow!("Whisper transcription failed: {}", e))?;

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(TranscriptSegment {
            raw_text: result.text,
            language: whisper_language.or_else(|| language.map(String::from)),
            engine_id: EngineId::Whisper,
            duration_ms,
        })
    }
}

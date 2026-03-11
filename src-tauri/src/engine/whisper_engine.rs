use super::{EngineId, SttEngine, TranscriptSegment};
use anyhow::Result;
use std::sync::Mutex;
use transcribe_rs::{whisper_cpp::WhisperEngine as Inner, SpeechModel, TranscribeOptions};

/// Wrapper around `transcribe_rs::whisper_cpp::WhisperEngine` implementing our `SttEngine` trait.
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

        let normalized_lang = language.and_then(|lang| match lang {
            "auto" => None,
            "zh-Hans" | "zh-Hant" => Some("zh".to_string()),
            other => Some(other.to_string()),
        });

        let options = TranscribeOptions {
            language: normalized_lang.clone(),
            translate: translate_to_english,
        };

        let mut engine = self
            .inner
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock whisper engine: {}", e))?;

        let result = engine
            .transcribe(audio_samples, &options)
            .map_err(|e| anyhow::anyhow!("Whisper transcription failed: {}", e))?;

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(TranscriptSegment {
            raw_text: result.text,
            language: normalized_lang.or_else(|| language.map(String::from)),
            engine_id: EngineId::Whisper,
            duration_ms,
        })
    }
}

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
    language: Option<String>,
    translate: bool,
}

impl WhisperSttEngine {
    pub fn new(engine: Inner, language: Option<String>, translate: bool) -> Self {
        Self {
            inner: Mutex::new(engine),
            language,
            translate,
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

    fn transcribe(&self, audio_samples: &[f32], _sample_rate: u32) -> Result<TranscriptSegment> {
        let start = std::time::Instant::now();

        let whisper_language = self.language.as_ref().map(|lang| {
            if lang == "zh-Hans" || lang == "zh-Hant" {
                "zh".to_string()
            } else {
                lang.clone()
            }
        });

        let params = WhisperInferenceParams {
            language: whisper_language,
            translate: self.translate,
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
            language: self.language.clone(),
            engine_id: EngineId::Whisper,
            duration_ms,
        })
    }
}

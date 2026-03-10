use super::{EngineId, SttEngine, TranscriptSegment};
use anyhow::Result;
use std::sync::Mutex;
use transcribe_rs::{
    engines::sense_voice::{
        Language as SenseVoiceLanguage, SenseVoiceEngine as Inner, SenseVoiceInferenceParams,
    },
    TranscriptionEngine,
};

/// Wrapper around `transcribe_rs::SenseVoiceEngine` implementing our `SttEngine` trait.
pub struct SenseVoiceSttEngine {
    inner: Mutex<Inner>,
    language: Option<String>,
}

impl SenseVoiceSttEngine {
    pub fn new(engine: Inner, language: Option<String>) -> Self {
        Self {
            inner: Mutex::new(engine),
            language,
        }
    }

    fn resolve_language(lang: &str) -> SenseVoiceLanguage {
        match lang {
            "zh" | "zh-Hans" | "zh-Hant" => SenseVoiceLanguage::Chinese,
            "en" => SenseVoiceLanguage::English,
            "ja" => SenseVoiceLanguage::Japanese,
            "ko" => SenseVoiceLanguage::Korean,
            "yue" => SenseVoiceLanguage::Cantonese,
            _ => SenseVoiceLanguage::Auto,
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

    fn transcribe(&self, audio_samples: &[f32], _sample_rate: u32) -> Result<TranscriptSegment> {
        let start = std::time::Instant::now();

        let language = self
            .language
            .as_deref()
            .map(Self::resolve_language)
            .unwrap_or(SenseVoiceLanguage::Auto);

        let params = SenseVoiceInferenceParams {
            language,
            use_itn: true,
        };

        let mut engine = self
            .inner
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock SenseVoice engine: {}", e))?;

        let result = engine
            .transcribe_samples(audio_samples.to_vec(), Some(params))
            .map_err(|e| anyhow::anyhow!("SenseVoice transcription failed: {}", e))?;

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(TranscriptSegment {
            raw_text: result.text,
            language: self.language.clone(),
            engine_id: EngineId::SenseVoice,
            duration_ms,
        })
    }
}

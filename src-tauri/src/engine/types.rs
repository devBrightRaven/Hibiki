use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EngineId {
    Whisper,
    Parakeet,
    Moonshine,
    SenseVoice,
}

impl std::fmt::Display for EngineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineId::Whisper => write!(f, "whisper"),
            EngineId::Parakeet => write!(f, "parakeet"),
            EngineId::Moonshine => write!(f, "moonshine"),
            EngineId::SenseVoice => write!(f, "sense_voice"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TranscriptSegment {
    pub raw_text: String,
    pub language: Option<String>,
    pub engine_id: EngineId,
    pub duration_ms: u64,
}

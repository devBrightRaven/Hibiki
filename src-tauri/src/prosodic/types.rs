#[derive(Debug, Clone, Default)]
pub struct ProsodicFeatures {
    pub speech_rate_wpm: Option<f32>,
    pub pause_count: Option<u32>,
    pub pause_durations_ms: Vec<u64>,
    pub avg_volume: Option<f32>,
}

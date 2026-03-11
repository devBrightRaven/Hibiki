#[derive(Debug, Clone)]
pub struct RefinedTranscript {
    pub original_text: String,
    pub refined_text: String,
    pub refinement_applied: bool,
}

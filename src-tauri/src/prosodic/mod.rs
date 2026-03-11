pub mod types;

pub use types::ProsodicFeatures;

/// Trait for prosodic analysis. NOT IMPLEMENTED in v1.
/// This exists only to define the interface for future modules.
pub trait ProsodicAnalyzer: Send + Sync {
    fn analyze(&self, audio_samples: &[f32], sample_rate: u32) -> ProsodicFeatures;
}

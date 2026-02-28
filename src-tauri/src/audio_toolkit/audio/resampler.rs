use rubato::{FftFixedIn, Resampler};
use std::time::Duration;

// Make this a constant you can tweak
const RESAMPLER_CHUNK_SIZE: usize = 1024;

pub struct FrameResampler {
    resampler: Option<FftFixedIn<f32>>,
    chunk_in: usize,
    in_buf: Vec<f32>,
    frame_samples: usize,
    pending: Vec<f32>,
}

impl FrameResampler {
    pub fn new(in_hz: usize, out_hz: usize, frame_dur: Duration) -> Self {
        let frame_samples = ((out_hz as f64 * frame_dur.as_secs_f64()).round()) as usize;
        assert!(frame_samples > 0, "frame duration too short");

        // Use fixed chunk size instead of GCD-based
        let chunk_in = RESAMPLER_CHUNK_SIZE;

        let resampler = (in_hz != out_hz).then(|| {
            FftFixedIn::<f32>::new(in_hz, out_hz, chunk_in, 1, 1)
                .expect("Failed to create resampler")
        });

        Self {
            resampler,
            chunk_in,
            in_buf: Vec::with_capacity(chunk_in),
            frame_samples,
            pending: Vec::with_capacity(frame_samples),
        }
    }

    pub fn push(&mut self, mut src: &[f32], mut emit: impl FnMut(&[f32])) {
        if self.resampler.is_none() {
            self.emit_frames(src, &mut emit);
            return;
        }

        while !src.is_empty() {
            let space = self.chunk_in - self.in_buf.len();
            let take = space.min(src.len());
            self.in_buf.extend_from_slice(&src[..take]);
            src = &src[take..];

            if self.in_buf.len() == self.chunk_in {
                // let start = std::time::Instant::now();
                if let Ok(out) = self
                    .resampler
                    .as_mut()
                    .unwrap()
                    .process(&[&self.in_buf[..]], None)
                {
                    // let duration = start.elapsed();
                    // log::debug!("Resampler took: {:?}", duration);
                    self.emit_frames(&out[0], &mut emit);
                }
                self.in_buf.clear();
            }
        }
    }

    pub fn finish(&mut self, mut emit: impl FnMut(&[f32])) {
        // Process any remaining input samples
        if let Some(ref mut resampler) = self.resampler {
            if !self.in_buf.is_empty() {
                // Pad with zeros to reach chunk size
                self.in_buf.resize(self.chunk_in, 0.0);
                if let Ok(out) = resampler.process(&[&self.in_buf[..]], None) {
                    self.emit_frames(&out[0], &mut emit);
                }
            }
        }

        // Emit any remaining pending frame (padded with zeros)
        if !self.pending.is_empty() {
            self.pending.resize(self.frame_samples, 0.0);
            emit(&self.pending);
            self.pending.clear();
        }
    }

    fn emit_frames(&mut self, mut data: &[f32], emit: &mut impl FnMut(&[f32])) {
        while !data.is_empty() {
            let space = self.frame_samples - self.pending.len();
            let take = space.min(data.len());
            self.pending.extend_from_slice(&data[..take]);
            data = &data[take..];

            if self.pending.len() == self.frame_samples {
                emit(&self.pending);
                self.pending.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate a sine wave at the given sample rate.
    fn sine_wave(sample_rate: usize, freq_hz: f64, duration_secs: f64) -> Vec<f32> {
        let num_samples = (sample_rate as f64 * duration_secs) as usize;
        (0..num_samples)
            .map(|i| {
                let t = i as f64 / sample_rate as f64;
                (2.0 * std::f64::consts::PI * freq_hz * t).sin() as f32
            })
            .collect()
    }

    #[test]
    fn passthrough_when_rates_match() {
        let frame_dur = Duration::from_millis(30);
        let mut resampler = FrameResampler::new(16000, 16000, frame_dur);
        let input = sine_wave(16000, 440.0, 0.1); // 100ms = 1600 samples

        let mut output = Vec::new();
        resampler.push(&input, |frame| output.extend_from_slice(frame));
        resampler.finish(|frame| output.extend_from_slice(frame));

        // Output >= input because finish() pads last frame to frame_samples
        assert!(
            output.len() >= input.len(),
            "output ({}) should be >= input ({})",
            output.len(),
            input.len()
        );
        // The first input.len() samples should be identical in passthrough mode
        for (i, (a, b)) in input.iter().zip(output.iter()).enumerate() {
            assert!(
                (a - b).abs() < 1e-6,
                "passthrough mismatch at sample {}: {} vs {}",
                i,
                a,
                b
            );
        }
    }

    #[test]
    fn downsample_44100_to_16000_produces_correct_count() {
        let frame_dur = Duration::from_millis(30);
        let mut resampler = FrameResampler::new(44100, 16000, frame_dur);
        let duration_secs = 1.0;
        let input = sine_wave(44100, 440.0, duration_secs);

        let mut output = Vec::new();
        resampler.push(&input, |frame| output.extend_from_slice(frame));
        resampler.finish(|frame| output.extend_from_slice(frame));

        // Expected ~16000 samples for 1 second at 16kHz
        // Allow some tolerance for framing/padding
        let expected = 16000;
        let tolerance = 480; // 1 frame = 480 samples at 16kHz/30ms
        assert!(
            (output.len() as i64 - expected as i64).unsigned_abs() <= tolerance as u64,
            "expected ~{} samples, got {}",
            expected,
            output.len()
        );
    }

    #[test]
    fn resampled_output_has_no_clipping() {
        let frame_dur = Duration::from_millis(30);
        let mut resampler = FrameResampler::new(44100, 16000, frame_dur);
        // Use amplitude 0.9 to leave headroom
        let input: Vec<f32> = sine_wave(44100, 440.0, 0.5)
            .iter()
            .map(|s| s * 0.9)
            .collect();

        let mut output = Vec::new();
        resampler.push(&input, |frame| output.extend_from_slice(frame));
        resampler.finish(|frame| output.extend_from_slice(frame));

        for (i, &sample) in output.iter().enumerate() {
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "clipping at sample {}: {}",
                i,
                sample
            );
        }
    }

    #[test]
    fn frames_have_consistent_size() {
        let frame_dur = Duration::from_millis(30);
        let mut resampler = FrameResampler::new(44100, 16000, frame_dur);
        let input = sine_wave(44100, 440.0, 0.5);
        let expected_frame_size = (16000.0 * 0.030) as usize; // 480

        let mut frame_sizes = Vec::new();
        resampler.push(&input, |frame| frame_sizes.push(frame.len()));
        resampler.finish(|frame| frame_sizes.push(frame.len()));

        // All frames should be exactly expected_frame_size
        for (i, &size) in frame_sizes.iter().enumerate() {
            assert_eq!(
                size, expected_frame_size,
                "frame {} has size {}, expected {}",
                i, size, expected_frame_size
            );
        }
    }

    #[test]
    fn empty_input_produces_no_output() {
        let frame_dur = Duration::from_millis(30);
        let mut resampler = FrameResampler::new(44100, 16000, frame_dur);

        let mut output = Vec::new();
        resampler.push(&[], |frame| output.extend_from_slice(frame));
        // Don't call finish — just push empty
        assert!(output.is_empty());
    }
}

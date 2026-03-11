//! Integration tests for Handy's audio and settings subsystems.
//!
//! These tests exercise real components (VAD model, WAV I/O, settings serialization)
//! without requiring a Tauri AppHandle.

use hound::{WavSpec, WavWriter};
use std::path::Path;
use tempfile::TempDir;

/// WAV spec matching the app's recording format (16kHz mono 16-bit PCM).
fn wav_spec() -> WavSpec {
    WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    }
}

/// Write f32 samples to a WAV file at the given path.
fn write_wav(path: &Path, samples: &[f32]) {
    let mut writer = WavWriter::create(path, wav_spec()).expect("create WAV writer");
    for &s in samples {
        let s16 = (s * i16::MAX as f32) as i16;
        writer.write_sample(s16).expect("write sample");
    }
    writer.finalize().expect("finalize WAV");
}

/// Generate a sine wave at 440 Hz (simulates speech-like energy).
fn tone_samples(duration_secs: f64) -> Vec<f32> {
    let n = (16000.0 * duration_secs) as usize;
    (0..n)
        .map(|i| {
            let t = i as f64 / 16000.0;
            (2.0 * std::f64::consts::PI * 440.0 * t).sin() as f32 * 0.8
        })
        .collect()
}

/// Generate silence.
fn silence_samples(duration_secs: f64) -> Vec<f32> {
    vec![0.0f32; (16000.0 * duration_secs) as usize]
}

// ---------------------------------------------------------------------------
// Task 2.5 – WAV fixture generation (generated on-the-fly, not committed)
// ---------------------------------------------------------------------------

#[test]
fn generate_and_read_speech_fixture() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("speech.wav");
    let samples = tone_samples(2.0);
    write_wav(&path, &samples);

    let reader = hound::WavReader::open(&path).expect("open WAV");
    assert_eq!(reader.spec().sample_rate, 16000);
    assert_eq!(reader.spec().channels, 1);
    assert_eq!(reader.len() as usize, samples.len());
}

#[test]
fn generate_and_read_silence_fixture() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("silence.wav");
    let samples = silence_samples(2.0);
    write_wav(&path, &samples);

    let reader = hound::WavReader::open(&path).expect("open WAV");
    assert_eq!(reader.len() as usize, samples.len());
    // All samples should be zero (or very close due to f32→i16→i16 roundtrip)
    let max_val: i16 = reader
        .into_samples::<i16>()
        .map(|s| s.unwrap().abs())
        .max()
        .unwrap_or(0);
    assert_eq!(max_val, 0, "silence fixture should contain all zeros");
}

#[test]
fn generate_and_read_mixed_fixture() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("mixed.wav");
    let mut samples = silence_samples(1.0);
    samples.extend(tone_samples(1.0));
    samples.extend(silence_samples(1.0));
    write_wav(&path, &samples);

    let reader = hound::WavReader::open(&path).expect("open WAV");
    assert_eq!(reader.len() as usize, samples.len());
    assert_eq!(reader.spec().sample_rate, 16000);
}

// ---------------------------------------------------------------------------
// Task 5.2 – VAD processing integration
// ---------------------------------------------------------------------------

#[test]
fn vad_processes_audio_frames_without_error() {
    use hibiki_app_lib::audio_toolkit::vad::{SileroVad, VoiceActivityDetector};

    let model_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("models")
        .join("silero_vad_v4.onnx");

    if !model_path.exists() {
        eprintln!("Skipping VAD test: model not found at {:?}", model_path);
        return;
    }

    let mut vad = SileroVad::new(&model_path, 0.5).expect("create VAD");

    // Feed 30ms frames of a 440Hz tone — verify VAD processes without error.
    // Note: Silero VAD is trained on human speech, so a pure sine wave may not
    // trigger speech detection. The key assertion is that processing succeeds.
    let frame_size = 480; // 30ms at 16kHz
    let tone = tone_samples(0.5); // 500ms
    let mut total_frames = 0;

    for chunk in tone.chunks_exact(frame_size) {
        total_frames += 1;
        // Should not return an error for valid 480-sample frames
        let result = vad.is_voice(chunk);
        assert!(
            result.is_ok(),
            "VAD should process frame {} without error: {:?}",
            total_frames,
            result.err()
        );
    }

    assert!(total_frames > 0, "should have processed at least one frame");
}

#[test]
fn vad_returns_no_speech_for_silence() {
    use hibiki_app_lib::audio_toolkit::vad::{SileroVad, VoiceActivityDetector};

    let model_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("models")
        .join("silero_vad_v4.onnx");

    if !model_path.exists() {
        eprintln!("Skipping VAD test: model not found at {:?}", model_path);
        return;
    }

    let mut vad = SileroVad::new(&model_path, 0.5).expect("create VAD");

    let frame_size = 480;
    let silence = silence_samples(1.0);
    let mut speech_count = 0;

    for chunk in silence.chunks_exact(frame_size) {
        if vad.is_voice(chunk).unwrap_or(false) {
            speech_count += 1;
        }
    }

    assert_eq!(
        speech_count, 0,
        "VAD should not detect speech in silence ({} false positives)",
        speech_count
    );
}

// ---------------------------------------------------------------------------
// Task 5.4 – Settings snapshot test
// ---------------------------------------------------------------------------

#[test]
fn settings_default_snapshot() {
    use hibiki_app_lib::settings::get_default_settings;

    let defaults = get_default_settings();
    let json = serde_json::to_string_pretty(&defaults).expect("serialize defaults");

    // Snapshot: verify key structural properties rather than exact JSON
    // (exact snapshot would break on any field addition)
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let obj = parsed.as_object().expect("settings should be an object");

    // Core fields must exist
    let required_keys = [
        "app_language",
        "push_to_talk",
        "audio_feedback",
        "log_level",
        "paste_method",
        "bindings",
        "post_process_providers",
        "sound_theme",
    ];
    for key in &required_keys {
        assert!(
            obj.contains_key(*key),
            "AppSettings default is missing required key: {}",
            key
        );
    }

    // Bindings should be a non-empty object (HashMap<String, ShortcutBinding>)
    let bindings = obj.get("bindings").unwrap();
    assert!(bindings.is_object(), "bindings should be an object");
    assert!(
        !bindings.as_object().unwrap().is_empty(),
        "bindings should not be empty"
    );

    // post_process_providers should be a non-empty array (Vec<PostProcessProvider>)
    let providers = obj.get("post_process_providers").unwrap();
    assert!(
        providers.is_array(),
        "post_process_providers should be an array"
    );
    assert!(
        !providers.as_array().unwrap().is_empty(),
        "post_process_providers should not be empty"
    );
}

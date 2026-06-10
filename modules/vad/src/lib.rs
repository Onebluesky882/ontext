use ontext_audio::AudioBuffer;
use serde::{Deserialize, Serialize};
use webrtc_vad::{Vad, VadMode, SampleRate};

// ---------------------------------------------------------------------------
// Streaming VAD (RMS-based, processes mic chunks in real-time)
// ---------------------------------------------------------------------------

pub struct StreamingVad {
    threshold: f32,
    silence_ms: u64,
    sample_rate: u32,
    max_chunk_ms: u64,
    min_chunk_ms: u64,

    is_speaking: bool,
    silence_samples: usize,
    speech_buffer: Vec<f32>,
    total_samples: usize,
}

impl Default for StreamingVad {
    fn default() -> Self { Self::new() }
}

impl StreamingVad {
    pub fn new() -> Self {
        Self {
            threshold: 0.01,
            silence_ms: 1200,
            sample_rate: 16000,
            max_chunk_ms: 8000,
            min_chunk_ms: 500,
            is_speaking: false,
            silence_samples: 0,
            speech_buffer: Vec::new(),
            total_samples: 0,
        }
    }

    /// Feed a chunk of 16kHz f32 samples. Returns (final_chunk, _partial).
    /// final_chunk is Some when a speech segment ends (silence detected or max length hit).
    pub fn process(&mut self, samples: &[f32]) -> (Option<Vec<f32>>, Option<Vec<f32>>) {
        let rms = rms(samples);
        let silence_limit = self.ms_to_samples(self.silence_ms);
        let max_limit = self.ms_to_samples(self.max_chunk_ms);
        let min_limit = self.ms_to_samples(self.min_chunk_ms);

        if rms > self.threshold {
            self.is_speaking = true;
            self.silence_samples = 0;
            self.speech_buffer.extend_from_slice(samples);
            self.total_samples += samples.len();

            if self.total_samples >= max_limit {
                return (self.do_flush(min_limit), None);
            }
            (None, None)
        } else {
            if self.is_speaking {
                self.silence_samples += samples.len();
                self.speech_buffer.extend_from_slice(samples);
                self.total_samples += samples.len();

                if self.silence_samples >= silence_limit || self.total_samples >= max_limit {
                    return (self.do_flush(min_limit), None);
                }
            }
            (None, None)
        }
    }

    /// Flush any remaining speech when recording stops.
    pub fn flush(&mut self) -> Option<Vec<f32>> {
        let min_limit = self.ms_to_samples(self.min_chunk_ms);
        self.do_flush(min_limit)
    }

    fn ms_to_samples(&self, ms: u64) -> usize {
        (self.sample_rate as f32 * ms as f32 / 1000.0) as usize
    }

    pub fn is_speaking(&self) -> bool { self.is_speaking }

    fn do_flush(&mut self, min_samples: usize) -> Option<Vec<f32>> {
        self.is_speaking = false;
        self.silence_samples = 0;
        self.total_samples = 0;
        let buf = std::mem::take(&mut self.speech_buffer);
        if buf.len() < min_samples { None } else { Some(buf) }
    }
}

fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() { return 0.0; }
    let sq: f32 = samples.iter().map(|s| s * s).sum();
    (sq / samples.len() as f32).sqrt()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioChunk {
    pub samples: Vec<f32>,
    pub start_ms: u64,
    pub end_ms: u64,
}

// Frame size in samples for 30ms at 16kHz
const FRAME_SAMPLES: usize = 480;
const FRAME_MS: u64 = 30;

fn f32_to_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) * 32767.0) as i16
}

pub fn process(buffer: &AudioBuffer) -> Vec<AudioChunk> {
    if buffer.samples.is_empty() {
        return Vec::new();
    }

    let mut vad = Vad::new_with_rate_and_mode(SampleRate::Rate16kHz, VadMode::Aggressive);

    let mut chunks: Vec<AudioChunk> = Vec::new();
    let mut speech_start: Option<u64> = None;
    let mut speech_samples: Vec<f32> = Vec::new();

    let frames = buffer.samples.chunks(FRAME_SAMPLES);

    for (frame_idx, frame) in frames.enumerate() {
        let frame_start_ms = frame_idx as u64 * FRAME_MS;

        // Pad incomplete last frame with silence
        let padded: Vec<i16> = if frame.len() == FRAME_SAMPLES {
            frame.iter().map(|&s| f32_to_i16(s)).collect()
        } else {
            let mut p: Vec<i16> = frame.iter().map(|&s| f32_to_i16(s)).collect();
            p.resize(FRAME_SAMPLES, 0);
            p
        };

        let is_voice = vad.is_voice_segment(&padded).unwrap_or(false);

        if is_voice {
            if speech_start.is_none() {
                speech_start = Some(frame_start_ms);
                speech_samples.clear();
            }
            // Only append actual (non-padded) samples
            speech_samples.extend_from_slice(frame);
        } else if let Some(start) = speech_start.take() {
            let end_ms = frame_start_ms;
            chunks.push(AudioChunk {
                samples: std::mem::take(&mut speech_samples),
                start_ms: start,
                end_ms,
            });
        }
    }

    // Close any open speech segment at end of buffer
    if let Some(start) = speech_start {
        let total_frames = (buffer.samples.len() + FRAME_SAMPLES - 1) / FRAME_SAMPLES;
        let end_ms = total_frames as u64 * FRAME_MS;
        chunks.push(AudioChunk {
            samples: std::mem::take(&mut speech_samples),
            start_ms: start,
            end_ms,
        });
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontext_audio::AudioBuffer;
    use std::f32::consts::PI;

    fn make_silence(sample_count: usize) -> Vec<f32> {
        vec![0.0f32; sample_count]
    }

    fn make_tone(freq_hz: f32, sample_rate: u32, duration_ms: u64) -> Vec<f32> {
        let num_samples = (sample_rate as u64 * duration_ms / 1000) as usize;
        (0..num_samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                (2.0 * PI * freq_hz * t).sin() * 0.8
            })
            .collect()
    }

    #[test]
    fn test_empty_input_returns_empty_vec() {
        let buffer = AudioBuffer {
            samples: vec![],
            sample_rate: 16000,
        };
        let result = process(&buffer);
        assert!(result.is_empty());
    }

    #[test]
    fn test_silence_only_returns_empty_vec() {
        let buffer = AudioBuffer {
            samples: make_silence(16000), // 1 second of silence
            sample_rate: 16000,
        };
        let result = process(&buffer);
        assert!(result.is_empty());
    }

    #[test]
    fn test_speech_preserved() {
        // 300ms silence + 500ms tone (speech-like) + 200ms silence
        let mut samples = make_silence(4800); // 300ms
        samples.extend(make_tone(400.0, 16000, 500)); // 500ms tone
        samples.extend(make_silence(3200)); // 200ms

        let buffer = AudioBuffer {
            samples,
            sample_rate: 16000,
        };

        let result = process(&buffer);
        // At least one chunk with actual samples
        assert!(!result.is_empty(), "expected at least one speech chunk");
        for chunk in &result {
            assert!(!chunk.samples.is_empty());
            assert!(chunk.end_ms > chunk.start_ms);
        }
    }

    #[test]
    fn test_chunk_timestamps_are_ordered() {
        let mut samples = make_silence(1600); // 100ms silence
        samples.extend(make_tone(400.0, 16000, 600));
        samples.extend(make_silence(1600));

        let buffer = AudioBuffer {
            samples,
            sample_rate: 16000,
        };

        let result = process(&buffer);
        for chunk in &result {
            assert!(
                chunk.end_ms > chunk.start_ms,
                "end_ms must be > start_ms: {} vs {}",
                chunk.end_ms,
                chunk.start_ms
            );
        }
    }

    #[test]
    fn test_very_short_input_no_panic() {
        // Less than one full frame (< 480 samples)
        let buffer = AudioBuffer {
            samples: vec![0.1f32; 100],
            sample_rate: 16000,
        };
        // Should not panic
        let _ = process(&buffer);
    }
}

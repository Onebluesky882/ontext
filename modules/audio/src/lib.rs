use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use ontext_hotkey::HotkeyEvent;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

const TARGET_SAMPLE_RATE: u32 = 16000;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioBuffer {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

#[derive(Debug)]
pub enum AudioError {
    NoInputDevice,
    StreamBuildError(cpal::BuildStreamError),
    StreamPlayError(cpal::PlayStreamError),
    UnsupportedFormat,
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioError::NoInputDevice => write!(f, "no input device available"),
            AudioError::StreamBuildError(e) => write!(f, "stream build error: {e}"),
            AudioError::StreamPlayError(e) => write!(f, "stream play error: {e}"),
            AudioError::UnsupportedFormat => write!(f, "unsupported audio format"),
        }
    }
}

pub struct AudioCapture {
    samples: Arc<Mutex<Vec<f32>>>,
    stream: Option<cpal::Stream>,
    device_sample_rate: u32,
}

impl AudioCapture {
    pub fn new() -> Self {
        AudioCapture {
            samples: Arc::new(Mutex::new(Vec::new())),
            stream: None,
            device_sample_rate: TARGET_SAMPLE_RATE,
        }
    }

    pub fn handle(&mut self, event: HotkeyEvent) -> Result<Option<AudioBuffer>, AudioError> {
        match event {
            HotkeyEvent::Start => {
                self.start()?;
                Ok(None)
            }
            HotkeyEvent::Stop => {
                let buffer = self.stop();
                Ok(Some(buffer))
            }
        }
    }

    fn start(&mut self) -> Result<(), AudioError> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or(AudioError::NoInputDevice)?;

        let supported = device
            .default_input_config()
            .map_err(|_| AudioError::UnsupportedFormat)?;

        let sample_rate = supported.sample_rate().0;
        self.device_sample_rate = sample_rate;

        let config = StreamConfig {
            channels: 1,
            sample_rate: supported.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        let shared = Arc::clone(&self.samples);
        {
            let mut guard = shared.lock().unwrap();
            guard.clear();
        }

        let err_fn = |err| eprintln!("audio stream error: {err}");

        let stream = match supported.sample_format() {
            SampleFormat::F32 => {
                let shared = Arc::clone(&self.samples);
                device.build_input_stream(
                    &config,
                    move |data: &[f32], _| {
                        let mut guard = shared.lock().unwrap();
                        guard.extend_from_slice(data);
                    },
                    err_fn,
                    None,
                )
            }
            SampleFormat::I16 => {
                let shared = Arc::clone(&self.samples);
                device.build_input_stream(
                    &config,
                    move |data: &[i16], _| {
                        let mut guard = shared.lock().unwrap();
                        guard.extend(data.iter().map(|&s| s as f32 / i16::MAX as f32));
                    },
                    err_fn,
                    None,
                )
            }
            SampleFormat::U16 => {
                let shared = Arc::clone(&self.samples);
                device.build_input_stream(
                    &config,
                    move |data: &[u16], _| {
                        let mut guard = shared.lock().unwrap();
                        guard.extend(
                            data.iter()
                                .map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0),
                        );
                    },
                    err_fn,
                    None,
                )
            }
            _ => return Err(AudioError::UnsupportedFormat),
        }
        .map_err(AudioError::StreamBuildError)?;

        stream.play().map_err(AudioError::StreamPlayError)?;
        self.stream = Some(stream);
        Ok(())
    }

    /// Start capturing and deliver resampled 16kHz f32 chunks to `on_chunk` in real-time.
    /// Drop the `AudioCapture` to stop the stream.
    pub fn start_with_callback<F>(&mut self, on_chunk: F) -> Result<(), AudioError>
    where
        F: Fn(Vec<f32>) + Send + Sync + 'static,
    {
        let host = cpal::default_host();
        let device = host.default_input_device().ok_or(AudioError::NoInputDevice)?;
        let supported = device
            .default_input_config()
            .map_err(|_| AudioError::UnsupportedFormat)?;
        let device_rate = supported.sample_rate().0;

        let config = StreamConfig {
            channels: 1,
            sample_rate: supported.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        let err_fn = |err| eprintln!("audio stream error: {err}");
        let cb = Arc::new(on_chunk);

        let stream = match supported.sample_format() {
            SampleFormat::F32 => {
                let cb = Arc::clone(&cb);
                device.build_input_stream(
                    &config,
                    move |data: &[f32], _| {
                        let s = if device_rate != TARGET_SAMPLE_RATE {
                            resample(data, device_rate, TARGET_SAMPLE_RATE)
                        } else {
                            data.to_vec()
                        };
                        (*cb)(s);
                    },
                    err_fn,
                    None,
                )
            }
            SampleFormat::I16 => {
                let cb = Arc::clone(&cb);
                device.build_input_stream(
                    &config,
                    move |data: &[i16], _| {
                        let f: Vec<f32> = data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                        let s = if device_rate != TARGET_SAMPLE_RATE {
                            resample(&f, device_rate, TARGET_SAMPLE_RATE)
                        } else {
                            f
                        };
                        (*cb)(s);
                    },
                    err_fn,
                    None,
                )
            }
            SampleFormat::U16 => {
                let cb = Arc::clone(&cb);
                device.build_input_stream(
                    &config,
                    move |data: &[u16], _| {
                        let f: Vec<f32> = data
                            .iter()
                            .map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0)
                            .collect();
                        let s = if device_rate != TARGET_SAMPLE_RATE {
                            resample(&f, device_rate, TARGET_SAMPLE_RATE)
                        } else {
                            f
                        };
                        (*cb)(s);
                    },
                    err_fn,
                    None,
                )
            }
            _ => return Err(AudioError::UnsupportedFormat),
        }
        .map_err(AudioError::StreamBuildError)?;

        stream.play().map_err(AudioError::StreamPlayError)?;
        self.stream = Some(stream);
        Ok(())
    }

    fn stop(&mut self) -> AudioBuffer {
        // dropping the stream stops recording
        self.stream = None;

        let raw = {
            let guard = self.samples.lock().unwrap();
            guard.clone()
        };

        let samples = if self.device_sample_rate != TARGET_SAMPLE_RATE {
            resample(&raw, self.device_sample_rate, TARGET_SAMPLE_RATE)
        } else {
            raw
        };

        AudioBuffer {
            samples,
            sample_rate: TARGET_SAMPLE_RATE,
        }
    }
}

impl Default for AudioCapture {
    fn default() -> Self {
        Self::new()
    }
}

fn resample(input: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate || input.is_empty() {
        return input.to_vec();
    }
    let ratio = from_rate as f64 / to_rate as f64;
    let out_len = ((input.len() as f64) / ratio).ceil() as usize;
    let mut output = Vec::with_capacity(out_len);
    for i in 0..out_len {
        let src = i as f64 * ratio;
        let idx = src as usize;
        let frac = (src - idx as f64) as f32;
        let a = input[idx.min(input.len() - 1)];
        let b = input[(idx + 1).min(input.len() - 1)];
        output.push(a + frac * (b - a));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_buffer_sample_rate_is_16000() {
        let buf = AudioBuffer {
            samples: vec![0.0, 0.5, -0.5],
            sample_rate: TARGET_SAMPLE_RATE,
        };
        assert_eq!(buf.sample_rate, 16000);
    }

    #[test]
    fn test_audio_buffer_stores_samples() {
        let samples = vec![0.1f32, 0.2, 0.3, -0.1, -0.2];
        let buf = AudioBuffer {
            samples: samples.clone(),
            sample_rate: TARGET_SAMPLE_RATE,
        };
        assert_eq!(buf.samples, samples);
    }

    #[test]
    fn test_resample_same_rate_returns_identical() {
        let input = vec![0.1f32, 0.2, 0.3, 0.4];
        let output = resample(&input, 16000, 16000);
        assert_eq!(input, output);
    }

    #[test]
    fn test_resample_empty_input() {
        let output = resample(&[], 44100, 16000);
        assert!(output.is_empty());
    }

    #[test]
    fn test_resample_downsample_length() {
        let input: Vec<f32> = (0..44100).map(|i| i as f32 / 44100.0).collect();
        let output = resample(&input, 44100, 16000);
        // expected length is roughly input.len() * (16000/44100)
        let expected = ((input.len() as f64 * 16000.0) / 44100.0).ceil() as usize;
        assert_eq!(output.len(), expected);
    }

    #[test]
    fn test_resample_upsample_length() {
        let input: Vec<f32> = (0..16000).map(|i| i as f32 / 16000.0).collect();
        let output = resample(&input, 16000, 44100);
        let expected = ((input.len() as f64 * 44100.0) / 16000.0).ceil() as usize;
        assert_eq!(output.len(), expected);
    }

    #[test]
    fn test_audio_buffer_serde_roundtrip() {
        let buf = AudioBuffer {
            samples: vec![0.1, -0.1, 0.5],
            sample_rate: 16000,
        };
        let json = serde_json::to_string(&buf).unwrap();
        assert!(json.contains("sampleRate"));
        let restored: AudioBuffer = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.sample_rate, 16000);
        assert_eq!(restored.samples.len(), 3);
    }

    #[test]
    fn test_audio_capture_new() {
        let capture = AudioCapture::new();
        let guard = capture.samples.lock().unwrap();
        assert!(guard.is_empty());
    }
}

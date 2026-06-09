use hound::{SampleFormat, WavSpec, WavWriter};
use ontext_vad::AudioChunk;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptResult {
    pub text: String,
    pub language: String,
}

#[derive(Debug, Error)]
pub enum TranscribeError {
    #[error("API timeout")]
    Timeout,
    #[error("API error {status}: {message}")]
    ApiError { status: u16, message: String },
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("WAV encoding error: {0}")]
    WavError(String),
}

#[derive(Deserialize)]
struct WhisperVerboseResponse {
    text: String,
    language: String,
}

pub async fn transcribe(
    chunks: Vec<AudioChunk>,
    api_key: &str,
) -> Result<TranscriptResult, TranscribeError> {
    transcribe_impl(chunks, api_key, "https://api.groq.com/openai", "whisper-large-v3-turbo").await
}

pub async fn transcribe_with_base_url(
    chunks: Vec<AudioChunk>,
    api_key: &str,
    base_url: &str,
) -> Result<TranscriptResult, TranscribeError> {
    transcribe_impl(chunks, api_key, base_url, "whisper-1").await
}

async fn transcribe_impl(
    chunks: Vec<AudioChunk>,
    api_key: &str,
    base_url: &str,
    model: &str,
) -> Result<TranscriptResult, TranscribeError> {
    let wav_bytes = encode_chunks_to_wav(&chunks)?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(TranscribeError::HttpError)?;

    let file_part = multipart::Part::bytes(wav_bytes)
        .file_name("audio.wav")
        .mime_str("audio/wav")
        .map_err(TranscribeError::HttpError)?;

    let form = multipart::Form::new()
        .part("file", file_part)
        .text("model", model.to_string())
        .text("response_format", "verbose_json");

    let url = format!("{}/v1/audio/transcriptions", base_url);

    let response = client
        .post(&url)
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                TranscribeError::Timeout
            } else {
                TranscribeError::HttpError(e)
            }
        })?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let message = response.text().await.unwrap_or_default();
        return Err(TranscribeError::ApiError { status, message });
    }

    let whisper_response: WhisperVerboseResponse = response
        .json()
        .await
        .map_err(TranscribeError::HttpError)?;

    Ok(TranscriptResult {
        text: whisper_response.text.trim().to_string(),
        language: whisper_response.language,
    })
}

fn encode_chunks_to_wav(chunks: &[AudioChunk]) -> Result<Vec<u8>, TranscribeError> {
    let samples: Vec<f32> = chunks
        .iter()
        .flat_map(|c| c.samples.iter().copied())
        .collect();

    let spec = WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut cursor = Cursor::new(Vec::new());
    let mut writer =
        WavWriter::new(&mut cursor, spec).map_err(|e| TranscribeError::WavError(e.to_string()))?;

    for sample in samples {
        writer
            .write_sample(sample)
            .map_err(|e| TranscribeError::WavError(e.to_string()))?;
    }

    writer
        .finalize()
        .map_err(|e| TranscribeError::WavError(e.to_string()))?;

    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_chunk(samples: Vec<f32>) -> AudioChunk {
        AudioChunk {
            samples,
            start_ms: 0,
            end_ms: 100,
        }
    }

    #[test]
    fn test_encode_chunks_to_wav_produces_valid_wav() {
        let chunks = vec![make_chunk(vec![0.0f32, 0.1, -0.1, 0.5])];
        let wav_bytes = encode_chunks_to_wav(&chunks).unwrap();

        // WAV files start with RIFF header
        assert!(wav_bytes.len() > 44);
        assert_eq!(&wav_bytes[0..4], b"RIFF");
        assert_eq!(&wav_bytes[8..12], b"WAVE");
    }

    #[test]
    fn test_encode_empty_chunks_produces_valid_wav() {
        let chunks: Vec<AudioChunk> = vec![];
        let wav_bytes = encode_chunks_to_wav(&chunks).unwrap();
        assert_eq!(&wav_bytes[0..4], b"RIFF");
    }

    #[test]
    fn test_encode_multiple_chunks_concatenates_samples() {
        let chunks = vec![
            make_chunk(vec![0.1, 0.2]),
            make_chunk(vec![0.3, 0.4, 0.5]),
        ];
        let wav_bytes = encode_chunks_to_wav(&chunks).unwrap();

        let mut cursor = Cursor::new(wav_bytes);
        let reader = hound::WavReader::new(&mut cursor).unwrap();
        assert_eq!(reader.len(), 5);
    }

    #[tokio::test]
    async fn test_transcribe_success_english() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/audio/transcriptions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"text":"  Hello world  ","language":"english"}"#)
            .create_async()
            .await;

        let chunks = vec![make_chunk(vec![0.0; 160])];
        let result = transcribe_with_base_url(chunks, "test-key", &server.url())
            .await
            .unwrap();

        assert_eq!(result.text, "Hello world");
        assert_eq!(result.language, "english");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_transcribe_success_thai() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/audio/transcriptions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"text":" สวัสดี ","language":"thai"}"#)
            .create_async()
            .await;

        let chunks = vec![make_chunk(vec![0.0; 160])];
        let result = transcribe_with_base_url(chunks, "test-key", &server.url())
            .await
            .unwrap();

        assert_eq!(result.text, "สวัสดี");
        assert_eq!(result.language, "thai");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_transcribe_text_is_trimmed() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock("POST", "/v1/audio/transcriptions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"text":"\n  trimmed text \t\n","language":"english"}"#)
            .create_async()
            .await;

        let chunks = vec![make_chunk(vec![0.0; 160])];
        let result = transcribe_with_base_url(chunks, "test-key", &server.url())
            .await
            .unwrap();

        assert_eq!(result.text, "trimmed text");
    }

    #[tokio::test]
    async fn test_transcribe_api_error_returns_structured_error() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock("POST", "/v1/audio/transcriptions")
            .with_status(401)
            .with_body(r#"{"error":{"message":"Invalid API key"}}"#)
            .create_async()
            .await;

        let chunks = vec![make_chunk(vec![0.0; 160])];
        let result = transcribe_with_base_url(chunks, "bad-key", &server.url()).await;

        match result {
            Err(TranscribeError::ApiError { status, .. }) => assert_eq!(status, 401),
            other => panic!("expected ApiError, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_transcribe_server_error_returns_structured_error() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock("POST", "/v1/audio/transcriptions")
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let chunks = vec![make_chunk(vec![0.0; 160])];
        let result = transcribe_with_base_url(chunks, "test-key", &server.url()).await;

        match result {
            Err(TranscribeError::ApiError { status, .. }) => assert_eq!(status, 500),
            other => panic!("expected ApiError, got {:?}", other),
        }
    }
}

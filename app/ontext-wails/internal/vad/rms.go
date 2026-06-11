package vad

import (
	"context"
	"math"

	"ontext-wails/internal/audio"
)

const (
	defaultThreshold  = float32(0.02)
	defaultSilenceMs  = 1200
	defaultMaxChunkMs = 8000
	defaultMinChunkMs = 500
	defaultSampleRate = 16000
)

// RMSDetector implements Detector using streaming RMS-based voice activity
// detection. Thresholds and timing constants are ported from modules/vad (Rust).
type RMSDetector struct {
	Threshold  float32 // RMS amplitude threshold to classify a frame as speech
	SilenceMs  int     // consecutive silence ms required to end a segment
	MaxChunkMs int     // maximum segment length before forced flush
	MinChunkMs int     // minimum segment length to emit (shorter discarded)
}

// NewRMSDetector returns an RMSDetector with defaults matching modules/vad.
func NewRMSDetector() *RMSDetector {
	return &RMSDetector{
		Threshold:  defaultThreshold,
		SilenceMs:  defaultSilenceMs,
		MaxChunkMs: defaultMaxChunkMs,
		MinChunkMs: defaultMinChunkMs,
	}
}

// Detect consumes frames from the input channel and emits speech Segments.
// The output channel is closed when frames is closed or ctx is canceled.
func (d *RMSDetector) Detect(ctx context.Context, frames <-chan audio.Frame) <-chan Segment {
	out := make(chan Segment)
	go func() {
		defer close(out)

		sr := defaultSampleRate
		silenceLimit := msToSamples(d.SilenceMs, sr)
		maxLimit := msToSamples(d.MaxChunkMs, sr)
		minLimit := msToSamples(d.MinChunkMs, sr)

		var (
			isSpeaking     bool
			silenceSamples int
			speechBuf      []float32
		)

		emit := func() {
			if len(speechBuf) >= minLimit {
				seg := Segment{
					Samples:    append([]float32(nil), speechBuf...),
					SampleRate: sr,
				}
				select {
				case out <- seg:
				case <-ctx.Done():
				}
			}
			speechBuf = speechBuf[:0]
			isSpeaking = false
			silenceSamples = 0
		}

		for {
			select {
			case <-ctx.Done():
				if isSpeaking {
					emit()
				}
				return
			case frame, ok := <-frames:
				if !ok {
					if isSpeaking {
						emit()
					}
					return
				}

				if frame.SampleRate > 0 && frame.SampleRate != sr {
					sr = frame.SampleRate
					silenceLimit = msToSamples(d.SilenceMs, sr)
					maxLimit = msToSamples(d.MaxChunkMs, sr)
					minLimit = msToSamples(d.MinChunkMs, sr)
				}

				r := rms(frame.Samples)
				if r > d.Threshold {
					isSpeaking = true
					silenceSamples = 0
					speechBuf = append(speechBuf, frame.Samples...)
					if len(speechBuf) >= maxLimit {
						emit()
					}
				} else if isSpeaking {
					silenceSamples += len(frame.Samples)
					speechBuf = append(speechBuf, frame.Samples...)
					if silenceSamples >= silenceLimit || len(speechBuf) >= maxLimit {
						emit()
					}
				}
			}
		}
	}()
	return out
}

func rms(samples []float32) float32 {
	if len(samples) == 0 {
		return 0
	}
	var sum float64
	for _, s := range samples {
		sum += float64(s) * float64(s)
	}
	return float32(math.Sqrt(sum / float64(len(samples))))
}

func msToSamples(ms, sampleRate int) int {
	return sampleRate * ms / 1000
}

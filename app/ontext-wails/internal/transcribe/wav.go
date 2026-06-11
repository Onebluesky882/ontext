package transcribe

import (
	"bytes"
	"encoding/binary"
	"math"
)

// encodeWAV encodes mono float32 samples (range [-1, 1]) as 16-bit PCM WAV.
func encodeWAV(samples []float32, sampleRate int) []byte {
	const bitsPerSample = 16
	const numChannels = 1

	dataSize := len(samples) * 2
	byteRate := sampleRate * numChannels * bitsPerSample / 8
	blockAlign := numChannels * bitsPerSample / 8

	var buf bytes.Buffer
	buf.WriteString("RIFF")
	binary.Write(&buf, binary.LittleEndian, uint32(36+dataSize))
	buf.WriteString("WAVE")

	buf.WriteString("fmt ")
	binary.Write(&buf, binary.LittleEndian, uint32(16)) // PCM fmt chunk size
	binary.Write(&buf, binary.LittleEndian, uint16(1))  // PCM format
	binary.Write(&buf, binary.LittleEndian, uint16(numChannels))
	binary.Write(&buf, binary.LittleEndian, uint32(sampleRate))
	binary.Write(&buf, binary.LittleEndian, uint32(byteRate))
	binary.Write(&buf, binary.LittleEndian, uint16(blockAlign))
	binary.Write(&buf, binary.LittleEndian, uint16(bitsPerSample))

	buf.WriteString("data")
	binary.Write(&buf, binary.LittleEndian, uint32(dataSize))
	for _, s := range samples {
		clamped := math.Max(-1, math.Min(1, float64(s)))
		binary.Write(&buf, binary.LittleEndian, int16(clamped*32767))
	}

	return buf.Bytes()
}

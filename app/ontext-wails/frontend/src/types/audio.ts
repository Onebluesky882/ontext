export interface AudioBuffer {
  samples: number[]
  sampleRate: number
}

export interface AudioChunk {
  samples: number[]
  startMs: number
  endMs: number
}

export type HotkeyEvent = "Start" | "Stop"

export type AppStatus = "idle" | "recording" | "transcribing" | "done" | "error"

export interface StatusEvent {
  status: AppStatus
  message?: string
}

export type MicrophonePermission = "authorized" | "denied" | "restricted" | "not_determined"

export interface PermissionStatus {
  accessibility: boolean
  microphone: MicrophonePermission
}

// Payload of the "transcript:partial" event: the cumulative transcript
// text for the in-progress recording session, emitted each time a new
// segment is transcribed.
export type TranscriptPartialEvent = string

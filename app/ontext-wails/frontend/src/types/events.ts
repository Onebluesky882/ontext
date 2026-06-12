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

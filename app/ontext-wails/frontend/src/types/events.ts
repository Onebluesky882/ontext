export type HotkeyEvent = "Start" | "Stop"

export type AppStatus = "idle" | "recording" | "transcribing" | "done" | "error"

export interface StatusEvent {
  status: AppStatus
  message?: string
}

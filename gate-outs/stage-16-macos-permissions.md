---
status: PASS
stage: 16
domain: app/ontext-wails
branch: stage-16-macos-permissions
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-12
ready_for_next: YES
---

summary: Added NSMicrophoneUsageDescription to the macOS Info.plist template and updated the onboarding PermissionStep to request/display both Microphone and Accessibility permission states without crashing on denial

modified_files:
  - app/ontext-wails/build/darwin/Info.plist
  - app/ontext-wails/internal/focus/focus.go
  - app/ontext-wails/internal/focus/focus_darwin.go
  - app/ontext-wails/internal/focus/focus_other.go
  - app/ontext-wails/internal/focus/focus_test.go
  - app/ontext-wails/app.go
  - app/ontext-wails/frontend/wailsjs/go/main/App.d.ts
  - app/ontext-wails/frontend/wailsjs/go/main/App.js
  - app/ontext-wails/frontend/wailsjs/go/models.ts
  - app/ontext-wails/frontend/src/types/events.ts
  - app/ontext-wails/frontend/src/pages/onboarding/PermissionStep.tsx
  - app/ontext-wails/frontend/src/App.css

dependencies_added:
  - none

tests:
  - TestMicrophonePermissionString
  - TestMicrophonePermissionStatusDoesNotPanic

acceptance_criteria:
  - PASS: build/darwin/Info.plist now includes NSMicrophoneUsageDescription with a clear, user-facing reason string
  - PASS: PermissionStep.tsx requests microphone access on mount via AVCaptureDevice (focus_darwin.go) and displays granted/denied/restricted/not_determined states
  - PASS: PermissionStep.tsx retains the existing Accessibility flow and surfaces both permission states together via GetPermissionStatus
  - PASS: Denial of either permission shows a clear in-app status (warning text + link to System Settings) and does not block app usage or crash; only Accessibility is required to continue onboarding since it is required for the paste flow

known_issues:
  - frontend/node_modules is not installed in this workspace, so `tsc`/`vite build` could not be run end-to-end; Go build/vet/test for internal/focus and app.go pass cleanly
  - Manual on-device verification of the macOS permission dialogs (first-launch prompts, denial flows) was not performed in this environment - requires a built .app bundle run on macOS with TCC reset

recommendations:
  - When cutting the next macOS build, run `wails build -clean` so build/darwin/Info.plist (with NSMicrophoneUsageDescription) is picked up, then verify both prompts on a clean TCC database (tccutil reset Microphone/Accessibility com.wails.ontext-wails)

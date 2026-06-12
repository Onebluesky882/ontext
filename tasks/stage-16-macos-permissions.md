# Stage 16 — macOS microphone & accessibility permission flow

Status: DONE

Domain: `app/ontext-wails` (Wails build config / `Info.plist` /
entitlements), `frontend/src/pages/onboarding`
Branch: `feature/stage-16-macos-permissions`

Goal:
Ensure the macOS build requests microphone access via
`NSMicrophoneUsageDescription` (with a clear, user-facing reason string), and
that the existing `PermissionStep.tsx` onboarding flow correctly triggers and
reflects both Microphone and Accessibility permission prompts on first
launch. Must not crash if either permission is denied.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-12
Completed At: 2026-06-12
Status: DONE

---

Checklist:
- [x] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 16)
- [x] Check Wails macOS build config / `Info.plist` for
      `NSMicrophoneUsageDescription` — add with descriptive string if missing
- [x] Verify first-launch macOS microphone permission dialog appears
- [x] Verify Accessibility permission prompt/flow (ties into Stage 13
      fallback behavior)
- [x] Update `PermissionStep.tsx` if needed to reflect both permission states
      clearly
- [x] Test denial of each permission — confirm no crash, clear in-app status
- [x] Create gate-outs/stage-16-macos-permissions.md

---

Gate-Out: gate-outs/stage-16-macos-permissions.md
Next Stage: none (independent verification stage)

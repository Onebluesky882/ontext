# Stage 16 — macOS microphone & accessibility permission flow

Status: READY

Domain: `app/ontext-wails` (Wails build config / `Info.plist` /
entitlements), `frontend/src/pages/onboarding`
Branch: `feature/stage-16-macos-permissions`

Goal:
Ensure the macOS build requests microphone access via
`NSMicrophoneUsageDescription` (with a clear, user-facing reason string), and
that the existing `PermissionStep.tsx` onboarding flow correctly triggers and
reflects both Microphone and Accessibility permission prompts on first
launch. Must not crash if either permission is denied.

Assigned To: (unassigned)
Started At: -
Completed At: -

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 16)
- [ ] Check Wails macOS build config / `Info.plist` for
      `NSMicrophoneUsageDescription` — add with descriptive string if missing
- [ ] Verify first-launch macOS microphone permission dialog appears
- [ ] Verify Accessibility permission prompt/flow (ties into Stage 13
      fallback behavior)
- [ ] Update `PermissionStep.tsx` if needed to reflect both permission states
      clearly
- [ ] Test denial of each permission — confirm no crash, clear in-app status
- [ ] Create gate-outs/stage-16-macos-permissions.md

---

Gate-Out: gate-outs/stage-16-macos-permissions.md
Next Stage: none (independent verification stage)

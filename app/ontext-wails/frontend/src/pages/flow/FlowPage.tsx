import { useState } from 'react'
import { PermissionsPage } from './PermissionsPage'
import { HotkeyStatusPage } from './HotkeyStatusPage'
import { LiveTranscriptPage } from './LiveTranscriptPage'

type Step = 'permissions' | 'status' | 'transcript'

const STEPS: Step[] = ['permissions', 'status', 'transcript']
const FLOW_DONE_KEY = 'ontext:flow-done'

export function FlowPage() {
  const [step, setStep] = useState<Step>(() =>
    localStorage.getItem(FLOW_DONE_KEY) === 'true' ? 'transcript' : 'permissions'
  )

  const goTo = (next: Step) => {
    setStep(next)
    if (next === 'transcript') localStorage.setItem(FLOW_DONE_KEY, 'true')
  }

  return (
    <div className="ob-shell">
      <div className="ob-progress">
        {STEPS.map((s, i) => (
          <div
            key={s}
            className={`ob-progress__dot ${step === s ? 'ob-progress__dot--active' : ''} ${
              STEPS.indexOf(step) > i ? 'ob-progress__dot--done' : ''
            }`}
          />
        ))}
      </div>

      {step === 'permissions' && <PermissionsPage onNext={() => goTo('status')} />}
      {step === 'status' && (
        <HotkeyStatusPage onNext={() => goTo('transcript')} onBack={() => setStep('permissions')} />
      )}
      {step === 'transcript' && <LiveTranscriptPage onBack={() => setStep('status')} />}
    </div>
  )
}

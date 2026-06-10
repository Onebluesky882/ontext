import { useState } from 'react'
import { WelcomeStep } from './onboarding/WelcomeStep'
import { LanguageStep } from './onboarding/LanguageStep'
import { PermissionStep } from './onboarding/PermissionStep'

type Step = 'welcome' | 'language' | 'permission'

interface Props {
  onComplete: (outputLanguage: string) => void
}

export function OnboardingPage({ onComplete }: Props) {
  const [step, setStep] = useState<Step>('welcome')
  const [outputLanguage, setOutputLanguage] = useState('en')

  return (
    <div className="ob-shell">
      <div className="ob-progress">
        {(['welcome', 'language', 'permission'] as Step[]).map((s, i) => (
          <div
            key={s}
            className={`ob-progress__dot ${step === s ? 'ob-progress__dot--active' : ''} ${
              ['welcome', 'language', 'permission'].indexOf(step) > i ? 'ob-progress__dot--done' : ''
            }`}
          />
        ))}
      </div>

      {step === 'welcome' && (
        <WelcomeStep onNext={() => setStep('language')} />
      )}

      {step === 'language' && (
        <LanguageStep
          selected={outputLanguage}
          onSelect={setOutputLanguage}
          onNext={() => setStep('permission')}
          onBack={() => setStep('welcome')}
        />
      )}

      {step === 'permission' && (
        <PermissionStep
          onDone={() => onComplete(outputLanguage)}
          onBack={() => setStep('language')}
        />
      )}
    </div>
  )
}

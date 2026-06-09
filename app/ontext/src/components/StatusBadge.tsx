import type { PipelineStatus } from '../store/appStore'

interface Props {
  status: PipelineStatus
}

const labels: Record<PipelineStatus, string> = {
  idle: 'Ready',
  running: 'Listening…',
  done: 'Done',
  error: 'Error',
}

export function StatusBadge({ status }: Props) {
  return (
    <div className={`status-badge status-badge--${status}`}>
      <span className="status-badge__dot" />
      <span className="status-badge__label">{labels[status]}</span>
    </div>
  )
}

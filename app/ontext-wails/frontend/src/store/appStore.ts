import { create } from 'zustand'
import type { PasteResult } from '../types/transcript'

export type PipelineStatus = 'idle' | 'running' | 'done' | 'error'

interface AppState {
  status: PipelineStatus
  lastText: string | null
  errorMessage: string | null
  setRunning: () => void
  setStatus: (status: PipelineStatus) => void
  setDone: (result: PasteResult, text?: string) => void
  reset: () => void
}

export const useAppStore = create<AppState>((set) => ({
  status: 'idle',
  lastText: null,
  errorMessage: null,

  setRunning: () => set({ status: 'running', errorMessage: null }),

  setStatus: (status) => set({ status }),

  setDone: (result, text) =>
    set({
      status: result.success ? 'done' : 'error',
      lastText: text ?? null,
      errorMessage: result.error ?? null,
    }),

  reset: () => set({ status: 'idle' }),
}))

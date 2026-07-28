import { useStore } from '@tanstack/react-store'
import { useEffect } from 'react'
import { $elapsedNow } from '@/stores/ui'

export function useElapsedTime(startedAt: number | null, running: boolean) {
  const now = useStore($elapsedNow)

  useEffect(() => {
    if (!running) return
    $elapsedNow.set(Date.now())
    const timer = window.setInterval(() => $elapsedNow.set(Date.now()), 1_000)
    return () => window.clearInterval(timer)
  }, [running])

  return startedAt === null ? 0 : Math.max(0, now - startedAt)
}

export function formatElapsed(milliseconds: number) {
  const seconds = Math.floor(milliseconds / 1_000)
  const minutes = Math.floor(seconds / 60)
  return `${minutes}:${String(seconds % 60).padStart(2, '0')}`
}

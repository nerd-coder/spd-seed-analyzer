import { useEffect, useState } from 'react'

export function useElapsedTime(startedAt: number | null, running: boolean) {
  const [now, setNow] = useState(Date.now())

  useEffect(() => {
    if (!running) return
    setNow(Date.now())
    const timer = window.setInterval(() => setNow(Date.now()), 1_000)
    return () => window.clearInterval(timer)
  }, [running])

  return startedAt === null ? 0 : Math.max(0, now - startedAt)
}

export function formatElapsed(milliseconds: number) {
  const seconds = Math.floor(milliseconds / 1_000)
  const minutes = Math.floor(seconds / 60)
  return `${minutes}:${String(seconds % 60).padStart(2, '0')}`
}

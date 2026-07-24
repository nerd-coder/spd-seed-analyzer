import { Loader2 } from 'lucide-react'
import { SeedReportView } from '@/components/seed/SeedReportView'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { formatElapsed, useElapsedTime } from '@/hooks/useElapsedTime'
import { cancelSeedAnalysis, type SeedSession } from '@/stores/app'

export function SessionPane({
  session,
  identitySpoilers,
  mapSpoilers,
}: {
  session: SeedSession
  identitySpoilers: boolean
  mapSpoilers: boolean
}) {
  const loading = session.status === 'pending' || session.status === 'loading'
  const elapsed = useElapsedTime(session.startedAt, loading)
  if (loading) {
    return (
      <div className="flex min-h-48 flex-col items-center justify-center gap-2 py-12">
        <Loader2 className="text-muted-foreground size-6 animate-spin" />
        <p className="text-muted-foreground text-sm">
          Analyzing{' '}
          <span className="text-foreground font-mono">{session.input}</span>…
        </p>
        <p
          className="text-muted-foreground font-mono text-xs"
          aria-live="polite"
        >
          Elapsed {formatElapsed(elapsed)}
        </p>
        <Button
          type="button"
          variant="destructive"
          size="sm"
          onClick={() => cancelSeedAnalysis(session.id)}
        >
          Cancel analysis
        </Button>
      </div>
    )
  }

  if (session.status === 'cancelled') {
    return (
      <Alert>
        <AlertTitle>Analysis cancelled</AlertTitle>
        <AlertDescription>
          The worker was stopped after {formatElapsed(elapsed)}.
        </AlertDescription>
      </Alert>
    )
  }

  if (session.status === 'error') {
    return (
      <Alert variant="destructive">
        <AlertTitle>Analysis failed</AlertTitle>
        <AlertDescription>
          {session.error ?? 'Unknown error analyzing this seed.'}
        </AlertDescription>
      </Alert>
    )
  }

  if (session.report) {
    return (
      <div className="relative">
        <SeedReportView
          report={session.report}
          identitySpoilers={identitySpoilers}
          mapSpoilers={mapSpoilers}
        />
      </div>
    )
  }

  return null
}

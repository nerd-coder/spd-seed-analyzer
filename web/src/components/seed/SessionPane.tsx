import { Loader2 } from 'lucide-react'
import { SeedReportView } from '@/components/seed/SeedReportView'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import type { SeedSession } from '@/stores/app'

export function SessionPane({
  session,
  identitySpoilers,
  mapSpoilers,
}: {
  session: SeedSession
  identitySpoilers: boolean
  mapSpoilers: boolean
}) {
  if (session.status === 'pending' || session.status === 'loading') {
    return (
      <div className="flex min-h-[12rem] flex-col items-center justify-center gap-2 py-12">
        <Loader2 className="text-muted-foreground size-6 animate-spin" />
        <p className="text-muted-foreground text-sm">
          Analyzing{' '}
          <span className="text-foreground font-mono">{session.input}</span>…
        </p>
      </div>
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

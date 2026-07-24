import {
  ArrowRightIcon,
  BinocularsIcon,
  CheckCircleIcon,
  StopIcon,
  WarningCircleIcon,
} from '@phosphor-icons/react'
import { ItemIcon } from '@/components/ItemIcon'
import { ItemName } from '@/components/ItemName'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import {
  Item,
  ItemContent,
  ItemDescription,
  ItemGroup,
  ItemMedia,
  ItemTitle,
} from '@/components/ui/item'
import { Progress } from '@/components/ui/progress'
import { formatElapsed, useElapsedTime } from '@/hooks/useElapsedTime'
import { formatItemSource } from '@/lib/labels'
import type { SeedSearchMatch } from '@/lib/spd-wasm'
import type { FinderRunState } from './finder-types'

function statusCopy(run: FinderRunState): {
  label: string
  description: string
} {
  if (run.status === 'running') {
    return {
      label: run.cancelRequested ? 'Cancelling' : 'Searching',
      description: run.cancelRequested
        ? 'Stopping the search worker.'
        : 'Scanning in a background worker so the interface stays responsive.',
    }
  }
  if (run.status === 'cancelled') {
    return {
      label: 'Cancelled',
      description:
        'The background worker stopped. Results found so far are retained.',
    }
  }
  if (run.status === 'error') {
    return {
      label: 'Error',
      description: 'The scan stopped after an unexpected search error.',
    }
  }
  if (run.completionReason === 'result-limit') {
    return {
      label: 'Completed',
      description: 'The configured result limit was reached.',
    }
  }
  if (run.completionReason === 'exhausted') {
    return {
      label: 'Completed',
      description: 'The scan reached the end of the numeric seed range.',
    }
  }
  return {
    label: 'Completed',
    description: 'Every configured candidate was scanned.',
  }
}

function ResultCard({
  match,
  index,
  disabled,
  onAnalyze,
}: {
  match: SeedSearchMatch
  index: number
  disabled: boolean
  onAnalyze: (match: SeedSearchMatch) => void
}) {
  const canonical = match.seed.code ?? match.seed.formatted

  return (
    <Card>
      <CardHeader>
        <CardTitle className="font-mono">{canonical}</CardTitle>
        <CardDescription>
          Result {index + 1} · numeric seed{' '}
          <span className="text-foreground font-mono">
            {match.seed.numeric}
          </span>
        </CardDescription>
        <CardAction>
          <Badge variant="secondary">
            {match.evidence.length}{' '}
            {match.evidence.length === 1 ? 'match' : 'matches'}
          </Badge>
        </CardAction>
      </CardHeader>
      <CardContent>
        <ItemGroup className="gap-2">
          {match.evidence.map((evidence) => {
            const source = formatItemSource(evidence.source)
            return (
              <Item
                key={`${evidence.constraintIndex}-${evidence.depth}-${evidence.className}`}
                variant="muted"
                size="sm"
              >
                <ItemMedia>
                  <ItemIcon
                    classNameItem={evidence.className}
                    size={24}
                    title={evidence.name}
                  />
                </ItemMedia>
                <ItemContent>
                  <ItemTitle>
                    <ItemName name={evidence.name} />
                  </ItemTitle>
                  <ItemDescription>
                    Floor {evidence.depth}
                    {evidence.level > 0 ? ` · +${evidence.level}` : ''}
                    {source ? ` · ${source}` : ''}
                  </ItemDescription>
                </ItemContent>
                <Badge variant="outline">
                  Item {evidence.constraintIndex + 1}
                </Badge>
              </Item>
            )
          })}
        </ItemGroup>
      </CardContent>
      <CardFooter className="justify-end">
        <Button
          type="button"
          variant="outline"
          disabled={disabled}
          onClick={() => onAnalyze(match)}
        >
          Analyze seed
          <ArrowRightIcon data-icon="inline-end" />
        </Button>
      </CardFooter>
    </Card>
  )
}

export function FinderResults({
  run,
  onAnalyze,
}: {
  run: FinderRunState
  onAnalyze: (match: SeedSearchMatch) => void
}) {
  const elapsed = useElapsedTime(run.startedAt, run.status === 'running')
  if (run.status === 'idle') {
    return (
      <Empty>
        <EmptyHeader>
          <EmptyMedia variant="icon">
            <BinocularsIcon />
          </EmptyMedia>
          <EmptyTitle>No search run yet</EmptyTitle>
          <EmptyDescription>
            Configure at least one item and scan a bounded seed range.
          </EmptyDescription>
        </EmptyHeader>
      </Empty>
    )
  }

  const copy = statusCopy(run)
  const progress =
    run.requestedCandidates > 0
      ? (run.scanned / run.requestedCandidates) * 100
      : 0

  return (
    <div className="flex flex-col gap-4">
      <Card size="sm">
        <CardHeader>
          <CardTitle>{copy.label}</CardTitle>
          <CardDescription>{copy.description}</CardDescription>
          <CardAction>
            <Badge
              variant={run.status === 'error' ? 'destructive' : 'secondary'}
            >
              {run.matches.length}{' '}
              {run.matches.length === 1 ? 'result' : 'results'}
            </Badge>
          </CardAction>
        </CardHeader>
        <CardContent>
          <Progress
            value={progress}
            aria-label={`${run.scanned} of ${run.requestedCandidates} candidates scanned`}
          />
        </CardContent>
        <CardFooter className="justify-between gap-2 text-muted-foreground">
          <span>
            {run.scanned} / {run.requestedCandidates} scanned
          </span>
          <span className="font-mono">Elapsed {formatElapsed(elapsed)}</span>
          {run.currentCandidateSeed !== null ? (
            <span className="font-mono">
              {run.status === 'running' ? 'Current' : 'Last'}:{' '}
              {run.currentCandidateSeed} (candidate{' '}
              {run.currentCandidateNumber ?? run.scanned}) · depth{' '}
              {run.currentDepth ?? '—'}
            </span>
          ) : (
            <span>No candidate evaluated</span>
          )}
        </CardFooter>
      </Card>

      {run.status === 'error' ? (
        <Alert variant="destructive">
          <WarningCircleIcon />
          <AlertTitle>Search failed</AlertTitle>
          <AlertDescription>{run.error}</AlertDescription>
        </Alert>
      ) : null}

      {run.status === 'cancelled' ? (
        <Alert>
          <StopIcon />
          <AlertTitle>Search cancelled</AlertTitle>
          <AlertDescription>
            Stopped after {run.scanned} candidates with {run.matches.length}{' '}
            {run.matches.length === 1 ? 'result' : 'results'} found. Results
            found before cancellation are shown below.
          </AlertDescription>
        </Alert>
      ) : null}

      {run.status === 'completed' && run.matches.length > 0 ? (
        <Alert>
          <CheckCircleIcon />
          <AlertTitle>Search complete</AlertTitle>
          <AlertDescription>{copy.description}</AlertDescription>
        </Alert>
      ) : null}

      {run.matches.length === 0 ? (
        <Empty>
          <EmptyHeader>
            <EmptyMedia variant="icon">
              <BinocularsIcon />
            </EmptyMedia>
            <EmptyTitle>
              {run.status === 'running'
                ? 'Scanning for matches'
                : 'No matches found'}
            </EmptyTitle>
            <EmptyDescription>
              {run.status === 'running'
                ? 'Matching seeds will appear here as each chunk finishes.'
                : 'Try a larger bounded range, another floor window, or ANY mode.'}
            </EmptyDescription>
          </EmptyHeader>
        </Empty>
      ) : (
        <section aria-labelledby="finder-results-heading">
          <div className="mb-3 flex items-center justify-between gap-2">
            <h2 id="finder-results-heading" className="text-sm font-medium">
              Matching seeds
            </h2>
            <Badge variant="outline">Ascending numeric order</Badge>
          </div>
          <div className="grid gap-4 xl:grid-cols-2">
            {run.matches.map((match, index) => (
              <ResultCard
                key={match.seed.numeric}
                match={match}
                index={index}
                disabled={run.status === 'running'}
                onAnalyze={onAnalyze}
              />
            ))}
          </div>
        </section>
      )}
    </div>
  )
}

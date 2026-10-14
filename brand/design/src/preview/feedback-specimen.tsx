import {
  InfoIcon,
  PlantIcon,
  StopIcon,
  WarningIcon,
  XCircleIcon,
} from '@phosphor-icons/react'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import { Kbd } from '@/components/ui/kbd'
import { Progress } from '@/components/ui/progress'
import { REGIONS } from '@/lib/regions'
import { Subsection } from '@/preview/section'

export function FeedbackSpecimen() {
  return (
    <>
      <Subsection
        title="Alerts"
        note="Severity is the left rule and the title color. The description stays in body ink so it reads easily."
      >
        <div className="grid gap-4 border-y py-6 lg:grid-cols-3">
          <Alert variant="info">
            <InfoIcon />
            <AlertTitle>Supports SPD v4.0.2</AlertTitle>
            <AlertDescription>
              Seeds from other versions generate different floors.
            </AlertDescription>
          </Alert>
          <Alert variant="warning">
            <WarningIcon />
            <AlertTitle>Partial report</AlertTitle>
            <AlertDescription>
              Shop stock is not covered yet. Everything else on this seed is.
            </AlertDescription>
          </Alert>
          <Alert variant="destructive">
            <XCircleIcon />
            <AlertTitle>Search stopped</AlertTitle>
            <AlertDescription>
              The tab ran out of memory after 1,204,000 seeds. Search a smaller
              range and try again.
            </AlertDescription>
          </Alert>
        </div>
      </Subsection>
      <Subsection
        title="Badges"
        note="Short facts about one thing. Region badges only ever label floors."
      >
        <div className="flex flex-col gap-4 border-y py-6">
          <div className="flex flex-wrap gap-2">
            <Badge>Notable</Badge>
            <Badge variant="secondary">Daily run</Badge>
            <Badge variant="outline">v4.0.2</Badge>
            <Badge variant="warning">Partial</Badge>
            <Badge variant="destructive">Cursed</Badge>
          </div>
          <div className="flex flex-wrap gap-2">
            {REGIONS.map((region) => (
              <Badge key={region.id} variant={region.id}>
                {region.name} {region.floors[0]}–{region.floors[1]}
              </Badge>
            ))}
          </div>
        </div>
      </Subsection>
      <Subsection
        title="Progress and empty states"
        note="Progress counts real units. An empty screen says what to do next."
      >
        <div className="grid gap-8 border-y py-6 md:grid-cols-2">
          <div className="flex flex-col justify-center gap-3">
            <div className="flex items-baseline justify-between gap-4 text-sm">
              <span className="font-semibold">Searching seeds</span>
              <span className="text-muted-foreground tabular-nums">
                41,250 of 100,000
              </span>
            </div>
            <Progress value={41.25} aria-label="Seeds searched" />
            <div className="flex items-center justify-between gap-4">
              <span className="text-sm text-muted-foreground tabular-nums">
                3 matches so far
              </span>
              <Button variant="destructive" size="sm">
                <StopIcon data-icon="inline-start" />
                Stop search
              </Button>
            </div>
            <p className="mt-2 text-sm text-muted-foreground">
              Press <Kbd>/</Kbd> to jump to the seed field.
            </p>
          </div>
          <Empty className="border">
            <EmptyHeader>
              <EmptyMedia variant="icon">
                <PlantIcon />
              </EmptyMedia>
              <EmptyTitle>No seeds analyzed yet</EmptyTitle>
              <EmptyDescription>
                Enter a seed to see what each floor holds.
              </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
              <Button variant="secondary">Analyze today's daily run</Button>
            </EmptyContent>
          </Empty>
        </div>
      </Subsection>
    </>
  )
}

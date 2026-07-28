import { InfoIcon, WarningIcon } from '@phosphor-icons/react'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import accuracy from '../../../../specs/accuracy.json'

export function AccuracyWarning() {
  return (
    <Alert className="border-amber-200 bg-amber-50 text-amber-900 dark:border-amber-900 dark:bg-amber-950 dark:text-amber-50 pr-2">
      <WarningIcon />
      <AlertTitle>Partial accuracy</AlertTitle>
      <AlertDescription>
        Results may differ from Shattered Pixel Dungeon{' '}
        <Badge variant="outline" className="font-mono text-[10px]">
          {accuracy.target.version}
        </Badge>
      </AlertDescription>
      <Dialog>
        <DialogTrigger asChild>
          <Button
            type="button"
            variant="ghost"
            size="icon-xs"
            aria-label="View accuracy details"
            className="absolute top-1 right-1"
          >
            <InfoIcon />
          </Button>
        </DialogTrigger>
        <DialogContent className="flex max-h-[calc(100svh-2rem)] flex-col sm:max-w-4xl lg:max-w-5xl">
          <DialogHeader className="shrink-0 pr-8">
            <DialogTitle>Accuracy details</DialogTitle>
            <DialogDescription>
              {accuracy.summary} Last reviewed {accuracy.lastReviewed} for{' '}
              {accuracy.target.version}.
            </DialogDescription>
          </DialogHeader>
          <div
            className="min-h-0 overflow-auto ring-1 ring-foreground/10"
            data-testid="accuracy-details-scroll"
          >
            <table className="w-full min-w-xl border-collapse text-left text-xs/relaxed">
              <thead className="bg-muted text-foreground">
                <tr>
                  <th className="px-2 py-1.5 font-medium">Area</th>
                  <th className="px-2 py-1.5 font-medium">Implemented</th>
                  <th className="px-2 py-1.5 font-medium">Remaining</th>
                  <th className="px-2 py-1.5 font-medium">Impact</th>
                  <th className="px-2 py-1.5 font-medium">Status</th>
                </tr>
              </thead>
              <tbody>
                {accuracy.areas.map((area) => (
                  <tr key={area.id} className="border-t">
                    <th className="min-w-28 px-2 py-2 align-top font-medium">
                      {area.area}
                    </th>
                    <td className="min-w-48 px-2 py-2 align-top text-muted-foreground">
                      <ul className="flex list-disc flex-col gap-1 pl-4">
                        {area.implemented.map((item) => (
                          <li key={item}>{item}</li>
                        ))}
                      </ul>
                    </td>
                    <td className="min-w-48 px-2 py-2 align-top text-muted-foreground">
                      {area.remaining.length > 0 ? (
                        <ul className="flex list-disc flex-col gap-1 pl-4">
                          {area.remaining.map((item) => (
                            <li key={item}>{item}</li>
                          ))}
                        </ul>
                      ) : (
                        <span className="text-muted-foreground/60">
                          Nothing known
                        </span>
                      )}
                    </td>
                    <td className="min-w-40 px-2 py-2 align-top text-muted-foreground">
                      {area.impact}
                    </td>
                    <td
                      className={`px-2 py-2 align-top capitalize ${
                        area.status === 'verified'
                          ? 'text-muted-foreground'
                          : 'text-warning'
                      }`}
                    >
                      {area.status.replaceAll('-', ' ')}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <Alert className="max-h-[30svh] shrink-0 overflow-auto">
            <InfoIcon />
            <AlertTitle>{accuracy.outOfScope.title}</AlertTitle>
            <AlertDescription>
              <p>{accuracy.outOfScope.note}</p>
              <ul className="flex list-disc flex-col gap-1 pl-4">
                {accuracy.outOfScope.items.map((item) => (
                  <li key={item}>{item}</li>
                ))}
              </ul>
            </AlertDescription>
          </Alert>
        </DialogContent>
      </Dialog>
    </Alert>
  )
}

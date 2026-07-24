import { InfoIcon, WarningIcon } from '@phosphor-icons/react'
import {
  Alert,
  AlertAction,
  AlertDescription,
  AlertTitle,
} from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import accuracy from '../../../../specs/accuracy.json'

export function AccuracyWarning() {
  return (
    <Alert variant="warning">
      <WarningIcon />
      <AlertTitle>Partial accuracy</AlertTitle>
      <AlertDescription>
        Results may differ from Shattered Pixel Dungeon{' '}
        {accuracy.target.version}.
      </AlertDescription>
      <AlertAction>
        <Popover>
          <PopoverTrigger asChild>
            <Button
              type="button"
              variant="ghost"
              size="icon-xs"
              aria-label="View accuracy details"
            >
              <InfoIcon />
            </Button>
          </PopoverTrigger>
          <PopoverContent
            align="end"
            className="max-h-[min(36rem,calc(100vh-2rem))] w-160 max-w-[calc(100vw-2rem)] overflow-auto"
          >
            <PopoverHeader>
              <PopoverTitle>Accuracy details</PopoverTitle>
              <PopoverDescription>
                {accuracy.summary} Last reviewed {accuracy.lastReviewed} for{' '}
                {accuracy.target.version}.
              </PopoverDescription>
            </PopoverHeader>
            <div className="overflow-x-auto ring-1 ring-foreground/10">
              <table className="w-full min-w-144 border-collapse text-left text-xs/relaxed">
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
                        <ul className="flex list-disc flex-col gap-1 pl-4">
                          {area.remaining.map((item) => (
                            <li key={item}>{item}</li>
                          ))}
                        </ul>
                      </td>
                      <td className="min-w-40 px-2 py-2 align-top text-muted-foreground">
                        {area.impact}
                      </td>
                      <td className="px-2 py-2 align-top text-warning capitalize">
                        {area.status.replaceAll('-', ' ')}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </PopoverContent>
        </Popover>
      </AlertAction>
    </Alert>
  )
}

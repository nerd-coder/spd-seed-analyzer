import { InfoIcon } from 'lucide-react'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemIcon } from '@/components/ItemIcon'
import { ItemName } from '@/components/ItemName'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'

export function TrinketRotationPopover({ sequence }: { sequence: string[] }) {
  if (!sequence.length) return null

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="Show trinket transmutation rotation"
        >
          <InfoIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-72">
        <PopoverHeader>
          <PopoverTitle>Trinket transmutation rotation</PopoverTitle>
          <PopoverDescription>
            Each later transmutation advances to the next trinket in this
            seed-determined order.
          </PopoverDescription>
        </PopoverHeader>
        <ol className="flex max-h-64 list-decimal flex-col gap-1.5 overflow-y-auto pl-5 text-sm">
          {sequence.map((className) => (
            <li key={className} className="pl-1">
              <div className="flex items-center gap-2">
                <ItemIcon
                  classNameItem={className}
                  category="trinket"
                  size={16}
                  title={finderItemLabel(className)}
                />
                <ItemName name={finderItemLabel(className)} />
              </div>
            </li>
          ))}
        </ol>
      </PopoverContent>
    </Popover>
  )
}

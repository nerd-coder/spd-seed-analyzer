import {
  BinocularsIcon,
  MagnifyingGlassIcon,
  MapTrifoldIcon,
  TrashIcon,
} from '@phosphor-icons/react'
import { ItemSlot } from '@/components/spd/item-slot'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Kbd } from '@/components/ui/kbd'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { SAMPLE_SEED } from '@/lib/sample-seed'
import { Subsection } from '@/preview/section'

const BLAZING = {
  sprite: 'longsword',
  name: 'Longsword of Blazing',
  level: 3,
  glow: '#ff4400',
} as const

export function OverlaysSpecimen() {
  return (
    <>
      <Subsection
        title="Tabs"
        note="The tray switches modes; the line variant switches views inside a report."
      >
        <div className="grid gap-8 border-y py-6 md:grid-cols-2">
          <Tabs defaultValue="analyze">
            <TabsList className="w-full">
              <TabsTrigger value="analyze">
                <MagnifyingGlassIcon data-icon="inline-start" />
                Analyze
              </TabsTrigger>
              <TabsTrigger value="find">
                <BinocularsIcon data-icon="inline-start" />
                Find
              </TabsTrigger>
            </TabsList>
            <TabsContent value="analyze" className="text-muted-foreground">
              Read one seed floor by floor.
            </TabsContent>
            <TabsContent value="find" className="text-muted-foreground">
              Search many seeds for the items you want.
            </TabsContent>
          </Tabs>
          <Tabs defaultValue="floors">
            <TabsList variant="line">
              <TabsTrigger value="floors">Floors</TabsTrigger>
              <TabsTrigger value="identities">Identities</TabsTrigger>
              <TabsTrigger value="quests">Quests</TabsTrigger>
            </TabsList>
            <TabsContent value="floors" className="text-muted-foreground">
              Items you can find on each floor.
            </TabsContent>
            <TabsContent value="identities" className="text-muted-foreground">
              Which color, rune or gem each potion, scroll and ring wears.
            </TabsContent>
            <TabsContent value="quests" className="text-muted-foreground">
              Who asks for help, and what they offer.
            </TabsContent>
          </Tabs>
        </div>
      </Subsection>
      <Subsection
        title="Dialogs, popovers and tooltips"
        note="Windows that float cast the hard drop. A dialog asks before something is lost; a popover explains a slot; a tooltip names an icon."
      >
        <div className="flex flex-wrap items-center gap-6 border-y py-6">
          <Dialog>
            <DialogTrigger asChild>
              <Button variant="outline">
                <TrashIcon data-icon="inline-start" />
                Remove seed
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Remove {SAMPLE_SEED}?</DialogTitle>
                <DialogDescription>
                  Its report closes. You can analyze the seed again at any time.
                </DialogDescription>
              </DialogHeader>
              <DialogFooter>
                <DialogClose asChild>
                  <Button variant="outline">Keep seed</Button>
                </DialogClose>
                <DialogClose asChild>
                  <Button variant="destructive">Remove seed</Button>
                </DialogClose>
              </DialogFooter>
            </DialogContent>
          </Dialog>
          <Popover>
            <PopoverTrigger asChild>
              <button
                type="button"
                className="rounded-none outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background"
                aria-label="Details for Longsword of Blazing +3"
              >
                <ItemSlot item={BLAZING} />
              </button>
            </PopoverTrigger>
            <PopoverContent align="start">
              <PopoverHeader>
                <PopoverTitle>
                  Longsword of Blazing <span className="text-tier-3">+3</span>
                </PopoverTitle>
                <PopoverDescription>
                  Sets enemies on fire. On floor 4 of this seed.
                </PopoverDescription>
              </PopoverHeader>
              <div className="flex gap-2">
                <Badge>Notable</Badge>
                <Badge variant="sewers">Sewers 1–5</Badge>
              </div>
            </PopoverContent>
          </Popover>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button variant="secondary" size="icon" aria-label="Open the map">
                <MapTrifoldIcon />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              Open the map <Kbd>M</Kbd>
            </TooltipContent>
          </Tooltip>
        </div>
      </Subsection>
    </>
  )
}

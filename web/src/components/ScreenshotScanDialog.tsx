import {
  ImageIcon,
  SpinnerGapIcon,
  WarningCircleIcon,
} from '@phosphor-icons/react'
import { useStore } from '@tanstack/react-store'
import { useRef, useState } from 'react'
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
} from '@/components/ui/dialog'
import {
  Field,
  FieldContent,
  FieldGroup,
  FieldLabel,
  FieldTitle,
} from '@/components/ui/field'
import { Switch } from '@/components/ui/switch'
import type { ScreenshotSeed } from '@/lib/seed-ocr/read-image'
import {
  $screenshotScan,
  $sessions,
  analyzeScreenshotSeeds,
  dismissScreenshotScan,
  sessionIdFor,
} from '@/stores/app'

/** Reading progress, errors, and the choice of seeds for one screenshot. */
export function ScreenshotScanDialog({
  onPickAnother,
}: {
  onPickAnother: () => void
}) {
  const scan = useStore($screenshotScan)
  // Keep the last content on screen while the dialog animates closed.
  const lastScan = useRef(scan)
  if (scan) lastScan.current = scan
  const shown = scan ?? lastScan.current

  return (
    <Dialog
      open={scan !== null}
      onOpenChange={(open) => {
        if (!open) dismissScreenshotScan()
      }}
    >
      <DialogContent
        aria-busy={shown?.status === 'reading'}
        showCloseButton={shown?.status !== 'dropping'}
      >
        {shown?.status === 'dropping' ? <DropTarget /> : null}
        {shown?.status === 'reading' ? <Reading /> : null}
        {shown?.status === 'error' ? (
          <ScanError
            title={shown.title}
            message={shown.message}
            onPickAnother={onPickAnother}
          />
        ) : null}
        {shown?.status === 'found' ? <SeedPicker seeds={shown.seeds} /> : null}
      </DialogContent>
    </Dialog>
  )
}

function DropTarget() {
  return (
    <>
      <DialogHeader className="pr-0">
        <DialogTitle>Drop screenshot</DialogTitle>
        <DialogDescription>
          Release anywhere on the page to read the seed codes in it.
        </DialogDescription>
      </DialogHeader>
      <div className="flex flex-col items-center gap-2 rounded-sm border-2 border-dashed border-ring bg-muted/50 py-10 text-muted-foreground">
        <ImageIcon className="size-10" aria-hidden />
        <span className="text-sm">PNG or JPEG of the hero stats window</span>
      </div>
    </>
  )
}

function Reading() {
  return (
    <>
      <DialogHeader>
        <DialogTitle>Reading screenshot</DialogTitle>
        <DialogDescription>
          Looking for seed codes in the image.
        </DialogDescription>
      </DialogHeader>
      <div className="flex justify-center py-4" role="status">
        <SpinnerGapIcon className="size-8 animate-spin text-muted-foreground" />
        <span className="sr-only">Reading screenshot</span>
      </div>
      <DialogFooter>
        <DialogClose asChild>
          <Button type="button" variant="outline">
            Cancel
          </Button>
        </DialogClose>
      </DialogFooter>
    </>
  )
}

function ScanError({
  title,
  message,
  onPickAnother,
}: {
  title: string
  message: string
  onPickAnother: () => void
}) {
  return (
    <>
      <DialogHeader>
        <DialogTitle className="flex items-center gap-2 text-destructive">
          <WarningCircleIcon className="size-5 shrink-0" aria-hidden />
          {title}
        </DialogTitle>
        <DialogDescription>{message}</DialogDescription>
      </DialogHeader>
      <DialogFooter>
        <DialogClose asChild>
          <Button type="button" variant="outline">
            Close
          </Button>
        </DialogClose>
        <Button type="button" onClick={onPickAnother}>
          <ImageIcon data-icon="inline-start" />
          Try another screenshot
        </Button>
      </DialogFooter>
    </>
  )
}

function SeedPicker({ seeds }: { seeds: ScreenshotSeed[] }) {
  const sessions = useStore($sessions)
  const [selected, setSelected] = useState(
    () => new Set(seeds.map((seed) => seed.code))
  )
  const openIds = new Set(sessions.map((session) => session.id))
  const chosen = seeds
    .map((seed) => seed.code)
    .filter((code) => selected.has(code))

  function toggle(code: string, on: boolean) {
    setSelected((current) => {
      const next = new Set(current)
      if (on) next.add(code)
      else next.delete(code)
      return next
    })
  }

  return (
    <>
      <DialogHeader>
        <DialogTitle>
          {seeds.length === 1 ? 'Seed found' : `${seeds.length} seeds found`}
        </DialogTitle>
        <DialogDescription>
          Check each code against the screenshot, then choose which to analyze.
        </DialogDescription>
      </DialogHeader>
      <FieldGroup className="max-h-[50svh] gap-2 overflow-y-auto">
        {seeds.map((seed) => {
          const id = `screenshot-seed-${seed.code}`
          return (
            <FieldLabel key={seed.code} htmlFor={id}>
              <Field orientation="horizontal">
                <FieldContent className="min-w-0 gap-2">
                  <FieldTitle className="font-mono">
                    {seed.code}
                    {openIds.has(sessionIdFor(seed.code)) ? (
                      <Badge variant="secondary">Open</Badge>
                    ) : null}
                  </FieldTitle>
                  {seed.preview ? (
                    <img
                      src={seed.preview}
                      alt={`${seed.code} as shown in the screenshot`}
                      className="h-9 w-auto max-w-full self-start rounded-none object-contain object-left"
                    />
                  ) : null}
                </FieldContent>
                <Switch
                  id={id}
                  checked={selected.has(seed.code)}
                  onCheckedChange={(on) => toggle(seed.code, on)}
                />
              </Field>
            </FieldLabel>
          )
        })}
      </FieldGroup>
      <DialogFooter>
        <DialogClose asChild>
          <Button type="button" variant="outline">
            Cancel
          </Button>
        </DialogClose>
        <Button
          type="button"
          disabled={chosen.length === 0}
          onClick={() => void analyzeScreenshotSeeds(chosen)}
        >
          {chosen.length > 1
            ? `Analyze ${chosen.length} seeds`
            : 'Analyze seed'}
        </Button>
      </DialogFooter>
    </>
  )
}

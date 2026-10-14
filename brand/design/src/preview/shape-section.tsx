import { Sprite } from '@/components/spd/sprite'
import { SPRITES } from '@/lib/sprites'
import { cn } from '@/lib/utils'
import { Section, Subsection } from '@/preview/section'

const RADII = [
  {
    name: 'Pixel surface',
    value: '0',
    cls: 'rounded-none bg-muted inset-shadow-recessed',
    use: 'Slots, sprites, maps, region fills. Art lives on a square grid.',
  },
  {
    name: 'Control',
    value: '2px, rounded-sm',
    cls: 'rounded-sm bg-secondary inset-shadow-raised',
    use: 'Buttons, inputs, toggles, badges, tooltips.',
  },
  {
    name: 'Window',
    value: '4px, rounded-lg',
    cls: 'rounded-lg border bg-popover inset-shadow-raised shadow-window',
    use: 'Cards, dialogs, popovers: anything that stands apart.',
  },
] as const

const BEVELS = [
  {
    name: 'Raised',
    token: 'inset-shadow-raised',
    cls: 'bg-secondary inset-shadow-raised',
    use: 'Buttons at rest, active tab, region fills',
  },
  {
    name: 'Pressed',
    token: 'inset-shadow-pressed',
    cls: 'bg-accent inset-shadow-pressed',
    use: 'Buttons while held, toggles that are on',
  },
  {
    name: 'Recessed',
    token: 'inset-shadow-recessed',
    cls: 'bg-background border border-input inset-shadow-recessed',
    use: 'Inputs, slots, tab trays, gauges',
  },
  {
    name: 'Lifted',
    token: 'shadow-window',
    cls: 'bg-popover border shadow-window',
    use: 'Overlays only: dialogs, popovers, tooltips',
  },
] as const

export function ShapeSection() {
  return (
    <Section
      id="shape"
      intro="Surfaces are carved, not floated. A 1px highlight and a 2px base shadow say raised; the reverse says pressed or recessed. Only overlays cast a shadow, and it is a hard 4px drop, the way the game draws depth."
    >
      <Subsection
        title="Corners"
        note="Radius follows the role of a surface, not a single global value."
      >
        <ul className="grid gap-6 sm:grid-cols-3">
          {RADII.map((radius) => (
            <li key={radius.name} className="flex flex-col gap-3">
              <div className={cn('h-20', radius.cls)} />
              <div className="text-sm">
                <p className="font-semibold">
                  {radius.name}{' '}
                  <span className="font-normal text-muted-foreground">
                    {radius.value}
                  </span>
                </p>
                <p className="text-muted-foreground">{radius.use}</p>
              </div>
            </li>
          ))}
        </ul>
      </Subsection>
      <Subsection
        title="Bevels"
        note="Defined as Tailwind inset-shadow tokens, so they compose with focus rings."
      >
        <ul className="grid grid-cols-2 gap-6 lg:grid-cols-4">
          {BEVELS.map((bevel) => (
            <li key={bevel.name} className="flex flex-col gap-3">
              <div className={cn('h-20 rounded-sm', bevel.cls)} />
              <div className="text-sm">
                <p className="font-semibold">{bevel.name}</p>
                <code className="font-mono text-xs text-muted-foreground">
                  {bevel.token}
                </code>
                <p className="text-muted-foreground">{bevel.use}</p>
              </div>
            </li>
          ))}
        </ul>
      </Subsection>
      <Subsection
        title="Pixel scale"
        note="Sprites and tiles scale by whole numbers with smoothing off: 2× in lists, 3× for quest givers and detail views. A fractional scale blurs the art."
      >
        <div className="flex flex-wrap items-end gap-8">
          {[1, 2, 3].map((scale) => (
            <figure key={scale} className="flex flex-col items-center gap-2">
              <Sprite sprite={SPRITES.ghost} scale={scale} label="Sad Ghost" />
              <figcaption className="text-sm font-semibold">
                {scale}×
              </figcaption>
            </figure>
          ))}
          <figure className="flex flex-col items-center gap-2">
            <span
              role="img"
              aria-label="Sad Ghost, smoothed at 2.5 times"
              className="inline-block"
              style={{
                width: 14 * 2.5,
                height: 15 * 2.5,
                backgroundImage: `url(${SPRITES.ghost.url})`,
                backgroundSize: `${128 * 2.5}px ${16 * 2.5}px`,
                imageRendering: 'auto',
              }}
            />
            <figcaption className="text-sm font-semibold text-destructive">
              2.5×, smoothed
            </figcaption>
          </figure>
        </div>
      </Subsection>
    </Section>
  )
}

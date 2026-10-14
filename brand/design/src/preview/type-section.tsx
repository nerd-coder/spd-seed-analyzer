import { Section, Subsection } from '@/preview/section'

const PIXEL_STEPS = [
  {
    cls: 'text-px-4',
    sampleCls: 'text-px-2 md:text-px-4',
    step: '×4',
    size: '81.92px',
    use: 'The seed on a report',
    sample: 'ABC-DEF-GHI',
  },
  {
    cls: 'text-px-3',
    sampleCls: 'text-px-2 md:text-px-3',
    step: '×3',
    size: '61.44px',
    use: 'Page titles',
    sample: 'Demon Halls',
  },
  {
    cls: 'text-px-2',
    sampleCls: 'text-px-2',
    step: '×2',
    size: '40.96px',
    use: 'Section titles, depth numbers',
    sample: 'Floor 26',
  },
  {
    cls: 'text-px-1',
    sampleCls: 'text-px-1',
    step: '×1',
    size: '20.48px',
    use: 'Subsections, dialog titles, +N on slots',
    sample: 'Ring of Wealth +2',
  },
] as const

const BODY_STEPS = [
  { cls: 'text-lg', size: '18 / 28', use: 'Introductions' },
  { cls: 'text-base', size: '16 / 24', use: 'Reading text' },
  { cls: 'text-sm', size: '14 / 20', use: 'Controls, item groups, tables' },
  { cls: 'text-xs', size: '12 / 16', use: 'Captions under slots' },
] as const

export function TypeSection() {
  return (
    <Section
      id="type"
      intro="Two voices. The game's own pixel face names places and numbers. Atkinson Hyperlegible Next carries everything you read, because seed codes and item names must never confuse I with 1 or O with 0."
    >
      <Subsection
        title="SPD Pixel"
        note="The font the game ships, drawn on a grid of 100 units in a 2048-unit em. It renders crisp only at whole multiples of 20.48px, so these four sizes are the whole scale. Never set it smaller, never add letter spacing."
      >
        <div className="flex flex-col divide-y border-y">
          {PIXEL_STEPS.map((step) => (
            <div
              key={step.step}
              className="flex flex-col gap-2 py-4 md:flex-row md:items-baseline md:gap-8"
            >
              <div className="flex shrink-0 flex-col text-sm md:w-48">
                <code className="font-mono">{step.cls}</code>
                <span className="text-muted-foreground">
                  {step.step}, {step.size}
                </span>
                <span className="text-muted-foreground">{step.use}</span>
              </div>
              <p className={`min-w-0 truncate font-pixel ${step.sampleCls}`}>
                {step.sample}
              </p>
            </div>
          ))}
        </div>
      </Subsection>
      <Subsection
        title="Atkinson Hyperlegible Next"
        note="Designed by the Braille Institute so that similar letters stay distinct. Body text sits on the classical scale; controls use 14px, never smaller."
      >
        <div className="grid gap-8 md:grid-cols-[1fr_16rem]">
          <div className="flex flex-col divide-y border-y">
            {BODY_STEPS.map((step) => (
              <div
                key={step.cls}
                className="flex flex-col gap-1 py-3 sm:flex-row sm:items-baseline sm:gap-6"
              >
                <span className="w-40 shrink-0 text-sm text-muted-foreground tabular-nums">
                  <code className="font-mono">{step.cls}</code> {step.size}
                </span>
                <p className={step.cls}>
                  The Sad Ghost on floor 3 offers a weapon or armor. {step.use}.
                </p>
              </div>
            ))}
          </div>
          <dl className="grid grid-cols-2 content-start gap-x-4 gap-y-3 border-y py-3">
            <dt className="text-sm text-muted-foreground">Ambiguous pairs</dt>
            <dd className="text-xl font-semibold tracking-wide">Il1 O0 5S</dd>
            <dt className="text-sm text-muted-foreground">Weights</dt>
            <dd className="flex gap-3 text-lg">
              <span className="font-normal">Aa</span>
              <span className="font-semibold">Aa</span>
              <span className="font-extrabold">Aa</span>
            </dd>
            <dt className="text-sm text-muted-foreground">Counts</dt>
            <dd className="text-lg tabular-nums">41,250</dd>
          </dl>
        </div>
      </Subsection>
      <Subsection
        title="Atkinson Hyperlegible Mono"
        note="Only for seed codes and dates someone types or copies: fixed width keeps the ABC-DEF-GHI mask steady while it fills in."
      >
        <p className="font-mono text-xl">
          ABC-DEF-GHI <span className="text-muted-foreground">2026-10-14</span>
        </p>
      </Subsection>
    </Section>
  )
}

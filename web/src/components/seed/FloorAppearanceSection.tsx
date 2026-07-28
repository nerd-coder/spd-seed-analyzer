import { CookingPotIcon } from '@phosphor-icons/react'

import { Badge } from '@/components/ui/badge'
import { formatItemSource } from '@/lib/labels'
import type { GuaranteedAppearance } from '@/lib/spd-wasm'

export function FloorAppearanceSection({
  appearances,
}: {
  appearances?: GuaranteedAppearance[]
}) {
  if (!appearances?.length) return null

  return (
    <div className="flex flex-col gap-1">
      <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
        Guaranteed appearances
        <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
          ({appearances.length})
        </span>
      </p>
      <ul className="flex flex-col gap-1.5 text-sm">
        {appearances.map((appearance, index) => {
          const source = formatItemSource(appearance.source)
          return (
            <li
              key={`${appearance.kind}-${appearance.source ?? index}`}
              className="flex items-center gap-2"
            >
              <CookingPotIcon className="size-4 shrink-0" aria-hidden />
              <span>{appearance.name}</span>
              {source ? <Badge variant="secondary">{source}</Badge> : null}
            </li>
          )
        })}
      </ul>
    </div>
  )
}

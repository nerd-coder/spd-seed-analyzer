import { MapTrifoldIcon } from '@phosphor-icons/react'
import type { ReactNode } from 'react'
import { FloorBanner } from '@/components/spd/floor-banner'
import { ItemSlot } from '@/components/spd/item-slot'
import { Sprite } from '@/components/spd/sprite'
import { Button } from '@/components/ui/button'
import type { SampleFloor } from '@/lib/sample-seed'
import { SPRITES } from '@/lib/sprites'

/** A labelled run of slots. The count is a fact, so it is shown, not decorated. */
export function ItemGroup({
  label,
  count,
  empty,
  children,
}: {
  label: string
  count?: number
  /** Shown instead of slots when the group has none. */
  empty?: string
  children?: ReactNode
}) {
  return (
    <section className="flex flex-col gap-2.5">
      <h4 className="flex items-baseline gap-2 text-sm font-semibold">
        {label}
        {count != null ? (
          <span className="font-normal text-muted-foreground tabular-nums">
            {count}
          </span>
        ) : null}
      </h4>
      {children ? (
        <div className="flex flex-wrap gap-2">{children}</div>
      ) : (
        <p className="text-sm text-muted-foreground">{empty}</p>
      )}
    </section>
  )
}

/** One floor of a seed report: banner, then quest, notable and other items. */
export function FloorReport({ floor }: { floor: SampleFloor }) {
  const { quest } = floor
  return (
    <article className="flex flex-col">
      <FloorBanner
        depth={floor.depth}
        action={
          <Button
            variant="secondary"
            size="icon-sm"
            aria-label={`Open the map of floor ${floor.depth}`}
          >
            <MapTrifoldIcon />
          </Button>
        }
      />
      <div className="flex flex-col gap-5 px-4 py-5">
        {quest ? (
          <section className="flex flex-wrap items-center gap-x-6 gap-y-3 border-l-4 border-l-primary bg-accent/50 py-3 pr-3 pl-4">
            <div className="flex items-center gap-3">
              <Sprite sprite={SPRITES[quest.giver]} scale={3} />
              <div>
                <h4 className="font-semibold">{quest.name}</h4>
                <p className="text-sm text-muted-foreground">
                  Reward choices for this seed
                </p>
              </div>
            </div>
            <div className="flex flex-wrap gap-2">
              {quest.rewards.map((item) => (
                <ItemSlot key={item.name} item={item} />
              ))}
            </div>
          </section>
        ) : null}
        <ItemGroup
          label="Notable"
          count={floor.notable.length}
          empty="No notable items on this floor."
        >
          {floor.notable.length > 0
            ? floor.notable.map((item) => (
                <ItemSlot key={item.name} item={item} />
              ))
            : null}
        </ItemGroup>
        <ItemGroup label="Other items" count={floor.other.length}>
          {floor.other.map((item) => (
            <ItemSlot key={item.name} item={item} />
          ))}
        </ItemGroup>
      </div>
    </article>
  )
}

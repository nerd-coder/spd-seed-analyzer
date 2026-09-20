import { InfoIcon } from 'lucide-react'
import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemIcon } from '@/components/ItemIcon'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Progress } from '@/components/ui/progress'
import type { ImpLevelRule, ImpRewardSlot, ImpSlotRole } from '@/lib/spd-wasm'

const ROLE_LABEL: Record<ImpSlotRole, string> = {
  artifact: 'Artifact',
  ring: 'Ring',
  weapon: 'Melee weapon',
  missile: 'Missile weapon',
  armor: 'Armor',
  wand: 'Wand',
}

function levelText(rule: ImpLevelRule): string {
  return rule.type === 'range'
    ? `+${rule.min}…+${rule.max}`
    : `+${rule.transfer} transferred`
}

/** `+1`, `−2`, or `baseline` for the fresh/no-history draw. */
function driftText(drifts: number[]): string {
  const parts = drifts.map((drift) =>
    drift === 0 ? 'baseline' : `${drift > 0 ? '+' : '−'}${Math.abs(drift)}`
  )
  return parts.join(' / ')
}

function SlotRow({
  slot,
  upgradesPinned,
}: {
  slot: ImpRewardSlot
  upgradesPinned: boolean
}) {
  const candidates = slot.candidates ?? []
  const fixed = slot.fixed_class
  // The artifact slot always transfers the same amount; only its cap-scaled
  // stored level differs by class, so show the transfer instead.
  const upgrade =
    slot.level_rule.type === 'transfer_upgrade'
      ? `+${slot.level_rule.transfer}`
      : `+${slot.baseline_level}`

  return (
    <li className="flex flex-col gap-1 border-t pt-2 first:border-t-0 first:pt-0">
      <div className="flex flex-wrap items-baseline gap-x-2 gap-y-0.5">
        <span className="text-xs font-medium tracking-wide uppercase">
          {ROLE_LABEL[slot.role]}
        </span>
        <span
          className="font-mono text-sm font-medium tabular-nums"
          title={
            upgradesPinned
              ? `Pinned by the seed. Rolled from ${levelText(slot.level_rule)}.`
              : `Rolled from ${levelText(slot.level_rule)}.`
          }
        >
          {upgrade}
        </span>
        <span className="font-mono text-xs text-muted-foreground tabular-nums">
          of {levelText(slot.level_rule)}
        </span>
        {slot.baseline_enchantment ? (
          <Badge variant="secondary">{slot.baseline_enchantment}</Badge>
        ) : slot.enchanted ? (
          <Badge variant="secondary">
            {slot.role === 'armor' ? 'glyphed' : 'enchanted'}
          </Badge>
        ) : null}
        {fixed ? <Badge variant="outline">always</Badge> : null}
      </div>

      {fixed ? (
        <div className="flex items-center gap-2">
          <ItemIcon
            classNameItem={fixed}
            category={slot.role}
            size={16}
            title={finderItemLabel(fixed)}
          />
          <span className="text-sm">{finderItemLabel(fixed)}</span>
          <span className="font-mono text-xs text-muted-foreground tabular-nums">
            100%
          </span>
        </div>
      ) : (
        <ul className="flex flex-col gap-1">
          {candidates.map((candidate) => {
            const share = (100 * candidate.weight) / slot.scenario_count
            const isBaseline = candidate.drifts.includes(0)
            return (
              <li
                key={candidate.class_name}
                className="grid grid-cols-[16px_minmax(0,1fr)_auto] items-center gap-x-2"
              >
                <ItemIcon
                  classNameItem={candidate.class_name}
                  category={slot.role}
                  size={16}
                  title={finderItemLabel(candidate.class_name)}
                />
                <span className="flex min-w-0 flex-col gap-0.5">
                  <span className="flex min-w-0 flex-wrap items-baseline gap-1.5">
                    <span
                      className={
                        isBaseline
                          ? 'truncate text-sm font-medium'
                          : 'truncate text-sm'
                      }
                    >
                      {finderItemLabel(candidate.class_name)}
                    </span>
                    {isBaseline ? (
                      <Badge variant="outline">fresh run</Badge>
                    ) : null}
                  </span>
                  <Progress value={share} className="h-1" />
                </span>
                <span className="flex items-baseline gap-1.5 font-mono text-xs tabular-nums">
                  <span>{share.toFixed(0)}%</span>
                  <span className="text-muted-foreground">
                    {driftText(candidate.drifts)}
                  </span>
                </span>
              </li>
            )
          })}
        </ul>
      )}
    </li>
  )
}

/**
 * Per-slot projection for the Imp's six take-out options.
 *
 * The percentages are the share of an enumerated deck-drift window, not an
 * observed player frequency: a category's draw order is fixed by the seed, so
 * run history only shifts which index the Imp lands on. Read them as how stable
 * a slot is against that shift. Only the level range, the enchant flag, and the
 * plate armor slot are seed-independent guarantees.
 */
export function ImpRewardDistribution({
  slots,
  upgradesPinned = true,
  totalUpgradeValue,
}: {
  slots: ImpRewardSlot[]
  upgradesPinned?: boolean
  totalUpgradeValue?: number
}) {
  if (slots.length === 0) return null
  const window = Math.max(
    ...slots.flatMap((slot) =>
      (slot.candidates ?? []).flatMap((candidate) =>
        candidate.drifts.map((drift) => Math.abs(drift))
      )
    ),
    0
  )

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="Per-slot reward outlook"
        >
          <InfoIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-96">
        <PopoverHeader>
          <PopoverTitle>Per-slot outlook</PopoverTitle>
          <PopoverDescription>
            Each category draws in a fixed order set by the seed, so earlier
            play only changes which index the Imp lands on. These shares cover a
            ±{window} shift either way — they measure how stable a slot is, not
            how often players see it. A fresh run gets the “fresh run” row.
          </PopoverDescription>
          <PopoverDescription>
            {upgradesPinned ? (
              <>
                The <strong>upgrades and enchants are fixed by the seed</strong>
                {typeof totalUpgradeValue === 'number' ? (
                  <> — {totalUpgradeValue} upgrades across the six options</>
                ) : null}
                . Every class in a category costs the same RNG, so drift changes
                which item a slot holds, never its +N.
              </>
            ) : (
              <>
                Upgrades are fixed by the seed <em>unless</em> the artifact slot
                lands on the Unstable Spellbook, which is reachable here: its
                constructor consumes extra RNG and shifts every later +N and
                enchant.
              </>
            )}
          </PopoverDescription>
        </PopoverHeader>
        <ul className="flex max-h-80 flex-col gap-2 overflow-y-auto">
          {slots.map((slot) => (
            <SlotRow
              key={slot.slot}
              slot={slot}
              upgradesPinned={upgradesPinned}
            />
          ))}
        </ul>
      </PopoverContent>
    </Popover>
  )
}

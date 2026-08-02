import { DiceFiveIcon, PawPrintIcon } from '@phosphor-icons/react'

import { ItemIcon } from '@/components/ItemIcon'
import { ItemName } from '@/components/ItemName'
import { Badge } from '@/components/ui/badge'
import {
  Item,
  ItemContent,
  ItemDescription,
  ItemGroup,
  ItemMedia,
  ItemTitle,
} from '@/components/ui/item'
import { itemAppearance } from '@/lib/identity'
import type {
  CombatReward,
  IdentityMaps,
  InitialEncounter,
} from '@/lib/spd-wasm'

function rewardText(reward: CombatReward) {
  if (reward.prediction === 'runtime_chance' && reward.chance) {
    const percentage =
      (100 * reward.chance.numerator) / reward.chance.denominator
    return `${percentage}% runtime chance: ${reward.name}`
  }
  if (reward.prediction === 'generated_with_floor') {
    return `Carries ${reward.name}`
  }
  return `Drops ${reward.name}`
}

function rewardBadge(reward: CombatReward) {
  if (reward.prediction === 'runtime_chance') {
    return (
      <Badge variant="outline">
        <DiceFiveIcon data-icon="inline-start" />
        Runtime roll
      </Badge>
    )
  }
  if (reward.prediction === 'generated_with_floor') {
    return <Badge variant="secondary">Generated with floor</Badge>
  }
  return <Badge variant="secondary">Guaranteed on defeat</Badge>
}

function EncounterReward({
  reward,
  identities,
}: {
  reward: CombatReward
  identities: IdentityMaps
}) {
  const item = { category: reward.category, class_name: reward.class_name }
  return (
    <ItemDescription className="line-clamp-none flex flex-wrap items-center gap-1.5">
      <ItemIcon
        classNameItem={reward.class_name}
        category={reward.category}
        appearance={itemAppearance(item, identities)}
        size={16}
        title={reward.name}
      />
      <span>{rewardText(reward)}</span>
      {rewardBadge(reward)}
    </ItemDescription>
  )
}

export function FloorEncounterSection({
  encounters,
  identities,
}: {
  encounters?: InitialEncounter[]
  identities: IdentityMaps
}) {
  if (!encounters?.length) return null

  const entityCount = encounters.reduce(
    (total, encounter) => total + encounter.quantity,
    0
  )

  return (
    <div className="flex flex-col gap-1.5">
      <div className="flex flex-col gap-0.5">
        <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
          Initial encounters
          <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
            ({entityCount})
          </span>
        </p>
        <p className="text-muted-foreground text-xs">
          Generated with this floor. Runtime summons and bonus drops are not
          predicted.
        </p>
      </div>
      <ItemGroup className="gap-1.5">
        {encounters.map((encounter) => (
          <Item key={encounter.class} variant="muted" size="xs">
            <ItemMedia variant="icon">
              <PawPrintIcon aria-hidden />
            </ItemMedia>
            <ItemContent>
              <ItemTitle className="flex-wrap">
                <ItemName name={encounter.name} />
                {encounter.quantity > 1 ? (
                  <span className="font-mono text-muted-foreground tabular-nums">
                    x{encounter.quantity}
                  </span>
                ) : null}
              </ItemTitle>
              {encounter.combat_rewards?.length ? (
                encounter.combat_rewards.map((reward, index) => (
                  <EncounterReward
                    key={`${reward.class_name ?? reward.name}-${index}`}
                    reward={reward}
                    identities={identities}
                  />
                ))
              ) : (
                <ItemDescription>No seed-determined base drop</ItemDescription>
              )}
            </ItemContent>
          </Item>
        ))}
      </ItemGroup>
    </div>
  )
}

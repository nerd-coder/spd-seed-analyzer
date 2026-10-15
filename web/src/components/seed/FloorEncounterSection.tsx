import { DiceFiveIcon, PawPrintIcon } from '@phosphor-icons/react'

import { ItemBox } from '@/components/ItemBox'
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
import { itemSprite, parseItemEnhancement } from '@/lib/item-kind'
import {
  enchantedTitle,
  enchantmentName,
  mobLabel,
  variantClassName,
  variantTitle,
} from '@/lib/item-title'
import type {
  CombatReward,
  IdentityMaps,
  InitialEncounter,
} from '@/lib/spd-wasm'

function rewardTitle(reward: CombatReward) {
  return enchantedTitle(
    variantTitle({ ...reward, label: `random ${reward.category}` }),
    enchantmentName(reward.enchantment)
  )
}

function rewardName(reward: CombatReward) {
  const title = rewardTitle(reward)
  return reward.cursed ? `cursed ${title}` : title
}

function rewardText(reward: CombatReward) {
  const name = rewardName(reward)
  if (reward.prediction === 'runtime_chance' && reward.chance) {
    const percentage =
      (100 * reward.chance.numerator) / reward.chance.denominator
    return `${percentage}% runtime chance: ${name}`
  }
  if (reward.prediction === 'generated_with_floor') {
    return `Carries ${name}`
  }
  return `Drops ${name}`
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
  const className = variantClassName(reward)
  const item = { category: reward.category, class_name: className }
  return (
    <ItemDescription className="line-clamp-none flex flex-wrap items-center gap-1.5">
      <ItemBox
        scale={1}
        sprites={itemSprite({
          className,
          category: reward.category,
          appearance: itemAppearance(item, identities),
        })}
        enhancement={parseItemEnhancement(enchantmentName(reward.enchantment))}
        cursed={reward.cursed === true}
        level={reward.level ?? null}
        title={rewardTitle(reward)}
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
        <p className="text-sm font-semibold">
          Initial encounters{' '}
          <span className="text-muted-foreground ml-1 font-normal tabular-nums">
            {entityCount}
          </span>
        </p>
        <p className="text-muted-foreground text-sm">
          Generated with this floor. Runtime summons and bonus drops are not
          predicted.
        </p>
      </div>
      <ItemGroup className="gap-1.5">
        {encounters.map((encounter) => (
          <Item key={encounter.mob} variant="muted" size="xs">
            <ItemMedia variant="icon">
              <PawPrintIcon aria-hidden />
            </ItemMedia>
            <ItemContent>
              <ItemTitle className="flex-wrap">
                <ItemName name={mobLabel(encounter.mob)} />
                {encounter.quantity > 1 ? (
                  <span className="text-muted-foreground tabular-nums">
                    x{encounter.quantity}
                  </span>
                ) : null}
              </ItemTitle>
              {encounter.combat_rewards?.length ? (
                encounter.combat_rewards.map((reward, index) => (
                  <EncounterReward
                    key={`${reward.item ?? reward.category}-${index}`}
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

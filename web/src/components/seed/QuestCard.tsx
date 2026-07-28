import { WarningIcon } from '@phosphor-icons/react'
import { ItemIcon } from '@/components/ItemIcon'
import { ItemName } from '@/components/ItemName'
import { FloorItemList } from '@/components/seed/FloorItemSections'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { type ParsedQuest, parseQuest } from '@/lib/labels'
import type { IdentityMaps, ItemEntry } from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

const QUEST_KIND_STYLES: Record<
  ParsedQuest['kind'],
  { badge: string; border: string }
> = {
  ghost: {
    badge: 'bg-sky-500/15 text-sky-800 dark:text-sky-200 border-sky-500/30',
    border: 'border-sky-500/25 bg-sky-500/5',
  },
  wandmaker: {
    badge:
      'bg-violet-500/15 text-violet-800 dark:text-violet-200 border-violet-500/30',
    border: 'border-violet-500/25 bg-violet-500/5',
  },
  blacksmith: {
    badge:
      'bg-amber-500/15 text-amber-900 dark:text-amber-200 border-amber-500/30',
    border: 'border-amber-500/25 bg-amber-500/5',
  },
  imp: {
    badge: 'bg-rose-500/15 text-rose-800 dark:text-rose-200 border-rose-500/30',
    border: 'border-rose-500/25 bg-rose-500/5',
  },
  other: {
    badge: '',
    border: 'border-border bg-muted/40',
  },
}

export function QuestCard({
  quest,
  rewards,
  identities,
  depth,
}: {
  quest: string
  rewards: ItemEntry[]
  identities: IdentityMaps
  depth: number
}) {
  const parsed = parseQuest(quest)
  const styles = QUEST_KIND_STYLES[parsed.kind]
  const hasDetailedRewards = rewards.length > 0
  const hasUnruledOutGhostOptions = parsed.kind === 'ghost'
  const rewardSummary =
    parsed.kind === 'imp' &&
    rewards.every((reward) => !reward.candidate_classes?.length)
      ? null
      : parsed.rewards
  return (
    <div
      className={cn(
        'space-y-1.5 rounded-none border px-3 py-2.5',
        styles.border
      )}
    >
      <div className="flex flex-wrap items-center gap-2">
        <Badge variant="outline" className={cn('font-medium', styles.badge)}>
          {parsed.title}
        </Badge>
        {parsed.detail && (
          <span className="text-muted-foreground text-xs">{parsed.detail}</span>
        )}
      </div>
      {parsed.rewards || hasDetailedRewards ? (
        <div className="flex flex-col gap-1 border-t pt-2">
          <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
            Rewards
          </p>
          {rewardSummary ? (
            <p className="text-sm leading-snug">
              {parsed.kind === 'blacksmith' &&
              rewardSummary ===
                'You will get access to those items if you select Smith' ? (
                <span>
                  You will get access to those items if you select{' '}
                  <strong>Smith</strong>
                </span>
              ) : parsed.kind === 'imp' ? (
                <span>
                  You’ll get the first option by default. It may change if{' '}
                  <span className="inline-flex items-center gap-1 align-middle">
                    <ItemIcon
                      classNameItem="MimicTooth"
                      size={16}
                      title="Mimic Tooth"
                    />
                    Mimic Tooth
                  </span>{' '}
                  is in your inventory or if every artifact has already
                  appeared.
                </span>
              ) : (
                <ItemName name={rewardSummary} />
              )}
            </p>
          ) : null}
          {hasDetailedRewards ? (
            <FloorItemList
              items={rewards}
              identities={identities}
              depth={depth}
            />
          ) : null}
          {hasUnruledOutGhostOptions ? (
            <Alert variant="warning">
              <WarningIcon weight="fill" />
              <AlertTitle>Other Ghost options may be possible</AlertTitle>
              <AlertDescription>
                The listed pair is the analyzer’s baseline. Earlier trinket
                choices or artifact history can change generation before the
                Ghost reward is rolled.
              </AlertDescription>
            </Alert>
          ) : null}
        </div>
      ) : null}
      {!parsed.rewards && !hasDetailedRewards && (
        <p className="text-muted-foreground text-xs">
          <ItemName name={parsed.raw} />
        </p>
      )}
    </div>
  )
}

import { Badge } from '@/components/ui/badge'
import { type ParsedQuest, parseQuest } from '@/lib/labels'
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

export function QuestCard({ quest }: { quest: string }) {
  const parsed = parseQuest(quest)
  const styles = QUEST_KIND_STYLES[parsed.kind]
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
      {parsed.rewards && (
        <p className="text-sm leading-snug">
          <span className="text-muted-foreground mr-1.5 text-xs font-medium tracking-wide uppercase">
            Rewards
          </span>
          {parsed.rewards}
        </p>
      )}
      {!parsed.rewards && (
        <p className="text-muted-foreground text-xs">{parsed.raw}</p>
      )}
    </div>
  )
}

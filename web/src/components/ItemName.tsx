import { UPGRADE_TOKEN_RE, upgradeLevelClass } from '@/lib/upgrade'
import { cn } from '@/lib/utils'

type Props = {
  name: string
  className?: string
}

/**
 * Renders an item title with colored upgrade suffixes (`+1`…`+4+`).
 * Also works for multi-item strings such as quest rewards.
 */
export function ItemName({ name, className }: Props) {
  const parts = name.split(UPGRADE_TOKEN_RE)
  return (
    <span className={cn(className)}>
      {parts.map((part, i) => {
        const m = /^\+(\d+)$/.exec(part)
        if (!m) return <span key={i}>{part}</span>
        const level = Number(m[1])
        return (
          <span key={i} className={upgradeLevelClass(level)}>
            {part}
          </span>
        )
      })}
    </span>
  )
}

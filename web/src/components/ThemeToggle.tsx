import { useStore } from '@nanostores/react'
import { Monitor, Moon, Sun } from 'lucide-react'

import { Button } from '@/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { cn } from '@/lib/utils'
import { $theme, cycleTheme, type Theme } from '@/stores/theme'

const THEME_META: Record<
  Theme,
  { label: string; next: string; Icon: typeof Sun }
> = {
  light: { label: 'Light', next: 'Dark', Icon: Sun },
  dark: { label: 'Dark', next: 'System', Icon: Moon },
  system: { label: 'System', next: 'Light', Icon: Monitor },
}

type ThemeToggleProps = {
  className?: string
}

export function ThemeToggle({ className }: ThemeToggleProps) {
  const theme = useStore($theme)
  const { label, next, Icon } = THEME_META[theme]

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          type="button"
          variant="outline"
          size="icon-sm"
          className={cn('bg-background/90 shadow-sm backdrop-blur', className)}
          aria-label={`Theme: ${label}. Click for ${next}.`}
          onClick={cycleTheme}
        >
          <Icon />
        </Button>
      </TooltipTrigger>
      <TooltipContent side="bottom" className="text-left">
        Theme: {label}
        <span className="text-muted-foreground"> · click for {next}</span>
      </TooltipContent>
    </Tooltip>
  )
}

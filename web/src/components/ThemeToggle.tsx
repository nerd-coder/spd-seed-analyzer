import { MonitorIcon, MoonIcon, SunIcon } from '@phosphor-icons/react'
import { useStore } from '@tanstack/react-store'

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
  { label: string; next: string; Icon: typeof SunIcon }
> = {
  light: { label: 'Light', next: 'Dark', Icon: SunIcon },
  dark: { label: 'Dark', next: 'System', Icon: MoonIcon },
  system: { label: 'System', next: 'Light', Icon: MonitorIcon },
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
          className={cn('bg-background/90 backdrop-blur', className)}
          aria-label={`Theme: ${label}. Click for ${next}.`}
          onClick={cycleTheme}
        >
          <Icon />
        </Button>
      </TooltipTrigger>
      <TooltipContent side="bottom" className="text-left">
        Theme: {label}
        <span className="text-muted-foreground">. Click for {next}.</span>
      </TooltipContent>
    </Tooltip>
  )
}

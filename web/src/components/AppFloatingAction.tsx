import { cn } from '@/lib/utils'
import { SettingsButton } from './SettingsButton'
import { ThemeToggle } from './ThemeToggle'

type AppFloatingActionProps = {
  className?: string
}

export function AppFloatingAction({ className }: AppFloatingActionProps) {
  const buttonClasses =
    'dark border-white/20 bg-black/55 text-white hover:bg-black/70 hover:text-white'

  return (
    <div
      className={cn(
        'absolute items-center gap-1.5 shrink-0 top-2 left-2 z-30',
        className
      )}
    >
      <SettingsButton className={buttonClasses} />
      <ThemeToggle className={buttonClasses} />
    </div>
  )
}

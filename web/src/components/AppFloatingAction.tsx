import { cn } from '@/lib/utils'
import { SettingsButton } from './SettingsButton'
import { ThemeToggle } from './ThemeToggle'

type AppFloatingActionProps = {
  className?: string
}

export function AppFloatingAction({ className }: AppFloatingActionProps) {
  const buttonClasses =
    'max-md:border-white/20 max-md:bg-black/55 max-md:text-white max-md:hover:bg-black/70 max-md:hover:text-white'

  return (
    <div
      className={cn(
        'absolute lg:fixed items-center gap-1.5 shrink-0 top-2 right-2 lg:right-[calc((100vw-72rem)/2+24px)] z-30 lg:flex',
        className
      )}
    >
      <SettingsButton className={buttonClasses} />
      <ThemeToggle className={buttonClasses} />
    </div>
  )
}

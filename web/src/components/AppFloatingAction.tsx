import { cn } from '@/lib/utils'
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
        'absolute top-2 left-2 right-2 z-30',
        'flex items-center justify-end',
        className
      )}
    >
      <ThemeToggle className={buttonClasses} />
    </div>
  )
}

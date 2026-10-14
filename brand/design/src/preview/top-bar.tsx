import { MoonIcon, SunIcon } from '@phosphor-icons/react'
import { useStore } from '@tanstack/react-store'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { $theme, setTheme } from '@/lib/preview-store'

export function TopBar() {
  const theme = useStore($theme)
  return (
    <header className="sticky top-0 z-40 border-b bg-background/95 backdrop-blur-sm">
      <div className="mx-auto flex h-14 max-w-6xl items-center gap-3 px-5 lg:px-8">
        <span className="font-pixel text-px-1">SPD Seed Analyzer</span>
        <span className="hidden text-sm text-muted-foreground sm:inline">
          Design system
        </span>
        <ToggleGroup
          type="single"
          variant="outline"
          size="sm"
          spacing={0}
          value={theme}
          onValueChange={(value) => {
            if (value === 'light' || value === 'dark') setTheme(value)
          }}
          aria-label="Theme"
          className="ml-auto"
        >
          <ToggleGroupItem value="light" aria-label="Limestone, light theme">
            <SunIcon data-icon="inline-start" />
            Light
          </ToggleGroupItem>
          <ToggleGroupItem value="dark" aria-label="Torchlit, dark theme">
            <MoonIcon data-icon="inline-start" />
            Dark
          </ToggleGroupItem>
        </ToggleGroup>
      </div>
    </header>
  )
}

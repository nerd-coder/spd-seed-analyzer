import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'

export function SpoilerToggle({
  id,
  label,
  info,
  checked,
  onCheckedChange,
}: {
  id: string
  label: string
  info: string
  checked: boolean
  onCheckedChange: (next: boolean) => void
}) {
  return (
    <div className="flex items-start justify-between gap-3">
      <div className="min-w-0 space-y-0.5">
        <Label htmlFor={id} className="text-sm font-medium">
          {label}
        </Label>
        <p className="text-muted-foreground text-[11px] leading-snug">{info}</p>
      </div>
      <Switch
        id={id}
        checked={checked}
        onCheckedChange={onCheckedChange}
        className="mt-0.5"
      />
    </div>
  )
}

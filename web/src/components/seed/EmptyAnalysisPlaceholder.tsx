import { MAX_SAVED_SEEDS } from '@/stores/app'

export function EmptyAnalysisPlaceholder() {
  return (
    <div className="flex min-h-[min(60svh,28rem)] flex-col items-center justify-center gap-3 px-6 text-center">
      <h2 className="font-heading text-base font-medium">
        No seeds analyzed yet
      </h2>
      <p className="text-muted-foreground max-w-sm text-sm leading-relaxed">
        Enter a seed in the left panel and click Analyze. Open seeds stay as
        tabs until you close them (max {MAX_SAVED_SEEDS}), and are restored
        after a refresh.
      </p>
    </div>
  )
}

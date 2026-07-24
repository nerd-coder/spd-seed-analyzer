import { forwardRef, type ReactNode } from 'react'
import { TabsList } from '@/components/ui/tabs'

export const ScrollableSessionTabs = forwardRef<
  HTMLDivElement,
  { children: ReactNode }
>(function ScrollableSessionTabs({ children }, ref) {
  return (
    <div
      ref={ref}
      className="border-border bg-background/95 sticky top-0 z-20 flex min-w-0 max-w-full items-start gap-2 border-b px-3 pt-3 pb-0 backdrop-blur supports-backdrop-filter:bg-background/80"
    >
      <TabsList
        variant="line"
        className="h-auto w-0 min-w-0 flex-1 flex-nowrap justify-start gap-1 overflow-x-auto"
      >
        {children}
      </TabsList>
    </div>
  )
})

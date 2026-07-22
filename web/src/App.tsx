import { useEffect, useState } from 'react'
import { AnalyzerWorkspace } from '@/components/AnalyzerWorkspace'
import { type AppMode, AppSidebar } from '@/components/AppSidebar'
import { SeedFinder } from '@/components/finder/SeedFinder'
import { SiteFooter } from '@/components/SiteFooter'
import { Tabs, TabsContent } from '@/components/ui/tabs'
import { TooltipProvider } from '@/components/ui/tooltip'
import { loadSpdMeta, startSessionRehydrate } from '@/stores/app'

export default function App() {
  const [mode, setMode] = useState<AppMode>('analyze')

  useEffect(() => {
    loadSpdMeta()
  }, [])

  useEffect(() => startSessionRehydrate(), [])

  return (
    <TooltipProvider delayDuration={200}>
      <Tabs
        value={mode}
        onValueChange={(value) => {
          if (value === 'analyze' || value === 'finder') setMode(value)
        }}
        className="bg-muted/40 flex min-h-svh w-full justify-center gap-0"
      >
        <div className="bg-background border-border flex min-h-svh w-full max-w-6xl flex-col lg:border-x">
          <div className="flex min-h-0 flex-1 flex-col lg:flex-row">
            <AppSidebar mode={mode} />
            <main className="relative min-w-0 flex-1">
              <TabsContent value="analyze" className="mt-0">
                <AnalyzerWorkspace />
              </TabsContent>
              <TabsContent value="finder" className="mt-0">
                <SeedFinder onOpenAnalyze={() => setMode('analyze')} />
              </TabsContent>
            </main>
          </div>
          <SiteFooter />
        </div>
      </Tabs>
    </TooltipProvider>
  )
}

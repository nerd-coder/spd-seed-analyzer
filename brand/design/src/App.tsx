import { TooltipProvider } from '@/components/ui/tooltip'
import { ColorSection } from '@/preview/color-section'
import { ComponentsSection } from '@/preview/components-section'
import { Hero } from '@/preview/hero'
import { PatternsSection } from '@/preview/patterns-section'
import { PiecesSection } from '@/preview/pieces-section'
import { SectionNav } from '@/preview/section'
import { ShapeSection } from '@/preview/shape-section'
import { TopBar } from '@/preview/top-bar'
import { TypeSection } from '@/preview/type-section'
import { VoiceSection } from '@/preview/voice-section'

export default function App() {
  return (
    <TooltipProvider delayDuration={150}>
      <div id="top" className="flex min-h-svh flex-col">
        <TopBar />
        <main className="flex-1">
          <Hero />
          <div className="mx-auto grid max-w-6xl gap-10 px-5 pb-24 lg:grid-cols-[11rem_1fr] lg:px-8">
            <SectionNav />
            <div className="flex min-w-0 flex-col gap-24">
              <ColorSection />
              <TypeSection />
              <ShapeSection />
              <ComponentsSection />
              <PiecesSection />
              <PatternsSection />
              <VoiceSection />
            </div>
          </div>
        </main>
        <footer className="border-t">
          <div className="mx-auto flex max-w-6xl flex-col gap-1 px-5 py-6 text-sm text-muted-foreground lg:px-8">
            <p>
              Tokens live in{' '}
              <code className="font-mono text-foreground">
                brand/design/src/theme/theme.css
              </code>
              . Rules and rationale are in{' '}
              <code className="font-mono text-foreground">
                brand/design/DESIGN.md
              </code>
              .
            </p>
            <p>
              Built on shadcn/ui and Tailwind CSS v4. Game art and the pixel
              font are from Shattered Pixel Dungeon by 00-Evan.
            </p>
          </div>
        </footer>
      </div>
    </TooltipProvider>
  )
}

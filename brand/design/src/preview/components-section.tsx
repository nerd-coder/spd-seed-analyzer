import { ControlsSpecimen } from '@/preview/controls-specimen'
import { FeedbackSpecimen } from '@/preview/feedback-specimen'
import { OverlaysSpecimen } from '@/preview/overlays-specimen'
import { Section } from '@/preview/section'

export function ComponentsSection() {
  return (
    <Section
      id="components"
      intro="shadcn/ui on the radix-lyra style, restyled through the tokens: 14px controls, 2px corners, bevels for state. Each one mirrors its copy in the analyzer, web/src/components/ui."
    >
      <ControlsSpecimen />
      <FeedbackSpecimen />
      <OverlaysSpecimen />
    </Section>
  )
}

import {
  type ChallengeChoice,
  ChallengePicker,
} from '@/components/seed/ChallengePicker'
import { SeedTitle } from '@/components/seed/SeedTitle'
import type { SeedReport } from '@/lib/spd-wasm'

/** Top of a report: the seed, then the run settings that change its floors. */
export function RunHeader({
  report,
  challengeChoice,
}: {
  report: SeedReport
  challengeChoice: ChallengeChoice
}) {
  return (
    <header data-run-header className="flex flex-col gap-6 border-b pb-6">
      <SeedTitle seed={report.seed} />
      <ChallengePicker
        choice={challengeChoice}
        identities={report.identities}
      />
    </header>
  )
}

import { Section } from '@/preview/section'

const RULES = [
  {
    write: 'Analyze seed',
    not: 'Submit',
    why: 'A button says what happens. The same verb names the result: Analyze, analyzing, analyzed.',
  },
  {
    write: 'That seed has 8 letters. Seed codes have 9, like ABC-DEF-GHI.',
    not: 'Invalid input!',
    why: 'An error says what went wrong and how to fix it. It never apologizes.',
  },
  {
    write: 'No seeds analyzed yet. Enter a seed to see what each floor holds.',
    not: 'Nothing to see here',
    why: 'An empty screen points to the next step.',
  },
  {
    write: 'Guaranteed on floor 4',
    not: 'Probably on floor 4',
    why: 'Claim only what the seed decides. When player choices change the result, list the outcomes and their conditions.',
  },
  {
    write: 'One of three wands, each +1',
    not: 'Random wand',
    why: 'Keep every fact the seed fixes, even when part of the result varies.',
  },
  {
    write: 'Scroll of Upgrade',
    not: 'ScrollOfUpgrade, deck index 3',
    why: "Use the game's names. Engine terms stay in the code.",
  },
  {
    write: 'Notable 2',
    not: 'NOTABLE (2)',
    why: 'Sentence case everywhere. Counts are plain numbers next to the label.',
  },
] as const

export function VoiceSection() {
  return (
    <Section
      id="voice"
      intro="Plain words from the player's side of the screen. Sentence case, active verbs, the game's own names, and nothing the seed does not decide."
    >
      <div className="overflow-x-auto">
        <table className="w-full min-w-[640px] text-left">
          <caption className="sr-only">Copy rules</caption>
          <thead className="text-sm text-muted-foreground">
            <tr className="border-b">
              <th className="w-[30%] py-2 pr-4 font-semibold">Write</th>
              <th className="w-[22%] py-2 pr-4 font-semibold">Not</th>
              <th className="py-2 font-semibold">Why</th>
            </tr>
          </thead>
          <tbody>
            {RULES.map((rule) => (
              <tr key={rule.write} className="border-b align-top">
                <td className="py-3 pr-4 font-semibold">{rule.write}</td>
                <td className="py-3 pr-4 text-muted-foreground line-through decoration-destructive/70">
                  {rule.not}
                </td>
                <td className="py-3 text-muted-foreground">{rule.why}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </Section>
  )
}

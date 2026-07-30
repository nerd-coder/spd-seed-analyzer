import { useState } from 'react'
import { FloorsSection } from '@/components/seed/FloorsSection'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import {
  Field,
  FieldContent,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import type { ModeledOutcome, SeedReport } from '@/lib/spd-wasm'

function outcomeAt(
  outcomes: ModeledOutcome[],
  selectedCondition: string
): ModeledOutcome | undefined {
  return outcomes.find((outcome) => outcome.condition === selectedCondition)
}

export function ModeledOutcomes({ report }: { report: SeedReport }) {
  const outcomes = report.modeled_outcomes ?? []
  const [selectedCondition, setSelectedCondition] = useState(
    outcomes[0]?.condition ?? ''
  )
  const selected = outcomeAt(outcomes, selectedCondition) ?? outcomes[0]

  if (!selected) {
    return (
      <FloorsSection floors={report.floors} identities={report.identities} />
    )
  }

  return (
    <div className="flex flex-col gap-4">
      <Alert variant="warning">
        <AlertTitle>Modeled run outcomes</AlertTitle>
        <AlertDescription className="flex flex-col gap-2">
          <p>
            {outcomes.length} modeled combinations were replayed. Selecting an
            outcome only changes the report view; it does not rerun analysis.
          </p>
          {report.analysis_notes?.map((note) => (
            <p key={note}>{note}</p>
          ))}
        </AlertDescription>
      </Alert>

      <FieldGroup>
        <Field orientation="responsive">
          <FieldContent>
            <FieldLabel htmlFor="modeled-outcome">Modeled outcome</FieldLabel>
            <FieldDescription>{selected.notes?.join(' ')}</FieldDescription>
          </FieldContent>
          <NativeSelect
            id="modeled-outcome"
            value={selected.condition}
            onChange={(event) => setSelectedCondition(event.target.value)}
            className="w-full @md/field-group:w-auto @md/field-group:min-w-80"
          >
            {outcomes.map((outcome) => (
              <NativeSelectOption
                key={outcome.condition}
                value={outcome.condition}
              >
                {outcome.condition}
              </NativeSelectOption>
            ))}
          </NativeSelect>
        </Field>
      </FieldGroup>

      <FloorsSection floors={selected.floors} identities={report.identities} />
    </div>
  )
}

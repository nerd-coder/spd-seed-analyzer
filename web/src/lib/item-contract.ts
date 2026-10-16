import type { ItemVariant } from '@/lib/spd-wasm'

/** Facts from a constrained room/quest contract shown under a variable sample. */
export type ItemContractSummary = {
  /** Short spawn label, e.g. `weapon reward`. */
  summary: string
  facts: string[]
}

function rangeLabel(
  label: string,
  range: { min: number; max: number },
  format: (value: number) => string
) {
  if (range.min === range.max) return `${label} ${format(range.min)}`
  return `${label} ${format(range.min)}–${format(range.max)}`
}

/** Build the examine-panel contract section from a constrained sibling. */
export function summarizeItemContract(
  contract: ItemVariant
): ItemContractSummary {
  const facts: string[] = []
  if (contract.cursed === true) facts.push('Cursed')
  else if (contract.cursed === false) facts.push('Uncursed')
  if (contract.tier_range) {
    facts.push(
      rangeLabel('Tier', contract.tier_range, (value) => String(value))
    )
  } else if (contract.tier != null) {
    facts.push(`Tier ${contract.tier}`)
  }
  if (contract.level_range) {
    facts.push(
      rangeLabel('Upgrade', contract.level_range, (value) => `+${value}`)
    )
  } else if (contract.level != null && contract.level !== 0) {
    facts.push(`Upgrade +${contract.level}`)
  }
  return {
    summary: contract.label?.trim() || contract.category,
    facts,
  }
}

import type { BranchFloorReport, QuestReport } from '@/lib/spd-wasm'

export function branchTitle(kind: BranchFloorReport['kind']) {
  switch (kind) {
    case 'blacksmith_mine':
      return 'Blacksmith Mine'
    case 'imp_vault':
      return 'Imp Vault'
  }
}

export function branchAccessText(branch: BranchFloorReport) {
  const conditions: string[] = []
  if (branch.access.requires_acceptance) {
    conditions.push(
      `accept ${branch.access.quest_id.replaceAll('_', ' ')} quest`
    )
  }
  if (branch.access.required_item) {
    conditions.push(`carry ${branch.access.required_item.replaceAll('_', ' ')}`)
  }
  return conditions.length > 0
    ? conditions.join(' and ')
    : 'No additional access condition'
}

export function branchKindForQuest(
  quest: QuestReport
): BranchFloorReport['kind'] | null {
  switch (quest.type) {
    case 'troll_blacksmith':
      return 'blacksmith_mine'
    case 'ambitious_imp':
      return 'imp_vault'
    default:
      return null
  }
}

export function matchingQuestBranch(
  quest: QuestReport,
  branches: BranchFloorReport[] | undefined
): BranchFloorReport | null {
  const kind = branchKindForQuest(quest)
  if (!kind || !branches) return null
  return branches.find((branch) => branch.kind === kind) ?? null
}

export function unclaimedBranches(
  branches: BranchFloorReport[] | undefined,
  quests: QuestReport[] | undefined
): BranchFloorReport[] {
  if (!branches?.length) return []
  const claimed = new Set(
    (quests ?? [])
      .map(branchKindForQuest)
      .filter((kind): kind is BranchFloorReport['kind'] => kind !== null)
  )
  return branches.filter((branch) => !claimed.has(branch.kind))
}

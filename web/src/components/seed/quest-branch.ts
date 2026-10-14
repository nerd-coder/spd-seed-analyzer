import type { BranchFloorReport, QuestReport } from '@/lib/spd-wasm'

const QUEST_TITLES: Record<QuestReport['type'], string> = {
  sad_ghost: 'Sad Ghost',
  old_wandmaker: 'Old Wandmaker',
  troll_blacksmith: 'Troll Blacksmith',
  ambitious_imp: 'Ambitious Imp',
}

export function questTitle(type: QuestReport['type']) {
  return QUEST_TITLES[type]
}

export function branchTitle(kind: BranchFloorReport['kind']) {
  switch (kind) {
    case 'blacksmith_mine':
      return 'Blacksmith Mine'
    case 'imp_vault':
      return 'Imp Vault'
  }
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

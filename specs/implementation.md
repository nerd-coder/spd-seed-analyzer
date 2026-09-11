# Implementation status

The analyzer nests Imp vault `BranchFloorReport`s under the Imp spawn floor
the same way MiningLevel is nested: `kind: imp_vault`, `id: { depth, branch: 1 }`,
`access.requires_acceptance: true`, no pickaxe. Reciprocal `BRANCH_EXIT` /
`BRANCH_ENTRANCE` link `AmbitiousImpRoom` to the vault entrance. Public vault
maps stay painter-complete. Floor-20 Imp shop stock is an `earnedShop`
condition (`score > 2000` after completion), not a guaranteed spawn.

## Next steps

1. PR 5 — QuestCard, FloorDetail, and finder copy for the Imp vault 6-choose-1
   pool and nested branch map.

# Implementation resume

Current state: Sad Ghost and Old Wandmaker rewards have been audited against
SPD v3.3.8 in `specs/quest-rewards/`. Ghost supports explicit no-Tooth and
Mimic Tooth +0…+3 replay profiles. Wandmaker reports only the cross-route
contract—two distinct uncursed +1…+3 options—because earlier trinket,
challenge, and artifact routes can change the concrete pair.

Next steps:

1. Audit Troll Blacksmith, then Ambitious Imp, with the same
   fixed-contract/state-route split.
2. Continue adding verified reachable-state profiles where a single player
   choice has a bounded, precomputable effect; retain baseline-only wording for
   unresolved route combinations.

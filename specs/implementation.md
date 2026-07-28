# Implementation resume

Current state: quest rewards are being re-audited one quest at a time against
SPD v3.3.8. The Sad Ghost phase is documented in
`specs/quest-rewards/sad-ghost.md`.
Its fixed-path reward call is correct, but concrete option details are not
universally seed-only when pre-Ghost trinket, challenge, or artifact state can
change generation. The analyzer now supports an explicit no-Tooth or Mimic
Tooth +0…+3 profile with a chosen first-held floor, and precomputes the Ghost
pair for that assumption. Other pre-Ghost state routes remain baseline-only.

Next steps:

1. Audit the Old Wandmaker reward with the same fixed-contract/state-route
   split and save the verified facts in `specs/quest-rewards/wandmaker.md`.
2. Repeat for Troll Blacksmith, then Ambitious Imp.
3. Continue adding verified reachable-state profiles where a single player
   choice has a bounded, precomputable effect; retain baseline-only wording for
   unresolved route combinations.

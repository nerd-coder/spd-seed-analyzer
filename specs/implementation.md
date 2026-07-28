# Implementation resume

Current state: Sad Ghost, Old Wandmaker, Troll Blacksmith, and Ambitious Imp
rewards are audited against SPD v3.3.8 in `specs/quest-rewards/`. Public
reports keep route-independent option counts, categories, ranges, curse and
claim conditions. Concrete Wandmaker, Blacksmith, and Imp samples are hidden;
Ghost retains its explicit no-Tooth and Mimic Tooth +0…+3 replay profiles.
The run profile resolves Forbidden Runes and a chronological held-trinket
history. Trinket entries support progressive +0…+3 upgrades and transmutation
without level reduction; the report exposes Catalyst offers, first-pot timing,
the earliest effective floor, and the first-deck transmutation order.

Next steps:

1. Promote concrete quest details only for replay profiles that close every
   relevant trinket, artifact-history, and floor-reset route.
2. Add bounded profiles one state input at a time; keep the cross-route
   contracts for unresolved combinations.

# Implementation resume

Current state: Sad Ghost, Old Wandmaker, Troll Blacksmith, and Ambitious Imp
rewards are audited against SPD v3.3.8 in `specs/quest-rewards/`. Public
reports keep route-independent option counts, categories, ranges, curse and
claim conditions. Concrete Wandmaker, Blacksmith, and Imp samples are hidden;
Ghost retains its explicit no-Tooth and Mimic Tooth +0…+3 replay profiles.
The run profile also resolves Forbidden Runes, including its every-second
Upgrade Scroll suppression and downstream generation effects.

Next steps:

1. Promote concrete quest details only for replay profiles that close every
   relevant trinket, artifact-history, and floor-reset route.
2. Add bounded profiles one state input at a time; keep the cross-route
   contracts for unresolved combinations.

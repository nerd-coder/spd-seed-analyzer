# Implementation resume

Current state: floor-layout inputs and certainty boundaries are audited against
SPD v3.3.8 in `specs/analysis/floor-layout-run-settings.md`. The existing
Forbidden Runes plus Mossy Clump, Trap Mechanism, and Mimic Tooth history
selects conditional continuations, but later regular maps remain assumed.
Missing exact-map inputs are Rat Skull, Cracked Spyglass, Barren Land, Badder
Bosses, artifact events, and trinket-instance resets. The separate layout pass
also skips population that must run privately to preserve later deck state.
Quest reward audits remain in the other `specs/analysis/` notes.

Next steps:

1. Split room-selection, room-graph, painter-map, and population certainty.
2. Extend the first-generation main-path profile with the missing challenge,
   trinket-instance, and artifact-event inputs.
3. Snapshot the public map, then continue one hidden full lifecycle so later
   floors inherit exact persistent state.
4. Add paired Java-oracle fixtures before promoting later maps from assumed.

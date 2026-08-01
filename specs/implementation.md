# Implementation resume

Current state: each floor contains every item variant found by the supported
profile replays. Item variants carry structured disjunctive spawn conditions
limited to challenge, chronological trinket, and artifact dependencies; the UI
shows those conditions beside the item's upgrade, curse, and enhancement data.
Profiles use the seed's real Catalyst offers, exact completed-floor
Transmutation Scroll spawns, Forbidden Runes, and chronological Mossy Clump,
Trap Mechanism, or Mimic Tooth upgrade timing. The finder remains on its
separate conservative seed-only projection, and public accuracy is `partial`.

Next steps:

1. Implement Rat Skull, Cracked Spyglass, Barren Land, full trinket-instance
   lifecycle, and artifact-event application before calling any later branch
   exact.
2. Add Java-oracle fixtures for baseline and stateful branches, including
   room rewards, ordinary containers/Mimics, Ghost options, and post-population
   persistent state.

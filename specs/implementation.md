# Implementation resume

Current state: the analyzer has a conservative fixed-path replay, exact
first-floor ordinary loot, guaranteed spawns, room-reward contracts, and
conditional quest contracts. The first-generation main-path profile records
the full challenge mask, chronological trinket acquire/upgrade/transmute
events, external artifact events, and Parchment Scrap claim state. Inputs not
yet applied by the replay keep affected maps and rewards assumed. A separate
layout replay also skips the private population tail. The detailed target
boundary is `specs/analysis/first-four-floor-loot-simulation.md`.

Next steps:

1. Replace the separate layout replay with one lifecycle that snapshots the
   painter-complete public map, then completes private NPC/mob/item population
   to carry exact persistent state to the next floor.
2. Add first-four-floor branch replay: fresh baseline plus only seed-reachable
   Catalyst/trinket branches; expose invariant results and precisely labelled
   conditional variants. Do not enumerate combat/runtime actions.
3. Implement Rat Skull, Cracked Spyglass, Barren Land, full trinket-instance
   lifecycle, and artifact-event application before calling any later branch
   exact.
4. Add Java-oracle fixtures for baseline and stateful branches, including
   room rewards, ordinary containers/Mimics, Ghost options, and post-population
   persistent state. Update `specs/accuracy.json` only after parity passes.

# Implementation resume

Current state: the analyzer has a conservative seed-only projection, exact
first-floor ordinary loot, guaranteed spawns, room-reward contracts, and
conditional quest contracts. There are no user-configurable Run settings. For
each analyzed seed, it automatically replays 26 currently modeled
first-generation conditions: Forbidden Runes on/off, with no trinket or Mossy
Clump, Trap Mechanism, and Mimic Tooth at +0…+3 from the earliest effective
floor. Each result is labelled; all other stateful paths remain partial. The
detailed target boundary is `specs/analysis/first-four-floor-loot-simulation.md`.

Next steps:

1. Replace the separate layout replay with one lifecycle that snapshots the
   painter-complete public map, then completes private NPC/mob/item population
   to carry exact persistent state to the next floor.
2. Expand first-four-floor branch replay from the current fixed modeled matrix
   to seed-reachable Catalyst/transmutation timing, while keeping combat and
   other runtime actions out of scope.
3. Implement Rat Skull, Cracked Spyglass, Barren Land, full trinket-instance
   lifecycle, and artifact-event application before calling any later branch
   exact.
4. Add Java-oracle fixtures for baseline and stateful branches, including
   room rewards, ordinary containers/Mimics, Ghost options, and post-population
   persistent state.

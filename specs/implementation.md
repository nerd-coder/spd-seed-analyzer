# SPD Seed Analyzer — Next Implementation Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`.

Accuracy remains `partial`; `specs/accuracy.json` is authoritative. Public
maps expose deterministic structural layout only. Keep generation in
`spd-core`, preserve pinned Java RNG order, and exclude NPC, mob, heap, and
item-population claims while retaining painter-complete terrain and metadata.

## Current boundary

Depth 5 `SewerBossLevel` is implemented and verified across five pinned Java
fixtures covering Thin Pillars, Walled, Diamond, and Thick Pillars Goo arenas.
Tests assert dimensions, normalized structural terrain, discoverability,
transitions, and the pre-items RNG boundary. Fixed layouts at depths 10, 20,
and 26 remain oracle-backed. Regular-floor coverage remains fixture-specific.

RNG-built boss layouts at depths 15 and 25 remain missing.

## Next phase: depth 15

Port pinned `CavesBossLevel` as a generated structural layout.

1. Trace its builder setup, landmark/room initialization, retries, and exact
   RNG order against the pinned Java source.
2. Port boss-specific room and painter geometry in `spd-core`; keep WASM thin.
3. Add multi-seed Java fixtures covering randomized arena variants and strict
   tests for dimensions, normalized terrain, discoverability, transitions,
   connections, and post-build RNG.
4. Publish painter-complete geometry, terrain, doors, transitions, traps,
   plants, and blobs. Exclude NPCs, mobs, heaps, and item population.
5. Update `specs/accuracy.json`, rebuild WASM, and run CI parity before commit.

Depth 25 `HallsBossLevel` follows depth 15. Quest-branch levels such as
`MiningLevel` remain later work unless needed for a deterministic reward.

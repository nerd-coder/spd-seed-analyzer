# SPD Seed Analyzer — Next Implementation Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`.

Accuracy remains `partial`; `specs/accuracy.json` is authoritative. Public
maps expose deterministic structural layout only. Keep generation in
`spd-core`, preserve pinned Java RNG order, and do not publish entity,
decoration, or placement claims.

## Current boundary

Fixed structural layouts at depths 10, 20, and 26 are oracle-backed. Regular
floor coverage remains fixture-specific. RNG-built boss floors at depths 5,
15, and 25 are not yet implemented.

Depth 5 has an uncommitted diagnostic implementation. It currently provides:

- exact SewerBoss room initialization, figure-eight building, room bounds,
  Rat King connection rules, and the pre-`paintDoors` RNG boundary for AAA;
- boss entrance/exit, all four Goo room families, Rat King painters, and the
  SewerBoss water/grass configuration;
- Java-oracle fixtures covering Thin (AAA), Walled (ABC/GFX), Diamond (HKT),
  and Thick Pillars (ZZZ), including RNG probes.

Do not commit or expose this implementation yet. Its strict oracle test fails:
Rust is one main-generator step behind Java immediately after `paintDoors`, so
pre-item RNG, entrance/door cells, terrain, discoverability, and transitions
are not verified.

## Next phase

Finish and commit depth-5 `SewerBossLevel` parity.

1. Trace the shared `paint_doors` merge/regular-door path. Pre-`paintDoors`
   RNG already matches Java; isolate the single missing Java RNG call and the
   entrance/door callback discrepancy.
2. Make all five depth-5 fixtures strictly match normalized structural
   terrain, discoverability, transitions, dimensions, and post-build RNG.
3. Confirm public maps remain layout-only and do not expose mobs, items,
   heaps, grass, traps, plants, blobs, or decorative tile claims.
4. Update `specs/accuracy.json` only after every strict fixture passes.
5. Rebuild WASM and run CI parity: `bun run check`, `bun run check:rust`,
   `bun run test:rust`, `bun run build`, and `bun run test:visual:only`.
6. Commit the completed depth-5 phase with a Conventional Commit, then rewrite
   this file toward depth 15 (`CavesBossLevel`) and stop.

Depth 25 (`HallsBossLevel`) follows depth 15. Quest-branch levels such as
`MiningLevel` remain later work unless needed to prove a deterministic reward.

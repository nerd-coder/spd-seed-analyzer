# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

Phase 1 is complete: Old Wandmaker rewards now expose exact non-cursed upgrade
levels and exact classes when Mimic Tooth cannot shift the wand deck. When it
can, both rewards expose ordered five-class candidate sets. The projection and
accuracy manifest describe this conditional coverage, and the UI renders the
Wandmaker reward entries.

## Next steps

1. Report the four Trinket Catalyst offers. Preview TRINKET deck draws 0–3 on
   a cloned generator, attach the ordered choose-one options to the forced
   catalyst entry, render them on the spawning floor, and update the accuracy
   manifest. Do not advance the real deck or imply that all four trinkets drop.
2. Add parity tests for catalyst offer order and public report serialization,
   then run the CI-equivalent Rust, Biome, WASM/Vite, and visual checks.

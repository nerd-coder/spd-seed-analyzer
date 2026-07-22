# Visual regression fixtures

These screenshots are reference compositions captured from pinned Shattered
Pixel Dungeon v3.3.8 (`7b8b845a7`) for seed `HKT-JZN-XQQ`:

- `HKT-JZN-XQQ_F1.png` — floor 1
- `HKT-JZN-XQQ_F6.png` — floor 6

Preserve the source images unchanged. Their hero positions reflect
post-generation gameplay state and are not deterministic outputs of
`Level.create()`; visual tests must either use a fixed hero overlay or exclude
the hero from the deterministic comparison.

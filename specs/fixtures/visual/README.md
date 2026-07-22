# Visual regression fixtures

These screenshots are reference compositions captured from pinned Shattered
Pixel Dungeon v3.3.8 (`7b8b845a7`). Name each fixture
`<seed>_<floor>.png`.

Preserve the source images unchanged. Their hero positions reflect
post-generation gameplay state and are not deterministic outputs of
`Level.create()`; visual tests must either use a fixed hero overlay or exclude
the hero from the deterministic comparison.

Current `HKT-JZN-XQQ` references cover floors 1, 6, and 8. Floors 6 and 8 have
pinned Java observations and partial browser comparisons; their lifecycle and
entity-cell parity remains open.

# Visual regression fixtures

These screenshots are reference compositions captured from pinned Shattered
Pixel Dungeon v3.3.8 (`7b8b845a7`). Name each fixture
`<seed>_F<floor>.png`. A `_Q` suffix identifies a quest-branch reference
entered from that regular floor.

`tools/visual/tests/map-render-fixtures.ts` is the canonical test-map input
list. Run `bun run test:map-render` to verify that every reference PNG has
matching seed/floor metadata in that registry and that no registered image is
missing.

Preserve the source images unchanged. Their hero positions reflect
post-generation gameplay state and are not deterministic outputs of
`Level.create()`; visual tests must either use a fixed hero overlay or exclude
the hero from the deterministic comparison.

Current references cover `HKT-JZN-XQQ` floors 1, 6, 8, and 12, its floor-12
Troll Blacksmith quest branch, plus `CXG-FJT-BFQ` floor 1. `tools/visual/` uses
regular-floor registry entries to run strict Playwright comparisons against
separate browser-canvas baselines. Quest-branch entries remain source-only
references until branch-level analysis is implemented. Passing the browser
suite protects the deterministic browser renderer; it does not establish
pixel parity with gameplay captures because fixture-only hero, explored-FOV,
and animation state remain outside the map-render contract.

# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

The Old Wandmaker, Troll Blacksmith, and Trinket Catalyst phases are complete.
Blacksmith rewards expose all four exact equipment choices, their shared exact
upgrade level, and the seed-rolled enchantment/glyph. When the effect needs
Parchment Scrap, the report keeps its identity and states the minimum Scrap
level instead of treating it as guaranteed by the seed alone. Wandmaker rewards
expose exact non-cursed upgrade levels and exact classes when Mimic Tooth
cannot shift the wand deck, otherwise ordered candidates. The Wandmaker quest
label shows only the quest name/type; its two guaranteed reward items remain
listed separately. Guaranteed Trinket
Catalysts now carry the four ordered TRINKET-deck offers as a choose-one list;
the Catalyst itself remains one searchable guaranteed spawn. The UI keeps the
Catalyst item/icon row and renders a separate “Trinket offers” group with the
offer item/icons, without OR labels or upgrade levels. The preview uses a
cloned generator and does not advance the real deck.

Find defaults are also settled: the sidebar generates one random numeric start
seed when the page mounts and preserves it across Analyze/Find tab switches;
new searches default to depth 20, and the random-seed control uses the primary
button style.

All web application and component UI state now uses TanStack Store, including
finder form state, map controls/render status, close confirmation, and elapsed
time. DOM refs and derived memoized values remain React implementation details.

Secret Honeypot room rewards are verified against pinned SPD: every such room
reports the guaranteed Shattered Pot and Honeypot as exact searchable items,
while the Bomb/Double Bomb result remains a constrained seed-rolled variant.

The depth-20 Imp shop stock is retained in the public report with the explicit
condition that the Ambitious Imp quest must have been completed before the shop
spawns. Seed-determined fixed stock, quantities, and safe deck constraints are
shown; inventory-, limited-drop-, and Hourglass-dependent stock stays
constrained.

The Old Wandmaker's three quest rooms now have their pinned terrain painters:
Mass Grave, Ritual Site, and Rot Garden. The Ritual Site fills its wall border
and empty interior, then blocks the 3×3 ritual marker; the fills consume no RNG,
but `Room.center()` still jitters as in Java. Its four guaranteed Ceremonial
Candles remain queued for floor placement.

Painter-complete maps now publish the pinned Ritual Site, Blacksmith,
Ambitious Imp, Weak Floor, and Demon Spawner overlays. The City boss publishes
its custom ground and wall layers, and the web renderer loads all corresponding
quest, weak-floor, Halls, and City texture sheets.

## Next steps

1. Narrow the public-taint blast radius. `callback_tail_sensitive` room paint
   sets `public_generation_tainted`, and `inherited_public_taint` turns that
   into `runtime_sensitive_layout` on *every* later floor. Measured over the
   reference seeds, only floor 1 and the boss floors keep an exact layout,
   room list, and ordinary loot; floors 2–24 fall back to an assumed
   continuation with guaranteed spawns, quest rewards, and shop stock only.
   Per `specs/generator-decks.md` the deck draws behind most of those
   callbacks are per-depth isolated, so audit them one room family at a time
   and demote the taint to `runtime_sensitive_placed_items_from` (the specific
   prize) where the level RNG stream provably cannot shift.
2. Finish fixture-backed overlays for remaining Java producers such as Mass
   Grave, then take the next user-reliability gap from `specs/accuracy.json`,
   verify its pinned Java generation path, and update the public projection
   only when the seed-determined guarantee is established.
3. Add focused parity/report/search tests, update this state file and the
   accuracy manifest, then run the CI-equivalent Rust, Biome, WASM/Vite, and
   visual checks before committing that phase.

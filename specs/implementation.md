# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

The quest-NPC oracle pins the Ghost and Wandmaker cell plus the eight-int RNG
tail immediately after reward generation across all Wandmaker depth/type
combinations and Ghost depths 2–4. Rust now matches 15 of 18 boundaries. Ghost
placement and reward tails match the full matrix, but Ghost rewards remain
omitted until the remaining prior-floor drift is resolved. The existing
Wandmaker constraint remains. Accuracy is still `partial`.

Public item entries carry exact stack quantities when known and consolidate
identical exact spawn/shop rows; differing properties remain separate.

## 1. Restore quest-NPC placement parity

`StatueRoom` now generates the armored variant's armor/glyph, `MassGraveRoom`
re-rolls its skeleton loop condition like Java, and Treasury mimics generate
their held prize. Focused room boundaries and the 18-case NPC matrix pin these
paths. The AAC floor-4 and floor-6 painter callbacks and `paintDoors` boundary
now match Java exactly.

Three Wandmaker cases remain:

- AAC floor 7 diverges before room construction even though floor 6 painting
  ends aligned; locate the floor-6 population/item lifecycle drift.
- AAU floor 9 likewise has a different pre-shuffle room list; find the first
  prior-floor population boundary that differs.
- AAD floor 8 has the exact reward RNG tail but chooses cell 586 instead of
  643; compare passability, occupancy, and distance-map inputs to
  `spawnWandmaker` without changing RNG.

Focused lifecycle probes narrow both upstream cases further: AAC matches Java
through floor 6's `createItems` entry and all final heap/mob placements, then
diverges before floor 7 construction; AAU does the same across floor 8→9. The
visible AAC floor-6 difference is only the inventory-sensitive shop bag
(Potion Bandolier in the oracle, Magical Holster in the baseline analyzer), so
locate the first post-`createItems` persistent-state mutation rather than
changing room construction directly.

Require all 18 NPC boundaries to match, remove both NPC tail guards, and only
then update the accuracy manifest and expose Ghost rewards.

## 2. Publish ordinary floor loot (independent of §1)

`RegularLevel.createItems` main-loop drops are already generated exactly
(`level/create_items.rs`) and already suppressed wholesale
(`level/state.rs:50-54`, `:164`). `generator-decks.md` §11 shows the suppression
is over-broad: the general category deck and every sub-deck except ARTIFACT are
levelgen-only, so this is ~3.8 items per floor — an order of magnitude more than
everything §3–§5 add combined.

**1. Prove it before claiming it.** The `final_placed_heaps` fixtures already
   carry every heap with class/quantity/level/cursed for ~45 seed×floor pairs
   (AAA 1–22/25/26, GFX 1/3/5/6/12/15/19/25, HKT 1/5–8, ABC, ZZZ), but only five
   hand-picked `main_drop_cells` sites assert against them
   (`final_heaps/replay_aaa.rs`, `floor_six.rs`). Replace those with a
   systematic comparison of *all* main-loop heaps over every covered pair. Treat
   any mismatch as a blocker. Then capture AAA floors 23–24 and a second
   full-run seed so the claim is not single-seed.

**2. Gate on the artifact deck.** `random_artifact` (`generator/state.rs:327`)
   is the only Rust site that moves `ARTIFACT.dropped`. Add a monotone
   `artifact_draws` counter there and stamp each generated item with
   `artifact_conditional = generator.artifact_draws > 0` evaluated *after* its
   own draw, so the item that triggers the first draw is itself conditional.
   The flag is sticky for the rest of the run: once the stream can desync (§11),
   nothing downstream recovers.

**3. Project it.** Replace the blanket skip at `level/state.rs:164`:
   - unconditional → `Exact`, with class, level and cursed exposed;
   - conditional → `Constrained` with the computed class as a single-element
     `candidate_classes`, so `search.rs:228-231` still matches it, plus a
     `conditional_notes` line naming the assumption ("no artifact obtained
     outside level generation earlier in this run").

   Landmine: `reported_level` (`level/state_map.rs:17`) returns `None` when
   constrained, and `search.rs:240` does `.expect("exact predictions expose
   level")` — conditional loot must keep its level or search panics.

   Honour **SPAWN-PRESENCE**: cell, heap type and mimic lifecycle stay internal.
   Start with plain heaps (`heap`/`chest`/`locked_chest`/`skeleton`); add
   mimic and golden-mimic carried items as a follow-up. **MAP-LAYOUT-GOAL** is
   untouched — maps stay layout-only and `runtime_sensitive_loot_cells` keeps
   suppressing markers.

**4. Manifest and UI.** In `specs/accuracy.json`, drop "Ordinary floor loot and
   Mimic contents are excluded because they shift with run history" from
   `intentional-scope-limits` and state the narrower artifact condition under
   `items-and-loot`. Floor lists grow from ~2 entries to ~6, so group guaranteed
   spawns separately from floor loot and render the conditional note once per
   floor, not once per item.

**5. Optional recovery.** On conditional floors, re-run generation with
   `ARTIFACT.dropped + 1` and `+2` and promote back to `Exact` any item whose
   class and level agree across all variants — the same "check agreement across
   candidates" trick as §4.3 below. Measure the recovery rate before building it.

## 3. Then sharpen the Sad Ghost reward

Everything except the weapon's class is independent of any deck (§10).

1. **Name the armor.** The tier *is* the class (2 Leather, 3 Mail, 4 Scale,
   5 Plate), chosen by a direct `Random.chances` with no deck involved. Report
   `GhostArmor` as `Exact` with its class name instead of "Ghost armor reward";
   the level is already exact and shared with the weapon.
2. **Report the enchant and glyph.** Both are always generated and fully
   seed-determined; only *whether they are kept* depends on Parchment Scrap.
   `ItemEntry` has no enchantment field — add one, and state the condition from
   the known roll, e.g. "Grim — kept only with Parchment Scrap +1 or better".
   At +2 or better it is always enchanted.
3. **Weapon class as an ordered candidate set** over the tier deck, reusing the
   `ImpRing { identity_exact, candidate_indices }` machinery. The index is 0 in
   64% of seeds and ≤1 in 85%, so a two-element set nearly always covers it.
4. Update `specs/accuracy.json`: the Ghost's armor and upgrade level are exact,
   the weapon is named or offered as a short ordered set, and enchantment is
   stated with its Parchment Scrap condition.

## 4. Lift the Old Wandmaker constraint

Today both wands are reported as `+1…+3 wand` with no class
(`level/state.rs:255`, `:308-313`). After step 1:

1. **`wand1`'s level becomes exact** — it is identity-independent and has no
   loop in front of it. Both wands are always distinct classes and never cursed;
   report those as invariants.
2. **Classes as ordered candidate sets** (`k` is 0–4, mode 1), flagging
   `identity_exact` when Mimic Tooth is absent from the four catalyst offers.
   No runtime path can move the wand deck (§9), so that is the only lever.
3. **`wand2`'s level: check agreement across candidates.** The duplicate-
   rejection loop that shifts it fires in only 5.5% of seeds, so simulate each
   candidate `k` and report the level as exact whenever they all agree — even
   when the class stays a set. Fall back to `+1…+3` only on disagreement.
4. Update `specs/accuracy.json` accordingly.

## 5. Then report the four Trinket Catalyst offers

The catalyst's four options are TRINKET deck draws 0–3, so they are exact and
knowable before the run starts — a stronger claim than anything we show for the
catalyst today, which is only its guaranteed spawn on floors 1–4.

1. Compute them on a cloned generator where step 2/3 above already reads a deck:
   the real draws happen at runtime when the player opens the window, and the
   analyzer must not advance the deck.
2. Attach them to the catalyst's forced-drop entry (`ForcedDropRole::
   TrinketCatalyst` in `level/state/forced_queue.rs`) as an exact, ordered
   four-item option set — the offer list, not a prediction of the pick.
3. Surface them in the UI on the floor that spawns the catalyst, rendered as a
   choose-one set so it never reads as four items dropping.
4. Update `specs/accuracy.json`: the catalyst's offered trinkets are named
   exactly; which one a run takes is a player choice.

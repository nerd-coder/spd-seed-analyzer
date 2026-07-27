# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

The quest-NPC oracle now pins the Ghost and Wandmaker cell plus the eight-int
RNG tail immediately after reward generation across all Wandmaker depth/type
combinations and Ghost depths 2–4. Rust matches 12 of 18 boundaries. The six
mismatches include Ghost spawn-floor drift, so Ghost rewards are omitted from
the public report; the existing Wandmaker constraint remains. Accuracy is
still `partial`.

## 1. Restore quest-NPC placement parity

Compare the failing fixture boundaries in `quests::placement_oracle_tests`
against painter output. Fix entrance/exit room terrain and flags before reward
logic. When all 18 boundaries match, remove both NPC tail guards and update the
accuracy manifest.

## 2. Then sharpen the Sad Ghost reward

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

## 3. Lift the Old Wandmaker constraint

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

## 4. Then report the four Trinket Catalyst offers

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

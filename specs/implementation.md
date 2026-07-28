# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

Sad Ghost armor is named exactly, while its weapon is named exactly or shown as
an ordered candidate set from the tier deck. Both rewards expose their
seed-rolled enchantment/glyph and the minimum Parchment Scrap level needed to
keep it. The existing Wandmaker constraint remains. Accuracy is still
`partial`.

Public item entries carry exact stack quantities when known and consolidate
identical exact spawn/shop rows; differing properties remain separate.

Ordinary `createItems` loot is now published with class, quantity, level, and
curse after systematic spawn-presence parity checks against Java. Loot stays
exact until the first artifact-deck draw; that item and the remaining run are
reported as constrained on no earlier out-of-level artifact acquisition. Plain
heap/chest/locked-chest/skeleton spawns are included, while Mimic-carried loot
and all placement details remain excluded.

## 1. Lift the Old Wandmaker constraint

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

## 2. Report the four Trinket Catalyst offers

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

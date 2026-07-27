# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

The Ambitious Imp's ring now reports its exact class when neither known deck
shift is available, or an ordered three-class set otherwise. Its level, curse,
and guaranteed presence remain exact; seed search matches exact classes or the
reported set.

The next wins are the other two NPC quests. Both are held back by one shared
fidelity gap rather than by anything the seed leaves open. Mechanism and
evidence live in [generator-decks.md](generator-decks.md) §8–§10.

## 1. Pin the quest NPC placement loop (unblocks 2 and 3)

Both `Ghost.Quest.spawn` and `Wandmaker.Quest.spawnWandmaker` run at the *start*
of their level's `createMobs`, after paint, and burn two `Random.IntRange` calls
per placement attempt against painted terrain that our port only approximates
(`quests/ghost.rs:84`, `quests/wandmaker.rs:143`). A wrong try count puts every
subsequent reward roll at the wrong stream offset.

We are currently inconsistent about this: the Wandmaker sets
`wand_rng_tail_sensitive` and taints the floor (`level/quest_rewards.rs:93`),
while the Ghost trusts the same post-placement stream enough to publish reward
tiers. Resolve it, do not paper over it.

1. Add a Java-oracle contract (shape: `imp-ring-deck`) recording the NPC's cell
   and the RNG tail immediately after reward generation, for reference seeds
   covering Ghost depths 2/3/4 and Wandmaker depths 7/8/9 × quest types 1/2/3.
2. Compare cell and tail in `quests/ghost.rs` / `quests/wandmaker.rs` tests. A
   matching cell means the try count matched, which is the whole claim.
3. If they match, drop `wand_rng_tail_sensitive`. If they do not, fix the
   entrance/exit-room flags first — and the Ghost's published tiers are wrong
   today and must be constrained until they do.

## 2. Sharpen the Sad Ghost reward

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

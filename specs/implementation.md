# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

LoopBuilder's narrower shop collision list is now pinned by independent AFU
and GFX floor-16 City shop traces.

## Report the Ambitious Imp ring class

The reward class is currently dropped entirely (`ImpRing` is `Constrained`,
reported as `+N ring`). On most seeds it is unconditionally seed-determined and
can be reported exactly.

Each `Generator.Category` carries its own `seed` long plus a `dropped` counter
(`Generator.java:709+`), so the class of the *k*-th ring a run draws is a pure
function of the dungeon seed and *k* — independent of floor RNG position. Level
streams are pushed per depth (`seedCurDepth`), so deck counters are the only
cross-floor channel: parity needs the ring-draw *sites* to fire in the same
order and count, nothing more. The `+2…+4` level stays exact — it comes from
the floor's own stream and no ring subclass overrides `random()`.

Measured over 44 seeds, 0–6 ring draws precede the Imp's (mode 4), and the
deck's 12 equal-weight classes mean an index off by one changes the class
~11/12 of the time. Only two things can shift *k*, both checkable per seed:

- **Mimic Tooth** adds levelgen mimic prizes (1/5 of which draw a ring).
  Trinkets have zero weight in both general decks, so the sole source is the
  one Trinket Catalyst on floors 1–4, whose four offers are TRINKET deck draws
  0–3 — seed-determined. Mimic Tooth is offered in ~25% of seeds (101/400
  sampled); otherwise it needs a Transmutation on the taken trinket (draw 4).
- **Artifact exhaustion** turns `random(Category.ARTIFACT)` into a ring draw.
  Sampled seeds keep 5–10 of the 11 artifact deck weight unspent at the Imp
  spawn (median 8), so this needs that many extra runtime Ring of Wealth
  artifact draws first. Compute the headroom rather than assuming it.

Other run history is ruled out: runtime ring sources bypass the deck
(`Mob.createLoot`, Ring of Wealth, Transmutation, Cursed Wand, runtime mimics
all use `randomUsingDefaults`); `Challenges.isItemBlocked` only blocks Dewdrops
under `NO_HERBALISM`, which no ring path can produce, so no challenge shifts
*k*; and the ungenerated Mining/Vault side-levels use `randomUsingDefaults`
only.

1. Verify the draw count first — it is the whole claim. Have the java-oracle
   dump `Category.RING.dropped` at the Imp spawn for the reference seeds and
   diff against ours. Sites to keep aligned: `createItems` general-deck picks,
   ordinary and golden mimic prizes, shop rare (1/10), `PitRoom` main loot,
   `CrystalChoiceRoom` hidden prize, `CrystalVaultRoom` prize cycle,
   GrassyGrave/MassGrave/SecretSummoning, artifact fallback.
2. In `quests/imp.rs`, emit the reward class plus a seed-only certainty flag:
   clear when Mimic Tooth is absent from the four catalyst offers *and* the
   artifact deck has headroom. Also capture the classes at draw *k+1* and *k+2*
   (clone the generator; the sub-deck draw does not touch the floor stream).
3. In `ItemEntry`, report the class as `Exact` when the flag is clear, and as
   an ordered candidate set otherwise, labelled as exact unless the run took
   Mimic Tooth. Level, curse, and spawn-presence stay exact either way. Seed
   search may match the exact class, but only the set when the flag is unclear.
4. Update `specs/accuracy.json`: the Imp ring identity moves from omitted to
   named, with the Mimic Tooth condition stated in player terms.

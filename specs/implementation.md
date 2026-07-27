# SPD Seed Analyzer — Next Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`; accuracy remains `partial`.

LoopBuilder's narrower shop collision list is now pinned by independent AFU
and GFX floor-16 City shop traces.

## Report the Ambitious Imp ring class

The reward class is currently dropped entirely (`ImpRing` is `Constrained`,
reported as `+N ring`). On most seeds it is unconditionally seed-determined and
can be reported exactly.

Mechanism and evidence live in [generator-decks.md](generator-decks.md); the
short version is that the class is a pure function of the seed and the ring
deck draw index *k*, the `+2…+4` level is exact regardless, and only two things
can move *k* — Mimic Tooth (absent from the Trinket Catalyst's four
seed-determined offers in ~75% of seeds) and artifact-deck exhaustion (5–10 of
11 deck weight still unspent at the spawn). Both are checkable per seed.

1. In `quests/imp.rs`, emit the reward class plus a seed-only certainty flag:
   clear when Mimic Tooth is absent from the four catalyst offers *and* the
   artifact deck has headroom. Also capture the classes at draw *k+1* and *k+2*
   (clone the generator; the sub-deck draw does not touch the floor stream).
2. In `ItemEntry`, report the class as `Exact` when the flag is clear, and as
   an ordered candidate set otherwise, labelled as exact unless the run took
   Mimic Tooth. Level, curse, and spawn-presence stay exact either way. Seed
   search may match the exact class, but only the set when the flag is unclear.
3. Update `specs/accuracy.json`: the Imp ring identity moves from omitted to
   named, with the Mimic Tooth condition stated in player terms.

## Then report the four Trinket Catalyst offers

The catalyst's four options are TRINKET deck draws 0–3, so they are exact and
knowable before the run starts — a stronger claim than anything we show for the
catalyst today, which is only its guaranteed spawn on floors 1–4.

1. Compute them where step 2 above already reads the TRINKET deck, on a cloned
   generator: the real draws happen at runtime when the player opens the
   window, and the analyzer must not advance the deck.
2. Attach them to the catalyst's forced-drop entry (`ForcedDropRole::
   TrinketCatalyst` in `level/state/forced_queue.rs`) as an exact, ordered
   four-item option set — the offer list, not a prediction of the pick.
3. Surface them in the UI on the floor that spawns the catalyst, rendered as a
   choose-one set so it never reads as four items dropping.
4. Update `specs/accuracy.json`: the catalyst's offered trinkets are named
   exactly; which one a run takes is a player choice.

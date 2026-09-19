# Implementation status

The analyzer renders the Troll Blacksmith's Crystal and Gnoll MiningLevel
branches for the fresh, once-generated-floor route. Branch maps appear inside
the origin floor's quest panel, with reciprocal transitions, access conditions,
objective-specific tilesets, and painter-complete layout. Rust generation
matches pinned Java painter fixtures exactly; browser snapshots cover both
objectives.

The overall analyzer remains partial. Mining entry still depends on accepting
the quest, carrying the Pickaxe, and confirming travel, while reset paths and
unmodeled pre-quest player/meta state are not enumerated.

The Ambitious Imp quest panel reports each of the six take-out slots with its
seed-independent guarantees and a candidate distribution over a +-3 deck-index
window, fresh-run draw first. The window is exact rather than sampled: a
category's draw order is fixed by the seed, so run history only shifts which
index the Imp lands on. Percentages are the share of that enumerated window, a
stability measure, not an observed player frequency. The finder's Deep search
switch fills the seed-only Imp entries with the same window so quest rewards
can be matched at all; it is off by default.

## Next steps

1. Extend conditional route discovery beyond its current verified depth when
   supported pre-quest state needs explicit alternate Blacksmith branch maps.
2. Re-verify MiningLevel generation and fixtures when the pinned SPD commit
   changes.
3. The finder item catalogue covers only Rings, Wands, and Artifacts, so the
   Imp's weapon, missile, and armor slots are surfaced in the report but are
   not yet searchable. Adding those groups widens the whole finder, not just
   the Imp, so scope it deliberately.
4. Apply the same deck-window projection to the Wandmaker, which still reports
   all thirteen wands as its candidate set (`quests/wandmaker.rs:232-238`).

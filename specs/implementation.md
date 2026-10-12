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
stability measure, not an observed player frequency.

Upgrades and enchants are stronger than the classes: they are pinned by the
seed, because every class in a category costs the same ambient RNG. The panel
reports each slot's concrete `+N`, its enchant by name, and the pool's total
Scroll of Upgrade worth. `upgrades_pinned` goes false only when an
`UnstableSpellbook` draw is reachable in slot 0, which shifts every later roll.

The finder's Deep search switch fills the seed-only Imp entries with the same
class window so quest rewards can be matched at all; it is off by default.

## Next steps

1. Conditional Blacksmith routes beyond the verified fresh path when pre-quest
   state needs alternate branch maps.
2. The finder catalogue is only Rings, Wands, and Artifacts, so Imp weapon,
   missile, and armor slots are reported but not searchable.
3. Wandmaker still reports all thirteen wands
   (`quests/wandmaker.rs:232-238`) rather than a deck window.

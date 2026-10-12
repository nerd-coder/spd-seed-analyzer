# Implementation status

The analyzer renders the Troll Blacksmith's Crystal or Gnoll mine as the
once-through painter-complete map for the seed. Branch maps appear inside the
origin floor's quest panel, with reciprocal transitions, access conditions,
objective-specific tilesets, and painter-complete layout. Rust generation
matches pinned Java painter fixtures exactly; browser snapshots cover both
objectives. Entry still requires accepting the quest, carrying the Pickaxe,
and confirming travel.

The overall analyzer remains partial.

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

The Old Wandmaker quest panel reports two ordered options, wand1 then wand2,
each over a +-3 deck-index window around the fresh wand1 index, fresh draw
first. The class at each index is a pure function of the seed, so run history
only shifts which index the reward lands on. Percentages are the share of that
enumerated window, a stability measure, not an observed player frequency.
wand1's +N is pinned across the window. wand2's +N is the fresh roll and stays
pinned only when every offset in the window rejects the same number of
duplicate draws.

The finder catalogue includes the weapon, missile, and armor classes those
quests can roll. Its upgrade filter reaches +5 for that equipment. Rings and
wands still stop at +4. The Deep search switch fills the seed-only Imp and Wandmaker entries with
the same class window so those quest rewards can be matched at all; it is off
by default.

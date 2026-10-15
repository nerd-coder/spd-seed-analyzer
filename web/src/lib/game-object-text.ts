/**
 * English examine-window `name` and `desc` for SPD v4.0.2 (`57a4e06a4`).
 * These are mobs, NPCs, bosses, the alchemy pot, and level feelings, not item
 * classes.
 * The geomancer text is the awake description.
 */
type GameObjectText = {
  name: string
  desc: string
}

const TEXTS: Record<string, GameObjectText> = {
  AlchemyPot: {
    name: 'Alchemy pot',
    desc: 'This pot is filled with magical water. Items can be mixed into the pot to create something new!',
  },
  FeelingChasm: {
    name: 'chasm floor',
    desc: 'Your steps echo across the dungeon.',
  },
  FeelingWater: {
    name: 'water floor',
    desc: 'You hear water splashing around you.',
  },
  FeelingGrass: {
    name: 'grass floor',
    desc: 'The smell of vegetation is thick in the air.',
  },
  FeelingDark: {
    name: 'dark floor',
    desc: 'You can hear enemies moving in the darkness.',
  },
  FeelingLarge: {
    name: 'large floor',
    desc: 'This dungeon floor seems unusually large.',
  },
  FeelingTraps: {
    name: 'traps floor',
    desc: 'The ground seems especially treacherous here.',
  },
  FeelingSecrets: {
    name: 'secrets floor',
    desc: 'The atmosphere hints that this floor hides many secrets.',
  },
  CrystalSpire: {
    name: 'crystal spire',
    desc: "This gigantic spire of ultra hard crystal is probably the source of all the strange crystalline creatures in this cave. Your regular weapons are ineffective against it, so you'll need to use your pickaxe if you want to destroy it.\n\n_Make sure you're prepared before you start breaking it._ Such a big crystal is likely to take a while to mine through, and you probably won't be able to do it undisturbed. The spire itself may even have some means of defending itself as well. _Perhaps the spire's own attacks could be used against its minions._",
  },
  FetidRat: {
    name: 'fetid rat',
    desc: "Something is clearly wrong with this rat. Its greasy black fur and rotting skin are very different from the healthy rats you've seen previously. Its pale green eyes make it seem especially menacing.\n\nThe rat carries a cloud of horrible stench with it, it's overpoweringly strong up close.\n\nDark ooze dribbles from the rat's mouth, it eats through the floor but seems to dissolve in water.",
  },
  GnollGeomancer: {
    name: 'gnoll geomancer',
    desc: "This impressively tall senior gnoll must be the source of the various earth-moving magic around here, and the organizer of all the gnoll activity. Unlike the gnoll sappers in the area, the geomancer is quite adept with its magic, and cannot be damaged by thrown or falling rocks. It doesn't have any way to fight in melee combat though.",
  },
  GnollTrickster: {
    name: 'gnoll trickster',
    desc: 'A strange looking creature, even by gnoll standards. It hunches forward with a wicked grin, almost cradling the satchel hanging over its shoulder. Its eyes are wide with a strange mix of fear and excitement.\n\nThere is a large collection of poorly made darts in its satchel, they all seem to be tipped with various harmful substances.',
  },
  GreatCrab: {
    name: 'great crab',
    desc: "This crab is gigantic, even compared to other sewer crabs. Its blue shell is covered in cracks and barnacles, showing great age. It lumbers around slowly, barely keeping balance with its massive claw.\n\nWhile the crab only has one claw, its size easily compensates. The crab holds the claw in front of any threat, shielding itself behind an impenetrable wall of carapace. However, the crab cannot block attacks it doesn't see coming, or attacks from multiple enemies at once.",
  },
  Imp: {
    name: 'ambitious imp',
    desc: "This little red person look like an Imp. Imps are lesser demons, notable for their intelligence and not their strength.\n\nYou've only heard about demons in ancient folk tales though, which usually describe them as travelers from another world. You've never seen any evidence demons actually exist, but this Imp is clearly real and seems very happy to see you.",
  },
  Ghost: {
    name: 'sad ghost',
    desc: 'The ghost is barely visible. It looks like a shapeless spot of faint light with a sorrowful face.',
  },
  Wandmaker: {
    name: 'old wandmaker',
    desc: 'This old yet hale gentleman wears a slightly confused expression. He is protected by a magic shield.',
  },
  Blacksmith: {
    name: 'troll blacksmith',
    desc: 'This troll blacksmith looks like all trolls look: he is tall and lean, and his skin resembles stone in both color and texture. The troll blacksmith is tinkering with disproportionally small tools.',
  },
  Goo: {
    name: 'Goo',
    desc: "Little is known about The Goo. It's quite possible that it is not even a creature, but rather a conglomerate of vile substances from the sewers that somehow gained basic intelligence. Regardless, dark magic is certainly what has allowed Goo to exist.\n\nIts gelatinous nature has let it absorb lots of dark energy, you feel a chill just from being near. If Goo is able to attack with this energy you won't live for long.",
  },
  Tengu: {
    name: 'Tengu',
    desc: 'A famous and enigmatic assassin, named for the mask grafted to his face.\n\nTengu is held down with large clasps on his wrists and knees, though he seems to have gotten rid of his chains long ago.\n\nHe will try to use traps, deceptive magic, and precise attacks to eliminate the only thing stopping his escape: you.',
  },
  DM300: {
    name: 'DM-300',
    desc: "The DM-300 is the largest and most powerful 'defense machine' that the dwarves ever built. Such an awesome machine is difficult to manufacture, so the dwarves only ever made a few to guard the entrances to their underground metropolis.\n\nIt is equipped with vents to jet its toxic exhaust fumes and a high power drill that it can use both to attack and disrupt the earth. DM-300 can also connect to an energy grid, further enhancing its power.",
  },
  DwarfKing: {
    name: 'King of Dwarves',
    desc: 'Many years ago one of the highest wizards in the dwarven court uncovered secrets which gave him tremendous power over life and death. He soon put this power to use, subjugating and corrupting his peers, his king, and eventually every dwarven citizen. Now he is king, ruler over a legion of undead subjects.\n\nThe King of Dwarves is an aggressive foe, who will attempt to surround and overwhelm his enemies with his horde of undead minions.',
  },
  YogDzewa: {
    name: 'Yog-Dzewa',
    desc: "Yog-Dzewa is an Old God; a great monstrosity which reaches into mortal worlds and devours them to fuel its endless growth. The dwarves must have attracted the attention of this cosmic horror while toying with dark magics, and have been trying to keep it contained ever since.\n\nYog-Dzewa originates from a dimension of raw magical chaos, and cannot bring its true form into this world. Instead it has created avatars and summoned demonic minions to do its bidding here.\n\nThis giant eye is the focal point of Yog-Dzewa's power in this world. While the eye itself is immobile, it is capable of shooting powerful beams of demonic magic and summoning a variety of minions to aid it.",
  },
}

export function gameObjectText(
  key: string | null | undefined
): GameObjectText | null {
  if (!key) return null
  return TEXTS[key] ?? null
}

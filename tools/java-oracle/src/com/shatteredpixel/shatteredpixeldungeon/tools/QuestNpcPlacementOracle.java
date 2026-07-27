/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.Mob;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Ghost;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Wandmaker;
import com.shatteredpixel.shatteredpixeldungeon.levels.PrisonLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.SewerLevel;
import com.watabou.utils.Random;

/** Records quest-NPC placement and the RNG stream immediately after rewards. */
final class QuestNpcPlacementOracle {

	private QuestNpcPlacementOracle() {
	}

	static String generateJson(String inputSeed, long numericSeed) {
		SpawnFact ghost = findGhost(numericSeed);
		SpawnFact wandmaker = findWandmaker(numericSeed);
		return toJson(inputSeed, numericSeed, ghost, wandmaker);
	}

	private static SpawnFact findGhost(long numericSeed) {
		for (int depth = 2; depth <= 4; depth++) {
			FloorOracle.initializeFreshRun(numericSeed);
			FloorOracle.generatePriorFloors(depth);
			FloorOracle.markTargetFloorGenerated(depth);
			ProbeSewerLevel level = new ProbeSewerLevel();
			createWithoutBones(level);
			if (level.fact != null) return level.fact;
		}
		throw new AssertionError("Ghost did not spawn by floor 4");
	}

	private static SpawnFact findWandmaker(long numericSeed) {
		for (int depth = 7; depth <= 9; depth++) {
			FloorOracle.initializeFreshRun(numericSeed);
			FloorOracle.generatePriorFloors(depth);
			FloorOracle.markTargetFloorGenerated(depth);
			ProbePrisonLevel level = new ProbePrisonLevel();
			createWithoutBones(level);
			if (level.fact != null) return level.fact;
		}
		throw new AssertionError("Wandmaker did not spawn by floor 9");
	}

	private static void createWithoutBones(com.shatteredpixel.shatteredpixeldungeon.levels.Level level) {
		Dungeon.daily = true;
		try {
			level.create();
		} finally {
			Dungeon.daily = false;
		}
	}

	private static int[] tail() {
		int[] result = new int[8];
		for (int i = 0; i < result.length; i++) result[i] = Random.Int();
		return result;
	}

	private static int npcCell(Iterable<Mob> mobs, Class<? extends Mob> type) {
		for (Mob mob : mobs) if (type.isInstance(mob)) return mob.pos;
		return -1;
	}

	private static String toJson(String inputSeed, long numericSeed, SpawnFact ghost, SpawnFact wandmaker) {
		return "{\n"
				+ "  \"schema_version\": 1,\n"
				+ "  \"contract\": \"quest_npc_placement\",\n"
				+ "  \"spd\": { \"version\": \"v3.3.8\", \"commit\": \"7b8b845a7\" },\n"
				+ "  \"input\": { \"seed\": \"" + JavaOracle.escape(inputSeed)
				+ "\", \"numeric\": " + numericSeed + " },\n"
				+ "  \"ghost\": " + ghost.toJson() + ",\n"
				+ "  \"wandmaker\": " + wandmaker.toJson() + "\n"
				+ "}\n";
	}

	private static final class ProbeSewerLevel extends SewerLevel {
		SpawnFact fact;

		@Override
		protected void createMobs() {
			Ghost.Quest.spawn(this, roomExit);
			int cell = npcCell(mobs, Ghost.class);
			if (cell >= 0) {
				fact = new SpawnFact(Dungeon.depth, Dungeon.depth - 1, cell, tail());
			}
		}
	}

	private static final class ProbePrisonLevel extends PrisonLevel {
		SpawnFact fact;

		@Override
		protected void createMobs() {
			Wandmaker.Quest.spawnWandmaker(this, roomEntrance);
			int cell = npcCell(mobs, Wandmaker.class);
			if (cell >= 0) {
				fact = new SpawnFact(Dungeon.depth, questType(), cell, tail());
			}
		}

		private int questType() {
			String roomName = rooms.stream()
					.map(room -> room.getClass().getSimpleName())
					.filter(name -> name.equals("MassGraveRoom") || name.equals("RitualSiteRoom") || name.equals("RotGardenRoom"))
					.findFirst().orElseThrow(() -> new AssertionError("Wandmaker quest room missing"));
			if (roomName.equals("MassGraveRoom")) return 1;
			if (roomName.equals("RitualSiteRoom")) return 2;
			return 3;
		}
	}

	private static final class SpawnFact {
		final int depth;
		final int questType;
		final int cell;
		final int[] rngTail;

		SpawnFact(int depth, int questType, int cell, int[] rngTail) {
			this.depth = depth;
			this.questType = questType;
			this.cell = cell;
			this.rngTail = rngTail;
		}

		String toJson() {
			StringBuilder tailJson = new StringBuilder("[");
			for (int i = 0; i < rngTail.length; i++) {
				if (i > 0) tailJson.append(", ");
				tailJson.append(rngTail[i]);
			}
			return "{ \"depth\": " + depth + ", \"quest_type\": " + questType
					+ ", \"cell\": " + cell + ", \"rng_tail\": " + tailJson + "] }";
		}
	}
}

/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.items.Generator;
import com.shatteredpixel.shatteredpixeldungeon.levels.CityLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;

import java.util.ArrayList;

/** Records the ring deck exactly around Ambitious Imp reward generation. */
final class ImpRingDeckOracle {

	private ImpRingDeckOracle() {
	}

	static String generateJson(String inputSeed, long numericSeed) {
		SpawnFact spawn = null;
		for (int depth = 17; depth <= 19; depth++) {
			FloorOracle.initializeFreshRun(numericSeed);
			FloorOracle.generatePriorFloors(depth);
			FloorOracle.markTargetFloorGenerated(depth);
			ProbeCityLevel level = new ProbeCityLevel();
			Dungeon.daily = true;
			try {
				level.create();
			} finally {
				Dungeon.daily = false;
			}
			if (level.before != level.after) {
				spawn = new SpawnFact(depth, level.before, level.after);
				break;
			}
		}
		if (spawn == null) throw new AssertionError("Imp did not spawn by floor 19");
		return toJson(inputSeed, numericSeed, spawn);
	}

	private static String toJson(String inputSeed, long numericSeed, SpawnFact spawn) {
		return "{\n"
				+ "  \"schema_version\": 1,\n"
				+ "  \"contract\": \"imp_ring_deck\",\n"
				+ "  \"spd\": { \"version\": \"v3.3.8\", \"commit\": \"7b8b845a7\" },\n"
				+ "  \"input\": { \"seed\": \"" + JavaOracle.escape(inputSeed)
				+ "\", \"numeric\": " + numericSeed + " },\n"
				+ "  \"spawn\": { \"depth\": " + spawn.depth
				+ ", \"ring_dropped_before\": " + spawn.before
				+ ", \"ring_dropped_after\": " + spawn.after + " }\n"
				+ "}\n";
	}

	private static final class ProbeCityLevel extends CityLevel {
		int before;
		int after;

		@Override
		protected ArrayList<Room> initRooms() {
			before = Generator.Category.RING.dropped;
			ArrayList<Room> rooms = super.initRooms();
			after = Generator.Category.RING.dropped;
			return rooms;
		}
	}

	private static final class SpawnFact {
		final int depth;
		final int before;
		final int after;

		SpawnFact(int depth, int before, int after) {
			this.depth = depth;
			this.before = before;
			this.after = after;
		}
	}
}

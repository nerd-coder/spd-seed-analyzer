/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Imp;
import com.shatteredpixel.shatteredpixeldungeon.items.Generator;
import com.shatteredpixel.shatteredpixeldungeon.items.Item;
import com.shatteredpixel.shatteredpixeldungeon.levels.CityLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.List;

/** Records Generator dropped counters and Imp rewardOptions around CityLevel.initRooms. */
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
			// Spawn always draws RING; the Quest.spawned flag is the v4 source of truth.
			if (level.spawned || level.before.ring != level.after.ring) {
				spawn = new SpawnFact(depth, level.before, level.after, level.rewards);
				break;
			}
		}
		if (spawn == null) throw new AssertionError("Imp did not spawn by floor 19");
		if (spawn.rewards.size() != 6) {
			throw new AssertionError("Imp rewardOptions size after initRooms: " + spawn.rewards.size());
		}
		return toJson(inputSeed, numericSeed, spawn);
	}

	private static String toJson(String inputSeed, long numericSeed, SpawnFact spawn) {
		StringBuilder json = new StringBuilder();
		json.append("{\n");
		json.append("  \"schema_version\": 1,\n");
		json.append("  \"contract\": \"imp_ring_deck\",\n");
		json.append("  \"spd\": ").append(JavaOracle.spdJson()).append(",\n");
		json.append("  \"input\": { \"seed\": \"").append(JavaOracle.escape(inputSeed))
				.append("\", \"numeric\": ").append(numericSeed).append(" },\n");
		json.append("  \"spawn\": {\n");
		json.append("    \"depth\": ").append(spawn.depth).append(",\n");
		appendDropped(json, "ring", spawn.before.ring, spawn.after.ring);
		appendDropped(json, "artifact", spawn.before.artifact, spawn.after.artifact);
		appendDropped(json, "wand", spawn.before.wand, spawn.after.wand);
		appendDropped(json, "wep_t4", spawn.before.wepT4, spawn.after.wepT4);
		appendDropped(json, "wep_t5", spawn.before.wepT5, spawn.after.wepT5);
		appendDropped(json, "mis_t4", spawn.before.misT4, spawn.after.misT4);
		appendDropped(json, "mis_t5", spawn.before.misT5, spawn.after.misT5);
		json.append("    \"reward_options\": [\n");
		for (int index = 0; index < spawn.rewards.size(); index++) {
			RewardFact reward = spawn.rewards.get(index);
			json.append("      { \"class\": \"").append(JavaOracle.escape(reward.className))
					.append("\", \"level\": ").append(reward.level)
					.append(", \"cursed\": ").append(reward.cursed).append(" }");
			if (index + 1 < spawn.rewards.size()) json.append(',');
			json.append('\n');
		}
		json.append("    ]\n");
		json.append("  }\n");
		json.append("}\n");
		return json.toString();
	}

	private static void appendDropped(StringBuilder json, String name, int before, int after) {
		json.append("    \"").append(name).append("_dropped_before\": ").append(before)
				.append(",\n");
		json.append("    \"").append(name).append("_dropped_after\": ").append(after)
				.append(",\n");
	}

	private static boolean questSpawned() {
		try {
			Field spawned = Imp.Quest.class.getDeclaredField("spawned");
			spawned.setAccessible(true);
			return spawned.getBoolean(null);
		} catch (ReflectiveOperationException error) {
			throw new AssertionError(error);
		}
	}

	private static List<RewardFact> copyRewards(List<Item> items) {
		List<RewardFact> rewards = new ArrayList<>();
		for (Item item : items) {
			rewards.add(new RewardFact(
					item.getClass().getSimpleName(), item.trueLevel(), item.cursed));
		}
		return rewards;
	}

	private static final class ProbeCityLevel extends CityLevel {
		DroppedCounters before;
		DroppedCounters after;
		boolean spawned;
		List<RewardFact> rewards = List.of();

		@Override
		protected ArrayList<Room> initRooms() {
			before = DroppedCounters.capture();
			ArrayList<Room> rooms = super.initRooms();
			after = DroppedCounters.capture();
			spawned = questSpawned();
			// Capture before VaultFinalRoom can clear the list on a later branch create.
			rewards = copyRewards(Imp.Quest.rewardOptions);
			return rooms;
		}
	}

	static final class DroppedCounters {
		final int ring;
		final int artifact;
		final int wand;
		final int wepT4;
		final int wepT5;
		final int misT4;
		final int misT5;

		DroppedCounters(
				int ring,
				int artifact,
				int wand,
				int wepT4,
				int wepT5,
				int misT4,
				int misT5) {
			this.ring = ring;
			this.artifact = artifact;
			this.wand = wand;
			this.wepT4 = wepT4;
			this.wepT5 = wepT5;
			this.misT4 = misT4;
			this.misT5 = misT5;
		}

		static DroppedCounters capture() {
			return new DroppedCounters(
					Generator.Category.RING.dropped,
					Generator.Category.ARTIFACT.dropped,
					Generator.Category.WAND.dropped,
					Generator.Category.WEP_T4.dropped,
					Generator.Category.WEP_T5.dropped,
					Generator.Category.MIS_T4.dropped,
					Generator.Category.MIS_T5.dropped);
		}
	}

	private static final class RewardFact {
		final String className;
		final int level;
		final boolean cursed;

		RewardFact(String className, int level, boolean cursed) {
			this.className = className;
			this.level = level;
			this.cursed = cursed;
		}
	}

	private static final class SpawnFact {
		final int depth;
		final DroppedCounters before;
		final DroppedCounters after;
		final List<RewardFact> rewards;

		SpawnFact(int depth, DroppedCounters before, DroppedCounters after, List<RewardFact> rewards) {
			this.depth = depth;
			this.before = before;
			this.after = after;
			this.rewards = rewards;
		}
	}
}

/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.actors.Actor;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Imp;
import com.shatteredpixel.shatteredpixeldungeon.items.Generator;
import com.shatteredpixel.shatteredpixeldungeon.levels.VaultLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;
import com.watabou.utils.Random;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.List;

/**
 * Forces a branch-1 VaultLevel and records RING/WAND/ARTIFACT.dropped around create().
 *
 * Vault equipment uses {@code Generator.randomUsingDefaults} only, so RING.dropped
 * (and ARTIFACT, which the vault never draws) must stay flat.
 */
final class VaultLevelOracle {

	private VaultLevelOracle() {
	}

	static String generateJson(String inputSeed, long numericSeed, int depth) {
		if (depth < 17 || depth > 19) {
			throw new IllegalArgumentException("VaultLevel depth must be 17 through 19");
		}

		FloorOracle.initializeFreshRun(numericSeed);
		Dungeon.depth = depth;
		Dungeon.branch = 1;
		Dungeon.generatedLevels.add(depth + 1000 * Dungeon.branch);
		forceImpQuestAccepted();

		DroppedCounters before = DroppedCounters.capture();
		RecordingVaultLevel level = new RecordingVaultLevel();
		Dungeon.level = null;
		Actor.clear();
		DroppedCounters after;
		List<Room> rooms = new ArrayList<>();
		try {
			level.create();
			after = DroppedCounters.capture();
			rooms.addAll(level.rooms());
		} catch (FloorOracle.SnapshotComplete expected) {
			// createMobs boundary: GridBuilder has placed rooms; paint has
			// shifted bounds. Room identities and bounds are stable either side
			// of paint (PR 2 records the list; PR 3 will add terrain).
			after = DroppedCounters.capture();
			rooms.addAll(level.rooms());
		} finally {
			Random.resetGenerators();
		}

		if (before.ring != after.ring) {
			throw new AssertionError(
					"VaultLevel moved RING.dropped: " + before.ring + " -> " + after.ring);
		}
		if (before.artifact != after.artifact) {
			throw new AssertionError(
					"VaultLevel moved ARTIFACT.dropped: " + before.artifact + " -> " + after.artifact);
		}
		if (before.wand != after.wand) {
			throw new AssertionError(
					"VaultLevel moved WAND.dropped: " + before.wand + " -> " + after.wand);
		}

		return toJson(inputSeed, numericSeed, depth, before, after, rooms);
	}

	private static String toJson(
			String inputSeed,
			long numericSeed,
			int depth,
			DroppedCounters before,
			DroppedCounters after,
			List<Room> rooms) {
		StringBuilder json = new StringBuilder();
		json.append("{\n")
				.append("  \"schema_version\": 1,\n")
				.append("  \"contract\": \"vault_level_using_defaults\",\n")
				.append("  \"spd\": ").append(JavaOracle.spdJson()).append(",\n")
				.append("  \"input\": { \"seed\": \"").append(JavaOracle.escape(inputSeed))
				.append("\", \"numeric\": ").append(numericSeed).append(" },\n")
				.append("  \"depth\": ").append(depth).append(",\n")
				.append("  \"branch\": 1,\n")
				.append("  \"imp_quest_forced\": true,\n")
				.append("  \"dropped\": {\n")
				.append("    \"RING\": { \"before\": ").append(before.ring)
				.append(", \"after\": ").append(after.ring).append(" },\n")
				.append("    \"WAND\": { \"before\": ").append(before.wand)
				.append(", \"after\": ").append(after.wand).append(" },\n")
				.append("    \"ARTIFACT\": { \"before\": ").append(before.artifact)
				.append(", \"after\": ").append(after.artifact).append(" }\n")
				.append("  },\n");
		appendRooms(json, rooms);
		return json.append("}\n").toString();
	}

	private static void appendRooms(StringBuilder json, List<Room> rooms) {
		json.append("  \"rooms\": [\n");
		for (int index = 0; index < rooms.size(); index++) {
			Room room = rooms.get(index);
			json.append("    { \"class\": \"").append(room.getClass().getSimpleName())
					.append("\", \"left\": ").append(room.left)
					.append(", \"top\": ").append(room.top)
					.append(", \"right\": ").append(room.right)
					.append(", \"bottom\": ").append(room.bottom).append(" }");
			if (index + 1 < rooms.size()) json.append(',');
			json.append('\n');
		}
		json.append("  ]\n");
	}

	/** Minimum flags so VaultLevel.create can run; no combat or score. */
	private static void forceImpQuestAccepted() {
		try {
			Field spawned = Imp.Quest.class.getDeclaredField("spawned");
			spawned.setAccessible(true);
			spawned.setBoolean(null, true);
			Field given = Imp.Quest.class.getDeclaredField("given");
			given.setAccessible(true);
			given.setBoolean(null, true);
			Field oldQuest = Imp.Quest.class.getDeclaredField("oldQuest");
			oldQuest.setAccessible(true);
			oldQuest.setBoolean(null, false);
		} catch (ReflectiveOperationException error) {
			throw new AssertionError(error);
		}
	}

	private static final class RecordingVaultLevel extends VaultLevel {
		@Override
		protected void createMobs() {
			throw new FloorOracle.SnapshotComplete();
		}
	}

	private static final class DroppedCounters {
		final int ring;
		final int wand;
		final int artifact;

		DroppedCounters(int ring, int wand, int artifact) {
			this.ring = ring;
			this.wand = wand;
			this.artifact = artifact;
		}

		static DroppedCounters capture() {
			return new DroppedCounters(
					Generator.Category.RING.dropped,
					Generator.Category.WAND.dropped,
					Generator.Category.ARTIFACT.dropped);
		}
	}
}

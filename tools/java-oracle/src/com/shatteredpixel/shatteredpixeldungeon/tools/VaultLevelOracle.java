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
import com.watabou.utils.Random;

import java.lang.reflect.Field;

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
		try {
			level.create();
			after = DroppedCounters.capture();
		} catch (FloorOracle.SnapshotComplete expected) {
			// Painter-complete: VaultLevel.build() already queued usingDefaults equipment.
			after = DroppedCounters.capture();
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

		return toJson(inputSeed, numericSeed, depth, before, after);
	}

	private static String toJson(
			String inputSeed,
			long numericSeed,
			int depth,
			DroppedCounters before,
			DroppedCounters after) {
		return "{\n"
				+ "  \"schema_version\": 1,\n"
				+ "  \"contract\": \"vault_level_using_defaults\",\n"
				+ "  \"spd\": " + JavaOracle.spdJson() + ",\n"
				+ "  \"input\": { \"seed\": \"" + JavaOracle.escape(inputSeed)
				+ "\", \"numeric\": " + numericSeed + " },\n"
				+ "  \"depth\": " + depth + ",\n"
				+ "  \"branch\": 1,\n"
				+ "  \"imp_quest_forced\": true,\n"
				+ "  \"dropped\": {\n"
				+ "    \"RING\": { \"before\": " + before.ring + ", \"after\": " + after.ring + " },\n"
				+ "    \"WAND\": { \"before\": " + before.wand + ", \"after\": " + after.wand + " },\n"
				+ "    \"ARTIFACT\": { \"before\": " + before.artifact
				+ ", \"after\": " + after.artifact + " }\n"
				+ "  }\n"
				+ "}\n";
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

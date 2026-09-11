/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Assets;
import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.actors.Actor;
import com.shatteredpixel.shatteredpixeldungeon.actors.blobs.VaultFlameTraps;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Imp;
import com.shatteredpixel.shatteredpixeldungeon.items.Generator;
import com.shatteredpixel.shatteredpixeldungeon.levels.Level;
import com.shatteredpixel.shatteredpixeldungeon.levels.Terrain;
import com.shatteredpixel.shatteredpixeldungeon.levels.VaultLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;
import com.shatteredpixel.shatteredpixeldungeon.tiles.CustomTilemap;
import com.shatteredpixel.shatteredpixeldungeon.tiles.DungeonTileSheet;
import com.shatteredpixel.shatteredpixeldungeon.tiles.custom.Carpet;
import com.watabou.utils.Random;
import com.watabou.utils.SparseArray;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.List;

/**
 * Forces a branch-1 VaultLevel and records RING/WAND/ARTIFACT.dropped plus
 * painter-complete layout at createMobs.
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

		Dungeon.level = level;
		DungeonTileSheet.setupVariance(level.length(), Dungeon.seedCurDepth());
		FloorVisualFacts visual = FloorVisualFacts.capture(level);
		return toJson(inputSeed, numericSeed, depth, before, after, rooms, level, visual);
	}

	private static String toJson(
			String inputSeed,
			long numericSeed,
			int depth,
			DroppedCounters before,
			DroppedCounters after,
			List<Room> rooms,
			VaultLevel level,
			FloorVisualFacts visual) {
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
				.append("  \"width\": ").append(level.width()).append(",\n")
				.append("  \"height\": ").append(level.height()).append(",\n")
				.append("  \"dropped\": {\n")
				.append("    \"RING\": { \"before\": ").append(before.ring)
				.append(", \"after\": ").append(after.ring).append(" },\n")
				.append("    \"WAND\": { \"before\": ").append(before.wand)
				.append(", \"after\": ").append(after.wand).append(" },\n")
				.append("    \"ARTIFACT\": { \"before\": ").append(before.artifact)
				.append(", \"after\": ").append(after.artifact).append(" }\n")
				.append("  },\n");
		appendRooms(json, rooms);
		json.append("  \"terrain\": [");
		appendInts(json, visual.terrain);
		json.append("],\n  \"discoverable\": [");
		appendBools(json, visual.discoverable);
		json.append("],\n  \"transitions\": [\n");
		JavaOracle.appendTransitions(json, visual.transitions);
		json.append("  ],\n  \"traps\": [\n");
		JavaOracle.appendTraps(json, visual.traps);
		json.append("  ],\n  \"blobs\": [\n");
		appendFlameBlobs(json, level);
		json.append("  ],\n  \"custom_tiles\": [\n");
		appendCustom(json, level.customTiles, level);
		json.append("  ],\n  \"custom_terrain\": [\n");
		appendCustom(json, level.customTerrain, level);
		json.append("  ]\n}\n");
		return json.toString();
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
		json.append("  ],\n");
	}

	private static void appendInts(StringBuilder json, List<Integer> values) {
		for (int index = 0; index < values.size(); index++) {
			if (index > 0) json.append(", ");
			json.append(values.get(index));
		}
	}

	private static void appendBools(StringBuilder json, List<Boolean> values) {
		for (int index = 0; index < values.size(); index++) {
			if (index > 0) json.append(", ");
			json.append(values.get(index));
		}
	}

	/**
	 * Flame traps seed amount 0, so {@code cur[]} is empty at createMobs.
	 * Pack layout cooldowns: initial | after&lt;&lt;8 | triggers&lt;&lt;16.
	 */
	private static void appendFlameBlobs(StringBuilder json, Level level) {
		VaultFlameTraps traps = (VaultFlameTraps) level.blobs.get(VaultFlameTraps.class);
		if (traps == null || traps.afterTriggerCooldowns == null) {
			return;
		}
		int volume = 0;
		StringBuilder cells = new StringBuilder();
		boolean first = true;
		for (int cell = 0; cell < traps.afterTriggerCooldowns.length; cell++) {
			if (traps.afterTriggerCooldowns[cell] < 0) continue;
			int value = traps.curCooldowns[cell]
					+ (traps.afterTriggerCooldowns[cell] << 8)
					+ (traps.triggersAfterCooldown[cell] << 16);
			volume += value;
			if (!first) cells.append(", ");
			first = false;
			cells.append("{ \"cell\": ").append(cell).append(", \"value\": ").append(value).append(" }");
		}
		json.append("        {\n");
		json.append("          \"class\": \"VaultFlameTraps\",\n");
		json.append("          \"volume\": ").append(volume).append(",\n");
		json.append("          \"always_visible\": false,\n");
		json.append("          \"cells\": [").append(cells).append("]\n        }\n");
	}

	private static void appendCustom(
			StringBuilder json, List<CustomTilemap> tiles, Level level) {
		for (int index = 0; index < tiles.size(); index++) {
			CustomTilemap tile = tiles.get(index);
			List<Integer> map = visualMap(tile, level);
			String texture = textureFor(tile);
			json.append("        { \"class\": \"").append(tile.getClass().getSimpleName())
					.append("\", \"x\": ").append(tile.tileX)
					.append(", \"y\": ").append(tile.tileY)
					.append(", \"width\": ").append(tile.tileW)
					.append(", \"height\": ").append(tile.tileH)
					.append(", \"texture\": \"").append(JavaOracle.escape(texture))
					.append("\", \"static_data\": [");
			for (int cell = 0; cell < map.size(); cell++) {
				if (cell > 0) json.append(", ");
				json.append(map.get(cell));
			}
			json.append("] }");
			if (index + 1 < tiles.size()) json.append(',');
			json.append('\n');
		}
	}

	private static String textureFor(CustomTilemap tile) {
		String name = tile.getClass().getSimpleName();
		if ("Carpet".equals(name)) return Assets.Environment.CARPET;
		return Assets.Environment.CITY_QUEST;
	}

	private static List<Integer> visualMap(CustomTilemap tile, Level level) {
		String name = tile.getClass().getSimpleName();
		if ("Carpet".equals(name)) return carpetMap((Carpet) tile);
		if ("MarkerTiles".equals(name)) return simpleImage(tile.tileW, tile.tileH, 5, 4, 256);
		if ("QuestEntranceInternal".equals(name)) return simpleImage(tile.tileW, tile.tileH, 8, 1, 256);
		if ("VaultTreasure".equals(name)) return vaultTreasureMap(tile, level);
		// WallBanners maps are render-time Random; layout only needs the layer rect.
		return new ArrayList<>();
	}

	private static List<Integer> simpleImage(int tileW, int tileH, int tx, int ty, int texW) {
		int texTileWidth = texW / 16;
		List<Integer> data = new ArrayList<>(tileW * tileH);
		int x = tx;
		int y = ty;
		for (int i = 0; i < tileW * tileH; i++) {
			data.add(x + texTileWidth * y);
			x++;
			if (x - tx == tileW) {
				x = tx;
				y++;
			}
		}
		return data;
	}

	@SuppressWarnings("unchecked")
	private static List<Integer> carpetMap(Carpet carpet) {
		int region = 16 * ((Dungeon.depth - 1) / 5);
		int tileW = carpet.tileW;
		int tileH = carpet.tileH;
		int[] data = new int[tileW * tileH];
		int i = 0;
		for (int y = 0; y < tileH; y++) {
			for (int x = 0; x < tileW; x++) {
				data[i] = region;
				if (y == 0) data[i] += 1;
				if (x == tileW - 1) data[i] += 2;
				if (y == tileH - 1) data[i] += 4;
				if (x == 0) data[i] += 8;
				i++;
			}
		}
		try {
			Field field = Carpet.class.getDeclaredField("tileOverrides");
			field.setAccessible(true);
			SparseArray<Integer> overrides = (SparseArray<Integer>) field.get(carpet);
			if (overrides != null) {
				for (int key : overrides.keyArray()) {
					data[key] = overrides.get(key);
				}
			}
		} catch (ReflectiveOperationException error) {
			throw new AssertionError(error);
		}
		List<Integer> out = new ArrayList<>(data.length);
		for (int value : data) out.add(value);
		return out;
	}

	private static List<Integer> vaultTreasureMap(CustomTilemap tile, Level level) {
		int tileW = tile.tileW;
		int tileH = tile.tileH;
		int tileX = tile.tileX;
		int tileY = tile.tileY;
		int width = level.width();
		List<Integer> data = new ArrayList<>(tileW * tileH);
		for (int i = 0; i < tileW * tileH; i++) {
			int value = -1;
			if (i < tileW) {
				if (i == 0) value = 5 * 16 + 4;
				if (i == tileW - 1) value = 5 * 16 + 3;
			} else {
				int cell = tileX + width * tileY + (i % tileW) + (i / tileW) * width;
				if (level.map[cell] == Terrain.PEDESTAL) {
					value = 7 * 16 + 3;
				} else if (level.map[cell - width] == Terrain.WALL
						|| level.map[cell - width] == Terrain.WALL_DECO) {
					value = 6 * 16 + 2;
					if (level.map[cell + 1] == Terrain.WALL) value += 1;
					else if (level.map[cell - 1] == Terrain.WALL) value += 2;
				} else if (level.map[cell + 1] == Terrain.WALL) {
					value = 6 * 16;
				} else if (level.map[cell - 1] == Terrain.WALL) {
					value = 6 * 16 + 1;
				} else {
					value = 7 * 16 + (DungeonTileSheet.tileVariance[cell] & 0xFF) / 34;
				}
			}
			data.add(value);
		}
		return data;
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

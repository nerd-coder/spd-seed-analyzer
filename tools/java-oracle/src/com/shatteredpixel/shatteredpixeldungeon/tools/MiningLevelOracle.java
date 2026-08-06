/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Assets;
import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.actors.Actor;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Blacksmith;
import com.shatteredpixel.shatteredpixeldungeon.levels.MiningLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.Level;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.quest.MineEntrance;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.quest.MineGiantRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.quest.MineLargeRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.quest.MineSecretRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.quest.MineSmallRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.standard.StandardRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.painters.MiningLevelPainter;
import com.shatteredpixel.shatteredpixeldungeon.levels.painters.Painter;
import com.shatteredpixel.shatteredpixeldungeon.tiles.CustomTilemap;
import com.watabou.utils.Random;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.List;

/** Captures a forced-objective MiningLevel immediately before mob population. */
final class MiningLevelOracle {

	private MiningLevelOracle() {
	}

	static String generateJson(String inputSeed, long numericSeed, int depth, String objective) {
		if (depth < 12 || depth > 14) {
			throw new IllegalArgumentException("MiningLevel depth must be 12 through 14");
		}
		int objectiveType;
		if ("crystal".equals(objective)) {
			objectiveType = Blacksmith.Quest.CRYSTAL;
		} else if ("gnoll".equals(objective)) {
			objectiveType = Blacksmith.Quest.GNOLL;
		} else {
			throw new IllegalArgumentException("MiningLevel objective must be crystal or gnoll");
		}

		FloorOracle.initializeFreshRun(numericSeed);
		Dungeon.depth = depth;
		Dungeon.branch = 1;
		Dungeon.generatedLevels.add(depth + 1000);
		forceObjective(objectiveType);
		RecordingMiningLevel level = new RecordingMiningLevel();
		// Dungeon.newLevel keeps this null until Level.create returns.
		Dungeon.level = null;
		Actor.clear();
		List<Integer> floorRngTail = new ArrayList<>();
		try {
			level.create();
		} catch (FloorOracle.SnapshotComplete expected) {
			// Painter-complete: Level.create has built flags but has not populated mobs or items.
			for (int index = 0; index < 8; index++) floorRngTail.add(Random.Int());
		} finally {
			Random.resetGenerators();
		}

		FloorVisualFacts visual = FloorVisualFacts.capture(level);
		StringBuilder json = new StringBuilder();
		json.append("{\n  \"schema_version\": 1,\n");
		json.append("  \"contract\": \"forced_blacksmith_mining_painter_complete\",\n");
		json.append("  \"spd\": { \"version\": \"v3.3.8\", \"commit\": \"7b8b845a7\" },\n");
		json.append("  \"input\": { \"seed\": \"").append(JavaOracle.escape(inputSeed))
				.append("\", \"numeric\": ").append(numericSeed).append(" },\n");
		json.append("  \"objective\": \"").append(objective).append("\",\n");
		json.append("  \"objective_forced\": true,\n");
		json.append("  \"depth\": ").append(depth).append(",\n");
		json.append("  \"branch\": 1,\n");
		json.append("  \"width\": ").append(level.width()).append(",\n");
		json.append("  \"height\": ").append(level.height()).append(",\n");
		appendRooms(json, "rooms", level.rooms());
		appendRooms(json, "paint_rooms", RecordingMiningLevel.paintRooms);
		appendCheckpoints(json, RecordingMiningLevel.roomCheckpoints);
		appendValues(json, "post_doors_rng", RecordingMiningLevel.postDoorsRng);
		appendValues(json, "pre_decoration_terrain", RecordingMiningLevel.preDecorationTerrain);
		appendValues(json, "floor_rng_tail", floorRngTail);
		appendValues(json, "terrain", visual.terrain);
		appendValues(json, "discoverable", visual.discoverable);
		json.append("  \"transitions\": [\n");
		JavaOracle.appendTransitions(json, visual.transitions);
		json.append("  ],\n  \"traps\": [\n");
		JavaOracle.appendTraps(json, visual.traps);
		json.append("  ],\n  \"plants\": [\n");
		JavaOracle.appendPlants(json, visual.plants);
		json.append("  ],\n  \"blobs\": [\n");
		JavaOracle.appendBlobs(json, visual.blobs);
		json.append("  ],\n");
		appendLayers(json, "custom_tiles", level.customTiles);
		appendLayers(json, "custom_walls", level.customWalls);
		json.setLength(json.length() - 2);
		return json.append("\n}\n").toString();
	}

	private static void forceObjective(int objective) {
		try {
			Field type = Blacksmith.Quest.class.getDeclaredField("type");
			type.setAccessible(true);
			type.setInt(null, objective);
		} catch (ReflectiveOperationException error) {
			throw new AssertionError(error);
		}
	}

	private static void appendRooms(StringBuilder json, String name, List<Room> rooms) {
		json.append("  \"").append(name).append("\": [\n");
		for (int index = 0; index < rooms.size(); index++) {
			Room room = rooms.get(index);
			json.append("    { \"order\": ").append(index).append(", \"class\": \"")
					.append(roomName(room)).append("\", \"left\": ")
					.append(room.left).append(", \"top\": ").append(room.top)
					.append(", \"right\": ").append(room.right)
					.append(", \"bottom\": ").append(room.bottom).append(" }");
			if (index + 1 < rooms.size()) json.append(',');
			json.append('\n');
		}
		json.append("  ],\n");
	}

	private static String roomName(Room room) {
		String name = room.getClass().getSimpleName();
		return name.startsWith("Trace") ? name.substring("Trace".length()) : name;
	}

	private static void appendCheckpoints(StringBuilder json, List<RoomCheckpoint> checkpoints) {
		json.append("  \"room_rng\": [\n");
		for (int index = 0; index < checkpoints.size(); index++) {
			RoomCheckpoint checkpoint = checkpoints.get(index);
			json.append("    { \"order\": ").append(index).append(", \"class\": \"")
					.append(roomName(checkpoint.room)).append("\", \"left\": ")
					.append(checkpoint.room.left).append(", \"top\": ").append(checkpoint.room.top)
					.append(", \"rng\": ").append(checkpoint.rng)
					.append(", \"doors\": ").append(checkpoint.doors).append(" }");
			if (index + 1 < checkpoints.size()) json.append(',');
			json.append('\n');
		}
		json.append("  ],\n");
	}

	private static void appendValues(StringBuilder json, String name, List<?> values) {
		json.append("  \"").append(name).append("\": [");
		for (int index = 0; index < values.size(); index++) {
			if (index > 0) json.append(", ");
			json.append(values.get(index));
		}
		json.append("],\n");
	}

	private static void appendLayers(StringBuilder json, String name, List<CustomTilemap> layers) {
		json.append("  \"").append(name).append("\": [\n");
		for (int index = 0; index < layers.size(); index++) {
			CustomTilemap layer = layers.get(index);
			List<Integer> map = semanticMap(layer);
			json.append("    { \"order\": ").append(index).append(", \"class\": \"")
					.append(layer.getClass().getSimpleName()).append("\", \"x\": ")
					.append(layer.tileX).append(", \"y\": ").append(layer.tileY)
					.append(", \"width\": ").append(layer.tileW)
					.append(", \"height\": ").append(layer.tileH)
					.append(", \"texture\": \"").append(Assets.Environment.CAVES_QUEST)
					.append("\", \"map\": [");
			for (int cell = 0; cell < map.size(); cell++) {
				if (cell > 0) json.append(", ");
				json.append(map.get(cell));
			}
			json.append("] }");
			if (index + 1 < layers.size()) json.append(',');
			json.append('\n');
		}
		json.append("  ],\n");
	}

	private static List<Integer> semanticMap(CustomTilemap layer) {
		List<Integer> map = new ArrayList<>();
		String name = layer.getClass().getSimpleName();
		for (int y = 0; y < layer.tileH; y++) {
			for (int x = 0; x < layer.tileW; x++) {
				int cell = y * layer.tileW + x;
				if ("QuestExit".equals(name)) {
					map.add(8 + x + y * 8);
				} else if ("BorderTopDarken".equals(name)) {
					map.add(1);
				} else if ("BorderWallsDarken".equals(name)) {
					map.add(x == 0 || x == layer.tileW - 1 ? 1
							: cell + 2 * layer.tileW > layer.tileW * layer.tileH ? 2 : -1);
				} else {
					throw new AssertionError("Unknown MiningLevel custom layer: " + name);
				}
			}
		}
		return map;
	}

	private static final class RecordingMiningLevel extends MiningLevel {
		private static List<Integer> postDoorsRng = List.of();
		private static List<Integer> preDecorationTerrain = List.of();
		private static List<Room> paintRooms = List.of();
		private static List<RoomCheckpoint> roomCheckpoints = new ArrayList<>();

		@Override
		protected ArrayList<Room> initRooms() {
			roomCheckpoints = new ArrayList<>();
			ArrayList<Room> rooms = new ArrayList<>();
			rooms.add(roomEntrance = new TraceMineEntrance());
			StandardRoom room = new TraceMineGiantRoom();
			room.setSizeCat();
			rooms.add(room);
			for (int index = 0; index < 3; index++) {
				room = new TraceMineLargeRoom();
				room.setSizeCat();
				rooms.add(room);
			}
			int count = Random.NormalIntRange(6, 8);
			for (int index = 0; index < count; index++) {
				room = new TraceMineSmallRoom();
				room.setSizeCat();
				rooms.add(room);
			}
			rooms.add(new TraceMineSecretRoom());
			rooms.add(new TraceMineSecretRoom());
			return rooms;
		}

		@Override
		protected Painter painter() {
			TraceMiningPainter painter = new TraceMiningPainter();
			painter.setGold(Random.NormalIntRange(45, 47));
			painter.setWater(0.35f, 6);
			painter.setGrass(0.10f, 3);
			return painter;
		}

		@Override
		protected void createMobs() {
			throw new FloorOracle.SnapshotComplete();
		}
	}

	private static void record(Room room) {
		RecordingMiningLevel.roomCheckpoints.add(new RoomCheckpoint(room, FigureEightTraceOracle.probe()));
	}

	private static final class RoomCheckpoint {
		final Room room;
		final List<Integer> rng;
		final List<String> doors;

		RoomCheckpoint(Room room, List<Integer> rng) {
			this.room = room;
			this.rng = rng;
			this.doors = new ArrayList<>();
			for (Room.Door door : room.connected.values()) {
				doors.add("\"" + door.x + "," + door.y + "," + door.type + "\"");
			}
		}
	}

	private static final class TraceMineEntrance extends MineEntrance {
		@Override public void paint(Level level) { super.paint(level); record(this); }
	}

	private static final class TraceMineGiantRoom extends MineGiantRoom {
		@Override public void paint(Level level) { super.paint(level); record(this); }
	}

	private static final class TraceMineLargeRoom extends MineLargeRoom {
		@Override public void paint(Level level) { super.paint(level); record(this); }
	}

	private static final class TraceMineSmallRoom extends MineSmallRoom {
		@Override public void paint(Level level) { super.paint(level); record(this); }
	}

	private static final class TraceMineSecretRoom extends MineSecretRoom {
		@Override public void paint(Level level) { super.paint(level); record(this); }
	}

	private static final class TraceMiningPainter extends MiningLevelPainter {
		@Override
		protected void paintDoors(com.shatteredpixel.shatteredpixeldungeon.levels.Level level,
				ArrayList<Room> rooms) {
			super.paintDoors(level, rooms);
			RecordingMiningLevel.postDoorsRng = FigureEightTraceOracle.probe();
			RecordingMiningLevel.paintRooms = new ArrayList<>(rooms);
			RecordingMiningLevel.preDecorationTerrain = new ArrayList<>(level.length());
			for (int tile : level.map) RecordingMiningLevel.preDecorationTerrain.add(tile);
		}
	}
}

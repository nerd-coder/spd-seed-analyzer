/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.ShatteredPixelDungeon;
import com.shatteredpixel.shatteredpixeldungeon.levels.HallsLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.Level;
import com.shatteredpixel.shatteredpixeldungeon.levels.SewerLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.PrisonLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.CavesLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.CityLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.builders.Builder;
import com.shatteredpixel.shatteredpixeldungeon.levels.builders.FigureEightBuilder;
import com.shatteredpixel.shatteredpixeldungeon.levels.builders.LoopBuilder;
import com.shatteredpixel.shatteredpixeldungeon.levels.Terrain;
import com.shatteredpixel.shatteredpixeldungeon.levels.painters.HallsPainter;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.special.SpecialRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.special.CrystalVaultRoom;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.CrystalMimic;
import com.shatteredpixel.shatteredpixeldungeon.items.Generator;
import com.shatteredpixel.shatteredpixeldungeon.items.Heap;
import com.shatteredpixel.shatteredpixeldungeon.items.Item;
import com.watabou.noosa.Game;
import com.watabou.utils.Point;
import com.watabou.utils.Random;
import com.watabou.utils.Rect;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.lang.reflect.Field;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

/** Captures non-advancing main-RNG probes after every depth-22 room callback. */
final class HallsPaintTraceOracle {

	private HallsPaintTraceOracle() {
	}

	static String generate(long seed, int depth) {
		if ((depth < 2 || depth > 4) && (depth < 6 || depth > 9) && depth != 12 && (depth < 16 || depth > 19)
				&& (depth < 21 || depth > 24)) {
			throw new IllegalArgumentException("Paint trace supports regular Sewer depths 2 through 4, Prison depths 6 through 9, depth 12, regular City depths 16 through 19, or regular Halls depths 21 through 24");
		}
		FloorOracle.initializeFreshRun(seed);
		FloorOracle.generatePriorFloors(depth);
		String generatorState = generatorState();
		FloorOracle.markTargetFloorGenerated(depth);
		TracePainter.checkpoints.clear();
		TracePainter.preShuffleRooms = null;
		TracePainter.crystalVaults.clear();
		TracePopulationHallsLevel.preMobsRng = null;
		TracePopulationHallsLevel.preItemsRng = null;
		TraceBuilder.attempts.clear();
		TraceLoopBuilder.attempts.clear();
		TraceHallsLevel.builderKind = null;
		TracePrisonLevel.drops.clear();
		Dungeon.daily = true;
		try {
			if (depth <= 4) new TraceSewerLevel().create();
			else if (depth <= 9) new TracePrisonLevel().create();
			else if (depth == 12) new TraceCavesLevel().create();
			else if (depth <= 19) new TraceCityLevel().create();
			else new TraceHallsLevel().create();
		} catch (FloorOracle.SnapshotComplete expected) {
			// The trace ends immediately after RegularPainter.paintDoors.
		} finally {
			Dungeon.daily = false;
		}
		if (depth >= 21 && depth <= 24) {
			FloorOracle.initializeFreshRun(seed);
			FloorOracle.generatePriorFloors(depth);
			FloorOracle.markTargetFloorGenerated(depth);
			Dungeon.daily = true;
			try {
				new TracePopulationHallsLevel().create();
			} catch (FloorOracle.SnapshotComplete expected) {
				// The population trace ends at the createItems entry boundary.
			} finally {
				Dungeon.daily = false;
			}
		}
		StringBuilder json = new StringBuilder("{\n  \"depth\": ").append(depth)
				.append(",\n  \"build_attempts\": [");
		List<String> builderAttempts = ("LoopBuilder".equals(TraceHallsLevel.builderKind)
				|| depth == 12) ? TraceLoopBuilder.attempts : TraceBuilder.attempts;
		for (int i = 0; i < builderAttempts.size(); i++) {
			if (i > 0) json.append(',');
			json.append("\n    ").append(builderAttempts.get(i));
		}
		json.append("\n  ],\n  \"builder_kind\": ")
				.append(TraceHallsLevel.builderKind == null ? "null" : "\"" + TraceHallsLevel.builderKind + "\"")
				.append(",\n  \"pre_shuffle_rooms\": ").append(TracePainter.preShuffleRooms)
				.append(",\n  \"pre_mobs_rng\": ").append(TracePopulationHallsLevel.preMobsRng)
				.append(",\n  \"pre_items_rng\": ").append(TracePopulationHallsLevel.preItemsRng)
				.append(",\n  \"crystal_vaults\": ").append(TracePainter.crystalVaults)
				.append(",\n  \"drops\": ").append(TracePrisonLevel.drops)
				.append(",\n  \"blacksmith_state\": ").append(blacksmithState())
				.append(",\n  \"generator_state\": ").append(generatorState)
				.append(",\n  \"checkpoints\": [");
		for (int i = 0; i < TracePainter.checkpoints.size(); i++) {
			if (i > 0) json.append(',');
			json.append("\n    ").append(TracePainter.checkpoints.get(i));
		}
		return json.append("\n  ]\n}\n").toString();
	}

	@SuppressWarnings("unchecked")
	private static String blacksmithState() {
		try {
			Class<?> quest = Class.forName(
					"com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Blacksmith$Quest");
			Field rewardsField = quest.getDeclaredField("smithRewards");
			Field enchantField = quest.getDeclaredField("smithEnchant");
			Field glyphField = quest.getDeclaredField("smithGlyph");
			List<Item> rewards = (List<Item>) rewardsField.get(null);
			StringBuilder json = new StringBuilder("{\"rewards\":[");
			if (rewards != null) {
				for (int i = 0; i < rewards.size(); i++) {
					if (i > 0) json.append(',');
					Item item = rewards.get(i);
					json.append("{\"class\":\"").append(item.getClass().getSimpleName())
							.append("\",\"level\":").append(item.trueLevel())
							.append(",\"quantity\":").append(item.quantity()).append('}');
				}
			}
			Object enchant = enchantField.get(null);
			Object glyph = glyphField.get(null);
			return json.append("],\"enchant\":")
					.append(enchant == null ? "null" : "\"" + enchant.getClass().getSimpleName() + "\"")
					.append(",\"glyph\":")
					.append(glyph == null ? "null" : "\"" + glyph.getClass().getSimpleName() + "\"")
					.append('}').toString();
		} catch (ReflectiveOperationException error) {
			throw new AssertionError(error);
		}
	}

	/** Exact persistent Generator decks at the target floor boundary. */
	@SuppressWarnings("unchecked")
	private static String generatorState() {
		try {
			Field usingFirst = Generator.class.getDeclaredField("usingFirstDeck");
			usingFirst.setAccessible(true);
			Field categoryProbsField = Generator.class.getDeclaredField("categoryProbs");
			categoryProbsField.setAccessible(true);
			Map<Generator.Category, Float> categoryProbs =
					(Map<Generator.Category, Float>) categoryProbsField.get(null);
			StringBuilder json = new StringBuilder("{\"using_first_deck\":")
					.append(usingFirst.getBoolean(null)).append(",\"category_probs\":[");
			Generator.Category[] categories = Generator.Category.values();
			for (int i = 0; i < categories.length; i++) {
				if (i > 0) json.append(',');
				json.append(categoryProbs.get(categories[i]));
			}
			json.append("],\"categories\":[");
			for (int i = 0; i < categories.length; i++) {
				if (i > 0) json.append(',');
				Generator.Category category = categories[i];
				json.append("{\"probs\":[");
				for (int j = 0; j < category.probs.length; j++) {
					if (j > 0) json.append(',');
					json.append(category.probs[j]);
				}
				json.append("],\"using_2nd_probs\":").append(category.using2ndProbs)
						.append(",\"seed\":").append(category.seed)
						.append(",\"dropped\":").append(category.dropped).append('}');
			}
			return json.append("]}").toString();
		} catch (ReflectiveOperationException error) {
			throw new AssertionError(error);
		}
	}

	private static final class TraceHallsLevel extends HallsLevel {
		static String builderKind;

		@Override protected Builder builder() {
			Builder selected = super.builder();
			if (selected instanceof LoopBuilder) {
				builderKind = "LoopBuilder";
				try {
					Field intensity = LoopBuilder.class.getDeclaredField("curveIntensity");
					Field offset = LoopBuilder.class.getDeclaredField("curveOffset");
					intensity.setAccessible(true);
					offset.setAccessible(true);
					return new TraceLoopBuilder().setLoopShape(
							2, intensity.getFloat(selected), offset.getFloat(selected));
				} catch (ReflectiveOperationException error) {
					throw new AssertionError(error);
				}
			}
			if (!(selected instanceof FigureEightBuilder)) return selected;
			builderKind = "FigureEightBuilder";
			try {
				Field intensity = FigureEightBuilder.class.getDeclaredField("curveIntensity");
				intensity.setAccessible(true);
				return new TraceBuilder().setLoopShape(2, intensity.getFloat(selected), 0f);
			} catch (ReflectiveOperationException error) {
				throw new AssertionError(error);
			}
		}

		@Override protected com.shatteredpixel.shatteredpixeldungeon.levels.painters.Painter painter() {
			return new TracePainter()
					.setWater(feeling == Level.Feeling.WATER ? 0.70f : 0.15f, 6)
					.setGrass(feeling == Level.Feeling.GRASS ? 0.65f : 0.10f, 3)
					.setTraps(nTraps(), trapClasses(), trapChances());
		}

	}

	/** Runs the unmodified Halls painter before stopping at population checkpoints. */
	private static final class TracePopulationHallsLevel extends HallsLevel {
		static String preMobsRng;
		static String preItemsRng;

		@Override protected void createMobs() {
			preMobsRng = probe().toString();
			super.createMobs();
		}

		@Override protected void createItems() {
			preItemsRng = probe().toString();
			throw new FloorOracle.SnapshotComplete();
		}
	}

	private static final class TracePrisonLevel extends PrisonLevel {
		static final List<String> drops = new ArrayList<>();

		@Override public Heap drop(Item item, int cell) {
			drops.add("{\"cell\":" + cell + ",\"class\":\""
					+ item.getClass().getSimpleName() + "\"}");
			return super.drop(item, cell);
		}

		@Override protected com.shatteredpixel.shatteredpixeldungeon.levels.painters.Painter painter() {
			return new TracePainter()
					.setWater(0f, 0)
					.setGrass(0f, 0)
					.setTraps(nTraps(), trapClasses(), trapChances());
		}
	}

	private static final class TraceSewerLevel extends SewerLevel {
		@Override protected com.shatteredpixel.shatteredpixeldungeon.levels.painters.Painter painter() {
			return new TracePainter()
					.setWater(feeling == Level.Feeling.WATER ? 0.85f : 0.30f, 5)
					.setGrass(feeling == Level.Feeling.GRASS ? 0.80f : 0.20f, 4)
					.setTraps(nTraps(), trapClasses(), trapChances());
		}
	}

	private static final class TraceCavesLevel extends CavesLevel {
		@Override protected Builder builder() {
			Builder selected = super.builder();
			if (!(selected instanceof LoopBuilder)) return selected;
			try {
				Field intensity = LoopBuilder.class.getDeclaredField("curveIntensity");
				Field offset = LoopBuilder.class.getDeclaredField("curveOffset");
				intensity.setAccessible(true);
				offset.setAccessible(true);
				return new TraceLoopBuilder().setLoopShape(
						2, intensity.getFloat(selected), offset.getFloat(selected));
			} catch (ReflectiveOperationException error) {
				throw new AssertionError(error);
			}
		}
	}

	/** City uses the same RegularPainter lifecycle; only `nTraps()` must run before tracing. */
	private static final class TraceCityLevel extends CityLevel {
		@Override protected Builder builder() {
			Builder selected = super.builder();
			if (selected instanceof LoopBuilder) {
				TraceHallsLevel.builderKind = "LoopBuilder";
				try {
					Field intensity = LoopBuilder.class.getDeclaredField("curveIntensity");
					Field offset = LoopBuilder.class.getDeclaredField("curveOffset");
					intensity.setAccessible(true);
					offset.setAccessible(true);
					return new TraceLoopBuilder().setLoopShape(
							2, intensity.getFloat(selected), offset.getFloat(selected));
				} catch (ReflectiveOperationException error) {
					throw new AssertionError(error);
				}
			}
			if (!(selected instanceof FigureEightBuilder)) return selected;
			TraceHallsLevel.builderKind = "FigureEightBuilder";
			try {
				Field intensity = FigureEightBuilder.class.getDeclaredField("curveIntensity");
				intensity.setAccessible(true);
				return new TraceBuilder().setLoopShape(2, intensity.getFloat(selected), 0f);
			} catch (ReflectiveOperationException error) {
				throw new AssertionError(error);
			}
		}

		@Override protected com.shatteredpixel.shatteredpixeldungeon.levels.painters.Painter painter() {
			return new TracePainter()
					.setWater(feeling == Level.Feeling.WATER ? 0.90f : 0.30f, 4)
					.setGrass(feeling == Level.Feeling.GRASS ? 0.80f : 0.20f, 3)
					.setTraps(nTraps(), trapClasses(), trapChances());
		}
	}

	/** Records the FigureEightBuilder retry boundary without consuming RNG. */
	private static final class TraceBuilder extends FigureEightBuilder {
		static final List<String> attempts = new ArrayList<>();

		@Override public ArrayList<Room> build(ArrayList<Room> rooms) {
			List<Integer> start = probe();
			ArrayList<Room> result = super.build(rooms);
			attempts.add("{\"attempt\":" + attempts.size()
					+ ",\"start_rng\":" + start + ",\"end_rng\":" + probe()
					+ ",\"success\":" + (result != null) + "}");
			return result;
		}
	}

	private static final class TraceLoopBuilder extends LoopBuilder {
		static final List<String> attempts = new ArrayList<>();

		@Override public ArrayList<Room> build(ArrayList<Room> rooms) {
			List<Integer> start = probe();
			ArrayList<Room> result = super.build(rooms);
			StringBuilder json = new StringBuilder("{\"attempt\":")
					.append(attempts.size()).append(",\"start_rng\":").append(start)
					.append(",\"end_rng\":").append(probe()).append(",\"success\":")
					.append(result != null).append(",\"rooms\":[");
			for (int i = 0; i < rooms.size(); i++) {
				if (i > 0) json.append(',');
				Room room = rooms.get(i);
				json.append("{\"class\":\"").append(room.getClass().getSimpleName())
						.append("\",\"bounds\":[").append(room.left).append(',').append(room.top)
						.append(',').append(room.right).append(',').append(room.bottom).append("]}");
			}
			attempts.add(json.append("]}").toString());
			return result;
		}
	}

	/** Exact pinned RegularPainter room/door lifecycle, with only checkpoints added. */
	private static final class TracePainter extends HallsPainter {
		static final List<String> checkpoints = new ArrayList<>();
		static String preShuffleRooms;
		static final List<String> crystalVaults = new ArrayList<>();

		@Override public boolean paint(Level level, ArrayList<Room> rooms) {
			int padding = padding(level);
			int leftMost = Integer.MAX_VALUE, topMost = Integer.MAX_VALUE;
			for (Room room : rooms) {
				if (room.left < leftMost) leftMost = room.left;
				if (room.top < topMost) topMost = room.top;
			}
			leftMost -= padding;
			topMost -= padding;
			int rightMost = 0, bottomMost = 0;
			for (Room room : rooms) {
				room.shift(-leftMost, -topMost);
				if (room.right > rightMost) rightMost = room.right;
				if (room.bottom > bottomMost) bottomMost = room.bottom;
			}
			level.setSize(rightMost + padding + 1, bottomMost + padding + 1);
			preShuffleRooms = roomList(rooms);
			Random.shuffle(rooms);
			for (Room room : rooms.toArray(new Room[0])) {
				if (room.connected.isEmpty()) {
					Game.reportException(new RuntimeException("Painting a room with no connections!"));
					if (room instanceof SpecialRoom) return false;
				}
				placeDoors(room);
				room.paint(level);
				if (room instanceof CrystalVaultRoom) crystalVaults.add(crystalVault(level, (CrystalVaultRoom) room));
				checkpoint("room", room.getClass().getSimpleName());
			}
			paintDoors(level, rooms);
			checkpoint("doors", "paintDoors");
			throw new FloorOracle.SnapshotComplete();
		}

		private void placeDoors(Room room) {
			for (Room neighbour : room.connected.keySet()) {
				Room.Door door = room.connected.get(neighbour);
				if (door != null) continue;
				Rect intersection = room.intersect(neighbour);
				ArrayList<Point> spots = new ArrayList<>();
				for (Point point : intersection.getPoints()) {
					if (room.canConnect(point) && neighbour.canConnect(point)) spots.add(point);
				}
				if (spots.isEmpty()) {
					ShatteredPixelDungeon.reportException(new RuntimeException("Could not place a door!"));
					continue;
				}
				door = new Room.Door(Random.element(spots));
				room.connected.put(neighbour, door);
				neighbour.connected.put(room, door);
			}
		}

		private static void checkpoint(String stage, String room) {
			checkpoints.add("{\"stage\":\"" + stage + "\",\"room\":\"" + room
					+ "\",\"rng\":" + probe() + "}");
		}

		private static String roomList(List<Room> rooms) {
			StringBuilder json = new StringBuilder("[");
			for (int i = 0; i < rooms.size(); i++) {
				if (i > 0) json.append(',');
				Room room = rooms.get(i);
				json.append("{\"class\":\"").append(room.getClass().getSimpleName())
						.append("\",\"bounds\":[").append(room.left).append(',').append(room.top)
						.append(',').append(room.right).append(',').append(room.bottom).append("]}");
			}
			return json.append(']').toString();
		}

		@SuppressWarnings("unchecked")
		private static String crystalVault(Level level, CrystalVaultRoom room) {
			try {
				Field prizeClasses = CrystalVaultRoom.class.getDeclaredField("prizeClasses");
				prizeClasses.setAccessible(true);
				List<Generator.Category> categories = (List<Generator.Category>) prizeClasses.get(room);
				StringBuilder json = new StringBuilder("{\"post_prize_classes\":[");
				for (int i = 0; i < categories.size(); i++) {
					if (i > 0) json.append(',');
					json.append('\"').append(categories.get(i).name()).append('\"');
				}
				json.append("],\"crystal_chests\":[");
				boolean first = true;
				for (Heap heap : level.heaps.valueList()) {
					Point point = level.cellToPoint(heap.pos);
					if (heap.type != Heap.Type.CRYSTAL_CHEST || !room.inside(point)) continue;
					if (!first) json.append(',');
					first = false;
					json.append("{\"cell\":").append(heap.pos).append(",\"item\":\"")
							.append(heap.items.getFirst().getClass().getSimpleName()).append("\"}");
				}
				json.append("],\"crystal_mimics\":[");
				first = true;
				for (com.shatteredpixel.shatteredpixeldungeon.actors.mobs.Mob mob : level.mobs) {
					Point point = level.cellToPoint(mob.pos);
					if (!(mob instanceof CrystalMimic) || !room.inside(point)) continue;
					if (!first) json.append(',');
					first = false;
					json.append(mob.pos);
				}
				return json.append("]}").toString();
			} catch (ReflectiveOperationException error) {
				throw new AssertionError(error);
			}
		}
	}

	@SuppressWarnings("unchecked")
	private static List<Integer> probe() {
		try {
			Field generators = Random.class.getDeclaredField("generators");
			generators.setAccessible(true);
			java.util.Random current = ((ArrayDeque<java.util.Random>) generators.get(null)).peekFirst();
			ByteArrayOutputStream bytes = new ByteArrayOutputStream();
			new ObjectOutputStream(bytes).writeObject(current);
			java.util.Random copy = (java.util.Random) new ObjectInputStream(
					new ByteArrayInputStream(bytes.toByteArray())).readObject();
			List<Integer> values = new ArrayList<>();
			for (int i = 0; i < 8; i++) values.add(copy.nextInt());
			return values;
		} catch (Exception error) {
			throw new AssertionError(error);
		}
	}
}

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
import com.shatteredpixel.shatteredpixeldungeon.levels.Terrain;
import com.shatteredpixel.shatteredpixeldungeon.levels.painters.HallsPainter;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.special.SpecialRoom;
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

/** Captures non-advancing main-RNG probes after every depth-22 room callback. */
final class HallsPaintTraceOracle {

	private HallsPaintTraceOracle() {
	}

	static String generate(long seed, int depth) {
		if (depth != 22) throw new IllegalArgumentException("Halls paint trace only supports depth 22");
		FloorOracle.initializeFreshRun(seed);
		FloorOracle.generatePriorFloors(depth);
		FloorOracle.markTargetFloorGenerated(depth);
		TracePainter.checkpoints.clear();
		Dungeon.daily = true;
		try {
			new TraceHallsLevel().create();
		} catch (FloorOracle.SnapshotComplete expected) {
			// The trace ends immediately after RegularPainter.paintDoors.
		} finally {
			Dungeon.daily = false;
		}
		StringBuilder json = new StringBuilder("{\n  \"depth\": ").append(depth)
				.append(",\n  \"checkpoints\": [");
		for (int i = 0; i < TracePainter.checkpoints.size(); i++) {
			if (i > 0) json.append(',');
			json.append("\n    ").append(TracePainter.checkpoints.get(i));
		}
		return json.append("\n  ]\n}\n").toString();
	}

	private static final class TraceHallsLevel extends HallsLevel {
		@Override protected com.shatteredpixel.shatteredpixeldungeon.levels.painters.Painter painter() {
			return new TracePainter();
		}
	}

	/** Exact pinned RegularPainter room/door lifecycle, with only checkpoints added. */
	private static final class TracePainter extends HallsPainter {
		static final List<String> checkpoints = new ArrayList<>();

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
			Random.shuffle(rooms);
			for (Room room : rooms.toArray(new Room[0])) {
				if (room.connected.isEmpty()) {
					Game.reportException(new RuntimeException("Painting a room with no connections!"));
					if (room instanceof SpecialRoom) return false;
				}
				placeDoors(room);
				room.paint(level);
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

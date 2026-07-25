/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.levels.HallsLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.builders.Builder;
import com.shatteredpixel.shatteredpixeldungeon.levels.builders.FigureEightBuilder;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;
import com.watabou.utils.Random;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.lang.reflect.Field;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;

/** Records every pinned FigureEightBuilder call without advancing its RNG. */
final class FigureEightTraceOracle {

	private FigureEightTraceOracle() {
	}

	static String generate(long seed, int depth) {
		FloorOracle.initializeFreshRun(seed);
		FloorOracle.generatePriorFloors(depth);
		FloorOracle.markTargetFloorGenerated(depth);
		TraceBuilder.attempts.clear();
		Dungeon.daily = true;
		try {
			new TraceHallsLevel().create();
		} finally {
			Dungeon.daily = false;
		}
		StringBuilder json = new StringBuilder("{\n  \"depth\": ").append(depth)
				.append(",\n  \"attempts\": [");
		for (int i = 0; i < TraceBuilder.attempts.size(); i++) {
			if (i > 0) json.append(',');
			json.append("\n    ").append(TraceBuilder.attempts.get(i));
		}
		return json.append("\n  ]\n}\n").toString();
	}

	private static final class TraceHallsLevel extends HallsLevel {
		@Override
		protected Builder builder() {
			Builder selected = super.builder();
			if (!(selected instanceof FigureEightBuilder)) return selected;
			try {
				Field intensity = FigureEightBuilder.class.getDeclaredField("curveIntensity");
				intensity.setAccessible(true);
				return new TraceBuilder().setLoopShape(2, intensity.getFloat(selected), 0f);
			} catch (ReflectiveOperationException error) {
				throw new AssertionError(error);
			}
		}
	}

	private static final class TraceBuilder extends FigureEightBuilder {
		static final List<String> attempts = new ArrayList<>();

		@Override
		public ArrayList<Room> build(ArrayList<Room> rooms) {
			List<Integer> start = probe();
			ArrayList<Room> result = super.build(rooms);
			List<Integer> end = probe();
			StringBuilder json = new StringBuilder("{\"attempt\":")
					.append(attempts.size()).append(",\"start_rng\":").append(start)
					.append(",\"end_rng\":").append(end).append(",\"failure_stage\":")
					.append(failureStage(result)).append(",\"success\":")
					.append(result != null).append(",\"rooms\":[");
			for (int i = 0; i < rooms.size(); i++) {
				Room room = rooms.get(i);
				if (i > 0) json.append(',');
				json.append("{\"class\":\"").append(room.getClass().getSimpleName())
						.append("\",\"bounds\":[").append(room.left).append(',').append(room.top)
						.append(',').append(room.right).append(',').append(room.bottom).append("]}");
			}
			attempts.add(json.append("]}").toString());
			return result;
		}

		private String failureStage(ArrayList<Room> result) {
			if (result != null) return "null";
			try {
				Field firstLoop = FigureEightBuilder.class.getDeclaredField("firstLoop");
				firstLoop.setAccessible(true);
				for (Room room : (ArrayList<Room>) firstLoop.get(this)) {
					if (room.width() == 1 && room.height() == 1) return "\"first_loop\"";
				}
				return "\"second_loop\"";
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

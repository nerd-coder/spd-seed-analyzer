/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.levels.Level;
import com.shatteredpixel.shatteredpixeldungeon.levels.PrisonLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.SewerLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.painters.Painter;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;
import com.watabou.utils.Random;

import java.util.ArrayList;
import java.util.List;

/** Recording level variants for schema-v3 lifecycle boundary probes. */
final class FloorProbeLevels {

	interface Probe {
		Level level();

		List<Integer> rngProbe();
	}

	private FloorProbeLevels() {
	}

	static Probe prePaint(int depth) {
		if (depth == 1) return new PrePaintSewerLevel();
		if (depth >= 6 && depth <= 8) return new PrePaintPrisonLevel();
		throw unsupported(depth);
	}

	static Probe preMobs(int depth) {
		if (depth == 1) return new PreMobsSewerLevel();
		if (depth >= 6 && depth <= 8) return new PreMobsPrisonLevel();
		throw unsupported(depth);
	}

	static Probe preItems(int depth) {
		if (depth == 1) return new PreItemsSewerLevel();
		if (depth >= 6 && depth <= 8) return new PreItemsPrisonLevel();
		throw unsupported(depth);
	}

	private static IllegalArgumentException unsupported(int depth) {
		return new IllegalArgumentException("Unsupported probe depth: " + depth);
	}

	private static List<Integer> captureRng() {
		List<Integer> result = new ArrayList<>();
		for (int index = 0; index < 8; index++) {
			result.add(Random.Int());
		}
		return result;
	}

	private static Painter recordingPainter(ProbeOwner owner) {
		return new Painter() {
			@Override
			public boolean paint(Level level, ArrayList<Room> rooms) {
				owner.setRngProbe(captureRng());
				throw new FloorOracle.SnapshotComplete();
			}
		};
	}

	private interface ProbeOwner {
		void setRngProbe(List<Integer> probe);
	}

	private static final class PrePaintSewerLevel extends SewerLevel
			implements Probe, ProbeOwner {
		private List<Integer> rngProbe;

		@Override
		protected Painter painter() {
			super.painter();
			return recordingPainter(this);
		}

		@Override
		public Level level() {
			return this;
		}

		@Override
		public List<Integer> rngProbe() {
			return rngProbe;
		}

		@Override
		public void setRngProbe(List<Integer> probe) {
			rngProbe = probe;
		}
	}

	private static final class PrePaintPrisonLevel extends PrisonLevel
			implements Probe, ProbeOwner {
		private List<Integer> rngProbe;

		@Override
		protected Painter painter() {
			super.painter();
			return recordingPainter(this);
		}

		@Override
		public Level level() {
			return this;
		}

		@Override
		public List<Integer> rngProbe() {
			return rngProbe;
		}

		@Override
		public void setRngProbe(List<Integer> probe) {
			rngProbe = probe;
		}
	}

	private static final class PreMobsSewerLevel extends SewerLevel implements Probe {
		private List<Integer> rngProbe;

		@Override
		protected void createMobs() {
			rngProbe = captureRng();
			throw new FloorOracle.SnapshotComplete();
		}

		@Override
		public Level level() {
			return this;
		}

		@Override
		public List<Integer> rngProbe() {
			return rngProbe;
		}
	}

	private static final class PreMobsPrisonLevel extends PrisonLevel implements Probe {
		private List<Integer> rngProbe;

		@Override
		protected void createMobs() {
			rngProbe = captureRng();
			throw new FloorOracle.SnapshotComplete();
		}

		@Override
		public Level level() {
			return this;
		}

		@Override
		public List<Integer> rngProbe() {
			return rngProbe;
		}
	}

	private static final class PreItemsSewerLevel extends SewerLevel implements Probe {
		private List<Integer> rngProbe;

		@Override
		protected void createItems() {
			rngProbe = captureRng();
			throw new FloorOracle.SnapshotComplete();
		}

		@Override
		public Level level() {
			return this;
		}

		@Override
		public List<Integer> rngProbe() {
			return rngProbe;
		}
	}

	private static final class PreItemsPrisonLevel extends PrisonLevel implements Probe {
		private List<Integer> rngProbe;

		@Override
		protected void createItems() {
			rngProbe = captureRng();
			throw new FloorOracle.SnapshotComplete();
		}

		@Override
		public Level level() {
			return this;
		}

		@Override
		public List<Integer> rngProbe() {
			return rngProbe;
		}
	}
}

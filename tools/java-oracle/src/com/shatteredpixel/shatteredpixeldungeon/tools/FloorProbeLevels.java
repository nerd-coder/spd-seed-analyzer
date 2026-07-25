/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.levels.Level;
import com.shatteredpixel.shatteredpixeldungeon.levels.CavesLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.CityLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.PrisonLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.SewerLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.painters.Painter;
import com.shatteredpixel.shatteredpixeldungeon.levels.painters.CavesPainter;
import com.shatteredpixel.shatteredpixeldungeon.levels.painters.SewerPainter;
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
		if (depth >= 1 && depth <= 4) return new PrePaintSewerLevel();
		if (depth >= 6 && depth <= 9) return new PrePaintPrisonLevel();
		if (depth >= 11 && depth <= 14) return new PrePaintCavesLevel();
		if (depth >= 16 && depth <= 19) return new PrePaintCityLevel();
		throw unsupported(depth);
	}

	static Probe preMobs(int depth) {
		if (depth >= 1 && depth <= 4) return new PreMobsSewerLevel();
		if (depth >= 6 && depth <= 9) return new PreMobsPrisonLevel();
		if (depth >= 11 && depth <= 14) return new PreMobsCavesLevel();
		if (depth >= 16 && depth <= 19) return new PreMobsCityLevel();
		throw unsupported(depth);
	}

	static Probe preDoors(int depth) {
		if (depth >= 1 && depth <= 4) return new DoorBoundarySewerLevel(false);
		if (depth >= 11 && depth <= 14) return new DoorBoundaryCavesLevel(false);
		throw unsupported(depth);
	}

	static Probe postDoors(int depth) {
		if (depth >= 1 && depth <= 4) return new DoorBoundarySewerLevel(true);
		if (depth >= 11 && depth <= 14) return new DoorBoundaryCavesLevel(true);
		throw unsupported(depth);
	}

	static Probe preItems(int depth) {
		if (depth >= 1 && depth <= 4) return new PreItemsSewerLevel();
		if (depth >= 6 && depth <= 9) return new PreItemsPrisonLevel();
		if (depth >= 11 && depth <= 14) return new PreItemsCavesLevel();
		if (depth >= 16 && depth <= 19) return new PreItemsCityLevel();
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

	private static final class PrePaintCavesLevel extends CavesLevel
			implements Probe, ProbeOwner {
		private List<Integer> rngProbe;

		@Override
		protected Painter painter() {
			super.painter();
			return recordingPainter(this);
		}

		@Override public Level level() { return this; }
		@Override public List<Integer> rngProbe() { return rngProbe; }
		@Override public void setRngProbe(List<Integer> probe) { rngProbe = probe; }
	}

	private static final class PrePaintCityLevel extends CityLevel
			implements Probe, ProbeOwner {
		private List<Integer> rngProbe;

		@Override protected Painter painter() { super.painter(); return recordingPainter(this); }
		@Override public Level level() { return this; }
		@Override public List<Integer> rngProbe() { return rngProbe; }
		@Override public void setRngProbe(List<Integer> probe) { rngProbe = probe; }
	}

	private static final class DoorBoundarySewerLevel extends SewerLevel implements Probe {
		private final boolean after;
		private List<Integer> rngProbe;

		private DoorBoundarySewerLevel(boolean after) {
			this.after = after;
		}

		@Override
		protected Painter painter() {
			return new SewerPainter() {
				@Override
				protected void paintDoors(Level level, ArrayList<Room> rooms) {
					if (after) super.paintDoors(level, rooms);
					rngProbe = captureRng();
					throw new FloorOracle.SnapshotComplete();
				}
			}.setWater(feeling == Feeling.WATER ? 0.85f : 0.30f, 5)
					.setGrass(feeling == Feeling.GRASS ? 0.80f : 0.20f, 4)
					.setTraps(nTraps(), trapClasses(), trapChances());
		}

		@Override public Level level() { return this; }
		@Override public List<Integer> rngProbe() { return rngProbe; }
	}

	private static final class DoorBoundaryCavesLevel extends CavesLevel implements Probe {
		private final boolean after;
		private List<Integer> rngProbe;

		private DoorBoundaryCavesLevel(boolean after) { this.after = after; }

		@Override
		protected Painter painter() {
			return new CavesPainter() {
				@Override
				protected void paintDoors(Level level, ArrayList<Room> rooms) {
					if (after) super.paintDoors(level, rooms);
					rngProbe = captureRng();
					throw new FloorOracle.SnapshotComplete();
				}
			}.setWater(feeling == Feeling.WATER ? 0.85f : 0.30f, 6)
					.setGrass(feeling == Feeling.GRASS ? 0.65f : 0.15f, 3)
					.setTraps(nTraps(), trapClasses(), trapChances());
		}

		@Override public Level level() { return this; }
		@Override public List<Integer> rngProbe() { return rngProbe; }
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

	private static final class PreMobsCavesLevel extends CavesLevel implements Probe {
		private List<Integer> rngProbe;

		@Override
		protected void createMobs() {
			rngProbe = captureRng();
			throw new FloorOracle.SnapshotComplete();
		}

		@Override public Level level() { return this; }
		@Override public List<Integer> rngProbe() { return rngProbe; }
	}

	private static final class PreMobsCityLevel extends CityLevel implements Probe {
		private List<Integer> rngProbe;

		@Override protected void createMobs() { rngProbe = captureRng(); throw new FloorOracle.SnapshotComplete(); }
		@Override public Level level() { return this; }
		@Override public List<Integer> rngProbe() { return rngProbe; }
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

	private static final class PreItemsCavesLevel extends CavesLevel implements Probe {
		private List<Integer> rngProbe;

		@Override
		protected void createItems() {
			rngProbe = captureRng();
			throw new FloorOracle.SnapshotComplete();
		}

		@Override public Level level() { return this; }
		@Override public List<Integer> rngProbe() { return rngProbe; }
	}

	private static final class PreItemsCityLevel extends CityLevel implements Probe {
		private List<Integer> rngProbe;

		@Override protected void createItems() { rngProbe = captureRng(); throw new FloorOracle.SnapshotComplete(); }
		@Override public Level level() { return this; }
		@Override public List<Integer> rngProbe() { return rngProbe; }
	}
}

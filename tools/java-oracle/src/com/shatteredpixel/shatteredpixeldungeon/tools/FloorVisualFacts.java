/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.actors.blobs.Blob;
import com.shatteredpixel.shatteredpixeldungeon.levels.Level;
import com.shatteredpixel.shatteredpixeldungeon.levels.features.LevelTransition;
import com.shatteredpixel.shatteredpixeldungeon.levels.traps.Trap;
import com.shatteredpixel.shatteredpixeldungeon.plants.Plant;
import com.shatteredpixel.shatteredpixeldungeon.tiles.DungeonTileSheet;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

/** Captures the stable, render-relevant facts of a generated floor. */
final class FloorVisualFacts {

	final List<Integer> terrain;
	final List<Boolean> discoverable;
	final List<Integer> tileVariance;
	final List<TransitionFact> transitions;
	final List<TrapFact> traps;
	final List<PlantFact> plants;
	final List<BlobFact> blobs;

	private FloorVisualFacts(
			List<Integer> terrain,
			List<Boolean> discoverable,
			List<Integer> tileVariance,
			List<TransitionFact> transitions,
			List<TrapFact> traps,
			List<PlantFact> plants,
			List<BlobFact> blobs) {
		this.terrain = terrain;
		this.discoverable = discoverable;
		this.tileVariance = tileVariance;
		this.transitions = transitions;
		this.traps = traps;
		this.plants = plants;
		this.blobs = blobs;
	}

	static FloorVisualFacts capture(Level level) {
		List<Integer> terrain = new ArrayList<>();
		for (int value : level.map) {
			terrain.add(value);
		}

		List<Boolean> discoverable = new ArrayList<>();
		for (boolean value : level.discoverable) {
			discoverable.add(value);
		}

		DungeonTileSheet.setupVariance(level.length(), Dungeon.seedCurDepth());
		List<Integer> tileVariance = new ArrayList<>();
		for (byte value : DungeonTileSheet.tileVariance) {
			tileVariance.add(Byte.toUnsignedInt(value));
		}

		List<TransitionFact> transitions = new ArrayList<>();
		for (LevelTransition transition : level.transitions) {
			transitions.add(new TransitionFact(
					transition.cell(),
					transition.type.name(),
					transition.left,
					transition.top,
					transition.right,
					transition.bottom,
					transition.destDepth,
					transition.destBranch,
					transition.destType == null ? null : transition.destType.name()));
		}
		transitions.sort(Comparator
				.comparingInt((TransitionFact transition) -> transition.cell)
				.thenComparing(transition -> transition.transitionType));

		List<TrapFact> traps = new ArrayList<>();
		for (Trap trap : level.traps.valueList()) {
			traps.add(new TrapFact(
					trap.pos,
					trap.getClass().getSimpleName(),
					trap.visible,
					trap.active,
					trap.color,
					trap.shape));
		}
		traps.sort(Comparator.comparingInt(trap -> trap.cell));

		List<PlantFact> plants = new ArrayList<>();
		for (Plant plant : level.plants.valueList()) {
			plants.add(new PlantFact(plant.pos, plant.getClass().getSimpleName(), plant.image));
		}
		plants.sort(Comparator.comparingInt(plant -> plant.cell));

		List<BlobFact> blobs = new ArrayList<>();
		for (Blob blob : level.blobs.values()) {
			List<BlobCellFact> cells = new ArrayList<>();
			if (blob.cur != null) {
				for (int cell = 0; cell < blob.cur.length; cell++) {
					if (blob.cur[cell] > 0) {
						cells.add(new BlobCellFact(cell, blob.cur[cell]));
					}
				}
			}
			if (!cells.isEmpty()) {
				blobs.add(new BlobFact(
						blob.getClass().getSimpleName(), blob.volume, blob.alwaysVisible, cells));
			}
		}
		blobs.sort(Comparator.comparing(blob -> blob.blobClass));

		return new FloorVisualFacts(
				terrain, discoverable, tileVariance, transitions, traps, plants, blobs);
	}

	static final class TransitionFact {
		final int cell;
		final String transitionType;
		final int left;
		final int top;
		final int right;
		final int bottom;
		final int destDepth;
		final int destBranch;
		final String destType;

		TransitionFact(
				int cell,
				String transitionType,
				int left,
				int top,
				int right,
				int bottom,
				int destDepth,
				int destBranch,
				String destType) {
			this.cell = cell;
			this.transitionType = transitionType;
			this.left = left;
			this.top = top;
			this.right = right;
			this.bottom = bottom;
			this.destDepth = destDepth;
			this.destBranch = destBranch;
			this.destType = destType;
		}
	}

	static final class TrapFact {
		final int cell;
		final String trapClass;
		final boolean visible;
		final boolean active;
		final int color;
		final int shape;

		TrapFact(
				int cell,
				String trapClass,
				boolean visible,
				boolean active,
				int color,
				int shape) {
			this.cell = cell;
			this.trapClass = trapClass;
			this.visible = visible;
			this.active = active;
			this.color = color;
			this.shape = shape;
		}
	}

	static final class PlantFact {
		final int cell;
		final String plantClass;
		final int image;

		PlantFact(int cell, String plantClass, int image) {
			this.cell = cell;
			this.plantClass = plantClass;
			this.image = image;
		}
	}

	static final class BlobFact {
		final String blobClass;
		final int volume;
		final boolean alwaysVisible;
		final List<BlobCellFact> cells;

		BlobFact(
				String blobClass, int volume, boolean alwaysVisible, List<BlobCellFact> cells) {
			this.blobClass = blobClass;
			this.volume = volume;
			this.alwaysVisible = alwaysVisible;
			this.cells = cells;
		}
	}

	static final class BlobCellFact {
		final int cell;
		final int value;

		BlobCellFact(int cell, int value) {
			this.cell = cell;
			this.value = value;
		}
	}
}

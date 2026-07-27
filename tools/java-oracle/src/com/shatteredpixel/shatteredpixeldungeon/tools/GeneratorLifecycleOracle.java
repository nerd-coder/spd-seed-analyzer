/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.items.Generator;

import java.util.ArrayList;
import java.util.List;

/** Records private weapon-deck state across the first SacrificeRoom lifecycle. */
final class GeneratorLifecycleOracle {

	private GeneratorLifecycleOracle() {
	}

	static String generateJson(String inputSeed, long numericSeed) {
		List<BoundaryFact> boundaries = new ArrayList<>();
		boundaries.add(completedFloor(numericSeed, 1, "floor_1_complete"));
		boundaries.add(completedFloor(numericSeed, 2, "floor_2_complete"));
		boundaries.add(floorThreeAfterPaint(numericSeed));
		boundaries.add(completedFloor(numericSeed, 3, "floor_3_create_items_complete"));
		boundaries.add(completedFloor(numericSeed, 4, "floor_4_create_items_complete"));
		boundaries.add(completedFloor(numericSeed, 5, "floor_5_complete"));
		boundaries.add(completedFloor(numericSeed, 6, "floor_6_create_items_complete"));
		boundaries.add(completedFloor(numericSeed, 19, "floor_19_create_items_complete"));
		return toJson(inputSeed, numericSeed, boundaries);
	}

	private static BoundaryFact completedFloor(long seed, int depth, String boundary) {
		FloorOracle.initializeFreshRun(seed);
		FloorOracle.generatePriorFloors(depth);
		Dungeon.daily = true;
		try {
			Dungeon.level = Dungeon.newLevel();
		} finally {
			Dungeon.daily = false;
		}
		return snapshot(boundary);
	}

	private static BoundaryFact floorThreeAfterPaint(long seed) {
		FloorOracle.initializeFreshRun(seed);
		FloorOracle.generatePriorFloors(3);
		FloorOracle.markTargetFloorGenerated(3);
		FloorProbeLevels.Probe level = FloorProbeLevels.preMobs(3);
		try {
			level.level().create();
		} catch (FloorOracle.SnapshotComplete expected) {
			// SacrificeRoom has painted; ambient mobs and createItems have not run.
		}
		return snapshot("floor_3_after_sacrifice_room");
	}

	private static BoundaryFact snapshot(String boundary) {
		return new BoundaryFact(
				boundary,
				deck(Generator.Category.SCROLL),
				deck(Generator.Category.WEP_T2),
				deck(Generator.Category.WEP_T4));
	}

	private static DeckFact deck(Generator.Category category) {
		List<Integer> probabilities = new ArrayList<>();
		for (float probability : category.probs) probabilities.add((int) probability);
		return new DeckFact(category.seed, category.dropped, probabilities);
	}

	private static String toJson(String inputSeed, long numericSeed, List<BoundaryFact> facts) {
		StringBuilder json = new StringBuilder();
		json.append("{\n");
		json.append("  \"schema_version\": 1,\n");
		json.append("  \"contract\": \"generator_lifecycle\",\n");
		json.append("  \"spd\": { \"version\": \"v3.3.8\", \"commit\": \"7b8b845a7\" },\n");
		json.append("  \"input\": { \"seed\": \"").append(JavaOracle.escape(inputSeed))
				.append("\", \"numeric\": ").append(numericSeed).append(" },\n");
		json.append("  \"boundaries\": [\n");
		for (int index = 0; index < facts.size(); index++) {
			BoundaryFact fact = facts.get(index);
			json.append("    { \"boundary\": \"").append(fact.boundary).append("\", ")
					.append("\"scroll\": ").append(deckJson(fact.scroll)).append(", ")
					.append("\"wep_t2\": ").append(deckJson(fact.wepT2)).append(", ")
					.append("\"wep_t4\": ").append(deckJson(fact.wepT4)).append(" }");
			if (index + 1 < facts.size()) json.append(',');
			json.append('\n');
		}
		json.append("  ]\n}\n");
		return json.toString();
	}

	private static String deckJson(DeckFact deck) {
		return "{ \"seed\": " + deck.seed + ", \"dropped\": " + deck.dropped
				+ ", \"probabilities\": " + deck.probabilities + " }";
	}

	private static final class BoundaryFact {
		final String boundary;
		final DeckFact scroll;
		final DeckFact wepT2;
		final DeckFact wepT4;

		BoundaryFact(String boundary, DeckFact scroll, DeckFact wepT2, DeckFact wepT4) {
			this.boundary = boundary;
			this.scroll = scroll;
			this.wepT2 = wepT2;
			this.wepT4 = wepT4;
		}
	}

	private static final class DeckFact {
		final long seed;
		final int dropped;
		final List<Integer> probabilities;

		DeckFact(long seed, int dropped, List<Integer> probabilities) {
			this.seed = seed;
			this.dropped = dropped;
			this.probabilities = probabilities;
		}
	}
}

/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.items.Generator;
import com.shatteredpixel.shatteredpixeldungeon.items.Item;
import com.watabou.utils.Random;

import java.util.ArrayList;
import java.util.List;

/** Records a source-backed category deck rollover without running a floor. */
final class GeneratorDeckOracle {

	private static final long BASE_STREAM_SEED = 0x5EEDL;
	private static final int DRAW_COUNT = 8;

	private GeneratorDeckOracle() {
	}

	static String generateJson(String inputSeed, long numericSeed) {
		FloorOracle.initializeFreshRun(numericSeed);
		Generator.Category category = Generator.Category.FOOD;
		List<DrawFact> draws = new ArrayList<>();
		Random.pushGenerator(BASE_STREAM_SEED);
		try {
			for (int draw = 1; draw <= DRAW_COUNT; draw++) {
				Item item = Generator.random(category);
				draws.add(new DrawFact(
						draw,
						item.getClass().getSimpleName(),
						category.dropped,
						remainingWeight(category),
						privateProbe(category),
						List.of(Random.Int(), Random.Int())));
			}
		} finally {
			Random.popGenerator();
		}
		return toJson(inputSeed, numericSeed, draws);
	}

	private static int remainingWeight(Generator.Category category) {
		int remaining = 0;
		for (float probability : category.probs) {
			remaining += Math.max(0, probability);
		}
		return remaining;
	}

	private static List<Integer> privateProbe(Generator.Category category) {
		Random.pushGenerator(category.seed);
		try {
			for (int dropped = 0; dropped < category.dropped; dropped++) {
				Random.Long();
			}
			return List.of(Random.Int(), Random.Int());
		} finally {
			Random.popGenerator();
		}
	}

	private static String toJson(String inputSeed, long numericSeed, List<DrawFact> draws) {
		StringBuilder json = new StringBuilder();
		json.append("{\n");
		json.append("  \"schema_version\": 1,\n");
		json.append("  \"contract\": \"generator_deck_rollover\",\n");
		json.append("  \"spd\": { \"version\": \"v3.3.8\", \"commit\": \"7b8b845a7\" },\n");
		json.append("  \"input\": { \"seed\": \"")
				.append(JavaOracle.escape(inputSeed))
				.append("\", \"numeric\": ")
				.append(numericSeed)
				.append(", \"base_stream_seed\": ")
				.append(BASE_STREAM_SEED)
				.append(" },\n");
		json.append("  \"category\": \"FOOD\",\n");
		json.append("  \"initial_weight\": 5,\n");
		json.append("  \"draws\": [\n");
		for (int index = 0; index < draws.size(); index++) {
			DrawFact draw = draws.get(index);
			json.append("    { \"draw\": ").append(draw.draw)
					.append(", \"class\": \"").append(draw.itemClass)
					.append("\", \"dropped\": ").append(draw.dropped)
					.append(", \"remaining_weight\": ").append(draw.remainingWeight)
					.append(", \"private_rng\": ").append(draw.privateRng)
					.append(", \"base_rng\": ").append(draw.baseRng).append(" }");
			if (index + 1 < draws.size()) json.append(',');
			json.append('\n');
		}
		json.append("  ]\n");
		json.append("}\n");
		return json.toString();
	}

	private static final class DrawFact {
		final int draw;
		final String itemClass;
		final int dropped;
		final int remainingWeight;
		final List<Integer> privateRng;
		final List<Integer> baseRng;

		DrawFact(
				int draw,
				String itemClass,
				int dropped,
				int remainingWeight,
				List<Integer> privateRng,
				List<Integer> baseRng) {
			this.draw = draw;
			this.itemClass = itemClass;
			this.dropped = dropped;
			this.remainingWeight = remainingWeight;
			this.privateRng = privateRng;
			this.baseRng = baseRng;
		}
	}
}

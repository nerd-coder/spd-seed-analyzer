/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.actors.hero.Belongings;
import com.shatteredpixel.shatteredpixeldungeon.items.Item;
import com.shatteredpixel.shatteredpixeldungeon.items.bags.Bag;
import com.shatteredpixel.shatteredpixeldungeon.items.bags.PotionBandolier;
import com.shatteredpixel.shatteredpixeldungeon.items.bags.ScrollHolder;
import com.shatteredpixel.shatteredpixeldungeon.items.potions.PotionOfHealing;
import com.shatteredpixel.shatteredpixeldungeon.items.scrolls.ScrollOfMagicMapping;
import com.shatteredpixel.shatteredpixeldungeon.items.scrolls.ScrollOfRemoveCurse;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.special.ShopRoom;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

/** Records unique-winner inventory profiles through pinned ShopRoom.ChooseBag. */
final class ShopBagOracle {

	private ShopBagOracle() {
	}

	static String generateJson(String inputSeed, long numericSeed) {
		List<ScenarioFact> scenarios = new ArrayList<>();
		scenarios.add(runScenario(
				"fresh_backpack_after_holster",
				numericSeed,
				List.of()));
		scenarios.add(runScenario(
				"scroll_heavy_after_holster",
				numericSeed,
				List.of(new ScrollOfRemoveCurse(), new ScrollOfMagicMapping())));
		return toJson(inputSeed, numericSeed, scenarios);
	}

	private static ScenarioFact runScenario(
			String name, long numericSeed, List<Item> addedItems) {
		FloorOracle.initializeFreshRun(numericSeed);
		Dungeon.depth = 11;
		// Model the state after floor 6 generated its holster. The oracle's
		// sequential floor replay never auto-collects generated floor items.
		Dungeon.LimitedDrops.MAGICAL_HOLSTER.drop();
		for (Item item : addedItems) {
			if (!item.collect()) throw new IllegalStateException("failed to add " + item);
		}

		Belongings pack = Dungeon.hero.belongings;
		List<String> backpack = new ArrayList<>();
		for (Item item : pack.backpack.items) {
			backpack.add(item.getClass().getSimpleName());
		}
		backpack.sort(String::compareTo);

		List<ScoreFact> scores = new ArrayList<>();
		scores.add(score(new ScrollHolder(), pack));
		scores.add(score(new PotionBandolier(), pack));
		scores.sort(Comparator.comparing(score -> score.bag));
		int max = scores.stream().mapToInt(score -> score.score).max().orElseThrow();
		if (scores.stream().filter(score -> score.score == max).count() != 1) {
			throw new IllegalStateException("scenario must have one portable winner: " + name);
		}

		Bag selected = ExposedShopRoom.chooseBag(pack);
		return new ScenarioFact(
				name,
				backpack,
				scores,
				selected.getClass().getSimpleName());
	}

	private static ScoreFact score(Bag bag, Belongings pack) {
		int score = 0;
		for (Item item : pack.backpack.items) {
			if (bag.canHold(item)) score++;
		}
		return new ScoreFact(bag.getClass().getSimpleName(), score);
	}

	private static String toJson(
			String inputSeed, long numericSeed, List<ScenarioFact> scenarios) {
		StringBuilder json = new StringBuilder();
		json.append("{\n");
		json.append("  \"schema_version\": 1,\n");
		json.append("  \"contract\": \"shop_bag_selection\",\n");
		json.append("  \"spd\": { \"version\": \"v3.3.8\", \"commit\": \"7b8b845a7\" },\n");
		json.append("  \"input\": { \"seed\": \"")
				.append(JavaOracle.escape(inputSeed))
				.append("\", \"numeric\": ")
				.append(numericSeed)
				.append(" },\n");
		json.append("  \"scenarios\": [\n");
		for (int index = 0; index < scenarios.size(); index++) {
			ScenarioFact scenario = scenarios.get(index);
			json.append("    {\n");
			json.append("      \"name\": \"").append(scenario.name).append("\",\n");
			json.append("      \"depth\": 11,\n");
			json.append("      \"main_backpack\": ").append(quoted(scenario.backpack)).append(",\n");
			json.append("      \"scores\": [");
			for (int scoreIndex = 0; scoreIndex < scenario.scores.size(); scoreIndex++) {
				ScoreFact score = scenario.scores.get(scoreIndex);
				if (scoreIndex > 0) json.append(", ");
				json.append("{ \"bag\": \"").append(score.bag)
						.append("\", \"score\": ").append(score.score).append(" }");
			}
			json.append("],\n");
			json.append("      \"selected\": \"").append(scenario.selected).append("\"\n");
			json.append("    }");
			if (index + 1 < scenarios.size()) json.append(',');
			json.append('\n');
		}
		json.append("  ]\n");
		json.append("}\n");
		return json.toString();
	}

	private static String quoted(List<String> values) {
		StringBuilder json = new StringBuilder("[");
		for (int index = 0; index < values.size(); index++) {
			if (index > 0) json.append(", ");
			json.append('"').append(values.get(index)).append('"');
		}
		return json.append(']').toString();
	}

	private static final class ExposedShopRoom extends ShopRoom {
		static Bag chooseBag(Belongings pack) {
			return ChooseBag(pack);
		}
	}

	private static final class ScenarioFact {
		final String name;
		final List<String> backpack;
		final List<ScoreFact> scores;
		final String selected;

		ScenarioFact(String name, List<String> backpack, List<ScoreFact> scores, String selected) {
			this.name = name;
			this.backpack = backpack;
			this.scores = scores;
			this.selected = selected;
		}
	}

	private static final class ScoreFact {
		final String bag;
		final int score;

		ScoreFact(String bag, int score) {
			this.bag = bag;
			this.score = score;
		}
	}
}

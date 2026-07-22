/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.items.Generator;
import com.shatteredpixel.shatteredpixeldungeon.items.potions.Potion;
import com.shatteredpixel.shatteredpixeldungeon.items.rings.Ring;
import com.shatteredpixel.shatteredpixeldungeon.items.scrolls.Scroll;
import com.shatteredpixel.shatteredpixeldungeon.utils.DungeonSeed;
import com.watabou.utils.Bundle;
import com.watabou.utils.Random;

import java.util.ArrayList;
import java.util.List;

/** A deliberately small, headless oracle for seed-dependent run facts. */
public final class JavaOracle {

	private static final String SPD_VERSION = "v3.3.8";
	private static final String SPD_COMMIT = "7b8b845a7";

	private JavaOracle() {
	}

	public static void main(String[] args) {
		if (args.length < 1 || args.length > 3) {
			System.err.println("Usage: JavaOracle SEED [DEPTH | final-heaps DEPTH]");
			System.exit(2);
		}

		String inputSeed = args[0];
		long numericSeed = DungeonSeed.convertFromText(inputSeed);
		boolean finalHeaps = args.length == 3 && "final-heaps".equals(args[1]);
		if (args.length == 3 && !finalHeaps) {
			System.err.println("Unknown floor oracle contract: " + args[1]);
			System.exit(2);
		}
		Integer depth = args.length == 1
				? null
				: Integer.valueOf(args[finalHeaps ? 2 : 1]);
		if (depth != null && depth != 1) {
			System.err.println("The floor oracle currently supports only depth 1");
			System.exit(2);
		}

		try {
			FloorOracle.FloorFacts floor = null;
			FloorOracle.FinalFloorFacts finalFloor = null;
			if (depth == null) {
				Random.pushGenerator(numericSeed + 1);
				// Keep this in Dungeon.init() order at the pinned commit.
				Scroll.initLabels();
				Potion.initColors();
				Ring.initGems();
			} else if (finalHeaps) {
				finalFloor = FloorOracle.generateFinalHeaps(numericSeed);
			} else {
				floor = FloorOracle.generate(numericSeed);
			}

			Bundle identities = new Bundle();
			Potion.save(identities);
			Scroll.save(identities);
			Ring.save(identities);

			List<Identity> potions = identities(Generator.Category.POTION, identities);
			List<Identity> scrolls = identities(Generator.Category.SCROLL, identities);
			List<Identity> rings = identities(Generator.Category.RING, identities);
			if (finalFloor != null) {
				System.out.print(toFinalHeapsJson(
						inputSeed, numericSeed, potions, scrolls, rings, finalFloor));
			} else if (floor != null) {
				System.out.print(toFloorJson(
						inputSeed, numericSeed, potions, scrolls, rings, floor));
			} else {
				System.out.print(toJson(inputSeed, numericSeed, potions, scrolls, rings));
			}
		} finally {
			Random.resetGenerators();
			Scroll.clearLabels();
			Potion.clearColors();
			Ring.clearGems();
		}
	}

	private static List<Identity> identities(Generator.Category category, Bundle bundle) {
		List<Identity> result = new ArrayList<>();
		for (Class<?> itemClass : category.classes) {
			String item = itemClass.getSimpleName();
			result.add(new Identity(item, bundle.getString(item + "_label")));
		}
		return result;
	}

	private static String toJson(
			String inputSeed,
			long numericSeed,
			List<Identity> potions,
			List<Identity> scrolls,
			List<Identity> rings) {
		StringBuilder json = new StringBuilder();
		json.append("{\n");
		json.append("  \"schema_version\": 1,\n");
		json.append("  \"spd\": {\n");
		json.append("    \"version\": \"").append(SPD_VERSION).append("\",\n");
		json.append("    \"commit\": \"").append(SPD_COMMIT).append("\"\n");
		json.append("  },\n");
		json.append("  \"input\": {\n");
		json.append("    \"seed\": \"").append(escape(inputSeed)).append("\",\n");
		json.append("    \"numeric\": ").append(numericSeed).append(",\n");
		json.append("    \"depths\": []\n");
		json.append("  },\n");
		json.append("  \"identities\": {\n");
		appendIdentities(json, "potions", potions, true);
		appendIdentities(json, "scrolls", scrolls, true);
		appendIdentities(json, "rings", rings, false);
		json.append("  }\n");
		json.append("}\n");
		return json.toString();
	}

	private static String toFloorJson(
			String inputSeed,
			long numericSeed,
			List<Identity> potions,
			List<Identity> scrolls,
			List<Identity> rings,
			FloorOracle.FloorFacts floor) {
		StringBuilder json = new StringBuilder();
		json.append("{\n");
		json.append("  \"schema_version\": 2,\n");
		json.append("  \"spd\": {\n");
		json.append("    \"version\": \"").append(SPD_VERSION).append("\",\n");
		json.append("    \"commit\": \"").append(SPD_COMMIT).append("\"\n");
		json.append("  },\n");
		json.append("  \"input\": {\n");
		json.append("    \"seed\": \"").append(escape(inputSeed)).append("\",\n");
		json.append("    \"numeric\": ").append(numericSeed).append(",\n");
		json.append("    \"depths\": [").append(floor.depth).append("]\n");
		json.append("  },\n");
		json.append("  \"identities\": {\n");
		appendIdentities(json, "potions", potions, true);
		appendIdentities(json, "scrolls", scrolls, true);
		appendIdentities(json, "rings", rings, false);
		json.append("  },\n");
		json.append("  \"floors\": [\n");
		json.append("    {\n");
		json.append("      \"depth\": ").append(floor.depth).append(",\n");
		json.append("      \"forced_items\": [\n");
		appendItems(json, floor.forcedItems, "        ");
		json.append("      ]\n");
		json.append("    }\n");
		json.append("  ]\n");
		json.append("}\n");
		return json.toString();
	}

	private static String toFinalHeapsJson(
			String inputSeed,
			long numericSeed,
			List<Identity> potions,
			List<Identity> scrolls,
			List<Identity> rings,
			FloorOracle.FinalFloorFacts floor) {
		StringBuilder json = new StringBuilder();
		json.append("{\n");
		json.append("  \"schema_version\": 3,\n");
		json.append("  \"contract\": \"final_placed_heaps\",\n");
		json.append("  \"spd\": {\n");
		json.append("    \"version\": \"").append(SPD_VERSION).append("\",\n");
		json.append("    \"commit\": \"").append(SPD_COMMIT).append("\"\n");
		json.append("  },\n");
		json.append("  \"input\": {\n");
		json.append("    \"seed\": \"").append(escape(inputSeed)).append("\",\n");
		json.append("    \"numeric\": ").append(numericSeed).append(",\n");
		json.append("    \"depths\": [").append(floor.depth).append("]\n");
		json.append("  },\n");
		json.append("  \"identities\": {\n");
		appendIdentities(json, "potions", potions, true);
		appendIdentities(json, "scrolls", scrolls, true);
		appendIdentities(json, "rings", rings, false);
		json.append("  },\n");
		json.append("  \"floors\": [\n");
		json.append("    {\n");
		json.append("      \"depth\": ").append(floor.depth).append(",\n");
		json.append("      \"width\": ").append(floor.width).append(",\n");
		json.append("      \"height\": ").append(floor.height).append(",\n");
		json.append("      \"pre_paint_rng\": [");
		for (int index = 0; index < floor.prePaintRng.size(); index++) {
			if (index > 0) json.append(", ");
			json.append(floor.prePaintRng.get(index));
		}
		json.append("],\n");
		json.append("      \"pre_mobs_rng\": [");
		for (int index = 0; index < floor.preMobsRng.size(); index++) {
			if (index > 0) json.append(", ");
			json.append(floor.preMobsRng.get(index));
		}
		json.append("],\n");
		json.append("      \"pre_items_rng\": [");
		for (int index = 0; index < floor.preItemsRng.size(); index++) {
			if (index > 0) json.append(", ");
			json.append(floor.preItemsRng.get(index));
		}
		json.append("],\n");
		json.append("      \"final_heaps\": [\n");
		appendHeaps(json, floor.heaps);
		json.append("      ],\n");
		json.append("      \"final_mobs\": [\n");
		appendMobs(json, floor.mobs);
		json.append("      ]\n");
		json.append("    }\n");
		json.append("  ]\n");
		json.append("}\n");
		return json.toString();
	}

	private static void appendMobs(StringBuilder json, List<FloorOracle.MobFact> mobs) {
		for (int index = 0; index < mobs.size(); index++) {
			FloorOracle.MobFact mob = mobs.get(index);
			json.append("        { \"cell\": ").append(mob.cell)
					.append(", \"class\": \"").append(escape(mob.mobClass)).append("\" }");
			if (index + 1 < mobs.size()) {
				json.append(',');
			}
			json.append('\n');
		}
	}

	private static void appendHeaps(StringBuilder json, List<FloorOracle.HeapFact> heaps) {
		for (int heapIndex = 0; heapIndex < heaps.size(); heapIndex++) {
			FloorOracle.HeapFact heap = heaps.get(heapIndex);
			json.append("        {\n");
			json.append("          \"cell\": ").append(heap.cell).append(",\n");
			json.append("          \"heap_type\": \"").append(escape(heap.heapType)).append("\",\n");
			json.append("          \"items\": [\n");
			appendBiomeItems(json, heap.items, "            ");
			json.append("          ]\n");
			json.append("        }");
			if (heapIndex + 1 < heaps.size()) {
				json.append(',');
			}
			json.append('\n');
		}
	}

	private static void appendBiomeItems(
			StringBuilder json, List<FloorOracle.ItemFact> items, String indent) {
		for (int index = 0; index < items.size(); index++) {
			FloorOracle.ItemFact item = items.get(index);
			StringBuilder compact = new StringBuilder("{ ");
			appendItemFields(compact, item);
			compact.append(" }");
			if (indent.length() + compact.length() <= 80) {
				json.append(indent).append(compact);
			} else {
				json.append(indent).append("{\n");
				json.append(indent).append("  \"class\": \"")
						.append(escape(item.itemClass)).append("\",\n");
				json.append(indent).append("  \"quantity\": ").append(item.quantity).append(",\n");
				json.append(indent).append("  \"level\": ").append(item.level).append(",\n");
				json.append(indent).append("  \"cursed\": ").append(item.cursed).append('\n');
				json.append(indent).append('}');
			}
			if (index + 1 < items.size()) {
				json.append(',');
			}
			json.append('\n');
		}
	}

	private static void appendItems(
			StringBuilder json, List<FloorOracle.ItemFact> items, String indent) {
		for (int index = 0; index < items.size(); index++) {
			json.append(indent).append("{ ");
			appendItemFields(json, items.get(index));
			json.append(" }");
			if (index + 1 < items.size()) {
				json.append(',');
			}
			json.append('\n');
		}
	}

	private static void appendItemFields(StringBuilder json, FloorOracle.ItemFact item) {
		json.append("\"class\": \"").append(escape(item.itemClass))
				.append("\", \"quantity\": ").append(item.quantity)
				.append(", \"level\": ").append(item.level)
				.append(", \"cursed\": ").append(item.cursed);
	}

	private static void appendIdentities(
			StringBuilder json, String name, List<Identity> identities, boolean trailingComma) {
		json.append("    \"").append(name).append("\": [\n");
		for (int index = 0; index < identities.size(); index++) {
			Identity identity = identities.get(index);
			json.append("      { \"item\": \"")
					.append(escape(identity.item))
					.append("\", \"appearance\": \"")
					.append(escape(identity.appearance))
					.append("\" }");
			if (index + 1 < identities.size()) {
				json.append(',');
			}
			json.append('\n');
		}
		json.append("    ]");
		json.append(trailingComma ? ",\n" : "\n");
	}

	private static String escape(String value) {
		StringBuilder escaped = new StringBuilder();
		for (int index = 0; index < value.length(); index++) {
			char character = value.charAt(index);
			switch (character) {
				case '\\':
					escaped.append("\\\\");
					break;
				case '"':
					escaped.append("\\\"");
					break;
				case '\n':
					escaped.append("\\n");
					break;
				case '\r':
					escaped.append("\\r");
					break;
				case '\t':
					escaped.append("\\t");
					break;
				default:
					if (character < 0x20) {
						escaped.append(String.format("\\u%04x", (int) character));
					} else {
						escaped.append(character);
					}
			}
		}
		return escaped.toString();
	}

	private static final class Identity {
		private final String item;
		private final String appearance;

		private Identity(String item, String appearance) {
			this.item = item;
			this.appearance = appearance;
		}
	}
}

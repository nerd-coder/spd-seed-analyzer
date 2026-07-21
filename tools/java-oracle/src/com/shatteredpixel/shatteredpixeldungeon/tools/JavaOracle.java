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

/** A deliberately small, headless oracle for seed-dependent identity mappings. */
public final class JavaOracle {

	private static final String SPD_VERSION = "v3.3.8";
	private static final String SPD_COMMIT = "7b8b845a7";

	private JavaOracle() {
	}

	public static void main(String[] args) {
		if (args.length != 1) {
			System.err.println("Usage: JavaOracle SEED");
			System.exit(2);
		}

		String inputSeed = args[0];
		long numericSeed = DungeonSeed.convertFromText(inputSeed);

		Random.pushGenerator(numericSeed + 1);
		try {
			// Keep this in Dungeon.init() order at the pinned commit.
			Scroll.initLabels();
			Potion.initColors();
			Ring.initGems();

			Bundle identities = new Bundle();
			Potion.save(identities);
			Scroll.save(identities);
			Ring.save(identities);

			List<Identity> potions = identities(Generator.Category.POTION, identities);
			List<Identity> scrolls = identities(Generator.Category.SCROLL, identities);
			List<Identity> rings = identities(Generator.Category.RING, identities);
			System.out.print(toJson(inputSeed, numericSeed, potions, scrolls, rings));
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

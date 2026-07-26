/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.levels.Patch;
import com.watabou.utils.Random;

/** Stable probe for the Caves boss water patch before trap rolls. */
final class PatchOracle {

	private PatchOracle() {
	}

	static String cavesBossJson(String inputSeed, long seed) {
		Dungeon.seed = seed;
		long depthSeed = Dungeon.seedForDepth(15, 0);
		Random.pushGenerator(depthSeed);
		try {
			boolean[] mask = Patch.generate(33, 28, 0.15f, 2, true);
			int[] postPatchRng = rngProbe();
			Random.popGenerator();
			Random.pushGenerator(depthSeed);
			mask = Patch.generate(33, 28, 0.15f, 2, true);
			int trapRolls = 0;
			for (int y = 0; y < 23; y++) {
				double radius = 23 / 2f;
				double rowY = -radius + 0.5 + y;
				double rowWidth = 2.0 * Math.sqrt((radius * radius) * (1.0 - (rowY * rowY) / (radius * radius)));
				rowWidth = Math.floor(rowWidth / 2.0) * 2.0 + 1;
				int left = 5 + (23 - (int) rowWidth) / 2;
				for (int x = left; x < left + (int) rowWidth; x++) {
					if (!mask[x + y * 33]) {
						Random.Int(8);
						trapRolls++;
					}
				}
			}
			StringBuilder json = new StringBuilder();
			json.append("{\n  \"schema_version\": 1,\n");
			json.append("  \"contract\": \"caves_boss_patch\",\n");
			json.append("  \"seed\": \"").append(inputSeed).append("\",\n");
			json.append("  \"depth_seed\": ").append(depthSeed).append(",\n");
			json.append("  \"mask\": [");
			for (int i = 0; i < mask.length; i++) {
				if (i > 0) json.append(',');
				json.append(mask[i]);
			}
			json.append("],\n  \"post_patch_rng\": [");
			for (int i = 0; i < postPatchRng.length; i++) {
				if (i > 0) json.append(',');
				json.append(postPatchRng[i]);
			}
			json.append("],\n  \"trap_rolls\": ").append(trapRolls).append(",\n");
			json.append("  \"entrance_variant\": ").append(Random.Int(4)).append(",\n");
			json.append("  \"corner_variant\": ").append(Random.Int(4)).append(",\n");
			json.append("  \"post_entrance_rng\": [");
			for (int i = 0; i < 8; i++) {
				if (i > 0) json.append(',');
				json.append(Random.Int());
			}
			json.append("]\n}\n");
			return json.toString();
		} finally {
			Random.popGenerator();
		}
	}

	private static int[] rngProbe() {
		int[] probe = new int[8];
		for (int i = 0; i < probe.length; i++) probe[i] = Random.Int();
		return probe;
	}
}

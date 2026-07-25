/*
 * This file is part of SPD Seed Analyzer.
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.actors.blobs.SacrificialFire;
import com.shatteredpixel.shatteredpixeldungeon.items.Item;
import com.shatteredpixel.shatteredpixeldungeon.items.trinkets.ParchmentScrap;
import com.shatteredpixel.shatteredpixeldungeon.items.weapon.Weapon;
import com.shatteredpixel.shatteredpixeldungeon.levels.RegularLevel;

import java.lang.reflect.Field;

/** Captures the blob-held floor-13 reward with explicit player state. */
final class SacrificeRewardOracle {

	private SacrificeRewardOracle() {
	}

	static String generateJson(String inputSeed, long seed) {
		FloorOracle.initializeFreshRun(seed);
		FloorOracle.generatePriorFloors(13);
		ParchmentScrap scrap = new ParchmentScrap();
		scrap.level(3);
		scrap.identify();
		Dungeon.hero.belongings.backpack.items.add(scrap);
		Dungeon.daily = true;
		RegularLevel level;
		try {
			level = (RegularLevel) Dungeon.newLevel();
		} finally {
			Dungeon.daily = false;
		}
		SacrificialFire fire = (SacrificialFire) level.blobs.get(SacrificialFire.class);
		if (fire == null) throw new IllegalStateException("floor 13 has no SacrificialFire");
		Item prize = readPrize(fire);
		Weapon weapon = (Weapon) prize;
		String enchantment = weapon.enchantment == null
				? null : weapon.enchantment.getClass().getSimpleName();
		return "{\n"
				+ "  \"schema_version\": 1,\n"
				+ "  \"contract\": \"sacrifice_reward_player_state\",\n"
				+ "  \"spd\": { \"version\": \"v3.3.8\", \"commit\": \"7b8b845a7\" },\n"
				+ "  \"input\": { \"seed\": \"" + inputSeed + "\", \"numeric\": " + seed
				+ ", \"depth\": 13, \"parchment_scrap_level\": 3 },\n"
				+ "  \"lifecycle\": \"created_during_room_paint_and_stored_in_blob\",\n"
				+ "  \"reward\": { \"class\": \"" + prize.getClass().getSimpleName()
				+ "\", \"quantity\": " + prize.quantity() + ", \"level\": " + prize.trueLevel()
				+ ", \"cursed\": " + prize.cursed + ", \"enchantment\": "
				+ (enchantment == null ? "null" : "\"" + enchantment + "\"") + " }\n}\n";
	}

	private static Item readPrize(SacrificialFire fire) {
		try {
			Field field = SacrificialFire.class.getDeclaredField("prize");
			field.setAccessible(true);
			return (Item) field.get(fire);
		} catch (ReflectiveOperationException error) {
			throw new IllegalStateException("cannot inspect SacrificialFire prize", error);
		}
	}
}

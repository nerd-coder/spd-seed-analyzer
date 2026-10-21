/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.Mob;
import com.shatteredpixel.shatteredpixeldungeon.items.Heap;
import com.shatteredpixel.shatteredpixeldungeon.items.Item;
import com.shatteredpixel.shatteredpixeldungeon.levels.Level;

import java.util.ArrayList;
import java.util.List;

/** Room-painted mobs and heaps of a level snapshotted at createMobs. */
final class LevelEntitiesJson {

	private LevelEntitiesJson() {
	}

	/** Mobs sorted by cell. Inner mob classes report their superclass (`FireElemental`). */
	static void appendMobs(StringBuilder json, Level level) {
		List<Mob> mobs = new ArrayList<>(level.mobs);
		mobs.sort((a, b) -> Integer.compare(a.pos, b.pos));
		for (int index = 0; index < mobs.size(); index++) {
			Mob mob = mobs.get(index);
			Class<?> cls = mob.getClass();
			String name = cls.getEnclosingClass() != null
					? cls.getSuperclass().getSimpleName()
					: cls.getSimpleName();
			String state = mob.state == mob.SLEEPING ? "sleeping"
					: mob.state == mob.WANDERING ? "wandering"
					: mob.state == mob.PASSIVE ? "passive"
					: "other";
			json.append("    { \"cell\": ").append(mob.pos)
					.append(", \"class\": \"").append(name)
					.append("\", \"state\": \"").append(state).append("\" }");
			if (index + 1 < mobs.size()) json.append(',');
			json.append('\n');
		}
	}

	/** Heaps sorted by cell. Inner item classes join names (`MageroyalSeed`). */
	static void appendHeaps(StringBuilder json, Level level) {
		List<Heap> heaps = new ArrayList<>(level.heaps.valueList());
		heaps.sort((a, b) -> Integer.compare(a.pos, b.pos));
		for (int index = 0; index < heaps.size(); index++) {
			Heap heap = heaps.get(index);
			json.append("    { \"cell\": ").append(heap.pos)
					.append(", \"type\": \"").append(heap.type.name().toLowerCase())
					.append("\", \"items\": [");
			int itemIndex = 0;
			for (Item item : heap.items) {
				if (itemIndex++ > 0) json.append(", ");
				Class<?> cls = item.getClass();
				String name = cls.getEnclosingClass() != null
						? cls.getEnclosingClass().getSimpleName() + cls.getSimpleName()
						: cls.getSimpleName();
				json.append("{ \"class\": \"").append(name)
						.append("\", \"quantity\": ").append(item.quantity())
						.append(", \"level\": ").append(item.level())
						.append(", \"cursed\": ").append(item.cursed).append(" }");
			}
			json.append("] }");
			if (index + 1 < heaps.size()) json.append(',');
			json.append('\n');
		}
	}
}

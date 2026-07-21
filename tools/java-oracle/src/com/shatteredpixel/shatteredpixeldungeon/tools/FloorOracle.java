/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.badlogic.gdx.utils.GdxNativesLoader;
import com.shatteredpixel.shatteredpixeldungeon.Assets;
import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.Statistics;
import com.shatteredpixel.shatteredpixeldungeon.actors.Actor;
import com.shatteredpixel.shatteredpixeldungeon.actors.hero.Hero;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Blacksmith;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Ghost;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Imp;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Wandmaker;
import com.shatteredpixel.shatteredpixeldungeon.items.Generator;
import com.shatteredpixel.shatteredpixeldungeon.items.Item;
import com.shatteredpixel.shatteredpixeldungeon.items.potions.Potion;
import com.shatteredpixel.shatteredpixeldungeon.items.rings.Ring;
import com.shatteredpixel.shatteredpixeldungeon.items.scrolls.Scroll;
import com.shatteredpixel.shatteredpixeldungeon.levels.SewerLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.secret.SecretRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.special.SpecialRoom;
import com.watabou.gltextures.TextureCache;
import com.watabou.noosa.Game;
import com.watabou.utils.GameSettings;
import com.watabou.utils.Random;
import com.watabou.utils.SparseArray;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;

/** Runs the pinned depth-one SewerLevel and records stable item facts. */
final class FloorOracle {

	private FloorOracle() {
	}

	static FloorFacts generate(long seed) {
		initializeFreshRun(seed);
		RecordingSewerLevel level = new RecordingSewerLevel();
		Actor.clear();
		try {
			level.create();
		} catch (SnapshotComplete expected) {
			// Stop at the exact Level.create pre-build boundary. Later room paint
			// mutates itemsToSpawn, and is outside this first floor-fact contract.
		}
		return new FloorFacts(1, level.forcedItems());
	}

	private static void initializeFreshRun(long seed) {
		GameSettings.set(new MemoryPreferences());
		Game.version = "3.3.8";
		GdxNativesLoader.load();
		// ItemSpriteSheet.Icons only needs dimensions during generation. Seed a
		// blank cache entry so its TextureFilm does not touch Gdx.files or GL.
		TextureCache.create(Assets.Sprites.ITEM_ICONS, 128, 128);
		Dungeon.seed = seed;
		Dungeon.challenges = 0;
		Dungeon.mobsToChampion = 1;

		Actor.clear();
		Actor.resetNextID();
		Random.pushGenerator(seed + 1);
		Scroll.initLabels();
		Potion.initColors();
		Ring.initGems();
		SpecialRoom.initForRun();
		SecretRoom.initForRun();
		Generator.fullReset();
		Random.resetGenerators();

		Statistics.reset();
		Dungeon.depth = 1;
		Dungeon.branch = 0;
		Dungeon.generatedLevels.clear();
		Dungeon.gold = 0;
		Dungeon.energy = 0;
		Dungeon.droppedItems = new SparseArray<>();
		Dungeon.LimitedDrops.reset();
		Dungeon.chapters = new HashSet<>();
		Ghost.Quest.reset();
		Wandmaker.Quest.reset();
		Blacksmith.Quest.reset();
		Imp.Quest.reset();
		Dungeon.hero = new Hero();
	}

	private static ItemFact itemFact(Item item) {
		return new ItemFact(
				item.getClass().getSimpleName(), item.quantity(), item.trueLevel(), item.cursed);
	}

	static final class FloorFacts {
		final int depth;
		final List<ItemFact> forcedItems;

		FloorFacts(int depth, List<ItemFact> forcedItems) {
			this.depth = depth;
			this.forcedItems = forcedItems;
		}
	}

	static final class ItemFact {
		final String itemClass;
		final int quantity;
		final int level;
		final boolean cursed;

		ItemFact(String itemClass, int quantity, int level, boolean cursed) {
			this.itemClass = itemClass;
			this.quantity = quantity;
			this.level = level;
			this.cursed = cursed;
		}
	}

	private static final class RecordingSewerLevel extends SewerLevel {
		private List<ItemFact> forcedItems;

		@Override
		protected boolean build() {
			if (forcedItems == null) {
				forcedItems = new ArrayList<>();
				for (Item item : itemsToSpawn) {
					forcedItems.add(itemFact(item));
				}
			}
			throw new SnapshotComplete();
		}

		List<ItemFact> forcedItems() {
			return forcedItems;
		}
	}

	private static final class SnapshotComplete extends RuntimeException {
		private static final long serialVersionUID = 1L;
	}
}

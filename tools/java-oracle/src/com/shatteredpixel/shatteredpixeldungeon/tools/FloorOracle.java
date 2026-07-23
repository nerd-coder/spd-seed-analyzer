/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.badlogic.gdx.utils.GdxNativesLoader;
import com.shatteredpixel.shatteredpixeldungeon.Assets;
import com.shatteredpixel.shatteredpixeldungeon.Dungeon;
import com.shatteredpixel.shatteredpixeldungeon.GamesInProgress;
import com.shatteredpixel.shatteredpixeldungeon.SPDSettings;
import com.shatteredpixel.shatteredpixeldungeon.Statistics;
import com.shatteredpixel.shatteredpixeldungeon.actors.Actor;
import com.shatteredpixel.shatteredpixeldungeon.actors.hero.Hero;
import com.shatteredpixel.shatteredpixeldungeon.actors.hero.HeroClass;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.Mob;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Blacksmith;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Ghost;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Imp;
import com.shatteredpixel.shatteredpixeldungeon.actors.mobs.npcs.Wandmaker;
import com.shatteredpixel.shatteredpixeldungeon.items.Generator;
import com.shatteredpixel.shatteredpixeldungeon.items.Heap;
import com.shatteredpixel.shatteredpixeldungeon.items.Item;
import com.shatteredpixel.shatteredpixeldungeon.items.potions.Potion;
import com.shatteredpixel.shatteredpixeldungeon.items.rings.Ring;
import com.shatteredpixel.shatteredpixeldungeon.items.scrolls.Scroll;
import com.shatteredpixel.shatteredpixeldungeon.journal.Document;
import com.shatteredpixel.shatteredpixeldungeon.levels.Level;
import com.shatteredpixel.shatteredpixeldungeon.levels.RegularLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.SewerLevel;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.secret.SecretRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.special.SpecialRoom;
import com.watabou.gltextures.TextureCache;
import com.watabou.noosa.Game;
import com.watabou.utils.GameSettings;
import com.watabou.utils.Random;
import com.watabou.utils.SparseArray;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;

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

	static FinalFloorFacts generateFinalHeaps(long seed, int depth) {
		initializeFreshRun(seed);
		generatePriorFloors(depth);
		// A committed seed fixture must not depend on the user's bones.dat. At
		// this pin, Dungeon.daily is consulted during generation only by Bones.
		Dungeon.daily = true;
		RegularLevel level;
		try {
			level = (RegularLevel) Dungeon.newLevel();
		} finally {
			Dungeon.daily = false;
		}
		List<HeapFact> heaps = new ArrayList<>();
		for (Heap heap : level.heaps.valueList()) {
			List<ItemFact> items = new ArrayList<>();
			for (Item item : heap.items) {
				items.add(itemFact(item));
			}
			heaps.add(new HeapFact(
					heap.pos, heap.type.name().toLowerCase(Locale.ROOT), items));
		}
		heaps.sort(Comparator.comparingInt(heap -> heap.cell));
		List<MobFact> mobs = new ArrayList<>();
		for (Mob mob : level.mobs) {
			mobs.add(new MobFact(mob.pos, mob.getClass().getSimpleName()));
		}
		mobs.sort(Comparator.comparingInt(mob -> mob.cell));
		List<ItemFact> questRewards = new ArrayList<>();
		if (Wandmaker.Quest.wand1 != null) {
			questRewards.add(itemFact(Wandmaker.Quest.wand1));
		}
		if (Wandmaker.Quest.wand2 != null) {
			questRewards.add(itemFact(Wandmaker.Quest.wand2));
		}
		List<String> rooms = new ArrayList<>();
		for (Room room : level.rooms()) {
			rooms.add(room.getClass().getSimpleName());
		}
		rooms.sort(String::compareTo);
		int width = level.width();
		int height = level.height();
		FloorVisualFacts visualFacts = FloorVisualFacts.capture(level);
		List<Integer> prePaintRng = generatePrePaintRng(seed, depth);
		List<Integer> preMobsRng = generatePreMobsRng(seed, depth);
		List<Integer> preItemsRng = generatePreItemsRng(seed, depth);
		return new FinalFloorFacts(
				depth,
				width,
				height,
				rooms,
				heaps,
				mobs,
				questRewards,
				prePaintRng,
				preMobsRng,
				preItemsRng,
				visualFacts);
	}

	private static List<Integer> generatePrePaintRng(long seed, int depth) {
		initializeFreshRun(seed);
		generatePriorFloors(depth);
		markTargetFloorGenerated(depth);
		FloorProbeLevels.Probe level = FloorProbeLevels.prePaint(depth);
		try {
			level.level().create();
		} catch (SnapshotComplete expected) {
			// The recording painter stops before RegularPainter.paint.
		}
		return level.rngProbe();
	}

	private static List<Integer> generatePreMobsRng(long seed, int depth) {
		initializeFreshRun(seed);
		generatePriorFloors(depth);
		markTargetFloorGenerated(depth);
		FloorProbeLevels.Probe level = FloorProbeLevels.preMobs(depth);
		try {
			level.level().create();
		} catch (SnapshotComplete expected) {
			// The override stops at the createMobs entry boundary.
		}
		return level.rngProbe();
	}

	private static List<Integer> generatePreItemsRng(long seed, int depth) {
		initializeFreshRun(seed);
		generatePriorFloors(depth);
		markTargetFloorGenerated(depth);
		FloorProbeLevels.Probe level = FloorProbeLevels.preItems(depth);
		try {
			level.level().create();
		} catch (SnapshotComplete expected) {
			// The override stops at the createItems entry boundary.
		}
		return level.rngProbe();
	}

	private static void generatePriorFloors(int targetDepth) {
		Dungeon.daily = true;
		try {
			for (int depth = 1; depth < targetDepth; depth++) {
				Dungeon.depth = depth;
				Dungeon.level = Dungeon.newLevel();
			}
		} finally {
			Dungeon.daily = false;
		}
		Dungeon.depth = targetDepth;
		// Dungeon.newLevel clears the previous floor before target creation. Probe
		// subclasses call Level.create directly, so mirror that boundary here.
		Dungeon.level = null;
		Actor.clear();
	}

	private static void markTargetFloorGenerated(int depth) {
		Dungeon.generatedLevels.add(depth);
	}

	private static void initializeFreshRun(long seed) {
		GameSettings.set(new MemoryPreferences());
		Game.version = "3.3.8";
		// Early guide placement intentionally uses an unseeded generator. Keep
		// this seed contract independent of tutorial/meta progression.
		SPDSettings.intro(false);
		Document.ADVENTURERS_GUIDE.readPage(Document.GUIDE_INTRO);
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
		Dungeon.hero.live();
		GamesInProgress.selectedClass = HeroClass.WARRIOR;
		GamesInProgress.selectedClass.initHero(Dungeon.hero);
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

	static final class FinalFloorFacts {
		final int depth;
		final int width;
		final int height;
		final List<String> rooms;
		final List<HeapFact> heaps;
		final List<MobFact> mobs;
		final List<ItemFact> questRewards;
		final List<Integer> prePaintRng;
		final List<Integer> preMobsRng;
		final List<Integer> preItemsRng;
		final List<Integer> terrain;
		final List<Boolean> discoverable;
		final List<Integer> tileVariance;
		final List<FloorVisualFacts.TransitionFact> transitions;
		final List<FloorVisualFacts.TrapFact> traps;
		final List<FloorVisualFacts.PlantFact> plants;
		final List<FloorVisualFacts.BlobFact> blobs;

		FinalFloorFacts(
				int depth,
				int width,
				int height,
				List<String> rooms,
				List<HeapFact> heaps,
				List<MobFact> mobs,
				List<ItemFact> questRewards,
				List<Integer> prePaintRng,
				List<Integer> preMobsRng,
				List<Integer> preItemsRng,
				FloorVisualFacts visualFacts) {
			this.depth = depth;
			this.width = width;
			this.height = height;
			this.rooms = rooms;
			this.heaps = heaps;
			this.mobs = mobs;
			this.questRewards = questRewards;
			this.prePaintRng = prePaintRng;
			this.preMobsRng = preMobsRng;
			this.preItemsRng = preItemsRng;
			this.terrain = visualFacts.terrain;
			this.discoverable = visualFacts.discoverable;
			this.tileVariance = visualFacts.tileVariance;
			this.transitions = visualFacts.transitions;
			this.traps = visualFacts.traps;
			this.plants = visualFacts.plants;
			this.blobs = visualFacts.blobs;
		}
	}

	static final class MobFact {
		final int cell;
		final String mobClass;

		MobFact(int cell, String mobClass) {
			this.cell = cell;
			this.mobClass = mobClass;
		}
	}

	static final class HeapFact {
		final int cell;
		final String heapType;
		final List<ItemFact> items;

		HeapFact(int cell, String heapType, List<ItemFact> items) {
			this.cell = cell;
			this.heapType = heapType;
			this.items = items;
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

	static final class SnapshotComplete extends RuntimeException {
		private static final long serialVersionUID = 1L;
	}
}

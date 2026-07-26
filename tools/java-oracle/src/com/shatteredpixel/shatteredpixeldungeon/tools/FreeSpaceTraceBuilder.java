/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.levels.builders.Builder;
import com.shatteredpixel.shatteredpixeldungeon.levels.builders.FigureEightBuilder;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.Room;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.connection.ConnectionRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.secret.SecretRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.special.ShopRoom;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.standard.StandardRoom;
import com.watabou.utils.GameMath;
import com.watabou.utils.Point;
import com.watabou.utils.PointF;
import com.watabou.utils.Random;
import com.watabou.utils.Rect;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

/**
 * Exact, trace-only copy of the FigureEight placement path.  Builder's placement helpers are
 * static, so copying them here is the smallest way to observe the RNG-consuming equal-axis
 * choice in findFreeSpace without modifying pinned SPD sources.
 */
final class FreeSpaceTraceBuilder extends FigureEightBuilder {

	static final List<String> attempts = new ArrayList<>();
	private static final int TARGET_LEFT = 3;
	private static final int TARGET_TOP = 22;
	private static final int TARGET_RIGHT = 8;
	private static final int TARGET_BOTTOM = 29;
	private final float curveIntensity;
	private String targetTrace = "null";
	private final List<String> tieEvents = new ArrayList<>();
	private int placementOrdinal;

	FreeSpaceTraceBuilder(float curveIntensity) {
		this.curveIntensity = curveIntensity;
		setLoopShape(2, curveIntensity, 0f);
	}

	@Override
	public ArrayList<Room> build(ArrayList<Room> rooms) {
		tieEvents.clear();
		placementOrdinal = 0;
		List<Integer> start = FigureEightTraceOracle.probe();
		ArrayList<Room> result = buildTraced(rooms);
		List<Integer> end = FigureEightTraceOracle.probe();
		attempts.add("{\"attempt\":" + attempts.size() + ",\"start_rng\":" + start
				+ ",\"end_rng\":" + end + ",\"target_free_space\":" + targetTrace
				+ ",\"equal_axis_ties\":" + tieEvents
				+ ",\"success\":" + (result != null) + "}");
		return result;
	}

	@SuppressWarnings("unchecked")
	private ArrayList<Room> buildTraced(ArrayList<Room> rooms) {
		setupRooms(rooms);
		Room landmark = null;
		for (Room room : mainPathRooms) {
			if (room.maxConnections(Room.ALL) >= 4 && (landmark == null
					|| landmark.minWidth() * landmark.minHeight() < room.minWidth() * room.minHeight())) landmark = room;
		}
		if (!multiConnections.isEmpty()) mainPathRooms.add(multiConnections.remove(0));
		mainPathRooms.remove(landmark);
		multiConnections.remove(landmark);
		float startAngle = Random.Float(0, 360);
		int firstCount = mainPathRooms.size() / 2;
		if (mainPathRooms.size() % 2 == 1) firstCount += Random.Int(2);
		ArrayList<Room> remaining = (ArrayList<Room>) mainPathRooms.clone();
		ArrayList<Room> firstTemp = new ArrayList<>();
		firstTemp.add(landmark);
		for (int i = 0; i < firstCount; i++) firstTemp.add(remaining.remove(0));
		firstTemp.add((firstTemp.size() + 1) / 2, entrance);
		float[] tunnels = pathTunnelChances.clone();
		ArrayList<Room> first = expandLoop(firstTemp, tunnels);
		ArrayList<Room> secondTemp = new ArrayList<>();
		secondTemp.add(landmark);
		secondTemp.addAll(remaining);
		if (exit != null) secondTemp.add((secondTemp.size() + 1) / 2, exit);
		ArrayList<Room> second = expandLoop(secondTemp, tunnels);
		landmark.setSize();
		landmark.setPos(0, 0);
		Room prev = landmark;
		for (int i = 1; i < first.size(); i++) {
			Room room = first.get(i);
			if (place(rooms, prev, room, startAngle + targetAngle(i / (float) first.size())) == -1) return null;
			prev = room;
			if (!rooms.contains(prev)) rooms.add(prev);
		}
		while (!prev.connect(landmark)) {
			ConnectionRoom connection = ConnectionRoom.createRoom();
			if (place(rooms, prev, connection, angleBetweenRooms(prev, landmark)) == -1) return null;
			first.add(connection); rooms.add(connection); prev = connection;
		}
		PointF firstCenter = center(first);
		prev = landmark;
		startAngle += 180f;
		for (int i = 1; i < second.size(); i++) {
			Room room = second.get(i);
			if (place(rooms, prev, room, startAngle + targetAngle(i / (float) second.size())) == -1) return null;
			prev = room;
			if (!rooms.contains(prev)) rooms.add(prev);
		}
		while (!prev.connect(landmark)) {
			ConnectionRoom connection = ConnectionRoom.createRoom();
			if (place(rooms, prev, connection, angleBetweenRooms(prev, landmark)) == -1) return null;
			second.add(connection); rooms.add(connection); prev = connection;
		}
		if (shop != null) {
			float angle; int tries = 10;
			do { angle = place(rooms, entrance, shop, Random.Float(360f)); tries--; } while (angle == -1 && tries >= 0);
			if (angle == -1) return null;
		}
		PointF secondCenter = center(second);
		ArrayList<Room> branchable = new ArrayList<>(first);
		branchable.addAll(second); branchable.remove(landmark);
		ArrayList<Room> toBranch = new ArrayList<>();
		toBranch.addAll(multiConnections); toBranch.addAll(singleConnections);
		weightRooms(branchable);
		if (!branches(rooms, branchable, toBranch, branchTunnelChances, first, firstCenter, secondCenter)) return null;
		findNeighbours(rooms);
		for (Room room : rooms) for (Room neighbour : room.neigbours) {
			if (!neighbour.connected.containsKey(room) && Random.Float() < extraConnectionChance) room.connect(neighbour);
		}
		return rooms;
	}

	private ArrayList<Room> expandLoop(ArrayList<Room> source, float[] tunnels) {
		ArrayList<Room> result = new ArrayList<>();
		for (Room room : source) {
			result.add(room);
			int count = Random.chances(tunnels);
			if (count == -1) {
				tunnels = pathTunnelChances.clone();
				count = Random.chances(tunnels);
			}
			tunnels[count]--;
			for (int i = 0; i < count; i++) {
				result.add(ConnectionRoom.createRoom());
			}
		}
		return result;
	}

	private boolean branches(ArrayList<Room> rooms, ArrayList<Room> branchable, ArrayList<Room> toBranch,
			float[] chances, ArrayList<Room> first, PointF firstCenter, PointF secondCenter) {
		int index = 0;
		int failed = 0;
		float[] remaining = chances.clone();
		ArrayList<Room> links = new ArrayList<>();
		while (index < toBranch.size()) {
			if (failed > 100) return false;
			Room room = toBranch.get(index);
			Room current;
			links.clear();
			do {
				current = Random.element(branchable);
			} while (room instanceof SecretRoom && current instanceof ConnectionRoom);
			int count = Random.chances(remaining);
			if (count == -1) {
				remaining = chances.clone();
				count = Random.chances(remaining);
			}
			remaining[count]--;
			for (int j = 0; j < count; j++) {
				ConnectionRoom link = room instanceof SecretRoom
						? new com.shatteredpixel.shatteredpixeldungeon.levels.rooms.connection.MazeConnectionRoom()
						: ConnectionRoom.createRoom();
				float angle = retryPlace(rooms, current, link, 3, first, firstCenter, secondCenter);
				if (angle == -1) {
					link.clearConnections();
					for (Room old : links) {
						old.clearConnections();
						rooms.remove(old);
					}
					links.clear();
					break;
				}
				links.add(link);
				rooms.add(link);
				current = link;
			}
			if (links.size() != count) {
				failed++;
				continue;
			}
			if (retryPlace(rooms, current, room, 10, first, firstCenter, secondCenter) == -1) {
				room.clearConnections();
				for (Room link : links) {
					link.clearConnections();
					rooms.remove(link);
				}
				links.clear();
				failed++;
				continue;
			}
			for (Room link : links) {
				if (Random.Int(3) <= 1) branchable.add(link);
			}
			if (room.maxConnections(Room.ALL) > 1 && Random.Int(3) == 0) {
				if (room instanceof StandardRoom) {
					for (int j = 0; j < ((StandardRoom) room).connectionWeight(); j++) {
						branchable.add(room);
					}
				} else {
					branchable.add(room);
				}
			}
			index++;
		}
		return true;
	}

	private float retryPlace(ArrayList<Room> rooms, Room prev, Room next, int tries,
			ArrayList<Room> first, PointF firstCenter, PointF secondCenter) {
		do {
			float result = place(rooms, prev, next, branchAngle(prev, first, firstCenter, secondCenter));
			if (result != -1) return result;
			tries--;
		} while (tries > 0);
		return -1;
	}

	private float branchAngle(Room room, ArrayList<Room> first, PointF firstCenter, PointF secondCenter) {
		PointF center = first != null && first.contains(room) ? firstCenter : secondCenter;
		if (center == null) return Random.Float(360f);
		float toCenter = angleBetweenPoints(new PointF((room.left + room.right) / 2f, (room.top + room.bottom) / 2f), center);
		if (toCenter < 0) toCenter += 360f;
		float best = Random.Float(360f);
		for (int i = 0; i < 4; i++) { float candidate = Random.Float(360f); if (Math.abs(toCenter - candidate) < Math.abs(toCenter - best)) best = candidate; }
		return best;
	}

	private PointF center(ArrayList<Room> rooms) {
		PointF result = new PointF();
		for (Room room : rooms) { result.x += (room.left + room.right) / 2f; result.y += (room.top + room.bottom) / 2f; }
		result.x /= rooms.size(); result.y /= rooms.size(); return result;
	}

	private float targetAngle(float part) {
		return 360f * (float) (curveIntensity * (Math.pow(4, 4) * Math.pow((part % .5f) - .25, 5) + .25 + .5 * Math.floor(2 * part)) + (1 - curveIntensity) * part);
	}

	private float place(ArrayList<Room> collisions, Room prev, Room next, float angle) {
		int placement = placementOrdinal++;
		angle %= 360f; if (angle < 0) angle += 360f;
		PointF prevCenter = new PointF((prev.left + prev.right) / 2f, (prev.top + prev.bottom) / 2f);
		double slope = Math.tan(angle / (180 / Math.PI) + Math.PI / 2), intercept = prevCenter.y - slope * prevCenter.x;
		Point start; int direction;
		if (Math.abs(slope) >= 1) { if (angle < 90 || angle > 270) { direction = Room.TOP; start = new Point((int) Math.round((prev.top - intercept) / slope), prev.top); } else { direction = Room.BOTTOM; start = new Point((int) Math.round((prev.bottom - intercept) / slope), prev.bottom); } }
		else if (angle < 180) { direction = Room.RIGHT; start = new Point(prev.right, (int) Math.round(slope * prev.right + intercept)); }
		else { direction = Room.LEFT; start = new Point(prev.left, (int) Math.round(slope * prev.left + intercept)); }
		if (direction == Room.TOP || direction == Room.BOTTOM) start.x = (int) GameMath.gate(prev.left + 1, start.x, prev.right - 1);
		else start.y = (int) GameMath.gate(prev.top + 1, start.y, prev.bottom - 1);
		Rect space = free(start, collisions, Math.max(next.maxWidth(), next.maxHeight()),
				isTarget(prev, next), placement, prev, next);
		if (!next.setSizeWithLimit(space.width() + 1, space.height() + 1)) return -1;
		PointF target = new PointF();
		if (direction == Room.TOP) { target.y = prev.top - (next.height() - 1) / 2f; target.x = (float) ((target.y - intercept) / slope); next.setPos(Math.round(target.x - (next.width() - 1) / 2f), prev.top - (next.height() - 1)); }
		else if (direction == Room.BOTTOM) { target.y = prev.bottom + (next.height() - 1) / 2f; target.x = (float) ((target.y - intercept) / slope); next.setPos(Math.round(target.x - (next.width() - 1) / 2f), prev.bottom); }
		else if (direction == Room.RIGHT) { target.x = prev.right + (next.width() - 1) / 2f; target.y = (float) (slope * target.x + intercept); next.setPos(prev.right, Math.round(target.y - (next.height() - 1) / 2f)); }
		else { target.x = prev.left - (next.width() - 1) / 2f; target.y = (float) (slope * target.x + intercept); next.setPos(prev.left - (next.width() - 1), Math.round(target.y - (next.height() - 1) / 2f)); }
		if (direction == Room.TOP || direction == Room.BOTTOM) { if (next.right < prev.left + 2) next.shift(prev.left + 2 - next.right, 0); else if (next.left > prev.right - 2) next.shift(prev.right - 2 - next.left, 0); if (next.right > space.right) next.shift(space.right - next.right, 0); else if (next.left < space.left) next.shift(space.left - next.left, 0); }
		else { if (next.bottom < prev.top + 2) next.shift(0, prev.top + 2 - next.bottom); else if (next.top > prev.bottom - 2) next.shift(0, prev.bottom - 2 - next.top); if (next.bottom > space.bottom) next.shift(0, space.bottom - next.bottom); else if (next.top < space.top) next.shift(0, space.top - next.top); }
		return next.connect(prev) ? angleBetweenRooms(prev, next) : -1;
	}

	private boolean isTarget(Room prev, Room next) {
		return prev.left == TARGET_LEFT && prev.top == TARGET_TOP && prev.right == TARGET_RIGHT && prev.bottom == TARGET_BOTTOM && next.getClass().getSimpleName().equals("SentryRoom");
	}

	private Rect free(Point start, ArrayList<Room> collision, int max, boolean trace,
			int placement, Room prev, Room next) {
		Rect space = new Rect(start.x - max, start.y - max, start.x + max, start.y + max); ArrayList<Room> colliding = new ArrayList<>(collision); StringBuilder log = trace ? new StringBuilder("{\"start\":[").append(start.x).append(',').append(start.y).append("],\"steps\":[") : null;
		boolean firstStep = true;
		do {
			Iterator<Room> it = colliding.iterator(); while (it.hasNext()) { Room room = it.next(); if (room.isEmpty() || Math.max(space.left, room.left) >= Math.min(space.right, room.right) || Math.max(space.top, room.top) >= Math.min(space.bottom, room.bottom)) it.remove(); }
			Room closest = null; int closestDiff = Integer.MAX_VALUE; boolean inside = true; int curDiff = 0;
			StringBuilder candidates = trace ? new StringBuilder("[") : null; boolean firstCandidate = true;
			for (Room room : colliding) {
				if (start.x <= room.left) { inside = false; curDiff += room.left - start.x; } else if (start.x >= room.right) { inside = false; curDiff += start.x - room.right; }
				if (start.y <= room.top) { inside = false; curDiff += room.top - start.y; } else if (start.y >= room.bottom) { inside = false; curDiff += start.y - room.bottom; }
				if (trace) { if (!firstCandidate) candidates.append(','); firstCandidate = false; candidates.append("{\"class\":\"").append(room.getClass().getSimpleName()).append("\",\"bounds\":[").append(room.left).append(',').append(room.top).append(',').append(room.right).append(',').append(room.bottom).append("],\"cur_diff\":").append(curDiff).append(",\"inside\":").append(inside).append('}'); }
				if (inside) { space.set(start.x, start.y, start.x, start.y); if (trace) targetTrace = log.append("]}").toString(); return space; }
				if (curDiff < closestDiff) { closestDiff = curDiff; closest = room; }
			}
			if (closest != null) { int w = Integer.MAX_VALUE, h = Integer.MAX_VALUE; if (closest.left >= start.x) w = (space.right - closest.left) * (space.height() + 1); else if (closest.right <= start.x) w = (closest.right - space.left) * (space.height() + 1); if (closest.top >= start.y) h = (space.bottom - closest.top) * (space.width() + 1); else if (closest.bottom <= start.y) h = (closest.bottom - space.top) * (space.width() + 1); Integer tie = null; boolean width = w < h; if (w == h) { tie = Random.Int(2); width = tie == 0; recordTie(placement, prev, next, start, closest, w, h, tie); } if (trace) { if (!firstStep) log.append(','); firstStep = false; log.append("{\"room\":\"").append(closest.getClass().getSimpleName()).append("\",\"bounds\":[").append(closest.left).append(',').append(closest.top).append(',').append(closest.right).append(',').append(closest.bottom).append("],\"closest_diff\":").append(closestDiff).append(",\"candidates\":").append(candidates.append(']')).append(",\"w_diff\":").append(w).append(",\"h_diff\":").append(h).append(",\"tie_draw\":").append(tie == null ? "null" : tie).append(",\"axis\":\"").append(width ? "width" : "height").append("\"}"); } if (width) { if (closest.left >= start.x && closest.left < space.right) space.right = closest.left; if (closest.right <= start.x && closest.right > space.left) space.left = closest.right; } else { if (closest.top >= start.y && closest.top < space.bottom) space.bottom = closest.top; if (closest.bottom <= start.y && closest.bottom > space.top) space.top = closest.bottom; } colliding.remove(closest); } else colliding.clear();
		} while (!colliding.isEmpty());
		if (trace) targetTrace = log.append("]}").toString(); return space;
	}

	private void recordTie(int placement, Room prev, Room next, Point start, Room closest,
			int widthDiff, int heightDiff, int draw) {
		tieEvents.add("{\"placement\":" + placement + ",\"prev\":"
				+ roomJson(prev) + ",\"next_class\":\"" + next.getClass().getSimpleName()
				+ "\",\"start\":[" + start.x + ',' + start.y + "],\"closest\":"
				+ roomJson(closest) + ",\"w_diff\":" + widthDiff + ",\"h_diff\":"
				+ heightDiff + ",\"draw\":" + draw + "}");
	}

	private String roomJson(Room room) {
		return "{\"class\":\"" + room.getClass().getSimpleName() + "\",\"bounds\":["
				+ room.left + ',' + room.top + ',' + room.right + ',' + room.bottom + "]}";
	}
}

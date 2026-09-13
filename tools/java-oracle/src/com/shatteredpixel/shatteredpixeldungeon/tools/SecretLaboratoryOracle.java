package com.shatteredpixel.shatteredpixeldungeon.tools;

import java.lang.reflect.Field;
import java.util.HashMap;
import java.util.Map;

/** Pins the JVM HashMap iteration order used by SecretLaboratoryRoom.paint. */
public final class SecretLaboratoryOracle {
	private SecretLaboratoryOracle() {}

	@SuppressWarnings("unchecked")
	public static String generateJson(
			String inputSeed, long numericSeed, FloorOracle.FinalFloorFacts facts) {
		try {
			Class<?> laboratory = Class.forName(
					"com.shatteredpixel.shatteredpixeldungeon.levels.rooms.secret.SecretLaboratoryRoom");
			Field field = laboratory.getDeclaredField("potionChances");
			field.setAccessible(true);
			Map<Class<?>, Float> template = (Map<Class<?>, Float>) field.get(null);
			Map<Class<?>, Float> chances = new HashMap<>(template);
			StringBuilder out = new StringBuilder("{\n  \"schema_version\": 1,\n"
					+ "  \"contract\": \"secret-laboratory-order\",\n"
					+ "  \"spd\": " + JavaOracle.spdJson() + ",\n"
					+ "  \"input\": { \"seed\": \"" + JavaOracle.escape(inputSeed)
					+ "\", \"numeric\": " + numericSeed + " },\n  \"entries\": [\n");
			int index = 0;
			for (Map.Entry<Class<?>, Float> entry : chances.entrySet()) {
				if (index++ > 0) out.append(",\n");
				out.append("    { \"class\": \"").append(entry.getKey().getSimpleName())
						.append("\", \"weight\": ").append(entry.getValue()).append(" }");
			}
			FloorOracle.RoomFact room = facts.roomBounds.stream()
					.filter(candidate -> "SecretLaboratoryRoom".equals(candidate.roomClass))
					.findFirst().orElseThrow();
			out.append("\n  ],\n  \"selected\": [");
			index = 0;
			for (FloorOracle.HeapFact heap : facts.heaps) {
				int x = heap.cell % facts.width;
				int y = heap.cell / facts.width;
				if (x <= room.left || x >= room.right || y <= room.top || y >= room.bottom) {
					continue;
				}
				for (FloorOracle.ItemFact item : heap.items) {
					if (!item.itemClass.startsWith("PotionOf")) {
						continue;
					}
					if (index++ > 0) {
						out.append(", ");
					}
					out.append("\"").append(item.itemClass).append("\"");
				}
			}
			return out.append("]\n}\n").toString();
		} catch (ReflectiveOperationException error) {
			throw new RuntimeException(error);
		}
	}
}

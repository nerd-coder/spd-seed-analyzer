package com.shatteredpixel.shatteredpixeldungeon.tools;

import java.lang.reflect.Field;
import java.util.HashMap;
import java.util.Map;

/** Pins the JVM HashMap iteration order used by SecretLibraryRoom.paint. */
public final class SecretLibraryOracle {
	private SecretLibraryOracle() {}

	@SuppressWarnings("unchecked")
	public static String generateJson(long numericSeed) {
		try {
			// Class identity hashes, and therefore HashMap iteration order, depend
			// on the pinned run's class-loading history. Generate the canonical
			// target floor before reflecting the map used by paint().
			FloorOracle.generateFinalHeaps(numericSeed, 6);
			Class<?> secretLibrary = Class.forName(
					"com.shatteredpixel.shatteredpixeldungeon.levels.rooms.secret.SecretLibraryRoom");
			Field field = secretLibrary.getDeclaredField("scrollChances");
			field.setAccessible(true);
			Map<Class<?>, Float> template = (Map<Class<?>, Float>) field.get(null);
			// paint() selects from a copy, whose capacity/iteration order can differ
			// from the static template even with identical entries.
			Map<Class<?>, Float> chances = new HashMap<>(template);
			StringBuilder out = new StringBuilder("{\n  \"schema_version\": 1,\n  \"contract\": \"secret-library-order\",\n  \"entries\": [\n");
			int index = 0;
			for (Map.Entry<Class<?>, Float> entry : chances.entrySet()) {
				if (index++ > 0) out.append(",\n");
				out.append("    { \"class\": \"").append(entry.getKey().getSimpleName())
						.append("\", \"weight\": ").append(entry.getValue()).append(" }");
			}
			return out.append("\n  ]\n}\n").toString();
		} catch (ReflectiveOperationException error) {
			throw new RuntimeException(error);
		}
	}
}

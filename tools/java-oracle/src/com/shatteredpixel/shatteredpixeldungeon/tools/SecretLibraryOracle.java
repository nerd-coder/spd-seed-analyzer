package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.shatteredpixel.shatteredpixeldungeon.items.scrolls.Scroll;
import com.shatteredpixel.shatteredpixeldungeon.levels.rooms.secret.SecretLibraryRoom;

import java.lang.reflect.Field;
import java.util.Map;

/** Pins the JVM HashMap iteration order used by SecretLibraryRoom.paint. */
public final class SecretLibraryOracle {
	private SecretLibraryOracle() {}

	@SuppressWarnings("unchecked")
	public static String generateJson() {
		try {
			Field field = SecretLibraryRoom.class.getDeclaredField("scrollChances");
			field.setAccessible(true);
			Map<Class<? extends Scroll>, Float> chances =
					(Map<Class<? extends Scroll>, Float>) field.get(null);
			StringBuilder out = new StringBuilder("{\n  \"schema_version\": 1,\n  \"contract\": \"secret-library-order\",\n  \"entries\": [\n");
			int index = 0;
			for (Map.Entry<Class<? extends Scroll>, Float> entry : chances.entrySet()) {
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

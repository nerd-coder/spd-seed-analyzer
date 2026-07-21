/*
 * This file is part of SPD Seed Analyzer.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

package com.shatteredpixel.shatteredpixeldungeon.tools;

import com.badlogic.gdx.Preferences;

import java.util.HashMap;
import java.util.Map;

/** Minimal preferences implementation for deterministic headless generation. */
final class MemoryPreferences implements Preferences {

	private final Map<String, Object> values = new HashMap<>();

	@Override
	public Preferences putBoolean(String key, boolean value) {
		values.put(key, value);
		return this;
	}

	@Override
	public Preferences putInteger(String key, int value) {
		values.put(key, value);
		return this;
	}

	@Override
	public Preferences putLong(String key, long value) {
		values.put(key, value);
		return this;
	}

	@Override
	public Preferences putFloat(String key, float value) {
		values.put(key, value);
		return this;
	}

	@Override
	public Preferences putString(String key, String value) {
		values.put(key, value);
		return this;
	}

	@Override
	public Preferences put(Map<String, ?> entries) {
		values.putAll(entries);
		return this;
	}

	@Override
	public boolean getBoolean(String key) {
		return getBoolean(key, false);
	}

	@Override
	public int getInteger(String key) {
		return getInteger(key, 0);
	}

	@Override
	public long getLong(String key) {
		return getLong(key, 0L);
	}

	@Override
	public float getFloat(String key) {
		return getFloat(key, 0f);
	}

	@Override
	public String getString(String key) {
		return getString(key, "");
	}

	@Override
	public boolean getBoolean(String key, boolean defaultValue) {
		return value(key, Boolean.class, defaultValue);
	}

	@Override
	public int getInteger(String key, int defaultValue) {
		return value(key, Integer.class, defaultValue);
	}

	@Override
	public long getLong(String key, long defaultValue) {
		return value(key, Long.class, defaultValue);
	}

	@Override
	public float getFloat(String key, float defaultValue) {
		return value(key, Float.class, defaultValue);
	}

	@Override
	public String getString(String key, String defaultValue) {
		return value(key, String.class, defaultValue);
	}

	private <T> T value(String key, Class<T> type, T defaultValue) {
		Object value = values.get(key);
		return type.isInstance(value) ? type.cast(value) : defaultValue;
	}

	@Override
	public Map<String, ?> get() {
		return new HashMap<>(values);
	}

	@Override
	public boolean contains(String key) {
		return values.containsKey(key);
	}

	@Override
	public void clear() {
		values.clear();
	}

	@Override
	public void remove(String key) {
		values.remove(key);
	}

	@Override
	public void flush() {
		// Deliberately in memory only.
	}
}

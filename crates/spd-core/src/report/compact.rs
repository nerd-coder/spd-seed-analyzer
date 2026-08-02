use std::collections::BTreeMap;

use super::*;

impl SeedReport {
    /// Render the compact, seed-finder-style view of the report.
    ///
    /// Baseline samples are identified by `ItemEntry::prediction`, keeping the
    /// output useful for planning while preserving the explicit caveat in the
    /// machine-readable projection. It intentionally omits routine food/potion
    /// stock and unresolved entries without a concrete class.
    pub fn compact_text(&self) -> String {
        let mut out = String::new();
        let seed = self
            .seed
            .code
            .as_deref()
            .unwrap_or(self.seed.formatted.as_str());
        out.push_str(seed);
        out.push('\n');
        out.push_str("ShPD ");
        out.push_str(&self.spd_version);
        out.push_str("\n\n");

        let mut last_depth = None;
        for floor in &self.floors {
            let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for item in &floor.items {
                if item.class_name.is_none() || !is_baseline_highlight(item) {
                    continue;
                }
                let label = compact_item_name(item, &self.identities);
                let source = compact_source(item.source.as_deref(), floor);
                let labels = grouped.entry(source.unwrap_or_default()).or_default();
                if !labels.contains(&label) {
                    labels.push(label);
                }
            }
            if grouped.is_empty() {
                continue;
            }
            if let Some(previous) = last_depth {
                if floor.depth > previous + 1 {
                    out.push_str("..\n\n");
                }
            }
            for (source, mut labels) in grouped {
                if source.is_empty() {
                    labels.sort();
                    for label in labels {
                        out.push_str(&floor.depth.to_string());
                        out.push(' ');
                        out.push_str(&label);
                        out.push('\n');
                    }
                    continue;
                }
                out.push_str(&floor.depth.to_string());
                out.push(' ');
                out.push_str(&labels.join(" , "));
                out.push(' ');
                out.push_str(&source);
                out.push('\n');
            }
            out.push('\n');
            last_depth = Some(floor.depth);
        }
        out.trim_end().to_string()
    }
}

pub(crate) fn is_baseline_highlight(item: &ItemEntry) -> bool {
    let class_name = item.class_name.as_deref().unwrap_or_default();
    let source = item.source.as_deref().unwrap_or_default();
    if source.contains("ShopRoom") {
        return false;
    }
    match source {
        "Ghost.Quest" | "Wandmaker.Quest" | "Imp.Quest" | "StatueRoom" => return true,
        "CrystalChoiceRoom" => {
            return matches!(item.category.as_str(), "ring" | "artifact" | "wand")
        }
        "PitRoom" => {
            return matches!(
                item.category.as_str(),
                "ring" | "artifact" | "weapon" | "armor" | "missile" | "wand"
            )
        }
        "SecretRunestoneRoom" => return class_name == "StoneOfEnchantment",
        _ => {}
    }
    matches!(
        class_name,
        "DriedRose" | "ScrollOfRemoveCurse" | "ScrollOfTransmutation"
    )
}

fn compact_item_name(item: &ItemEntry, identities: &IdentityMaps) -> String {
    let class_name = item.class_name.as_deref().unwrap_or_default();
    let cursed = item.cursed == Some(true) || item.name.starts_with("cursed ");
    let mut name = if let Some(effect) = class_name.strip_prefix("RingOf") {
        compact_humanize(effect)
    } else if let Some(effect) = class_name.strip_prefix("ScrollOf") {
        let mut name = compact_humanize(effect);
        if let Some(identity) = identities
            .scrolls
            .iter()
            .find(|entry| entry.item == class_name)
        {
            name.push(' ');
            name.push_str(&identity.appearance);
        }
        name
    } else if let Some(effect) = class_name.strip_prefix("WandOf") {
        compact_humanize(effect)
    } else if let Some(effect) = class_name.strip_prefix("StoneOf") {
        format!("Stone of {}", compact_humanize(effect))
    } else if class_name == "DriedRose" {
        "Rose".into()
    } else {
        item.name
            .strip_prefix("cursed ")
            .unwrap_or(&item.name)
            .split_whitespace()
            .map(compact_capitalize)
            .collect::<Vec<_>>()
            .join(" ")
    };
    if matches!(item.category.as_str(), "ring" | "wand")
        && item.level.is_some_and(|level| level > 0)
    {
        name.push_str(&format!(" +{}", item.level.expect("positive level")));
    }
    if cursed {
        format!("(cursed) {name}")
    } else {
        name
    }
}

fn compact_humanize(value: &str) -> String {
    let mut words = Vec::new();
    let mut current = String::new();
    for ch in value.chars() {
        if ch.is_uppercase() && !current.is_empty() {
            words.push(current);
            current = String::new();
        }
        current.push(ch);
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
        .into_iter()
        .enumerate()
        .map(|(index, word)| {
            if index > 0 && word.eq_ignore_ascii_case("of") {
                "of".into()
            } else {
                compact_capitalize(&word)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn compact_capitalize(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    format!(
        "{}{}",
        first.to_uppercase(),
        chars.as_str().to_ascii_lowercase()
    )
}

fn compact_source(source: Option<&str>, floor: &FloorReport) -> Option<String> {
    let source = source?;
    let label = match source {
        "CrystalChoiceRoom" => "Crystal Choice".into(),
        "SecretRunestoneRoom" | "SecretArtilleryRoom" | "SecretLibraryRoom" => "Secret Room".into(),
        "StatueRoom" => "Statue".into(),
        "PitRoom" => "Pit".into(),
        "Ghost.Quest" => "Ghost".into(),
        "Wandmaker.Quest" => floor.quests.iter().find_map(|quest| match quest {
            QuestReport::OldWandmaker { baseline, .. } => Some(format!(
                "Wandmaker - {}",
                match baseline.objective {
                    WandmakerObjective::CorpseDust => "Dust",
                    WandmakerObjective::ElementalEmbers => "Embers",
                    WandmakerObjective::Rotberry => "Rotberry",
                }
            )),
            _ => None,
        })?,
        "Imp.Quest" => floor.quests.iter().find_map(|quest| match quest {
            QuestReport::AmbitiousImp { baseline, .. } => Some(format!(
                "Imp/{}",
                match baseline.target {
                    ImpTarget::Monk => "Monks",
                    ImpTarget::Golem => "Golems",
                }
            )),
            _ => None,
        })?,
        _ => return None,
    };
    Some(label)
}

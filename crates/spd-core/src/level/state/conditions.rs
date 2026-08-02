use crate::items::model::QuestRewardRole;
use crate::report::{ItemCondition, ItemDependencyCondition, ItemSpawnCondition};
use crate::trinkets::{TrinketEvent, TrinketEventAction, TrinketKind};

pub(super) fn item_conditions(artifact_conditional: bool) -> Vec<ItemSpawnCondition> {
    let mut all_of = Vec::new();
    if artifact_conditional {
        all_of.push(ItemDependencyCondition::Artifact { events: Vec::new() });
    }
    (!all_of.is_empty())
        .then_some(ItemSpawnCondition { all_of })
        .into_iter()
        .collect()
}

pub(super) fn legacy_item_notes(
    quest_role: Option<QuestRewardRole>,
    blacksmith_smith_option: bool,
    imp_shop_conditional: bool,
    parchment_level: Option<i8>,
    enchantment: Option<&str>,
) -> Vec<String> {
    if let Some(level) = parchment_level {
        return vec![format!(
            "{} — kept only with Parchment Scrap +{} or better",
            enchantment.unwrap_or("Enchantment"),
            level
        )];
    }
    let mut notes = Vec::new();
    if quest_role == Some(QuestRewardRole::WandmakerWand) {
        notes.push(
            "One of two distinct wand options; completing the quest lets you choose one.".into(),
        );
    }
    if blacksmith_smith_option {
        notes.push("One of four mutually exclusive options, available after spending 2,000 favor on Smith.".into());
        notes.push("All four options share one +0…+3 level roll. A weapon enchantment and armor glyph are retained together; Parchment Scrap +1 guarantees both when held before this floor is generated.".into());
    }
    if quest_role == Some(QuestRewardRole::ImpRing) {
        notes.push(
            "Conditional on accepting and completing the quest: 5 Monk tokens or 4 Golem tokens."
                .into(),
        );
    }
    if imp_shop_conditional {
        notes.push(
            "Appears only if the Ambitious Imp quest was completed before this shop is spawned."
                .into(),
        );
    }
    notes
}

pub(super) fn parchment_condition(depth: u32, level: i8) -> ItemCondition {
    let events = vec![TrinketEvent {
        before_depth: depth,
        action: TrinketEventAction::Acquired {
            trinket: TrinketKind::ParchmentScrap,
            min_upgrades: (level > 0).then_some(level as u8),
        },
    }];
    ItemCondition::Trinket { events }
}

pub(super) fn item_conditions_typed(
    quest_role: Option<QuestRewardRole>,
    imp_shop_conditional: bool,
) -> Vec<ItemCondition> {
    let mut conditions = Vec::new();
    match quest_role {
        Some(QuestRewardRole::WandmakerWand) => conditions.push(ItemCondition::Choice {
            group_id: "wandmaker_reward".into(),
            option_count: 2,
            selected_count: 1,
            favor_requirement: None,
        }),
        Some(QuestRewardRole::BlacksmithWeapon { .. })
        | Some(QuestRewardRole::BlacksmithMissile { .. })
        | Some(QuestRewardRole::BlacksmithArmor { .. }) => conditions.push(ItemCondition::Choice {
            group_id: "blacksmith_smith_reward".into(),
            option_count: 4,
            selected_count: 1,
            favor_requirement: Some(2000),
        }),
        Some(QuestRewardRole::ImpRing) => conditions.push(ItemCondition::Quest {
            quest_id: "ambitious_imp".into(),
            depth: None,
        }),
        _ => {}
    }
    if imp_shop_conditional {
        conditions.push(ItemCondition::Quest {
            quest_id: "ambitious_imp".into(),
            depth: None,
        });
    }
    conditions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parchment_is_an_enchantment_condition_not_a_spawn_condition() {
        let condition = parchment_condition(3, 1);
        assert!(matches!(condition, ItemCondition::Trinket { ref events }
        if matches!(events.as_slice(), [TrinketEvent {
            before_depth: 3,
            action: TrinketEventAction::Acquired {
                trinket: TrinketKind::ParchmentScrap,
                min_upgrades: Some(1),
            },
        }])));
        assert!(item_conditions(false).is_empty());
    }

    #[test]
    fn quest_and_choice_conditions_have_stable_ids() {
        let wandmaker = item_conditions_typed(Some(QuestRewardRole::WandmakerWand), false);
        assert!(
            matches!(&wandmaker[0], ItemCondition::Choice { group_id, option_count: 2, selected_count: 1, favor_requirement: None } if group_id == "wandmaker_reward")
        );

        let blacksmith = item_conditions_typed(
            Some(QuestRewardRole::BlacksmithWeapon {
                tier: 3,
                minimum_parchment_level: None,
            }),
            false,
        );
        assert!(
            matches!(&blacksmith[0], ItemCondition::Choice { group_id, favor_requirement: Some(2000), .. } if group_id == "blacksmith_smith_reward")
        );
        assert_eq!(blacksmith.len(), 1);

        let imp = item_conditions_typed(Some(QuestRewardRole::ImpRing), false);
        assert!(
            matches!(&imp[0], ItemCondition::Quest { quest_id, depth: None } if quest_id == "ambitious_imp")
        );
    }
}

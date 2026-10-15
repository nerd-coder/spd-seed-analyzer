//! Renders `web/src/generated/game-ids.ts`.

use std::fmt::Write;

use crate::{BlobId, Enchantment, ItemCategory, ItemId, MobId, PlantId, RoomId, TrapId};

struct Table {
    /// TS const and type name, e.g. `ItemId`.
    name: &'static str,
    /// Prefix for the helper functions, e.g. `item` → `itemClassName`.
    helper: &'static str,
    names: Vec<&'static str>,
}

fn tables() -> Vec<Table> {
    vec![
        Table {
            name: "ItemId",
            helper: "item",
            names: ItemId::ALL.iter().map(|v| v.class_name()).collect(),
        },
        Table {
            name: "Enchantment",
            helper: "enchantment",
            names: Enchantment::ALL.iter().map(|v| v.class_name()).collect(),
        },
        Table {
            name: "MobId",
            helper: "mob",
            names: MobId::ALL.iter().map(|v| v.class_name()).collect(),
        },
        Table {
            name: "RoomId",
            helper: "room",
            names: RoomId::ALL.iter().map(|v| v.class_name()).collect(),
        },
        Table {
            name: "TrapId",
            helper: "trap",
            names: TrapId::ALL.iter().map(|v| v.class_name()).collect(),
        },
        Table {
            name: "PlantId",
            helper: "plant",
            names: PlantId::ALL.iter().map(|v| v.class_name()).collect(),
        },
        Table {
            name: "BlobId",
            helper: "blob",
            names: BlobId::ALL.iter().map(|v| v.class_name()).collect(),
        },
    ]
}

/// The full generated module, formatted to match the repo's Biome style.
pub fn render() -> String {
    let mut out =
        String::from("// Generated from crates/spd-objects by `bun run gen:ids`. Do not edit.\n");
    let _ = write!(
        out,
        "\nexport type ItemCategory =\n{}\n",
        ItemCategory::ALL
            .iter()
            .map(|category| format!("  | '{}'", category.as_str()))
            .collect::<Vec<_>>()
            .join("\n")
    );
    for table in tables() {
        let constant = screaming(table.name);
        let _ = write!(out, "\nexport const {} = {{\n", table.name);
        for (id, name) in table.names.iter().enumerate() {
            let _ = writeln!(out, "  {name}: {id},");
        }
        let _ = write!(
            out,
            "}} as const\n\nexport type {name} = (typeof {name})[keyof typeof {name}]\n",
            name = table.name
        );
        let _ = write!(out, "\nexport const {constant}_CLASS_NAMES = [\n");
        for name in &table.names {
            let _ = writeln!(out, "  '{name}',");
        }
        let _ = write!(
            out,
            "] as const\n\
             \n\
             export type {base}ClassName = (typeof {constant}_CLASS_NAMES)[number]\n\
             \n\
             export function {helper}ClassName(id: {name}): {base}ClassName {{\n\
             \x20 return {constant}_CLASS_NAMES[id] ?? 'Unknown'\n\
             }}\n\
             \n\
             export function {helper}IdFromClassName(\n\
             \x20 className: string | null | undefined\n\
             ): {name} | undefined {{\n\
             \x20 if (!className || className === 'Unknown') return undefined\n\
             \x20 const id = ({constant}_CLASS_NAMES as readonly string[]).indexOf(className)\n\
             \x20 return id < 0 ? undefined : (id as {name})\n\
             }}\n",
            name = table.name,
            helper = table.helper,
            base = table.name.strip_suffix("Id").unwrap_or(table.name),
        );
    }
    out
}

/// `ItemId` → `ITEM`, `MobId` → `MOB`, `Enchantment` → `ENCHANTMENT`.
fn screaming(name: &str) -> String {
    name.strip_suffix("Id").unwrap_or(name).to_ascii_uppercase()
}

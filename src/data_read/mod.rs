use hecs::EntityBuilder;
use serde_json::from_str;
use serde::Deserialize;
use std::{fs, sync::Mutex, collections::HashMap};
use lazy_static::lazy_static;

mod living_structs;
use living_structs::Living;
use bracket_terminal::prelude::{RGB, PURPLE, YELLOW4};

use crate::{actor::{Position, CharSprite, Player}, fov::ViewShed, monster::Breed};

lazy_static! {
    pub static ref ENTITY_DB: Mutex<EntityDatabase> = Mutex::new(EntityDatabase::empty());
}

#[derive(Deserialize, Debug)]
pub struct EntityData {
    monsters: Vec<Living>,
}

pub struct EntityDatabase {
    entity_data: EntityData,
    monster_index: HashMap<String, usize>,
}

impl EntityDatabase {
    fn empty() -> Self {
        Self {
            entity_data: EntityData{ monsters: Vec::new(),},
            monster_index : HashMap::new(),
        }
    }

    /// Initializes the entity database to hold the objects as well as where they are located
    fn load(&mut self, data: EntityData) {
        self.entity_data = data;
        for (idx, monster) in self.entity_data.monsters.iter().enumerate() {
            self.monster_index.insert(monster.name.clone(), idx);
        }
    }
}

/// Loads all the json data of entities stored in the resources folder within the data folder
pub fn load_data_for_entities() {
    let contents: String =
        fs::read_to_string("resources/data/living.json").expect("Unable to read to a string, please check file.");
    let entity_data: EntityData = from_str(&contents).expect("Bad JSON fix it");

    ENTITY_DB.lock().unwrap().load(entity_data);
}

pub fn named_monster_builder(edb: &EntityDatabase, name: &str, pos: Position) -> Option<EntityBuilder> {
    if !edb.monster_index.contains_key(name) {
        return None;
    }
    let monster_info = &edb.entity_data.monsters[edb.monster_index[name]];
    let mut eb = EntityBuilder::new();

    eb.add(pos);

    if let Some(sprite) = &monster_info.sprite {
        let fg = RGB::from_hex(&sprite.fg).unwrap_or(RGB::named(PURPLE));
        let bg = RGB::from_hex(&sprite.bg).unwrap_or(RGB::named(YELLOW4));

        eb.add(CharSprite::new(sprite.glyph, fg, bg));
    }

    if let Some(breed) = &monster_info.breed {
        eb.add(Breed::from(breed));
    }

    if let Some(view_distance) = &monster_info.view_range {
        eb.add(ViewShed::new(*view_distance));
    }

    if let Some(_) = &monster_info.player {
        eb.add(Player);
    }

    Some(eb)
}

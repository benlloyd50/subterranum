use hecs::{Entity, World};

use crate::{
    actor::{Name, Position},
    map::{self, Map},
    monster::Breed,
};

/// Stats that are used for damage calculation
pub struct CombatStats {
    pub health: u32,
    pub strength: i32,
    pub defense: i32,
}

impl CombatStats {
    pub fn new(health: u32, strength: i32, defense: i32) -> Self {
        CombatStats {
            health,
            strength,
            defense,
        }
    }
}

/// Attacks a defender by modifying health components
pub fn attack((defender, breed): (&mut CombatStats, &Breed), (attacker, name): (&CombatStats, &Name)) -> String {
    let damage_given = (attacker.strength - defender.defense).abs();
    let new_hp = defender.health.saturating_sub(damage_given as u32);
    defender.health = new_hp;
    format!("{0:?} took {damage_given} hp from {1}", breed.name, name.0)
}

/// Iterates all beings with health and if health is 0 then it is destroyed
pub fn destroy_dead_beings(world: &mut World, map: &mut Map) {
    let mut entities_to_destroy: Vec<(Entity, Position)> = vec![];
    for (entity, (stats, pos)) in world.query::<(&CombatStats, &Position)>().iter() {
        if stats.health == 0 {
            entities_to_destroy.push((entity, pos.clone()));
        }
    }
    for (entity, pos) in entities_to_destroy {
        match world.despawn(entity) {
            Ok(..) => {
                map.beings[pos.0.to_index(map.width)] = None;
            }
            Err(e) => println!("Out of sync, couldn't destroy entity, {e}"),
        }
    }
}

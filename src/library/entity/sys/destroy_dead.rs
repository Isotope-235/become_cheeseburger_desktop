use crate::library::Entity;

pub fn run(ents: &mut Vec<Entity>) {
    ents.retain(|e| match e.hp {
        Some(hp) => hp > 0.00,
        None => true
    });
}

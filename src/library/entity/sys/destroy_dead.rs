use crate::library::Entity;

pub fn run(ents: &mut Vec<Entity>) {
    ents.retain(|e| e.alive);
}

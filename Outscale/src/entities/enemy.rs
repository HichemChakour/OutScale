use std::any::Any;
// enemy.rs
use super::entity::{Entity, HasEntity};


pub struct Enemy {
    pub entity: Entity,
}

impl Enemy {
    pub fn new(entity: Entity) -> Self {
        Enemy { entity }
    }
}

impl HasEntity for Enemy {
    fn entity(&self) -> &Entity {
        &self.entity
    }
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for crate::entities::enemy::Enemy {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
        }
    }
}


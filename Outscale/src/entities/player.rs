use std::any::Any;
use crate::entities::shadow::Shadow;
pub(crate) use super::entity::{Entity, HasEntity};


pub struct Player {
    pub entity: Entity,
    pub ombres: Vec<Shadow>
}

impl Player {
    pub fn new(entity: Entity, ombres : Vec<Shadow>) -> Self {
        Player { entity, ombres }
    }
}

impl HasEntity for Player {
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

impl Clone for crate::entities::player::Player {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            ombres: self.ombres.clone(),
        }
    }
}
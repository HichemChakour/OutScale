use std::any::Any;
// shadow.rs
pub(crate) use super::entity::{Entity, HasEntity};

pub struct Shadow {
    pub entity: Entity,
}

impl Shadow {
    pub fn new(entity: Entity) -> Self {
        Shadow { entity }
    }
}

impl HasEntity for Shadow {
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

impl Clone for crate::entities::shadow::Shadow {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
        }
    }
}
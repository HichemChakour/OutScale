use crate::entities::enemy::Enemy;
use super::entity::Entity;

pub struct Shadow {
    pub entity: Entity,
}

impl Shadow {
    pub fn new(entity: Entity) -> Self {
        Shadow { entity }
    }

    pub fn based_attack(&self) -> i32 {
        self.entity.based_attack()
    }

    pub fn use_skills(&mut self, skill_index: usize, target: &mut Entity) -> Result<String, String> {
        self.entity.use_skills(skill_index, target)
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
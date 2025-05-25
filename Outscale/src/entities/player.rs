use super::entity::Entity;

pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn new(entity: Entity) -> Self {
        Player { entity }
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

impl Clone for crate::entities::player::Player {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
        }
    }
}
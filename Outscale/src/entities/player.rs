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

    pub fn use_skills(&self, skill_index: usize) -> Result<String, String> {
        self.entity.use_skills(skill_index)
    }
}
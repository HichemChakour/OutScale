use super::entity::Entity;

pub struct Enemy {
    pub entity: Entity,
}

impl Enemy {
    pub fn new(entity: Entity) -> Self {
        Enemy { entity }
    }

    pub fn based_attack(&self) -> i32 {
        self.entity.based_attack()
    }

    pub fn use_skills(&self, skill_index: usize) -> Result<String, String> {
        self.entity.use_skills(skill_index)
    }
}
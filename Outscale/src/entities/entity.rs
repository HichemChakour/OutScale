use std::any::Any;
use crate::skills::skill::Skill;

#[derive(Debug, PartialEq, Clone)]
pub struct Entity {
    pub name: String,
    pub hp: i32,
    pub mana: i32,
    pub magic_resist: i32,
    pub armor: i32,
    pub attack_dmg: i32,
    pub magic_dmg: i32,
    pub speed: i32,
    pub dodge_chance: f32,
    pub skills: Vec<Skill>,
    pub level: i32,
}

impl Entity {
    pub fn new(
        name: String,
        hp: i32,
        mana: i32,
        magic_resist: i32,
        armor: i32,
        attack_dmg: i32,
        magic_dmg: i32,
        speed: i32,
        dodge_chance: f32,
        skills: Vec<Skill>,
        level: i32,
    ) -> Self {
        Entity {
            name,
            hp,
            mana,
            magic_resist,
            armor,
            attack_dmg,
            magic_dmg,
            speed,
            dodge_chance,
            skills,
            level,
        }
    }

    pub fn based_attack(&self) -> i32 {
        self.attack_dmg
    }

    pub fn use_skills(&mut self, skill_index: usize, target: &mut Entity) -> Result<String, String> {
        if skill_index < self.skills.len() {
            let skill = self.skills[skill_index].clone(); // Clone la compétence
            Ok(skill.apply_effects(self, target))
        } else {
            Err("Skill index out of bounds".to_string())
        }
    }

    pub fn apply_damage(&mut self, damage: i32) -> i32 {
        let damage_taken = std::cmp::min(damage, self.hp);
        self.hp -= damage_taken;
        damage_taken
    }

}

pub trait HasEntity {
    fn entity(&self) -> &Entity;
    fn entity_mut(&mut self) -> &mut Entity;

    fn based_attack(&self) -> i32 {
        self.entity().based_attack()
    }

    fn use_skills(&mut self, skill_index: usize, target: &mut Entity) -> Result<String, String> {
        self.entity_mut().use_skills(skill_index, target)
    }

    fn as_any(&self) -> &dyn Any;

}

impl Clone for Box<dyn HasEntity> {
    fn clone(&self) -> Self {
        if let Some(player) = self.as_any().downcast_ref::<crate::entities::player::Player>() {
            Box::new((*player).clone()) // Déréférencement de `player` avant de cloner
        } else if let Some(enemy) = self.as_any().downcast_ref::<crate::entities::enemy::Enemy>() {
            Box::new((*enemy).clone()) // Déréférencement de `enemy` avant de cloner
        } else if let Some(shadow) = self.as_any().downcast_ref::<crate::entities::shadow::Shadow>() {
            Box::new((*shadow).clone()) // Déréférencement de `shadow` avant de cloner
        } else {
            panic!("Unsupported type for cloning");
        }
    }
}
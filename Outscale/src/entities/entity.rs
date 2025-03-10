use crate::skills::skill::Skill;

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub hp: i32,
    pub mana: i32,
    pub magic_resist: i32,
    pub armor: i32,
    pub attack_dmg: i32,
    pub magic_dmg: i32,
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
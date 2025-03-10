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
    pub skills: Vec<String>,
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
        skills: Vec<String>,
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

    pub fn use_skills(&self, skill_index: usize) -> Result<String, String> {
        if skill_index < self.skills.len() {
            Ok(format!("Used skill: {}", self.skills[skill_index]))
        } else {
            Err("Skill index out of bounds".to_string())
        }
    }
}
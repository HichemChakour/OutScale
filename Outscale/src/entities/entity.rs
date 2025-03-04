#[derive(Debug)]
#[allow(dead_code)]
pub struct Entity {
     hp: i32,
     mana: i32,
     magique_resist: i32,
     armor: i32,
     attack_dmg: i32,
     magic_dmg: i32,
     dodge_chance: f32,
     skills: Vec<String>,
     level: i32,
}

impl Entity {
    pub fn new(
        hp: i32,
        mana: i32,
        magique_resist: i32,
        armor: i32,
        attack_dmg: i32,
        magic_dmg: i32,
        dodge_chance: f32,
        skills: Vec<String>,
        level: i32,
    ) -> Self {
        Entity {
            hp,
            mana,
            magique_resist,
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

    pub fn use_skills(&self, index: usize) -> Result<String, String> {
        if index < self.skills.len() {
            Ok(format!("Used skill: {}", self.skills[index]))
        } else {
            Err(String::from("Invalid skill index"))
        }
    }
}
use rand::prelude::IteratorRandom;
use rand::Rng;
use rand::seq::{IndexedMutRandom, /*SliceRandom*/};
use crate::entities::entity::HasEntity;
use crate::skills::skill::Skill;
pub struct EnnemiManager;

impl EnnemiManager {
    pub fn enemy_action(
        mut enemy: Box<dyn HasEntity>,
        allies: &mut Vec<Box<dyn HasEntity>>,
        enemies: &mut Vec<Box<dyn HasEntity>>,
    ) {
        let mut rng = rand::rng();
        let mut usable_skills: Vec<Skill> = vec![];
        for skill in enemy.entity().skills.iter() {
            if skill.mana_cost <= enemy.entity().mana{
                usable_skills.push(skill.clone());
            }
        }

        if usable_skills.is_empty() {
            // Si aucune compétence n'est utilisable, effectuer une attaque de base
            if let Some(target) = allies.iter_mut().choose(&mut rng) {
                let damage = enemy.based_attack();
                let damage_taken = target.entity_mut().apply_damage(damage);
                println!(
                    "\x1b[31m{}\x1b[0m effectue une attaque de base sur \x1b[34m{}\x1b[0m, infligeant \x1b[33m{}\x1b[0m dégâts ",
                    enemy.entity().name,
                    target.entity().name,
                    damage_taken
                );
            }
            return;
        }

        // Choisir une compétence aléatoire parmi celles utilisables
        let skill = usable_skills[rng.random_range(0..usable_skills.len())].clone();

        // Déterminer la cible en fonction de la compétence
        let target_pool = if !skill.for_allies { allies } else { enemies };

        if let Some(target) = target_pool.as_mut_slice().choose_mut(&mut rng) {
            if !skill.for_allies {
                let mut rng = rand::rng();
                let dodge_roll: f32 = rng.random_range(0..100) as f32;
                if dodge_roll < target.entity().dodge_chance {
                    println!(
                        "\x1b[31m{}\x1b[0m tente d'utiliser la compétence \x1b[35m{}\x1b[0m sur \x1b[34m{}\x1b[0m, mais l'attaque est esquivée ",
                        enemy.entity().name,
                        skill.name,
                        target.entity().name
                    );
                    return;
                }
                else {
                    let result = skill.apply_effects(enemy.entity_mut(), target.entity_mut());
                    println!(
                        "\x1b[31m{}\x1b[0m utilise la compétence \x1b[35m{}\x1b[0m sur \x1b[34m{}\x1b[0m \n{}",
                        enemy.entity().name,
                        skill.name,
                        target.entity().name,
                        result
                    );
                }
            }
            else {
                let result = skill.apply_effects(enemy.entity_mut(), target.entity_mut());
                println!(
                    "\x1b[31m{}\x1b[0m utilise la compétence \x1b[35m{}\x1b[0m sur \x1b[31m{}\x1b[0m \n{}",
                    enemy.entity().name,
                    skill.name,
                    target.entity().name,
                    result
                );
            }
        }
    }
}
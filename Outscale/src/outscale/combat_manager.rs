use std::thread;
use std::time::Duration;
//use rand::seq::SliceRandom;
use crate::entities::entity::HasEntity;
use crate::outscale::ennemi_manager::EnnemiManager;
use rand::Rng;

pub struct CombatManager {
    pub allies: Vec<Box<dyn HasEntity>>,
    pub enemies: Vec<Box<dyn HasEntity>>,
    pub turn_order: Vec<Box<dyn HasEntity>>,
}

#[derive(Debug)]
pub enum CancelAction {
    BackToActionMenu,
}

impl CombatManager {
    pub fn new(allies: Vec<Box<dyn HasEntity>>, enemies: Vec<Box<dyn HasEntity>>) -> Self {
        assert!(allies.len() <= 3, "Le nombre maximum d'alliés est 3.");
        assert!(enemies.len() <= 3, "Le nombre maximum d'ennemis est 3.");
        CombatManager {
            allies,
            enemies,
            turn_order: Vec::new(),
        }
    }

    pub fn determine_turn_order(&mut self) {
        let mut all_entities: Vec<&Box<dyn HasEntity>> = self.allies.iter().chain(self.enemies.iter()).collect();

        // Trier par vitesse, et aléatoire en cas d'égalité
        all_entities.sort_by(|a, b| {
            b.entity().speed.cmp(&a.entity().speed).then_with(|| rand::random::<bool>().cmp(&false))
        });

        self.turn_order = all_entities.into_iter().cloned().collect();
    }

    pub fn next_turn(&mut self) {
        if self.turn_order.is_empty() {
            self.determine_turn_order();
        }

        if let Some(current_entity) = self.turn_order.pop() {
            let is_ally = self.allies.iter().any(|a| a.entity().name == current_entity.entity().name);

            if is_ally {
                self.player_turn(current_entity);
            } else {
                self.enemy_turn(current_entity);
            }
            thread::sleep(Duration::from_millis(1500));

            if self.turn_order.is_empty() {
                self.determine_turn_order();
            }
        }
    }

    fn player_turn(&mut self, mut player: Box<dyn HasEntity>) {
        println!("C'est au tour de  \x1b[34m{}\x1b[0m de jouer ", player.entity().name);

        'main_menu: loop {
            println!("Choisissez une action :");
            println!("1. Attaque de base");
            println!("2. Utiliser une compétence");
            println!("3. Utiliser un objet");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            match choice {
                "1" => {
                    println!("\x1b[34m{}\x1b[0m effectue une attaque de base ", player.entity().name);
                    match CombatManager::choose_target(&mut self.enemies) {
                        Ok(target) => {
                            let damage = player.based_attack();
                            let damage_taken = target.entity_mut().apply_damage(damage);
                            println!(
                                "\x1b[34m{}\x1b[0m inflige \x1b[33m{}\x1b[0m dégâts à \x1b[31m{}!\x1b[0m",
                                player.entity().name,
                                damage_taken,
                                target.entity().name
                            );
                            break 'main_menu;
                        }
                        Err(CancelAction::BackToActionMenu) => {
                            println!("Retour au menu principal.");
                            continue 'main_menu;
                        }
                    }
                }

                "2" => {
                    'skill_menu: loop {
                        println!("Choisissez une compétence (ou tapez 'q' pour revenir) :");

                        let skills = player.entity().skills.clone(); // Clone pour éviter les problèmes d'emprunt
                        for (i, skill) in skills.iter().enumerate() {
                            println!("\x1b[33m{}\x1b[0m. \x1b[35m{}\x1b[0m", i + 1, skill.name);
                        }

                        let mut skill_choice = String::new();
                        std::io::stdin().read_line(&mut skill_choice).unwrap();
                        let skill_choice = skill_choice.trim();

                        if skill_choice.eq_ignore_ascii_case("q") {
                            println!("Retour au menu principal.");
                            break 'skill_menu;
                        }

                        if let Ok(skill_index) = skill_choice.parse::<usize>() {
                            if skill_index > 0 && skill_index <= skills.len() {
                                let skill = skills[skill_index - 1].clone();
                                println!(
                                    "\x1b[34m{}\x1b[0m utilise la compétence \x1b[35m{}\x1b[0m ",
                                    player.entity().name,
                                    skill.name
                                );
                                match CombatManager::choose_target(&mut self.enemies) {
                                    Ok(target) => {
                                        let mut rng = rand::rng();
                                        let dodge_roll: i32 = rng.random_range(0..100);
                                        if dodge_roll < target.entity().dodge_chance as i32 && !skill.for_allies {
                                            println!(
                                                "\x1b[31m{}\x1b[0m à ésquivé ",
                                                target.entity().name
                                            );
                                        } else {
                                            let result = skill.apply_effects(player.entity_mut(), target.entity_mut());
                                            println!("\x1b[33m{}\x1b[0m", result);
                                        }
                                        break 'main_menu;
                                    }
                                    Err(CancelAction::BackToActionMenu) => {
                                        println!("Retour au menu des compétences.");
                                        break 'skill_menu;
                                    }
                                }
                            }
                        }

                        println!("Choix invalide !");
                    }
                }

                "3" => {
                    println!("Utiliser un objet n'est pas encore implémenté.");
                    continue 'main_menu;
                }

                _ => {
                    println!("Choix invalide ");
                    continue;
                }
            }
        }
    }

    fn choose_target(targets: &mut Vec<Box<dyn HasEntity>>) -> Result<&mut Box<dyn HasEntity>, CancelAction> {
        loop {
            println!("Choisissez une cible (ou tapez 'q' pour revenir) :");
            for (i, target) in targets.iter().enumerate() {
                println!("\x1b[33m{}\x1b[0m. \x1b[31m{}\x1b[0m", i + 1, target.entity().name);
            }

            let mut target_choice = String::new();
            std::io::stdin().read_line(&mut target_choice).unwrap();
            let trimmed = target_choice.trim();

            if trimmed.eq_ignore_ascii_case("q") {
                return Err(CancelAction::BackToActionMenu);
            }

            if let Ok(index) = trimmed.parse::<usize>() {
                if index > 0 && index <= targets.len() {
                    return Ok(&mut targets[index - 1]);
                }
            }

            println!("Choix invalide ");
        }
    }

    fn enemy_turn(&mut self, enemy: Box<dyn HasEntity>) {
        println!("C'est au tour de \x1b[31m{}\x1b[0m de jouer ", enemy.entity().name);

        EnnemiManager::enemy_action(enemy, &mut self.allies, &mut self.enemies);
    }
}

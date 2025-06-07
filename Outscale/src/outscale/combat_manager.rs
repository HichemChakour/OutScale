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
    allies_sans_stat_inventaire: Vec<Box<dyn HasEntity>>,
    pub victory: bool,
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
            allies_sans_stat_inventaire: Vec::new(),
            victory: false,
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
            println!("4. Voir les statistiques des entités");

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
                                                "\x1b[31m{}\x1b[0m a esquivé ",
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
                    println!("Quel objet voulez-vous utiliser ? (ou tapez 'q' pour revenir) :");
                    println!("1. Potion de vie \x1b[32m(+65% PV max)\x1b[0m");
                    println!("2. Potion de mana \x1b[34m(+65% Mana max)\x1b[0m");

                    let mut obj_choice = String::new();
                    std::io::stdin().read_line(&mut obj_choice).unwrap();
                    let obj_choice = obj_choice.trim();

                    if obj_choice.eq_ignore_ascii_case("q") {
                        continue 'main_menu;
                    }

                    // Modifier directement l'entité
                    match obj_choice {
                        "1" => {
                            let max_hp = player.entity().max_hp;
                            let current_hp = player.entity().hp;
                            let heal = ((max_hp as f32) * 0.65).round() as i32;
                            let new_hp = (current_hp + heal).min(max_hp);

                            // Appliquer la guérison
                            player.entity_mut().hp = new_hp;

                            // Mettre à jour l'entité dans le vecteur des alliés si nécessaire
                            for ally in &mut self.allies {
                                if ally.entity().name == player.entity().name {
                                    ally.entity_mut().hp = new_hp;
                                    break;
                                }
                            }

                            println!("Vous récupérez {} PV (\x1b[32m{}/{}\x1b[0m)", heal, new_hp, max_hp);
                            break 'main_menu;
                        }
                        "2" => {
                            let max_mana = player.entity().max_mana;
                            let current_mana = player.entity().mana;
                            let mana_gain = ((max_mana as f32) * 0.65).round() as i32;
                            let new_mana = (current_mana + mana_gain).min(max_mana);

                            // Appliquer le gain de mana
                            player.entity_mut().mana = new_mana;

                            // Mettre à jour l'entité dans le vecteur des alliés si nécessaire
                            for ally in &mut self.allies {
                                if ally.entity().name == player.entity().name {
                                    ally.entity_mut().mana = new_mana;
                                    break;
                                }
                            }

                            println!("Vous récupérez {} Mana (\x1b[34m{}/{}\x1b[0m)", mana_gain, new_mana, max_mana);
                            break 'main_menu;
                        }
                        _ => {
                            println!("Choix invalide !");
                            continue 'main_menu;
                        }
                    }
                }

                "4" => {
                    // Créer une liste combinée de toutes les entités
                    let mut all_entities: Vec<(&str, &Box<dyn HasEntity>)> = Vec::new();
                    all_entities.push(("Vous", &player));
                    for ally in &self.allies {
                        if ally.entity().name != player.entity().name {
                            all_entities.push(("Allié", ally));
                        }
                    }
                    for enemy in &self.enemies {
                        all_entities.push(("Ennemi", enemy));
                    }

                    println!("Sélectionnez une entité pour voir ses statistiques (ou tapez 'q' pour revenir) :");
                    for (i, (typ, ent)) in all_entities.iter().enumerate() {
                        println!("{}. [{}] {}", i + 1, typ, ent.entity().name);
                    }

                    let mut stat_choice = String::new();
                    std::io::stdin().read_line(&mut stat_choice).unwrap();
                    let stat_choice = stat_choice.trim();

                    if stat_choice.eq_ignore_ascii_case("q") {
                        continue 'main_menu;
                    }

                    if let Ok(index) = stat_choice.parse::<usize>() {
                        if index > 0 && index <= all_entities.len() {
                           let (_typ, ent) = &all_entities[index - 1];
                            let entity = ent.entity();
                            println!("--- Statistiques de {} ---", entity.name);
                            println!("PV: \x1b[32m{}\x1b[0m", entity.hp);
                            println!("Mana: \x1b[32m{}\x1b[0m", entity.mana);
                            println!("Attaque dmg: \x1b[32m{}\x1b[0m", entity.attack_dmg);
                            println!("Dégâts magiques: \x1b[32m{}\x1b[0m", entity.magic_dmg);
                            println!("Armure: \x1b[32m{}\x1b[0m", entity.armor);
                            println!("Résistance magique: \x1b[32m{}\x1b[0m", entity.magic_resist);
                        } else {
                            println!("Choix invalide !");
                        }
                    } else {
                        println!("Choix invalide !");
                    }
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
        thread::sleep_ms(1000);
        EnnemiManager::enemy_action(enemy, &mut self.allies, &mut self.enemies);
        thread::sleep_ms(500);
    }

    // Dans combat_manager.rs, modifiez le code existant de start_combat_loop
    pub fn start_combat_loop(&mut self) {
        println!("Le combat commence !");
        let defeated_enemies = self.enemies.clone();
        self.rajout_stat_objets();
        while !self.allies.is_empty() && !self.enemies.is_empty() {
            self.determine_turn_order();

            for entity in self.turn_order.clone() {
                // Vérifier si l'entité est encore en vie
                if entity.entity().hp <= 0 {
                    continue;
                }

                let is_ally = self.allies.iter().any(|a| a.entity().name == entity.entity().name);

                if is_ally {
                    self.player_turn(entity);
                } else {
                    self.enemy_turn(entity);
                }

                // Retirer les entités mortes après chaque action
                self.allies.retain(|ally| ally.entity().hp > 0);
                self.enemies.retain(|enemy| enemy.entity().hp > 0);

                // Vérifier si le combat est terminé
                if self.allies.is_empty() || self.enemies.is_empty() {
                    break;
                }
            }
        }

        // Déterminer le vainqueur
        if self.allies.is_empty() {
            self.victory = false;
            println!("Les ennemis ont gagné !");
        } else {
            self.victory = true;
            println!("Les alliés ont gagné !");

            // Distribution de l'XP aux alliés
            use crate::outscale::levelup_manager::LevelUpManager;
            let xp_result = LevelUpManager::distribute_xp(&mut self.allies, &defeated_enemies);
            println!("{}", xp_result);

            // Afficher la progression vers le prochain niveau
            let progress = LevelUpManager::show_xp_progress(&self.allies);
            println!("{}", progress);

            // Proposer l'extraction des ennemis vaincus
            use crate::outscale::extraction_manager::ExtractionManager;
            ExtractionManager::offer_extraction(&defeated_enemies);
        }
        self.retire_stat_objets();
    }

    pub(crate) fn rajout_stat_objets(&mut self) {
        self.allies_sans_stat_inventaire.clear();

        for ally in &mut self.allies {
            // Sauvegarde l'état original
            self.allies_sans_stat_inventaire.push(ally.clone());

            // Clone l'inventaire AVANT toute modification
            let inventaire_opt = ally.entity().inventaire.clone();
            if let Some(inventaire) = inventaire_opt {
                let objets = [inventaire.tete, inventaire.jambes, inventaire.torse, inventaire.main1, inventaire.main2];
                let entity_mut = ally.entity_mut();
                for objet in &objets {
                    entity_mut.attack_dmg += objet.degats;
                    entity_mut.magic_dmg += objet.magic_resist;
                    entity_mut.armor += objet.armure;
                    entity_mut.magic_resist += objet.magic_resist;
                    entity_mut.hp += objet.hp;
                    entity_mut.hp += objet.mana;
                    entity_mut.speed += objet.vitesse;
                }
            }
        }
        
    }

    pub(crate) fn retire_stat_objets(&mut self) {
        if self.allies_sans_stat_inventaire.len() != self.allies.len() {
            println!("Impossible de restaurer les stats : sauvegarde incohérente.");
            return;
        }
        for (ally, original) in self.allies.iter_mut().zip(self.allies_sans_stat_inventaire.iter()) {
            // Remplace l'entité actuelle par la sauvegarde originale
            let original_entity = original.entity().clone();
            let entity_mut = ally.entity_mut();
            *entity_mut = original_entity;
        }
    }

}

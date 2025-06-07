use crate::entities::player::Player;
use crate::outscale::{cli_manager, database_manager::DatabaseManager};
use crate::outscale::combat_manager::CombatManager;
use crate::entities::{enemy::Enemy, entity::{Entity, HasEntity}};
use std::io::{self, Write};

static mut DRAGON_OBTENU: bool = false;

pub fn lancement_mode_histoire(db_manager: &DatabaseManager, player: &mut Player) {
    cli_manager::redaction_histoire("src/resources/dialogue/Introduction.txt");
    boucle_ville(db_manager, player);
}

fn boucle_ville(db_manager: &DatabaseManager, player: &mut Player) {
    loop {
        cli_manager::redaction_histoire("src/resources/dialogue/Avignaura.txt");

        cli_manager::menu_principal(db_manager, "AvignAura", player);
        print!("Vers quelle zone voulez-vous aller ? : ");
        io::stdout().flush().unwrap();
        let mut zone = String::new();
        io::stdin().read_line(&mut zone).unwrap();
        let zone = zone.trim();

        match zone {
            "Les Remparts" => boucle_remparts(db_manager, player),
            "Rocher des Doms" => boucle_rocher(db_manager, player),
            "MontFavé" => boucle_mont_favet(db_manager, player),
            "Palais des Papes" => {
                unsafe {
                    if DRAGON_OBTENU {
                        boucle_palais(db_manager, player);
                        break;
                    } else {
                        cli_manager::redaction_histoire("src/resources/dialogue/PP.txt");
                    }
                }
            }
            _ => {
                println!("Zone inconnue ou inaccessible.");
            }
        }
    }
}

fn boucle_remparts(db_manager: &DatabaseManager, player: &mut Player) {
    cli_manager::redaction_histoire("src/resources/dialogue/Remparts.txt");

    loop {
        let enemy = Enemy::new(Entity::new(1, "Gobelin".to_string(), 30, 30, 0, 0, 10, 0, 5, 0, 1, 0.0, vec![], 1, 0, None));
        let mut cm = CombatManager::new(
            create_allies(player),
            vec![Box::new(enemy)],
        );

        cm.start_combat_loop();
        if cm.allies.is_empty() {
            cli_manager::redaction_histoire("src/resources/dialogue/Retour_Ville.txt");
            return;
        }

        cli_manager::redaction_histoire("src/resources/dialogue/Combat_Suivant_Remparts.txt");
        println!("Continuer ? (o/n) ");
        let mut reponse = String::new();
        io::stdin().read_line(&mut reponse).unwrap();
        if reponse.trim().to_lowercase() != "o" {
            return;
        }
    }
}

fn boucle_rocher(db_manager: &DatabaseManager, player: &mut Player) {
    cli_manager::redaction_histoire("src/resources/dialogue/Rocher.txt");
    let mut compteur = 0;

    loop {
        let enemy = Enemy::new(Entity::new(1, "Chien Maudit".to_string(), 50, 30, 5, 5, 15, 0, 5, 0, 2, 0.05, vec![], 1, 0, None));
        let mut cm = CombatManager::new(
            create_allies(player),
            vec![Box::new(enemy)],
        );

        cm.start_combat_loop();
        if cm.allies.is_empty() {
            cli_manager::redaction_histoire("src/resources/dialogue/Retour_Ville.txt");
            return;
        }

        compteur += 1;
        if compteur % 5 == 0 {
            cli_manager::redaction_histoire("src/resources/dialogue/Vue_MF.txt");
        } else {
            cli_manager::redaction_histoire("src/resources/dialogue/Combat_Suivant_Rocher.txt");
        }

        println!("Continuer ? (o/n) ");
        let mut reponse = String::new();
        io::stdin().read_line(&mut reponse).unwrap();
        if reponse.trim().to_lowercase() != "o" {
            return;
        }
    }
}

fn boucle_mont_favet(db_manager: &DatabaseManager, player: &mut Player) {
    cli_manager::redaction_histoire("src/resources/dialogue/MF.txt");

    let enemy = Enemy::new(Entity::new(1, "Dragon".to_string(), 120, 80, 10, 10, 25, 20, 5, 0.1, 5, 0.1, vec![], 1, 0, None));
    let mut cm = CombatManager::new(
        create_allies(player),
        vec![Box::new(enemy)],
    );

    cm.start_combat_loop();
    if cm.allies.is_empty() {
        cli_manager::redaction_histoire("src/resources/dialogue/Retour_Ville.txt");
        return;
    }

    cli_manager::redaction_histoire("src/resources/dialogue/MF_Fin.txt");

    unsafe {
        DRAGON_OBTENU = true;
    }
}

fn boucle_palais(db_manager: &DatabaseManager, player: &mut Player) {
    cli_manager::redaction_histoire("src/resources/dialogue/PP_Dragon.txt");

    let enemy = Enemy::new(Entity::new(1, "Pape".to_string(), 180, 100, 20, 20, 35, 35, 5, 0.1, 7, 0.05, vec![], 1, 0, None));
    let mut cm = CombatManager::new(
        create_allies(player),
        vec![Box::new(enemy)],
    );

    cm.start_combat_loop();
    if cm.allies.is_empty() {
        cli_manager::redaction_histoire("src/resources/dialogue/Retour_Ville.txt");
    } else {
        cli_manager::redaction_histoire("src/resources/dialogue/Fin.txt");
    }
}

fn create_allies(player: &Player) -> Vec<Box<dyn HasEntity>> {
    let mut allies: Vec<Box<dyn HasEntity>> = player
        .ombres
        .iter()
        .map(|s| Box::new(s.clone()) as Box<dyn HasEntity>)
        .collect();
    allies.push(Box::new(player.clone()));
    allies
}

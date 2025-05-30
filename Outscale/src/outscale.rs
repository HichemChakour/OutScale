/*use rusqlite::{params, Connection, Result};
use std::path::Path;*/
pub(crate) mod database_manager;
mod init_tables;
mod cli_manager;
mod combat_manager;
mod ennemi_manager;

//use std::env;
use crate::entities::entity::{Entity, HasEntity};
use crate::outscale::combat_manager::CombatManager;
// crate::skills::inventaire::Inventaire;

// const RESOURCE_DIR: &str = "src/resources";
// const DB_PATH: &str = "src/save.db";

/*pub fn get_db_path() -> String {
    let current_dir = env::current_dir().unwrap();
    format!("{}/src/save.db", current_dir.display())
}*/

pub fn run() {

    /*if !database_manager::DatabaseManager::file_exists(DB_PATH) {
        println!("Le fichier save.db n'existe pas. Création d'une nouvelle partie...");

        let db_manager = database_manager::DatabaseManager::new(DB_PATH).unwrap();
        if let Err(e) = db_manager.execute_sql_file("././insertBDD/init_db.sql") {
            eprintln!("Erreur lors de l'exécution du fichier SQL : {}", e);
            return;
        }
        println!("Base de données initialisée avec succès.");
        return;
    }

    // Instanciation de DatabaseManager
    let db_manager = match database_manager::DatabaseManager::new(DB_PATH) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Erreur lors de la connexion à la base de données : {}", e);
            return;
        }
    };

    match db_manager.has_player_data() {
        Ok(true) => {
            println!("Une partie existante a été trouvée. Chargement...");
            lancement_mode_histoire();
        }
        Ok(false) => {
            if let Err(e) = db_manager.insert_player() {
                eprintln!("Erreur lors de l'insertion du joueur : {}", e);
                return;
            } else {
                println!("Joueur inséré avec succès !");
                lancement_mode_histoire();
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de la vérification de la table player : {}", e);
            return;
        }
    }*/

    test_combat();
    return;
}


pub fn test_combat(){
    let boule_de_feu = crate::skills::skill::Skill::new(
        "Boule de Feu".to_string(),
        0, 10, 0, 0, 0, 0, 0, 20, 0, 30, 0, false,
    );

    let coup_de_poing = crate::skills::skill::Skill::new(
        "Coup de Poing".to_string(),
        0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, false,
    );

    let heal = crate::skills::skill::Skill::new(
        "Soin".to_string(),
        20, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, true,
    );
    let hero = Box::new(crate::entities::player::Player::new(Entity::new("Hero".to_string(), 180,180,50, 50, 10, 5, 20, 15, 10, 10.0, vec![boule_de_feu.clone(), coup_de_poing.clone(),heal.clone()], 1, None))) as Box<dyn HasEntity>;

    let enemy1 = Box::new(crate::entities::enemy::Enemy::new(Entity::new(
        "Enemy1".to_string(),
        80,80, 30,30, 8, 4, 15, 10, 12, 10.0,
        vec![boule_de_feu.clone(), coup_de_poing.clone()],
        1,None
    ))) as Box<dyn HasEntity>;

    let enemy2 = Box::new(crate::entities::enemy::Enemy::new(Entity::new(
        "Enemy2".to_string(),
        90, 90, 40,40, 9, 6, 18, 12, 11, 10.0,
        vec![heal.clone()],
        1,None
    ))) as Box<dyn HasEntity>;

    let mut combat_manager = CombatManager::new(vec![hero], vec![enemy1, enemy2]);

    println!("Le combat commence !");
    while !combat_manager.allies.is_empty() && !combat_manager.enemies.is_empty() {
        combat_manager.next_turn();

        // Vérifier si un camp a été vaincu
        combat_manager.allies.retain(|ally| ally.entity().hp > 0);
        combat_manager.enemies.retain(|enemy| enemy.entity().hp > 0);
    }

    if combat_manager.allies.is_empty() {
        println!("Les ennemis ont gagné !");
    } else {
        println!("Les alliés ont gagné !");
    }

}
/*pub fn lancement_mode_histoire() {
   cli_manager::redaction_histoire(&*(RESOURCE_DIR.to_owned() + "/dialogue/Introduction.txt"));
}*/
/*use rusqlite::{params, Connection, Result};
use std::path::Path;*/
pub(crate) mod database_manager;
mod init_tables;
pub(crate) mod cli_manager;
mod combat_manager;
mod ennemi_manager;
pub mod zone;

use std::env;
//use std::env;
use crate::entities::entity::{Entity, HasEntity};
use crate::entities::player;
use crate::entities::player::Player;
use crate::entities::shadow::Shadow;
use crate::outscale::combat_manager::CombatManager;

use crate::skills::inventaire;
use crate::skills::skill::Skill;

const RESOURCE_DIR: &str = "src/resources";
const DB_PATH: &str = "src/save.db";

pub fn get_db_path() -> String {
    let current_dir = env::current_dir().unwrap();
    format!("{}/src/save.db", current_dir.display())
}

pub fn run() {

    let mut player : Player;
    if !database_manager::DatabaseManager::file_exists(DB_PATH) {
        println!("Le fichier save.db n'existe pas. Création d'une nouvelle partie...");

        let db_manager = database_manager::DatabaseManager::new(DB_PATH).unwrap();
        if let Err(e) = db_manager.execute_sql_file("././insertBDD/init_db.sql") {
            eprintln!("Erreur lors de l'exécution du fichier SQL : {}", e);
            return;
        }
        println!("Base de données initialisée avec succès.");
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
            player = db_manager.get_player_data();
            //lancement_mode_histoire();
        }
        Ok(false) => {
            if let Err(e) = db_manager.insert_player() {
                eprintln!("Erreur lors de l'insertion du joueur : {}", e);
                return;
            } else {
                player = db_manager.get_player_data();
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de la vérification de la table player : {}", e);
            return;
        }
    }

    //lancement_mode_histoire();
    test_skills(&mut player);
    db_manager.sauvegarde(player);
    let mut player2 = db_manager.get_player_data();
    test_recup_skills(&mut player2);
    return;
}

pub fn lancement_mode_histoire() {
   cli_manager::redaction_histoire(&*(RESOURCE_DIR.to_owned() + "/dialogue/Introduction.txt"));
}

pub fn test_skills(player: &mut Player) {
    let skill1 = Skill::new(0, "Coup de Poing".parse().unwrap(), "Inflige des dégâts physiques à l'ennemi.".parse().unwrap(), 0, 10, 0, 0, 0, 0, 5, 0, 0, i32::from(false), 1, false, -1);
    let skill2 = Skill::new(0,"GROS COUP DE BITE".parse().unwrap(), "Inflige des dégâts physiques à l'ennemi.".parse().unwrap(), 0, 10, 0, 0, 0, 0, 5, 0, 0, i32::from(false), 1, false, -1);
    let mut skill3 = skill2.clone();
    skill3.entity_id=1;
    player.entity.skills.push(skill1);
    player.entity.skills.push(skill2);
    player.ombres[0].entity.skills.push(skill3);

}

pub fn test_recup_skills(player: &mut Player) {
    println!("Affichage des compétences du joueur :");
    for skill in &player.entity.skills {
        println!("ID: {}, Nom: {}, Description: {}, Coût en mana: {}, Dégâts physiques: {}, Dégâts magiques: {}",
            skill.id, skill.name, skill.description, skill.mana_cost, skill.attack_dmg, skill.magic_dmg);
    }

    println!("AFfichage des compétences des ombres :");
    for shadow in &player.ombres {
        println!("Ombre: {}", shadow.entity.name);
        for skill in &shadow.entity.skills {
            println!("ID: {}, Nom: {}, Description: {}, Coût en mana: {}, Dégâts physiques: {}, Dégâts magiques: {}",
                skill.id, skill.name, skill.description, skill.mana_cost, skill.attack_dmg, skill.magic_dmg);
        }
    }
}
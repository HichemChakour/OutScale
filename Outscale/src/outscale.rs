use rusqlite::{params, Connection, Result};
use std::path::Path;
mod database_manager;
mod init_tables;
mod cli_manager;
use std::env;

const RESOURCE_DIR: &str = "src/resources";
const DB_PATH: &str = "src/save.db";

pub fn get_db_path() -> String {
    let current_dir = env::current_dir().unwrap();
    format!("{}/src/save.db", current_dir.display())
}

pub fn run() {

    if !database_manager::DatabaseManager::file_exists(DB_PATH) {
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
    }
}

pub fn lancement_mode_histoire() {
   cli_manager::redaction_histoire(&*(RESOURCE_DIR.to_owned() + "/dialogue/Introduction.txt"));
}
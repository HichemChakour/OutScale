use rusqlite::{params, Connection, Result};
use std::path::Path;

mod database_manager;

pub fn run() {
    let db_path = "./src/save.db";

    if !database_manager::DatabaseManager::file_exists(db_path) {
        println!("Le fichier save.db n'existe pas. Création d'une nouvelle partie...");

        let db_manager = database_manager::DatabaseManager::new(db_path).unwrap();
        if let Err(e) = db_manager.execute_sql_file("././insertBDD/init_db.sql") {
            eprintln!("Erreur lors de l'exécution du fichier SQL : {}", e);
            return;
        }
        println!("Base de données initialisée avec succès.");
        return;
    }

    // Instanciation de DatabaseManager
    let db_manager = match database_manager::DatabaseManager::new(db_path) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Erreur lors de la connexion à la base de données : {}", e);
            return;
        }
    };

    match db_manager.has_player_data() {
        Ok(true) => {
            println!("Une partie existante a été trouvée. Chargement...");

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
    println!("Le mode histoire a été lancé avec succès.");
}
/*use rusqlite::{params, Connection, Result};
use std::path::Path;*/
pub(crate) mod database_manager;
mod init_tables;
pub(crate) mod cli_manager;
mod combat_manager;
mod ennemi_manager;
pub mod zone
mod les_remparts;
mod extraction_manager;
mod levelup_manager;

use std::env;
use crate::entities::enemy::Enemy;
use crate::entities::entity::{Entity, HasEntity};
use crate::entities::player;
use crate::entities::player::Player;
use crate::entities::shadow::Shadow;
use crate::outscale::cli_manager::menu_principal;
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
        }
        Ok(false) => {
            if let Err(e) = db_manager.insert_player() {
                eprintln!("Erreur lors de l'insertion du joueur : {}", e);
                return;
            } else {
                player = db_manager.get_player_data();
               // lancement_mode_histoire();
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de la vérification de la table player : {}", e);
            return;
        }
    }

    //lancement_mode_histoire();
   // test_skills_et_combat(&mut player);
    //db_manager.sauvegarde(player);
    //let mut player2 = db_manager.get_player_data();
    //test_recup_skills(&mut player2);
    test_les_remparts();
    let mut player_mut = player;
    let zone_initiale = "AvignAura"; // Zone de départ par défaut

    // Marquer la zone comme visitée
    db_manager.visite_lieu(zone_initiale);

    // Lancer le menu principal
    use crate::outscale::cli_manager::menu_principal;
    menu_principal(&db_manager, zone_initiale, &mut player_mut);
    return;
}

pub fn lancement_mode_histoire() {
   cli_manager::redaction_histoire(&*(RESOURCE_DIR.to_owned() + "/dialogue/Introduction.txt"));
}



pub fn test_skills_et_combat(player: &mut Player) {
    // Création des compétences
    let skill1 = Skill::new(
        0,
        "Coup de Poing".to_string(),
        "Inflige des dégâts physiques à l'ennemi.".to_string(),
        0, 10, 0, 0, 0, 0, 5, 0, 0, 0, 1, false, -1,
    );
    let skill2 = Skill::new(
        0,
        "GROS COUP DE BITE".to_string(),
        "Inflige des dégâts physiques à l'ennemi.".to_string(),
        0, 10, 0, 0, 0, 0, 0, 1000, 0, 0, 0, false, -1,
    );
    let mut skill3 = skill2.clone();
    skill3.entity_id = 1;

    // Ajout des compétences au joueur et à ses ombres
    player.entity.skills.push(skill1);
    player.entity.skills.push(skill2);
    if let Some(first_shadow) = player.ombres.get_mut(0) {
        first_shadow.entity.skills.push(skill3);
    }

    // Création des alliés
    let mut allies: Vec<Box<dyn HasEntity>> = player
        .ombres
        .iter()
        .map(|shadow| Box::new(shadow.clone()) as Box<dyn HasEntity>)
        .collect();
    allies.push(Box::new(player.clone()));

    // Création des ennemis
    let ennemi1 = Entity::new(
        2,
        "Ennemi 1".to_string(),
        2000, 2000, 0, 0, 0, 0, 0, 0, 1,1.1, vec![], 1, 125, 2,None
    );
    let ennemi2 = Entity::new(
        3,
        "Ennemi 2".to_string(),
        2000, 2000, 0, 0, 0, 0, 0, 0, 1,1.1, vec![], 1, 125,  2,None
    );
    let ennemies: Vec<Box<dyn HasEntity>> = vec![
        Box::new(Enemy::new(ennemi1)),
        Box::new(Enemy::new(ennemi2)),
    ];

    // Initialisation du gestionnaire de combat
    let mut combat_manager = CombatManager::new(allies, ennemies);

    // Lancement du combat (si une méthode existe pour cela)
    combat_manager.start_combat_loop();
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

pub fn test_les_remparts(){
    let mut OperatorSkill = Skill::new(
        0,
        "Opérateur".to_string(),
        "Un opérateur de combat surentraîné qui élimine tout sur son passage.".to_string(),
        0,
        -150,
        0,
        0,
        0,
        0,
        0,
        9999,
        0,
        9999,
        0,
        false,
        -1
    );
    let mut playerTest = Player::new(
        Entity::new(0, "TestPlayer".to_string(), 400, 700, 10, 10, 5, 5, 1, 1, 1, 1.0, vec![], 1, 0, 1, None),
        vec![],
    );
    playerTest.entity.skills.push(OperatorSkill);
    les_remparts::lancer_les_remparts(&mut playerTest);
}
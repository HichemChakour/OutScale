use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::entities::player::Player;
use crate::outscale::database_manager::DatabaseManager;
use crate::outscale::zone::*;
use crate::outscale::combat_manager::CombatManager;
use crate::entities::entity::HasEntity;
use crate::entities::shadow::Shadow;

use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{ClearType, Clear},
};
// Fonction qui applique les styles aux balises du texte
fn apply_styles(text: &str) -> String {
    let styles = vec![
        ("[italique]", "\x1b[3m", "[/italique]", "\x1b[0m"),
        ("[gras]", "\x1b[1m", "[/gras]", "\x1b[0m"),
        ("[bleu]", "\x1b[34m", "[/bleu]", "\x1b[0m"),
        ("[vert]", "\x1b[32m", "[/vert]", "\x1b[0m"),
        ("[rouge]", "\x1b[31m", "[/rouge]", "\x1b[0m"),
    ];

    let mut styled_text = text.to_string();
    for (start_tag, start_style, end_tag, end_style) in styles {
        styled_text = styled_text.replace(start_tag, start_style).replace(end_tag, end_style);
    }

    styled_text
}

// Fonction qui demande une entrée à l'utilisateur
pub fn demander_au_joueur(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    input.trim().to_string()
}

// Fonction qui lit un fichier texte et applique les balises de style
pub fn redaction_histoire(fichier: &str) {
    use std::{thread, time};

    if !Path::new(fichier).exists() {
        eprintln!("Erreur : Le fichier spécifié n'existe pas : {}", fichier);
        return;
    }

    let contenu = match fs::read_to_string(fichier) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Erreur lors de la lecture du fichier : {}", e);
            return;
        }
    };

    let sortie = apply_styles(&contenu);
    let lignes: Vec<&str> = sortie.split('\n').collect();

    for (i, ligne) in lignes.iter().enumerate() {
        for c in ligne.chars() {
            print!("{}", c);
            io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(20));
        }

        if i < lignes.len() - 1 {
            println!();
            print!("\x1b[90mAppuyez sur Espace pour continuer...\x1b[0m");
            io::stdout().flush().unwrap();

            loop {
                if let Event::Key(key_event) = read().unwrap() {
                    if key_event.code == KeyCode::Char(' ') {
                        break;
                    }
                }
            }

            execute!(io::stdout(), Clear(ClearType::CurrentLine)).unwrap();
            println!("");
        }
    }
    println!();
}

fn combattre_ennemi_zone(db_manager: &DatabaseManager, zone_actuelle: &str, player: &mut Player) {
    // Obtenir l'ennemi spécifique à la zone
    let mut ennemis: Vec<Box<dyn HasEntity>> = Vec::new();

    match zone_actuelle {
        "MontFavé" => {
            // Récupérer le dragon noir depuis la base de données
            if let Some(dragon) = DatabaseManager::get_ennemi_by_name(&db_manager.conn, "Le dragon noir") {
                if dragon.entity.hp <= 0 {
                    println!("Le dragon noir a déjà été vaincu!");
                    return;
                }
                ennemis.push(Box::new(dragon));
            } else {
                println!("Erreur: Impossible de trouver l'ennemi de cette zone!");
                return;
            }
        },
        "Rocher des Doms" => {
            // Récupérer les ennemis spécifiques à cette zone
            if let Some(gardien) = DatabaseManager::get_ennemi_by_name(&db_manager.conn, "Gardien du temple") {
                if gardien.entity.hp > 0 {
                    ennemis.push(Box::new(gardien));
                }
            }
            if let Some(pretre) = DatabaseManager::get_ennemi_by_name(&db_manager.conn, "Le prêtre") {
                if pretre.entity.hp > 0 {
                    ennemis.push(Box::new(pretre));
                }
            }
            if let Some(imam) = DatabaseManager::get_ennemi_by_name(&db_manager.conn, "L`imame") {
                if imam.entity.hp > 0 {
                    ennemis.push(Box::new(imam));
                }
            }

            if ennemis.is_empty() {
                println!("Tous les ennemis de cette zone ont déjà été vaincus!");
                return;
            }
        },
        "Les Remparts" => {
            // Zone spéciale avec génération aléatoire d'ennemis
            println!("Les Remparts sont assaillis par des hordes de monstres!");
            println!("Une nouvelle vague arrive...");

            // Générer des ennemis aléatoires pour cette zone
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let nb_ennemis = rng.gen_range(1..=3);

            for i in 0..nb_ennemis {
                let hp = rng.gen_range(50..150);
                let atk = rng.gen_range(10..30);

                let ennemi = Shadow {
                    entity: crate::entities::shadow::Entity::new(
                        -100 - i, // ID négatif pour les ennemis temporaires
                        format!("Monstre des Remparts #{}", i+1),
                        hp,
                        hp,
                        50,
                        50,
                        5,
                        5,
                        atk,
                        5,
                        5,
                        0.1,
                        vec![],
                        1,
                        rng.gen_range(10..30),
                        1, // Classe Guerrier par défaut
                        None
                    )
                };

                ennemis.push(Box::new(ennemi));
            }
        },
        "AvignAura" => {
            // Ennemi de la ville d'AvignAura
            if let Some(corrupted) = DatabaseManager::get_ennemi_by_name(&db_manager.conn, "Gardien Corrompu") {
                if corrupted.entity.hp <= 0 {
                    println!("Le Gardien Corrompu a déjà été vaincu!");
                    return;
                }
                ennemis.push(Box::new(corrupted));
            } else {
                println!("Erreur: Impossible de trouver l'ennemi de cette zone!");
                return;
            }
        },
        "Palais des Papes" => {
            // Le boss final - Le Pape corrompu
            if let Some(pape) = DatabaseManager::get_ennemi_by_name(&db_manager.conn, "Pape corrompu") {
                if pape.entity.hp <= 0 {
                    println!("Le Pape corrompu a déjà été vaincu!");
                    return;
                }
                ennemis.push(Box::new(pape));
            } else {
                println!("Erreur: Impossible de trouver l'ennemi de cette zone!");
                return;
            }
        },
        _ => {
            println!("Cette zone ne contient pas d'ennemis à combattre.");
            return;
        }
    }

    // Préparer le joueur et ses alliés
    let mut allies: Vec<Box<dyn HasEntity>> = Vec::new();

    // Ajouter le joueur principal
    allies.push(Box::new(player.clone()));

    // Ajouter jusqu'à deux ombres comme alliés
    let mut ombres_dispo = player.ombres.clone();
    let mut i = 0;
    while i < ombres_dispo.len() && allies.len() < 3 {
        if ombres_dispo[i].entity.hp > 0 {
            allies.push(Box::new(ombres_dispo[i].clone()));
        }
        i += 1;
    }

    // Créer et lancer le combat
    let mut combat_manager = CombatManager::new(allies, ennemis);
    combat_manager.start_combat_loop();

    // Mettre à jour le joueur après le combat
    // Cette partie est simplifiée - dans une implémentation complète,
    // il faudrait synchroniser toutes les entités après le combat
    db_manager.sauvegarde(player.clone());
}

pub fn menu_principal(db_manager: &DatabaseManager, zone_actuelle : &str, player: &mut Player) {
    println!("Vous êtes actuellement dans la zone : {}. Que comptez vous faire ?", zone_actuelle);
    println!("i. Ouvrir l'inventaire de vos personnages");
    println!("j. Ouvrir le journal");
    println!("s. Ouvrir l'inventaire des Ombres");
    println!("f. Combattre l'ennemie de zone");
    println!("c. Voir tout les lieux visités");
    println!("indice. Avoir un indice");
    println!("q. Quitter le jeu");
    
    let choix = demander_au_joueur("Votre choix : ");

    loop {
        match choix.as_str() {
            "i" => {
                println!("Ouverture de l'inventaire de vos personnages...");
            },
            "j" => {
                println!("Ouverture du journal...");
            },
            "s" => {
                println!("Ouverture de l'inventaire des Ombres...");

            },
            "c" => {
                println!("Affichage des lieux visités...");
                afficher_lieux_visites(db_manager);
                menu_principal(db_manager, zone_actuelle, player);
            },
            "f" => {
                println!("Vous vous préparez au combat...");
                combattre_ennemi_zone(db_manager, zone_actuelle, player);
                menu_principal(db_manager, zone_actuelle, player);
            },
            "indice" => {println!("Voici un indice...");
                indice();
            },
            "MontFavé" => {
                deplacement_zone(db_manager, "MontFavé");
                menu_principal(db_manager, "MontFavé", player);
            }
            "Rocher des Doms" => {
                deplacement_zone(db_manager, "Rocher des Doms");
                menu_principal(db_manager, "Rocher des Doms", player);
            }
            "Les Remparts" => {
                deplacement_zone(db_manager, "Les Remparts");
                menu_principal(db_manager, "Les Remparts", player);
            }
            "AvignAura" => {
                deplacement_zone(db_manager, "AvignAura");
                menu_principal(db_manager, "AvignAura", player);
            }
            "Palais des Papes" => {
                deplacement_zone(db_manager, "Palais des Papes");
                menu_principal(db_manager, "Palais des Papes", player);
            }
            "q" => {
                println!("Quitter le jeu...");
                println!("Sauvegarde en cours ...");
                sauvegarde(db_manager, player.clone());
                break;
            }
            _ => {
                println!("Choix invalide. Veuillez réessayer.");
                continue;
            }
        }
        break;
    }
}

fn indice() {
    println!("Ps encore fait");
}

fn sauvegarde(db_manager: &DatabaseManager, player : Player) {
    db_manager.sauvegarde(player);
}


fn afficher_lieux_visites(db_manager: &DatabaseManager) {
    let zones_visitees = db_manager.get_visited_zones();
    if zones_visitees.is_empty() {
        println!("Aucune zone visitée pour le moment.");
    } else {
        println!("Zones visitées : {}", zones_visitees);
    }
}
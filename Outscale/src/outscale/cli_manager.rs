use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::entities::player::Player;
use crate::outscale::database_manager::DatabaseManager;
use crate::outscale::zone::*;

// Fonction qui applique les styles aux balises du texte
fn apply_styles(text: &str) -> String {
    // Liste des styles avec leurs balises et styles correspondants
    let styles = vec![
        ("[italique]", "\x1b[3m", "[/italique]", "\x1b[0m"),
        ("[gras]", "\x1b[1m", "[/gras]", "\x1b[0m"),
        ("[bleu]", "\x1b[34m", "[/bleu]", "\x1b[0m"),
        ("[vert]", "\x1b[32m", "[/vert]", "\x1b[0m"),
        ("[rouge]", "\x1b[31m", "[/rouge]", "\x1b[0m"),
    ];

    let mut styled_text = text.to_string();

    // Remplacer les balises par les codes ANSI de style
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
    // Vérification de l'existence du fichier
    if !Path::new(fichier).exists() {
        eprintln!("Erreur : Le fichier spécifié n'existe pas : {}", fichier);
        return;
    }

    // Lecture du contenu du fichier
    let contenu = match fs::read_to_string(fichier) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Erreur lors de la lecture du fichier : {}", e);
            return;
        }
    };

    // Applique les styles au texte du fichier
    let sortie = apply_styles(&contenu);

    // Affichage du texte modifié
    println!("{}", sortie);
}

// Test avec du texte statique
pub fn redaction_histoire_test() {

}
pub fn menu_principal(db_manager: &DatabaseManager, zone_actuelle : &str, player: &mut Player) {
    println!("Vous êtes actuellement dans la zone : {}. Que comptez vous faire ?", zone_actuelle);
    println!("i. Ouvrir l'inventaire de vos personnages");
    println!("j. Ouvrir le journal");
    println!("s. Ouvrir l'inventaire des Ombres");
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
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::entities::player::Player;
use crate::outscale::database_manager::DatabaseManager;
use crate::outscale::zone::*;

use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{ClearType, Clear},
};

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

pub fn demander_au_joueur(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    input.trim().to_string()
}

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

pub fn menu_principal(db_manager: &DatabaseManager, zone_actuelle: &str, player: &mut Player) {
    println!("Vous êtes actuellement dans la zone : {}. Que comptez vous faire ?", zone_actuelle);
    println!("m. Aller quelque part");
    println!("i. Ouvrir l'inventaire de vos personnages");
    //println!("j. Ouvrir le journal");
    println!("s. Ouvrir l'inventaire des Ombres");
    println!("c. Voir tout les lieux visités");
    //println!("indice. Avoir un indice");
    println!("q. Quitter le jeu");

    let choix = demander_au_joueur("Votre choix : ");

    match choix.as_str() {
        "i" => {
            println!("Ouverture de l'inventaire de vos personnages...");
            menu_principal(db_manager, zone_actuelle, player);
        },
        "j" => {
            println!("Ouverture du journal...");
            menu_principal(db_manager, zone_actuelle, player);
        },
        "s" => {
            println!("Ouverture de l'inventaire des Ombres...");
            menu_principal(db_manager, zone_actuelle, player);
        },
        "c" => {
            afficher_lieux_visites(db_manager);
            menu_principal(db_manager, zone_actuelle, player);
        },
        "indice" => {
            afficher_indice(zone_actuelle);
            menu_principal(db_manager, zone_actuelle, player);
        },
        "q" => {
            println!("Quitter le jeu...");
            println!("Sauvegarde en cours ...");
            db_manager.sauvegarde(player.clone());
            std::process::exit(0);
        },
        "m" => {
            /*let zone = demander_au_joueur("Vers quelle zone voulez-vous aller ? : ");
            match zone.as_str() {
                "Les Remparts" | "Rocher des Doms" | "MontFavé" | "Palais des Papes" | "AvignAura" => {
                    // Laisse le menu_principal se terminer et boucle_ville traitera la zone
                    return;
                }
                _ => {
                    println!("Zone inconnue ou inaccessible.");
                    menu_principal(db_manager, zone_actuelle, player);
                }
            }*/
            return;
        }
        _ => {
            println!("Choix invalide. Veuillez réessayer.");
            menu_principal(db_manager, zone_actuelle, player);
        }
    }
}

fn afficher_indice(zone: &str) {
    println!("Pas encore d'indice défini pour la zone : {}", zone);
}

fn afficher_lieux_visites(db_manager: &DatabaseManager) {
    let zones_visitees = db_manager.get_visited_zones();
    if zones_visitees.is_empty() {
        println!("Aucune zone visitée pour le moment.");
    } else {
        println!("Zones visitées : {}", zones_visitees);
    }
}

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::entities::player::Player;
use crate::outscale::database_manager::DatabaseManager;
use crate::outscale::zone::*;
use crate::outscale::combat_manager::CombatManager;
use crate::entities::entity::HasEntity;
use crate::skills::skill::Skill;
use crate::outscale::inventaire_manager;
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{ClearType, Clear},
};
use crate::outscale::les_remparts::lancer_les_remparts;

fn apply_styles(text: &str) -> String {
    let styles = vec![
        ("[italique]", "\x1b[3m", "[/italique]", "\x1b[0m"),
        ("[gras]", "\x1b[1m", "[/gras]", "\x1b[0m"),
        ("[bleu]", "\x1b[34m", "[/bleu]", "\x1b[0m"),
        ("[vert]", "\x1b[32m", "[/vert]", "\x1b[0m"),
        ("[rouge]", "\x1b[31m", "[/rouge]", "\x1b[0m"),
        ("[jaune]", "\x1b[33m", "[/jaune]", "\x1b[0m"),
        ("[cyan]", "\x1b[36m", "[/cyan]", "\x1b[0m"),
        ("[magenta]", "\x1b[35m", "[/magenta]", "\x1b[0m"),
        ("[blanc]", "\x1b[37m", "[/blanc]", "\x1b[0m"),
        ("[noir]", "\x1b[30m", "[/noir]", "\x1b[0m"),
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
            if let Some(mut gardien) = DatabaseManager::get_ennemi_by_name(&db_manager.conn, "Gardien du temple") {
                if gardien.entity.hp > 0 {
                    let mut skills = DatabaseManager::get_skills_by_entity_id(&db_manager.conn, gardien.entity.id.clone(), 0);
                    if skills.is_empty() {
                        println!("Le Gardien du temple n'a pas de compétences assignées.");
                    } else {
                        gardien.entity.skills = skills;
                        println!("{}", gardien.entity.skills.len());
                    }
                    ennemis.push(Box::new(gardien.clone()));
                }
            }
            if let Some(mut pretre) = DatabaseManager::get_ennemi_by_name(&db_manager.conn, "Le prêtre") {
                if pretre.entity.hp > 0 {
                    let mut skills = DatabaseManager::get_skills_by_entity_id(&db_manager.conn, pretre.entity.id.clone(), 0);
                    if skills.is_empty() {
                        println!("Le prêtre n'a pas de compétences assignées.");
                    } else {
                        pretre.entity.skills = skills;
                    }
                    ennemis.push(Box::new(pretre.clone()));
                }
            }
            if let Some(mut imam) = DatabaseManager::get_ennemi_by_name(&db_manager.conn, "L`imame") {
                if imam.entity.hp > 0 {
                    let mut skills = DatabaseManager::get_skills_by_entity_id(&db_manager.conn, imam.entity.id.clone(), 0);
                    if skills.is_empty() {
                        println!("L'imame n'a pas de compétences assignées.");
                    } else {
                        imam.entity.skills = skills;
                    }
                    ennemis.push(Box::new(imam.clone()));
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
            lancer_les_remparts(player);
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

pub fn menu_competences(db_manager: &DatabaseManager, player: &mut Player) {
    loop {
        // Récupérer les compétences découvertes
        let skills = get_discovered_skills(db_manager);
        if skills.is_empty() {
            println!("Vous n'avez pas encore découvert de compétences.");
            return;
        }

        // Afficher les compétences découvertes
        println!("\x1b[33m=== Compétences découvertes ===\x1b[0m");
        for (i, skill) in skills.iter().enumerate() {
            println!("{}. \x1b[36m{}\x1b[0m - {}", i+1, skill.name, skill.description);
        }

        // Afficher les compétences actuelles du joueur
        println!("\n\x1b[33m=== Vos compétences actuelles ===\x1b[0m");
        if player.entity.skills.is_empty() {
            println!("Vous n'avez aucune compétence équipée.");
        } else {
            for (i, skill) in player.entity.skills.iter().enumerate() {
                println!("{}. \x1b[32m{}\x1b[0m - {}", i+1, skill.name, skill.description);
            }
        }

        // Menu d'actions
        println!("\n\x1b[33m=== Actions ===\x1b[0m");
        println!("1. Équiper une compétence");
        println!("2. Retirer une compétence");
        println!("q. Quitter le menu des compétences");

        let choix = demander_au_joueur("Votre choix : ");

        match choix.as_str() {
            "1" => {
                // Vérifier si le joueur a déjà le maximum de compétences
                if player.entity.skills.len() >= 3 {
                    println!("Vous ne pouvez pas équiper plus de 3 compétences. Retirez-en une d'abord.");
                    continue;
                }

                // Demander quelle compétence équiper
                let skill_choice = demander_au_joueur("Quelle compétence voulez-vous équiper ? (numéro ou 'q' pour annuler) : ");
                if skill_choice.eq_ignore_ascii_case("q") {
                    continue;
                }

                if let Ok(index) = skill_choice.parse::<usize>() {
                    if index > 0 && index <= skills.len() {
                        let skill_to_equip = skills[index - 1].clone();

                        // Vérifier si le joueur a déjà cette compétence
                        if player.entity.skills.iter().any(|s| s.name == skill_to_equip.name) {
                            println!("Vous avez déjà équipé cette compétence.");
                            continue;
                        }

                        // Copier la compétence et l'assigner au joueur
                        let mut new_skill = skill_to_equip.clone();
                        new_skill.entity_id = -1; // ID du joueur (-1)
                        player.entity.skills.push(new_skill);
                        println!("La compétence \x1b[32m{}\x1b[0m a été équipée avec succès !", skill_to_equip.name);
                    } else {
                        println!("Choix invalide !");
                    }
                } else {
                    println!("Choix invalide !");
                }
            },
            "2" => {
                if player.entity.skills.is_empty() {
                    println!("Vous n'avez aucune compétence à retirer.");
                    continue;
                }

                // Demander quelle compétence retirer
                let skill_choice = demander_au_joueur("Quelle compétence voulez-vous retirer ? (numéro ou 'q' pour annuler) : ");
                if skill_choice.eq_ignore_ascii_case("q") {
                    continue;
                }

                if let Ok(index) = skill_choice.parse::<usize>() {
                    if index > 0 && index <= player.entity.skills.len() {
                        let removed_name = player.entity.skills[index - 1].name.clone();
                        player.entity.skills.remove(index - 1);
                        println!("La compétence \x1b[31m{}\x1b[0m a été retirée !", removed_name);
                    } else {
                        println!("Choix invalide !");
                    }
                } else {
                    println!("Choix invalide !");
                }
            },
            "q" => {
                return;
            },
            _ => {
                println!("Choix invalide !");
            }
        }
    }
}

// Fonction pour récupérer les compétences découvertes
fn get_discovered_skills(db_manager: &DatabaseManager) -> Vec<Skill> {
    let query = "SELECT * FROM skills WHERE discovered = 1 AND player_id IS NULL";
    let mut stmt = db_manager.conn.prepare(query).expect("Erreur lors de la préparation de la requête");
    let skills_iter = stmt.query_map([], |row| {
        Ok(Skill {
            id: row.get(0)?,
            name: row.get(1)?,
            discovered: row.get(2)?,
            description: row.get(3)?,
            hp_refound: row.get(4)?,
            mana_cost: row.get(5)?,
            mana_refound: row.get(6)?,
            magic_resist_debuff: row.get(7)?,
            magic_resist_buff: row.get(8)?,
            armor_debuff: row.get(9)?,
            armor_buff: row.get(10)?,
            attack_dmg: row.get(11)?,
            attack_dmg_buff: row.get(12)?,
            magic_dmg: row.get(13)?,
            magic_dmg_buff: row.get(14)?,
            for_allies: row.get(15)?,
            entity_id: row.get::<_, Option<i32>>(16)?.unwrap_or(-1),
            player_id: row.get::<_, Option<i32>>(17)?.unwrap_or(-1),
        })
    }).expect("Erreur lors de l'exécution de la requête");

    let mut skills = Vec::new();
    for skill in skills_iter {
        match skill {
            Ok(skill) => skills.push(skill),
            Err(e) => println!("Erreur lors de la récupération d'une compétence: {}", e)
        }
    }
    skills
}

pub fn menu_principal(db_manager: &DatabaseManager, zone_actuelle : &str, player: &mut Player) {
    println!("Vous êtes actuellement dans la zone : {}. Que comptez vous faire ?", zone_actuelle);
    println!("i. Ouvrir l'inventaire de votre personnage");
    println!("j. Ouvrir le journal");
    println!("s. Ouvrir l'inventaire des Ombres");
    println!("f. Combattre l'ennemie de zone");
    println!("c. Voir tout les lieux visités");
    println!("k. Gérer vos compétences");
    println!("indice. Avoir un indice");
    println!("q. Quitter le jeu");
    
    let choix = demander_au_joueur("Votre choix : ");

    loop {
        match choix.as_str() {
            "i" => {
                inventaire_manager::gerer_inventaire_joueur(player);
                menu_principal(db_manager, zone_actuelle, player);
            },
            "j" => {
                println!("Ouverture du journal...");
            },
            "s" => {
                println!("Ouverture de l'inventaire des Ombres...");
                use crate::outscale::shadow_manager::ShadowManager;
                ShadowManager::select_shadow(db_manager, player);
                menu_principal(db_manager, zone_actuelle, player);
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
            "k" => {  // Nouveau cas
                println!("Gestion des compétences...");
                menu_competences(db_manager, player);
                menu_principal(db_manager, zone_actuelle, player);
            },
            "indice" => {println!("Voici un indice...");
                indice();
            },
            "MontFavé" => {
                deplacement_zone(db_manager, "MontFavé");
                self::redaction_histoire("src/resources/dialogue/MF.txt");
                menu_principal(db_manager, "MontFavé", player);
            }
            "Rocher des Doms" => {
                deplacement_zone(db_manager, "Rocher des Doms");
                self::redaction_histoire("src/resources/dialogue/Rocher.txt");
                menu_principal(db_manager, "Rocher des Doms", player);
            }
            "Les Remparts" => {
                deplacement_zone(db_manager, "Les Remparts");
                self::redaction_histoire("src/resources/dialogue/Remparts.txt");
                menu_principal(db_manager, "Les Remparts", player);
            }
            "AvignAura" => {
                deplacement_zone(db_manager, "AvignAura");
                self::redaction_histoire("src/resources/dialogue/Avignaura.txt");
                menu_principal(db_manager, "AvignAura", player);
            }
            "Palais des Papes" => {
                deplacement_zone(db_manager, "Palais des Papes");
                self::redaction_histoire("src/resources/dialogue/PP_Dragon.txt");
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
                menu_principal(db_manager, zone_actuelle, player);
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
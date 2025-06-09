use crate::skills::inventaire::Inventaire;
use crate::skills::object::Objet;
use std::io::{self, Write};
use crate::entities::player::Player;

pub fn gerer_inventaire_joueur(player: &mut Player) {
    // On accède à l'inventaire du joueur
    if let Some(ref mut inventaire) = player.entity.inventaire {
        loop {
            println!("\n{}", inventaire.to_string());
            println!("{}", inventaire.to_string_liste_objet());
            println!("Que voulez-vous faire ?");
            println!("1. Déséquiper un objet");
            println!("2. Équiper un objet");
            println!("q. Quitter l'inventaire");

            print!("Votre choix: ");
            io::stdout().flush().unwrap();
            let mut choix = String::new();
            io::stdin().read_line(&mut choix).unwrap();

            match choix.trim() {
                "1" => {
                    // Vérifie s'il y a au moins un équipement
                    let objet_vide = Objet::objet_vide();
                    let equipements = [
                        &inventaire.tete,
                        &inventaire.torse,
                        &inventaire.jambes,
                        &inventaire.main1,
                        &inventaire.main2,
                    ];
                    if equipements.iter().all(|o| o.id == objet_vide.id) {
                        println!("Aucun équipement à déséquiper.");
                        return;
                    }
                    desequiper_objet(inventaire);
                },
                "2" => equiper_objet(inventaire),
                "q" => break,
                _ => println!("Choix invalide."),
            }
        }
    } else {
        println!("Aucun inventaire trouvé pour ce joueur.");
    }
}

fn desequiper_objet(inventaire: &mut Inventaire) {
    let mut slots = vec![];
    let objet_vide = Objet::objet_vide();
    if inventaire.tete.id != objet_vide.id {
        slots.push(("Tête", "tete"));
    }
    if inventaire.torse.id != objet_vide.id {
        slots.push(("Torse", "torse"));
    }
    if inventaire.jambes.id != objet_vide.id {
        slots.push(("Jambes", "jambes"));
    }
    if inventaire.main1.id != objet_vide.id {
        slots.push(("Main principale", "main1"));
    }
    if inventaire.main2.id != objet_vide.id {
        slots.push(("Main secondaire", "main2"));
    }

    if slots.is_empty() {
        println!("Aucun équipement à déséquiper.");
        return;
    }

    println!("Quel équipement voulez-vous déséquiper ?");
    for (i, (nom, _)) in slots.iter().enumerate() {
        println!("{}. {}", i + 1, nom);
    }
    print!("Numéro de l'équipement (0 pour annuler): ");
    io::stdout().flush().unwrap();
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    if let Ok(idx) = choix.trim().parse::<usize>() {
        if idx == 0 {
            return;
        }
        if let Some((_, slot)) = slots.get(idx - 1) {
            match *slot {
                "tete" => {
                    inventaire.liste_objets.push(inventaire.tete.clone());
                    inventaire.tete = objet_vide;
                    println!("Équipement de la tête déséquipé.");
                }
                "torse" => {
                    inventaire.liste_objets.push(inventaire.torse.clone());
                    inventaire.torse = objet_vide;
                    println!("Équipement du torse déséquipé.");
                }
                "jambes" => {
                    inventaire.liste_objets.push(inventaire.jambes.clone());
                    inventaire.jambes = objet_vide;
                    println!("Équipement des jambes déséquipé.");
                }
                "main1" => {
                    inventaire.liste_objets.push(inventaire.main1.clone());
                    inventaire.main1 = objet_vide;
                    println!("Équipement de la main principale déséquipé.");
                }
                "main2" => {
                    inventaire.liste_objets.push(inventaire.main2.clone());
                    inventaire.main2 = objet_vide;
                    println!("Équipement de la main secondaire déséquipé.");
                }
                _ => println!("Slot inconnu."),
            }
        } else {
            println!("Choix invalide.");
        }
    } else {
        println!("Choix invalide.");
    }
}

fn equiper_objet(inventaire: &mut Inventaire) {
    if inventaire.liste_objets.is_empty() {
        println!("Aucun objet à équiper.");
        return;
    }
    println!("Quel objet voulez-vous équiper ?");
    for (i, obj) in inventaire.liste_objets.iter().enumerate() {
        println!("{}. {} (type: {})", i + 1, obj.nom, obj.type_objet);
    }
    print!("Numéro de l'objet (0 pour annuler): ");
    io::stdout().flush().unwrap();
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    if let Ok(idx) = choix.trim().parse::<usize>() {
        if idx == 0 {
            println!("Équipement annulé.");
            return;
        } else if idx <= inventaire.liste_objets.len() {
            let obj = inventaire.liste_objets[idx - 1].clone();
            if equiper_selon_type(inventaire, obj) {
                inventaire.liste_objets.remove(idx - 1);
            }
        } else {
            println!("Numéro invalide.");
        }
    } else {
        println!("Entrée invalide.");
    }
}

fn equiper_selon_type(inventaire: &mut Inventaire, objet: Objet) -> bool {
    match objet.type_objet.as_str() {
        "tete" => {
            std::mem::swap(&mut inventaire.tete, &mut objet.clone());
            println!("{} équipé sur la tête.", objet.nom);
            true
        }
        "jambes" => {
            std::mem::swap(&mut inventaire.jambes, &mut objet.clone());
            println!("{} équipé sur les jambes.", objet.nom);
            true
        }
        "torse" => {
            std::mem::swap(&mut inventaire.torse, &mut objet.clone());
            println!("{} équipé sur le torse.", objet.nom);
            true
        }
        "arme" => {
            if inventaire.main1.id == Objet::objet_vide().id {
                std::mem::swap(&mut inventaire.main1, &mut objet.clone());
                println!("{} équipé en main principale.", objet.nom);
                true
            } else if inventaire.main2.id == Objet::objet_vide().id {
                std::mem::swap(&mut inventaire.main2, &mut objet.clone());
                println!("{} équipé en main secondaire.", objet.nom);
                true
            } else {
                println!("Les deux mains sont déjà occupées.");
                false
            }
        }
        _ => {
            println!("Impossible d'équiper cet objet à cet emplacement.");
            false
        }
    }
}
use crate::entities::shadow::Shadow;
use crate::outscale::database_manager::DatabaseManager;
use crate::entities::player::Player;
use std::io;
use rand::prelude::IndexedRandom;

pub struct ShadowManager;

impl ShadowManager {
 pub fn show_available_shadows(db_manager: &DatabaseManager) -> Vec<Shadow> {
     let conn = &db_manager.conn;
     // Récupérer les ombres disponibles (enemy = false et used = false)
     let query = "SELECT id, nom, max_hp, hp, max_mana, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, level, xp, classe_id FROM entity WHERE enemy = false AND used = false";
     let mut stmt = conn.prepare(query).expect("Erreur lors de la préparation de la requête");

     let shadows_iter = stmt.query_map([], |row| {
         Ok(Shadow {
             entity: crate::entities::shadow::Entity::new(
                 row.get(0)?,
                 row.get(1)?,
                 row.get(2)?,
                 row.get(3)?,
                 row.get(4)?,
                 row.get(5)?,
                 row.get(6)?,
                 row.get(7)?,
                 row.get(8)?,
                 row.get(9)?,
                 row.get(10)?,
                 row.get::<_, f32>(11)?,
                 vec![],
                 row.get(12)?,
                 row.get(13)?,
                 row.get(14)?,
                 None
             ),
         })
     }).expect("Erreur lors de l'exécution de la requête");

     let mut shadows = Vec::new();
     for shadow in shadows_iter {
         shadows.push(shadow.expect("Erreur lors de la récupération d'une ombre"));
     }

     shadows
 }

 pub fn select_shadow(db_manager: &DatabaseManager, player: &mut Player) {
     println!("=== Gestion des Ombres ===");
     println!("Ombres actuellement dans votre équipe :");
 
     // Si le joueur a plus de 2 ombres, on en enlève une au hasard
     if player.ombres.len() > 2 {
         let mut rng = rand::rng();
         let idx = (0..player.ombres.len()).collect::<Vec<_>>().choose(&mut rng).cloned().unwrap();
         let removed_shadow = player.ombres.remove(idx);
         // On libère l'ombre dans la base de données
         let update_query = "UPDATE entity SET used = false WHERE id = ?1";
         db_manager.conn.execute(update_query, [removed_shadow.entity.id])
             .expect("Erreur lors de la mise à jour du statut de l'ombre retirée");
         println!("Une ombre a été retirée au hasard de votre équipe car votre équipe était complète : {}", removed_shadow.entity.name);
     }
 
     let mut used_shadows = Vec::new();
     for (i, shadow) in player.ombres.iter().enumerate() {
         println!("{}: {} (Nv {}) - PV: {}/{}",
             i+1, shadow.entity.name, shadow.entity.level,
             shadow.entity.hp, shadow.entity.max_hp);
         used_shadows.push(shadow.clone());
     }
 
     println!("\nOmbres disponibles :");
     let available_shadows = Self::show_available_shadows(db_manager);
 
     if available_shadows.is_empty() {
         println!("Aucune ombre disponible pour le moment.");
         return;
     }
 
     for (i, shadow) in available_shadows.iter().enumerate() {
         let classe_name = Self::get_class_name(db_manager, shadow.entity.classe_id);
         println!("{}: {} (Nv {}) - PV: {}/{} - Classe: {}",
             i+1, shadow.entity.name, shadow.entity.level,
             shadow.entity.hp, shadow.entity.max_hp, classe_name);
     }
 
     println!("\nQue souhaitez-vous faire ?");
     println!("1. Ajouter une ombre à votre équipe");
     println!("2. Remplacer une ombre de votre équipe");
     println!("q. Retour au menu principal");
 
     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Impossible de lire l'entrée");
     let input = input.trim();
 
     match input {
         "1" => Self::add_shadow(db_manager, player, &available_shadows),
         "2" => Self::replace_shadow(db_manager, player, &available_shadows),
         "q" => return,
         _ => println!("Choix invalide.")
     }
 }

 fn get_class_name(db_manager: &DatabaseManager, classe_id: i32) -> String {
     let query = "SELECT nom FROM classe WHERE id = ?1";
     match db_manager.conn.query_row(query, [classe_id], |row| row.get::<_, String>(0)) {
         Ok(name) => name,
         Err(_) => "Inconnue".to_string()
     }
 }

 fn add_shadow(db_manager: &DatabaseManager, player: &mut Player, available_shadows: &[Shadow]) {
     if player.ombres.len() >= 2 {
         println!("Vous ne pouvez pas avoir plus de 2 ombres dans votre équipe.");
         println!("Vous devez d'abord remplacer une ombre existante.");
         return;
     }

     println!("\nQuelle ombre souhaitez-vous ajouter ? (numéro ou q pour annuler)");

     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Impossible de lire l'entrée");
     let input = input.trim();

     if input == "q" {
         return;
     }

     if let Ok(index) = input.parse::<usize>() {
         if index > 0 && index <= available_shadows.len() {
             let new_shadow = &available_shadows[index - 1];

             // Marquer l'ombre comme utilisée
             let update_query = "UPDATE entity SET used = true WHERE id = ?1";
             db_manager.conn.execute(update_query, [new_shadow.entity.id])
                 .expect("Erreur lors de la mise à jour du statut de l'ombre");

             // Ajouter l'ombre à l'équipe du joueur
             player.ombres.push(new_shadow.clone());

             println!("{} a été ajoutée à votre équipe !", new_shadow.entity.name);
         } else {
             println!("Numéro d'ombre invalide.");
         }
     } else {
         println!("Entrée invalide.");
     }
 }

 fn replace_shadow(db_manager: &DatabaseManager, player: &mut Player, available_shadows: &[Shadow]) {
     if player.ombres.is_empty() {
         println!("Vous n'avez pas d'ombres dans votre équipe à remplacer.");
         return;
     }

     println!("\nQuelle ombre souhaitez-vous remplacer ? (numéro ou q pour annuler)");

     for (i, shadow) in player.ombres.iter().enumerate() {
         println!("{}: {}", i+1, shadow.entity.name);
     }

     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Impossible de lire l'entrée");
     let input_replace = input.trim();

     if input_replace == "q" {
         return;
     }

     if let Ok(index_replace) = input_replace.parse::<usize>() {
         if index_replace > 0 && index_replace <= player.ombres.len() {
             println!("\nPar quelle ombre disponible souhaitez-vous la remplacer ? (numéro ou q pour annuler)");

             for (i, shadow) in available_shadows.iter().enumerate() {
                 println!("{}: {}", i+1, shadow.entity.name);
             }

             let mut input = String::new();
             io::stdin().read_line(&mut input).expect("Impossible de lire l'entrée");
             let input_new = input.trim();

             if input_new == "q" {
                 return;
             }

             if let Ok(index_new) = input_new.parse::<usize>() {
                 if index_new > 0 && index_new <= available_shadows.len() {
                     // Stocker les noms pour l'affichage ultérieur
                     let removed_name = player.ombres[index_replace - 1].entity.name.clone();
                     let removed_id = player.ombres[index_replace - 1].entity.id;
                     let new_shadow = available_shadows[index_new - 1].clone();
                     let new_name = new_shadow.entity.name.clone();

                     // Libérer l'ancienne ombre
                     let update_query = "UPDATE entity SET used = false WHERE id = ?1";
                     db_manager.conn.execute(update_query, [removed_id])
                         .expect("Erreur lors de la mise à jour du statut de l'ancienne ombre");

                     // Marquer la nouvelle ombre comme utilisée
                     let update_query = "UPDATE entity SET used = true WHERE id = ?1";
                     db_manager.conn.execute(update_query, [new_shadow.entity.id])
                         .expect("Erreur lors de la mise à jour du statut de la nouvelle ombre");

                     // Remplacer l'ombre dans l'équipe du joueur
                     player.ombres[index_replace - 1] = new_shadow;

                     println!("{} a été remplacé par {} !", removed_name, new_name);
                 } else {
                     println!("Numéro d'ombre invalide.");
                 }
             } else {
                 println!("Entrée invalide.");
             }
         } else {
             println!("Numéro d'ombre invalide.");
         }
     } else {
         println!("Entrée invalide.");
     }
 }
}
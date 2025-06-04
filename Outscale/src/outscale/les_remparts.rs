use crate::entities::enemy::Enemy;
use crate::entities::entity::{Entity, HasEntity};
use crate::outscale::combat_manager::CombatManager;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::prelude::IndexedRandom;
use crate::skills::skill::Skill;

pub fn lancer_les_remparts(player: &mut crate::entities::player::Player) {
    println!("Bienvenue dans les Remparts ! Préparez-vous à affronter des vagues d'ennemis.");

    // Charger les données des fichiers
    let prefixes = lire_lignes_depuis_fichier("src/resources/prefixes_suffixes.txt", "# Préfixes");
    let suffixes = lire_lignes_depuis_fichier("src/resources/prefixes_suffixes.txt", "# Suffixes");
    let noms_complets = lire_lignes_depuis_fichier("src/resources/noms_et_titres.txt", "# Noms complets");
    let titres = lire_lignes_depuis_fichier("src/resources/noms_et_titres.txt", "# Titres");

    loop {
        // Calcul de la moyenne des niveaux
        let total_level: i32 = player.ombres.iter().map(|ombre| ombre.entity.level).sum::<i32>() + player.entity.level;
        let average_level = total_level / (player.ombres.len() as i32 + 1);

        // Génération des ennemis
        let mut rng = rand::rng();
        let mut enemies: Vec<Box<dyn HasEntity>> = Vec::new();
        for i in 0..rng.random_range(1..=3) {
            let enemy_level = average_level + rng.random_range(-1..=1); // Niveau légèrement ajusté
            let enemy_name = generer_nom_aleatoire(&prefixes, &suffixes, &noms_complets, &titres, &mut rng);
            let enemy = Enemy::new(Entity::new(
                i + 1,
                enemy_name,
                50 + enemy_level * 10,
                50 + enemy_level * 10,
                30 + enemy_level * 5,
                30 + enemy_level * 5,
                5 + enemy_level,
                5 + enemy_level,
                10 + enemy_level * 2,
                5 + enemy_level,
                10 + enemy_level,
                5.0,
                vec![],
                enemy_level,
                0,
                None,
            ));
            enemies.push(Box::new(enemy));
        }

        // Initialisation du combat
        let mut allies: Vec<Box<dyn HasEntity>> = player
            .ombres
            .iter()
            .map(|ombre| Box::new(ombre.clone()) as Box<dyn HasEntity>)
            .collect();
        allies.push(Box::new(player.clone()));

        let mut combat_manager = CombatManager::new(allies, enemies);
        combat_manager.start_combat_loop();

        // Vérification de la défaite
        if player.entity.hp <= 0 {
            println!("Vous avez été vaincu. Fin des Remparts.");
            break;
        }

        // Demander au joueur s'il veut continuer
        println!("Souhaitez-vous continuer à combattre ? (o/n)");
        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).unwrap();
        if choix.trim().eq_ignore_ascii_case("n") {
            println!("Vous quittez les Remparts. Bravo pour votre performance !");
            break;
        }
    }
}

fn generer_nom_aleatoire(
    prefixes: &[String],
    suffixes: &[String],
    noms_complets: &[String],
    titres: &[String],
    rng: &mut rand::rngs::ThreadRng,
) -> String {
    let nom = if rng.random_bool(0.7) {
        // Combinaison préfixe + suffixe
        let prefix = prefixes.choose(rng).map(|s| s.as_str()).unwrap_or("Inconnu");
        let suffix = suffixes.choose(rng).map(|s| s.as_str()).unwrap_or("Inconnu");
        format!("{}{}", prefix, suffix)
    } else {
        // Choisir un nom complet existant
        noms_complets.choose(rng).unwrap_or(&"Inconnu".to_string()).to_string()
    };

    let binding = "Sans titre".to_string();
    let titre = titres.choose(rng).unwrap_or(&binding);
    format!("{} {}", nom, titre)
}

fn lire_lignes_depuis_fichier(chemin: &str, section: &str) -> Vec<String> {
    if let Ok(file) = File::open(chemin) {
        let reader = io::BufReader::new(file);
        let mut lignes = Vec::new();
        let mut dans_section = false;

        for ligne in reader.lines() {
            if let Ok(ligne) = ligne {
                if ligne.trim() == section {
                    dans_section = true;
                    continue;
                }
                if ligne.starts_with('#') && dans_section {
                    break;
                }
                if dans_section && !ligne.trim().is_empty() {
                    lignes.push(ligne.trim().to_string());
                }
            }
        }
        lignes
    } else {
        vec![]
    }
}

fn assigner_skills_aux_ennemis(enemies: &mut Vec<Box<dyn HasEntity>>) {
    // Charger les compétences depuis le fichier JSON
    let skills = charger_skills_depuis_fichier("src/resources/skills.json");

    if skills.is_empty() {
        println!("Aucune compétence trouvée dans le fichier JSON.");
        return;
    }

    let mut rng = rand::rng();

    for enemy in enemies.iter_mut() {
        // Nombre aléatoire de compétences à attribuer (entre 1 et 10)
        let nombre_de_skills = rng.random_range(1..=10);

        // Sélection aléatoire des compétences
        let mut skills_aleatoires: Vec<Skill> = skills
            .iter()
            .cloned()
            .collect::<Vec<Skill>>()
            .choose_multiple(&mut rng, nombre_de_skills.min(skills.len()))
            .cloned()
            .collect();

        // Mettre l'id des compétences à 0 et assigner l'id de l'ennemi pour la sauvegarde si jamais l'ennemi devient une ombre
        for skill in &mut skills_aleatoires {
            skill.id = 0;
            skill.entity_id= enemy.entity().id;
        }

        // Ajouter les compétences à l'ennemi
        enemy.entity_mut().skills = skills_aleatoires;
    }
}

fn charger_skills_depuis_fichier(chemin: &str) -> Vec<Skill> {
    let file = File::open(chemin).expect("Impossible d'ouvrir le fichier skills.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Erreur lors du parsing du fichier JSON")
}
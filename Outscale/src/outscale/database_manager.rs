use rusqlite::{Connection, Result};
use std::{fs, io};
use std::path::Path;
use crate::entities::player::Player;
use crate::entities::shadow::Shadow;
use crate::skills::inventaire::Inventaire;
use crate::skills::object::Objet;
use crate::skills::skill::Skill;

#[allow(dead_code)]
pub struct DatabaseManager {
    pub(crate) conn: Connection,
}
impl DatabaseManager {

    pub fn execute_sql_file(&self, sql_file_path: &str) -> Result<()> {
        // Lire le contenu du fichier SQL
        let sql_content = fs::read_to_string(sql_file_path)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        // Exécuter le contenu du fichier SQL
        self.conn.execute_batch(&sql_content)?;
        Ok(())
    }

    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn file_exists(db_path: &str) -> bool {
        Path::new(db_path).exists()
    }

    pub fn has_player_data(&self) -> Result<bool> {
        let query = "SELECT COUNT(*) FROM player";
        let count: i64 = self.conn.query_row(query, [], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn insert_player(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut nom = String::new();
        println!("Bonjour Éclatax quel est votre nom ?:");
        io::stdin().read_line(&mut nom)?;
        let nom = nom.trim();
        self.conn.execute(
            "INSERT INTO player (nom,hp,mana,magic_resist,armor,attack_damage,magic_damage,speed,dodge_chance,level,xp)\
             VALUES (?1, 30, 30, 5, 5, 10, 10, 5, 10.0,1,0)",
            &[nom],
        )?;
        Ok(())
    }

    pub fn get_player_inventory_id(&self) -> Result<i32> {
        let query = "SELECT inventaire_id FROM player";
        let id: i32 = self.conn.query_row(query, [], |row| row.get(0))?;
        Ok(id)
    }

    pub fn get_objet_by_id(&self, id: i32) -> Result<Objet> {
        let query = "SELECT id, inventaire_id, nom, degats, degats_magiques, armure, magic_resist, mana, taux_critique, vitesse, hp, type_objet
                     FROM objet WHERE id = ?1";
        self.conn.query_row(query, [id], |row| {
            Ok(Objet {
                id: row.get(0)?,
                inventaire_id: row.get(1)?,
                nom: row.get(2)?,
                degats: row.get(3)?,
                degats_magiques: row.get(4)?,
                armure: row.get(5)?,
                magic_resist: row.get(6)?,
                mana: row.get(7)?,
                taux_critique: row.get(8)?,
                vitesse: row.get(9)?,
                hp: row.get(10)?,
                type_objet: row.get(11)?,
            })
        })
    }
    pub fn get_player_inventory(&self) -> Option<Inventaire> {
        let id_inventaire = self.get_player_inventory_id().ok()?;

        let query = "SELECT equipement_tete, equipement_torse, equipement_jambe, main1, main2
                 FROM inventaire WHERE id = ?1";
        let (id_tete, id_torse, id_jambe, id_main1, id_main2): (i32, i32, i32, i32, i32) =
            self.conn.query_row(query, [id_inventaire], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            }).ok()?;

        let tete = self.get_objet_by_id(id_tete).ok()?;
        let torse = self.get_objet_by_id(id_torse).ok()?;
        let jambes = self.get_objet_by_id(id_jambe).ok()?;
        let main1 = self.get_objet_by_id(id_main1).ok()?;
        let main2 = self.get_objet_by_id(id_main2).ok()?;

        let mut stmt = self.conn.prepare(
            "SELECT id, inventaire_id, nom, degats, degats_magiques, armure, magic_resist, mana, taux_critique, vitesse, hp, type_objet
         FROM objet
         WHERE inventaire_id = ?1
         AND id NOT IN (?2, ?3, ?4, ?5, ?6)",
        ).ok()?;
        let objets_iter = stmt.query_map(
            [id_inventaire, id_tete, id_torse, id_jambe, id_main1, id_main2],
            |row| {
                Ok(Objet {
                    id: row.get(0)?,
                    inventaire_id: row.get(1)?,
                    nom: row.get(2)?,
                    degats: row.get(3)?,
                    degats_magiques: row.get(4)?,
                    armure: row.get(5)?,
                    magic_resist: row.get(6)?,
                    mana: row.get(7)?,
                    taux_critique: row.get(8)?,
                    vitesse: row.get(9)?,
                    hp: row.get(10)?,
                    type_objet: row.get(11)?,
                })
            },
        ).ok()?;

        let mut liste_objets = Vec::new();
        for objet in objets_iter {
            liste_objets.push(objet.ok()?);
        }

        Some(Inventaire {
            id: id_inventaire,
            tete,
            jambes,
            torse,
            main1,
            main2,
            liste_objets,
        })
    }

    pub fn get_inventaire_by_id_entity(&self, id_entity: i32) -> Result<Inventaire> {
        // Récupérer l'ID de l'inventaire associé à l'entité
        let query = "SELECT id, equipement_tete, equipement_torse, equipement_jambe, main1, main2
                 FROM inventaire WHERE entity_id = ?1";
        let (id_inventaire, id_tete, id_torse, id_jambe, id_main1, id_main2): (i32, i32, i32, i32, i32, i32) =
            self.conn.query_row(query, [id_entity], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })?;

        // Récupérer les objets spécifiques
        let tete = self.get_objet_by_id(id_tete)?;
        let torse = self.get_objet_by_id(id_torse)?;
        let jambes = self.get_objet_by_id(id_jambe)?;
        let main1 = self.get_objet_by_id(id_main1)?;
        let main2 = self.get_objet_by_id(id_main2)?;

        // Récupérer tous les objets associés à l'inventaire_id, sauf ceux déjà instanciés
        let mut stmt = self.conn.prepare(
            "SELECT id, inventaire_id, nom, degats, degats_magiques, armure, magic_resist, mana, taux_critique, vitesse, hp, type_objet
         FROM objet
         WHERE inventaire_id = ?1
         AND id NOT IN (?2, ?3, ?4, ?5, ?6)",
        )?;
        let objets_iter = stmt.query_map(
            [id_inventaire, id_tete, id_torse, id_jambe, id_main1, id_main2],
            |row| {
                Ok(Objet {
                    id: row.get(0)?,
                    inventaire_id: row.get(1)?,
                    nom: row.get(2)?,
                    degats: row.get(3)?,
                    degats_magiques: row.get(4)?,
                    armure: row.get(5)?,
                    magic_resist: row.get(6)?,
                    mana: row.get(7)?,
                    taux_critique: row.get(8)?,
                    vitesse: row.get(9)?,
                    hp: row.get(10)?,
                    type_objet: row.get(11)?,
                })
            },
        )?;

        let mut liste_objets = Vec::new();
        for objet in objets_iter {
            liste_objets.push(objet?);
        }

        // Créer l'inventaire
        Ok(Inventaire {
            id: id_inventaire,
            tete,
            jambes,
            torse,
            main1,
            main2,
            liste_objets,
        })
    }


    pub fn get_visited_zones(&self) -> String {
        let query = "SELECT nom FROM zones where visited = 1";
        let mut stmt = self.conn.prepare(query).expect("Erreur lors de la préparation de la requête");
        let zones_iter = stmt.query_map([], |row| {
            row.get::<_, String>(0)
        }).expect("Erreur lors de l'exécution de la requête");
        let mut zones = String::new();
        for zone in zones_iter {
            match zone {
                Ok(nom) => {
                    if !zones.is_empty() {
                        zones.push_str(", ");
                    }
                    zones.push_str(&nom);
                },
                Err(e) => eprintln!("Erreur lors de la récupération de la zone : {}", e),
            }
        }
        zones

    }

    pub fn visite_lieu(&self, nom: &str) {
        // Vérifier si la zone est déjà visitée
        let query_check = "SELECT visited FROM zones WHERE nom = ?1";
        let visited: Option<i32> = self.conn.query_row(query_check, [nom], |row| row.get(0)).ok();

        match visited {
            Some(1) => {
                println!("Vous avez déjà visité cette zone : {}", nom);
            },
            Some(0) => {
                println!("C'est votre première visite dans la zone : {}", nom);
                let query_update = "UPDATE zones SET visited = 1 WHERE nom = ?1";
                self.conn.execute(query_update, [nom]).expect("Erreur lors de la mise à jour de la zone");
            },
            None => {
                println!("La zone spécifiée n'existe pas dans la base de données : {}", nom);
            },
            Some(i32::MIN..=-1_i32) | Some(2_i32..=i32::MAX) => todo!()
        }
    }


    // Correction pour la méthode sauvegarde dans DatabaseManager
    pub fn sauvegarde(&self, player: Player) {
        Self::update_player(&self.conn, &player);

        let mut tout_les_objets: Vec<Objet> = vec![];
        if let Some(inventaire) = &player.entity.inventaire {
            for objet in &inventaire.liste_objets {
                tout_les_objets.push(objet.clone());
            }
        }

        for ombre in &player.ombres {
            Self::sauvegarde_ombre(&self.conn, ombre.clone());
            if let Some(inventaire) = &ombre.entity.inventaire {
                for objet in &inventaire.liste_objets {
                    tout_les_objets.push(objet.clone());
                }
            }
        }

        let mut objets_a_inserer : Vec<Objet> = vec![];
        let mut objets_a_modifier : Vec<Objet> = vec![];
        let mut objets_a_supprimer : Vec<Objet> = vec![];
        for objet in tout_les_objets {
            if objet.id == 0 {
                objets_a_inserer.push(objet);
            } else if objet.id == -1 {
                objets_a_supprimer.push(objet);
            } else {
                objets_a_modifier.push(objet);
            }
        }

        // Insertion des nouveaux objets
        for objet in objets_a_inserer {
            self.conn.execute(
                "INSERT INTO objet (inventaire_id, nom, degats, degats_magiques, armure, magic_resist, mana, taux_critique, vitesse, hp, type_objet)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                rusqlite::params![
                objet.inventaire_id,
                objet.nom,
                objet.degats,
                objet.degats_magiques,
                objet.armure,
                objet.magic_resist,
                objet.mana,
                objet.taux_critique as i32,
                objet.vitesse,
                objet.hp,
                objet.type_objet
            ],
            ).expect("Erreur lors de l'insertion d'un nouvel objet dans la base de données");
        }

        for objet in objets_a_modifier {
            Self::sauvegarde_modification_objet(&self.conn, objet);
        }

        for objet in objets_a_supprimer {
            Self::delete_objet(&self.conn, objet.id);
        }

        Self::sauvegarde_skills(&self.conn, &player);
    }

    // Correction pour la méthode sauvegarde_modification_objet
    fn sauvegarde_modification_objet(conn: &Connection, objet: Objet) {
        conn.execute(
            "UPDATE objet SET inventaire_id = ?1, nom = ?2, degats = ?3, degats_magiques = ?4, armure = ?5, magic_resist = ?6, mana = ?7, taux_critique = ?8, vitesse = ?9, hp = ?10, type_objet = ?11 WHERE id = ?12",
            rusqlite::params![
            objet.inventaire_id,
            objet.nom,
            objet.degats,
            objet.degats_magiques,
            objet.armure,
            objet.magic_resist,
            objet.mana,
            objet.taux_critique as i32,
            objet.vitesse,
            objet.hp,
            objet.type_objet,
            objet.id
        ],
        ).expect("Erreur lors de la sauvegarde de l'objet dans la base de données");
    }

    // Correction pour la méthode update_player
    fn update_player(conn: &Connection, player: &Player) {
        conn.execute(
            "UPDATE player SET hp = ?1, mana = ?2, magic_resist = ?3, armor = ?4, attack_damage = ?5, magic_damage = ?6, speed = ?7, dodge_chance = ?8, level = ?9, xp = ?10 WHERE nom = ?11",
            rusqlite::params![
            player.entity.hp,
            player.entity.mana,
            player.entity.magic_resist,
            player.entity.armor,
            player.entity.attack_dmg,
            player.entity.magic_dmg,
            player.entity.speed,
            player.entity.dodge_chance as i32,
            player.entity.level,
            player.entity.xp,
            player.entity.name
        ],
        ).expect("Erreur lors de la mise à jour du joueur dans la base de données");

        if let Some(inventaire) = &player.entity.inventaire {
            conn.execute(
                "UPDATE inventaire SET equipement_tete = ?1, equipement_torse = ?2, equipement_jambe = ?3, main1 = ?4, main2 = ?5 WHERE id = ?6",
                rusqlite::params![
                inventaire.tete.id,
                inventaire.torse.id,
                inventaire.jambes.id,
                inventaire.main1.id,
                inventaire.main2.id,
                inventaire.id
            ],
            ).expect("Erreur lors de la sauvegarde de l'inventaire du joueur dans la base de données");
        }
    }

    fn delete_objet(conn: &Connection, id_objet: i32) {
        conn.execute(
            "DELETE FROM objet WHERE id = ?1",
            rusqlite::params![id_objet],
        ).expect("Erreur lors de la suppression de l'objet dans la base de données");
    }

    fn sauvegarde_ombre(conn: &Connection, ombre: Shadow) {
        conn.execute(
            "UPDATE entity SET hp = ?1, mana = ?2, magic_resist = ?3, armor = ?4, attack_damage = ?5, magic_damage = ?6, speed = ?7, dodge_chance = ?8, level = ?9, xp = ?10 WHERE id = ?11",
            rusqlite::params![
                ombre.entity.hp,
                ombre.entity.mana,
                ombre.entity.magic_resist,
                ombre.entity.armor,
                ombre.entity.attack_dmg,
                ombre.entity.magic_dmg,
                ombre.entity.speed,
                ombre.entity.dodge_chance as i32,
                ombre.entity.level,
                ombre.entity.xp,
                ombre.entity.id
            ],
        ).expect("Erreur lors de la mise à jour de l'ombre dans la base de données");

        if let Some(inventaire) = &ombre.entity.inventaire {
            conn.execute(
                "UPDATE inventaire SET equipement_tete = ?1, equipement_torse = ?2, equipement_jambe = ?3, main1 = ?4, main2 = ?5 WHERE id = ?6",
                rusqlite::params![
                    inventaire.tete.id,
                    inventaire.torse.id,
                    inventaire.jambes.id,
                    inventaire.main1.id,
                    inventaire.main2.id,
                    inventaire.id
                ],
            ).expect("Erreur lors de la sauvegarde de l'inventaire de l'ombre dans la base de données");
        }
    }

    pub fn get_player_data(&self) -> Player {
        // Récupérer les données du joueur
        let query = "SELECT id,nom, hp, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, level, xp FROM player";
        let mut player: Player = self.conn.query_row(query, [], |row| {
            Ok(Player {
                entity: crate::entities::shadow::Entity::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get::<_, f32>(9)?,
                    vec![],
                    row.get(10)?,
                    row.get(11)?,
                    None
                ),
                ombres: vec![],
            })
        }).expect("Erreur lors de la récuperation des données du joueur");

        player.entity.inventaire = Self::get_player_inventory(&self);
        player.ombres = Self::get_shadows(&self.conn);
        player.entity.skills = Self::get_skills_by_entity_id(&self.conn, -1);
        for ombre in player.ombres.iter_mut() {
            ombre.entity.inventaire = Self::get_inventaire_by_id_entity(&self, ombre.entity.id).ok();
            println!("ombre: {:?}", ombre.entity.id);
            ombre.entity.skills = Self::get_skills_by_entity_id(&self.conn, ombre.entity.id);
        }
        return player;
    }

    fn get_shadows(conn: &Connection) -> Vec<Shadow> {
        let query = "SELECT id,nom, hp, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, level, xp FROM entity WHERE enemy = false";
        let mut stmt = conn.prepare(query).expect("Erreur lors de la préparation de la requête");
        let shadows_iter = stmt.query_map([], |row| {
            Ok(Shadow {
                entity: crate::entities::shadow::Entity::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get::<_, f32>(9)?,
                    vec![],
                    row.get(10)?,
                    row.get(11)?,
                    None
                ),
            })
        }).expect("Erreur lors de l'exécution de la requête");

        let mut shadows = Vec::new();
        for shadow in shadows_iter {
            println!("INSERTION DE LA SHADOW d'ID {}", shadow.as_ref().unwrap().entity.id);
            shadows.push(shadow.expect("Erreur lors de la récupération d'une ombre"));
        }
        shadows
    }

    fn sauvegarde_skills(conn: &Connection, player: &Player) {
        let mut tout_les_skills: Vec<crate::skills::skill::Skill> = vec![];
        for skill in &player.entity.skills {
            tout_les_skills.push(skill.clone());
        }
        for ombre in &player.ombres {
            for skill in &ombre.entity.skills {
                tout_les_skills.push(skill.clone());
            }
        }
        let mut skill_a_inserer: Vec<crate::skills::skill::Skill> = vec![]; // Les skills a insérer on un id de 0
        for skill in tout_les_skills {
            if skill.id == 0 {
                skill_a_inserer.push(skill);
            } 
        }
        for skill in skill_a_inserer {
            Self::inserer_skills(&conn, skill);
        }
    }

    fn inserer_skills(conn: &Connection, skill: Skill) {
        conn.execute(
            "INSERT INTO skills (name,description, hp_refound, mana_cost, mana_refound, magic_resist_debuff, magic_resist_buff, armor_debuff, armor_buff, attack_dmg, attack_dmg_buff, magic_dmg, magic_dmg_buff, entity_id) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![
            skill.name,
            skill.description,
            skill.hp_refound,
            skill.mana_cost,
            skill.mana_refound,
            skill.magic_resist_debuff,
            skill.magic_resist_buff,
            skill.armor_debuff,
            skill.armor_buff,
            skill.attack_dmg,
            skill.attack_dmg_buff,
            skill.magic_dmg,
            skill.magic_dmg_buff,
            skill.entity_id,
        ],
        ).expect("Erreur lors de l'insertion du skill dans la base de données");
    }

    fn get_skills_by_entity_id(conn: &Connection, entity_id: i32) -> Vec<Skill> {
        let query = "SELECT * FROM skills WHERE entity_id = ?1";
        let mut stmt = conn.prepare(query).expect("Erreur lors de la préparation de la requête");
        let skills_iter = stmt.query_map([entity_id], |row| {
            Ok(Skill {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                hp_refound: row.get(3)?,
                mana_cost: row.get(4)?,
                mana_refound: row.get(5)?,
                magic_resist_debuff: row.get(6)?,
                magic_resist_buff: row.get(7)?,
                armor_debuff: row.get(8)?,
                armor_buff: row.get(9)?,
                attack_dmg: row.get(10)?,
                attack_dmg_buff: row.get(11)?,
                magic_dmg: row.get(12)?,
                magic_dmg_buff: row.get(13)?,
                for_allies: row.get(14)?,
                entity_id: row.get(15)?
            })
        }).expect("Erreur lors de l'exécution de la requête");
        let mut skills: Vec<Skill> = Vec::new();
        for skill in skills_iter {
            skills.push(skill.expect("Erreur lors de la récupération d'une compétence"));
        }
        skills
    }
    
}
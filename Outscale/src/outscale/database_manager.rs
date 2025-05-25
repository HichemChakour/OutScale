use rusqlite::{Connection, Result};
use std::{fs, io};
use std::path::Path;
use crate::skills::inventaire::Inventaire;
use crate::skills::object::Objet;
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
    }//
    //

    pub fn insert_player(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut nom = String::new();
        println!("Entrez votre nom :");
        io::stdin().read_line(&mut nom)?;
        let nom = nom.trim();
        self.conn.execute(
            "INSERT INTO player (nom) VALUES (?1)",
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
    pub fn get_player_inventory(&self) -> Result<Inventaire> {
        let id_inventaire = self.get_player_inventory_id()?;

        // Récupérer les IDs des objets spécifiques pour les emplacements
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

    pub fn get_inventaire_by_id_entity(&self, id_entity: i32) -> Result<Inventaire> {
        // Récupérer l'ID de l'inventaire associé à l'entité
        let query = "SELECT id, equipement_tete, equipement_torse, equipement_jambe, main1, main2
                 FROM inventaire WHERE entite_id = ?1";
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


}
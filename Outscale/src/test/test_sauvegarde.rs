use rusqlite::Connection;
use crate::entities::entity::Entity;
use crate::entities::player::Player;
use crate::skills::inventaire::Inventaire;
use crate::skills::object::Objet;
use crate::outscale::database_manager::DatabaseManager;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> DatabaseManager {
        let conn = Connection::open_in_memory().unwrap();

        // Création des tables nécessaires pour le test
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS player (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                nom TEXT DEFAULT NULL,
                hp INTEGER DEFAULT NULL,
                mana INTEGER DEFAULT NULL,
                magic_resist INTEGER DEFAULT NULL,
                armor INTEGER DEFAULT NULL,
                attack_damage INTEGER DEFAULT NULL,
                magic_damage INTEGER DEFAULT NULL,
                speed INTEGER DEFAULT NULL,
                dodge_chance INTEGER DEFAULT NULL,
                level INTEGER DEFAULT 1,
                xp INTEGER DEFAULT 0,
                inventaire_id INTEGER DEFAULT NULL
            );

            CREATE TABLE IF NOT EXISTS shadow (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                nom TEXT DEFAULT NULL,
                hp INTEGER DEFAULT NULL,
                mana INTEGER DEFAULT NULL,
                magic_resist INTEGER DEFAULT NULL,
                armor INTEGER DEFAULT NULL,
                attack_damage INTEGER DEFAULT NULL,
                magic_damage INTEGER DEFAULT NULL,
                speed INTEGER DEFAULT NULL,
                dodge_chance INTEGER DEFAULT NULL,
                level INTEGER DEFAULT 1,
                xp INTEGER DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS inventaire (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                equipement_tete INTEGER,
                equipement_torse INTEGER,
                equipement_jambe INTEGER,
                main1 INTEGER,
                main2 INTEGER
            );

            CREATE TABLE IF NOT EXISTS objet (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                inventaire_id INTEGER,
                nom TEXT DEFAULT NULL,
                degats INTEGER DEFAULT NULL,
                degats_magiques INTEGER DEFAULT NULL,
                armure INTEGER DEFAULT NULL,
                magic_resist INTEGER DEFAULT NULL,
                mana INTEGER DEFAULT NULL,
                taux_critique INTEGER DEFAULT NULL,
                vitesse INTEGER DEFAULT NULL,
                hp INTEGER DEFAULT NULL,
                type_objet TEXT DEFAULT NULL
            );
            ",
        ).unwrap();

        DatabaseManager { conn }
    }

    #[test]
    fn test_sauvegarde_player() {
        let db_manager = setup_test_db();

        // Création d'un inventaire fictif avec des objets
        let tete = Objet::new(0, 1, "Casque".to_string(), 0, 0, 10, 0, 0, 0, 0, 0, "equipement".to_string());
        let torse = Objet::new(0, 1, "Armure".to_string(), 0, 0, 20, 0, 0, 0, 0, 0, "equipement".to_string());
        let jambes = Objet::new(0, 1, "Jambières".to_string(), 0, 0, 15, 0, 0, 0, 0, 0, "equipement".to_string());
        let main1 = Objet::new(0, 1, "Épée".to_string(), 30, 0, 0, 0, 0, 0, 0, 0, "arme".to_string());
        let main2 = Objet::new(0, 1, "Bouclier".to_string(), 0, 0, 25, 0, 0, 0, 0, 0, "equipement".to_string());

        let liste_objets = vec![
            tete.clone(),
            torse.clone(),
            jambes.clone(),
            main1.clone(),
            main2.clone(),
        ];
        let inventaire = Inventaire {
            id: 1,
            tete,
            torse,
            jambes,
            main1,
            main2,
            liste_objets
        };

        // Création d'un joueur fictif
        let entity = Entity::new(
            1,
            "JoueurTest".to_string(),
            100,
            100,
            50,
            50,
            10,
            20,
            30,
            40,
            10,
            1.0,
            vec![],
            1,
            1,
            Some(inventaire),
        );

        let player = Player::new(entity, vec![]);

        // Insérer d'abord le joueur dans la base de données
        db_manager.conn.execute(
            "INSERT INTO player (nom, hp, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, level, xp, inventaire_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![
                player.entity.name,
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
                1
            ],
        ).unwrap();

        // Insérer l'inventaire
        db_manager.conn.execute(
            "INSERT INTO inventaire (id, equipement_tete, equipement_torse, equipement_jambe, main1, main2)
             VALUES (1, ?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                1, 2, 3, 4, 5
            ],
        ).unwrap();

        // Insérer les objets
        for (id, obj) in [
            (1, &player.entity.inventaire.as_ref().unwrap().tete),
            (2, &player.entity.inventaire.as_ref().unwrap().torse),
            (3, &player.entity.inventaire.as_ref().unwrap().jambes),
            (4, &player.entity.inventaire.as_ref().unwrap().main1),
            (5, &player.entity.inventaire.as_ref().unwrap().main2)
        ] {
            db_manager.conn.execute(
                "INSERT INTO objet (id, inventaire_id, nom, degats, degats_magiques, armure, magic_resist, mana, taux_critique, vitesse, hp, type_objet)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                rusqlite::params![
                    id,
                    1,
                    &obj.nom,
                    obj.degats,
                    obj.degats_magiques,
                    obj.armure,
                    obj.magic_resist,
                    obj.mana,
                    obj.taux_critique,
                    obj.vitesse,
                    obj.hp,
                    &obj.type_objet
                ],
            ).unwrap();
        }

        // Modifier les stats du joueur
        let mut modified_player = player.clone();
        modified_player.entity.hp = 80;
        modified_player.entity.mana = 40;
        modified_player.entity.level = 2;

        // Sauvegarde du joueur modifié
        db_manager.sauvegarde(modified_player.clone());

        // Vérification des données sauvegardées
        let query = "SELECT hp, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, level, xp FROM player WHERE nom = ?1";
        let (hp, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, level, xp): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) =
            db_manager.conn.query_row(query, [&modified_player.entity.name], |row| {
                Ok((
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
                ))
            }).unwrap();

        // Vérifions que les données ont bien été mises à jour
        assert_eq!(hp, modified_player.entity.hp);
        assert_eq!(mana, modified_player.entity.mana);
        assert_eq!(magic_resist, modified_player.entity.magic_resist);
        assert_eq!(armor, modified_player.entity.armor);
        assert_eq!(attack_damage, modified_player.entity.attack_dmg);
        assert_eq!(magic_damage, modified_player.entity.magic_dmg);
        assert_eq!(speed, modified_player.entity.speed);

        // Pour dodge_chance, comparer la valeur entière stockée avec la conversion de la valeur f32
        assert_eq!(dodge_chance, modified_player.entity.dodge_chance as i32);

        assert_eq!(level, modified_player.entity.level);
        assert_eq!(xp, modified_player.entity.xp);

        let equipements = [(1, "Casque"), (1, "Armure"), (1, "Jambières"), (1, "Épée"), (1, "Bouclier")];
        for objet in equipements {
            let objet_inv_id: i32 = db_manager.conn.query_row(
                "SELECT inventaire_id FROM objet WHERE nom = ?1",
                [objet.1],
                |row| row.get(0)
            ).unwrap();
            assert_eq!(objet_inv_id, objet.0);  // Comparer avec l'ID d'inventaire attendu (objet.0)
        }
    }
}
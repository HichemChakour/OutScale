#[cfg(test)]
mod tests {
    use crate::outscale::database_manager::DatabaseManager;
    use rusqlite::{Connection, Result};

    fn setup_test_db() -> Result<DatabaseManager> {
        // Crée une base de données SQLite en mémoire
        let conn = Connection::open_in_memory()?;

        // Crée les tables nécessaires
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS player (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                nom TEXT DEFAULT NULL,
                inventaire_id INTEGER DEFAULT NULL
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
                nom TEXT,
                degats INTEGER,
                degats_magiques INTEGER,
                armure INTEGER,
                magic_resist INTEGER,
                mana INTEGER,
                taux_critique INTEGER,
                vitesse INTEGER,
                hp INTEGER,
                type_objet TEXT
            );
            ",
        )?;

        Ok(DatabaseManager { conn })
    }

    #[test]
    fn test_get_player_inventory() {
        let db_manager = setup_test_db().unwrap();

        // Insère des données de test
        db_manager.conn.execute(
            "INSERT INTO player (id, nom, inventaire_id) VALUES (1, 'TestPlayer', 1);",
            [],
        ).unwrap();

        db_manager.conn.execute(
            "INSERT INTO inventaire (id, equipement_tete, equipement_torse, equipement_jambe, main1, main2)
             VALUES (1, 1, 2, 3, 4, 5);",
            [],
        ).unwrap();

        db_manager.conn.execute(
            "INSERT INTO objet (id, inventaire_id, nom, degats, degats_magiques, armure, magic_resist, mana, taux_critique, vitesse, hp, type_objet)
             VALUES
             (1, 1, 'Casque', 0, 0, 5, 0, 0, 0, 0, 10, 'Tete'),
             (2, 1, 'Plastron', 0, 0, 10, 0, 0, 0, 0, 20, 'Torse'),
             (3, 1, 'Jambières', 0, 0, 8, 0, 0, 0, 0, 15, 'Jambe'),
             (4, 1, 'Epée', 15, 0, 0, 0, 0, 5, 0, 0, 'Main'),
             (5, 1, 'Bouclier', 0, 0, 12, 5, 0, 0, 0, 0, 'Main'),
             (6, 1, 'Potion', 0, 0, 0, 0, 0, 0, 0, 0, 'Consommable');",
            [],
        ).unwrap();

        // Récupère l'inventaire du joueur
        let inventaire = db_manager.get_player_inventory().unwrap();

        // Vérifie les objets spécifiques
        assert_eq!(inventaire.tete.nom, "Casque");
        assert_eq!(inventaire.torse.nom, "Plastron");
        assert_eq!(inventaire.jambes.nom, "Jambières");
        assert_eq!(inventaire.main1.nom, "Epée");
        assert_eq!(inventaire.main2.nom, "Bouclier");

        // Vérifie la liste des objets
        assert_eq!(inventaire.liste_objets.len(), 1);
        assert_eq!(inventaire.liste_objets[0].nom, "Potion");

        println!("Inventaire récupéré : {:?}", inventaire);

    }
}

#[cfg(test)]
mod test_get_inventaire_entity {
    use super::*;
    use rusqlite::Connection;
    use crate::outscale::database_manager::DatabaseManager;

    fn setup_test_db() -> DatabaseManager {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS inventaire (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entite_id INTEGER,
                equipement_tete INTEGER,
                equipement_torse INTEGER,
                equipement_jambe INTEGER,
                main1 INTEGER,
                main2 INTEGER
            );

            CREATE TABLE IF NOT EXISTS objet (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                inventaire_id INTEGER,
                nom TEXT,
                degats INTEGER,
                degats_magiques INTEGER,
                armure INTEGER,
                magic_resist INTEGER,
                mana INTEGER,
                taux_critique INTEGER,
                vitesse INTEGER,
                hp INTEGER,
                type_objet TEXT
            );
            ",
        )
            .unwrap();

        DatabaseManager { conn }
    }

#[test]
fn test_get_inventaire_by_id_entity() {
    let db_manager = setup_test_db();

    // Insérer des données de test
    db_manager.conn.execute(
        "INSERT INTO inventaire (id, entite_id, equipement_tete, equipement_torse, equipement_jambe, main1, main2)
         VALUES (1, 1, 1, 2, 3, 4, 5);",
        [],
    ).unwrap();

    db_manager.conn.execute(
        "INSERT INTO objet (id, inventaire_id, nom, degats, degats_magiques, armure, magic_resist, mana, taux_critique, vitesse, hp, type_objet)
         VALUES
         (1, 1, 'Casque', 0, 0, 5, 0, 0, 0, 0, 10, 'Tete'),
         (2, 1, 'Plastron', 0, 0, 10, 0, 0, 0, 0, 20, 'Torse'),
         (3, 1, 'Jambières', 0, 0, 8, 0, 0, 0, 0, 15, 'Jambe'),
         (4, 1, 'Epée', 15, 0, 0, 0, 0, 5, 0, 0, 'Main'),
         (5, 1, 'Bouclier', 0, 0, 12, 5, 0, 0, 0, 0, 'Main'),
         (6, 1, 'Potion', 0, 0, 0, 0, 0, 0, 0, 0, 'Consommable');",
        [],
    ).unwrap();

    // Tester la récupération de l'inventaire
    let inventaire = db_manager.get_inventaire_by_id_entity(1).unwrap();

    // Vérifier les objets spécifiques
    assert_eq!(inventaire.tete.nom, "Casque");
    assert_eq!(inventaire.torse.nom, "Plastron");
    assert_eq!(inventaire.jambes.nom, "Jambières");
    assert_eq!(inventaire.main1.nom, "Epée");
    assert_eq!(inventaire.main2.nom, "Bouclier");

    // Vérifier la liste des objets
    assert_eq!(inventaire.liste_objets.len(), 1);
    assert_eq!(inventaire.liste_objets[0].nom, "Potion");

    println!("{:?}", inventaire.to_string());
    println!("{}", inventaire.to_string_liste_objet());
}
}
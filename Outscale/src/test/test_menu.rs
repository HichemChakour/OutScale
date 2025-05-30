#[cfg(test)]
mod tests {
    use crate::outscale::database_manager::DatabaseManager;
    use crate::outscale::cli_manager::menu_principal;
    use rusqlite::Connection;

    fn setup_test_db() -> DatabaseManager {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS zones_visitees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                nom TEXT,
                visited INTEGER
            );

            CREATE TABLE IF NOT EXISTS player (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                nom TEXT DEFAULT NULL,
                inventaire_id INTEGER DEFAULT NULL
            );
            ",
        )
            .unwrap();

        DatabaseManager { conn }
    }

    #[test]
    fn test_get_visited_zones() {
        let db_manager = setup_test_db();

        // Insérer des données de test
        db_manager.conn.execute(
            "INSERT INTO zones_visitees (nom, visited) VALUES ('MontFavé', 1);",
            [],
        ).unwrap();
        db_manager.conn.execute(
            "INSERT INTO zones_visitees (nom, visited) VALUES ('Rocher des Doms', 1);",
            [],
        ).unwrap();
        db_manager.conn.execute(
            "INSERT INTO zones_visitees (nom, visited) VALUES ('Les Remparts', 0);",
            [],
        ).unwrap();

        // Récupérer les zones visitées
        let zones_visitees = db_manager.get_visited_zones();
        assert_eq!(zones_visitees, "MontFavé, Rocher des Doms");
    }
}
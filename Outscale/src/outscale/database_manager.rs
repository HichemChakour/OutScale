use rusqlite::{Connection, Result};
use std::{fs, io};
use std::path::Path;
pub struct DatabaseManager {
    conn: Connection,
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
}
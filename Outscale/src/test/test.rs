
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_create_table() {
        let db_manager = DatabaseManager::new(":memory:").unwrap();
        let result = db_manager.create_table();
        assert!(result.is_ok());
    }
}
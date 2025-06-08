use crate::outscale::database_manager::DatabaseManager;
// Fichier qui gère les déplacements dans le jeu et ce qui se passe dans les zones
pub fn deplacement_zone(db_manager: &DatabaseManager, nom : &str) {
    db_manager.visite_lieu(nom);
}

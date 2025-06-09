use crate::entities::entity::HasEntity;
use crate::entities::shadow::Shadow;

pub struct ExtractionManager;

impl ExtractionManager {
    /// Permet au joueur de choisir un ennemi vaincu pour tenter de l'extraire
    pub fn extract_enemy(defeated_enemies: &[Box<dyn HasEntity>]) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if defeated_enemies.is_empty() {
            println!("Aucun ennemi disponible pour l'extraction.");
            return Ok(());
        }

        println!("Vous pouvez extraire un ennemi pour en faire un shadow:");
        for (i, enemy) in defeated_enemies.iter().enumerate() {
            println!("{}. {} (Niveau: {})", i + 1, enemy.entity().name, enemy.entity().level);
        }
        println!("0. Annuler l'extraction");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Échec de la lecture de l'entrée");
        let choice = choice.trim().parse::<usize>().unwrap_or(0);

        if choice == 0 || choice > defeated_enemies.len() {
            println!("Extraction annulée.");
            return Ok(());
        }

        let selected_enemy = &defeated_enemies[choice - 1];
        Self::try_extraction(selected_enemy)
    }

    /// Tente l'extraction d'un ennemi avec une chance basée sur différents facteurs
    fn try_extraction(enemy: &Box<dyn HasEntity>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("\x1b[32mExtraction de {}\x1b[0m, il rejoint vos shadows vous pourrez le choisir dans le menu !", enemy.entity().name);

        let mut shadow = Shadow::new(enemy.entity().clone());

        // Créer une connexion à la base de données et insérer le shadow
        let db_manager = crate::outscale::database_manager::DatabaseManager::new("src/save.db")?;
        db_manager.sauvegarde_shadow(&shadow)?;

        Ok(())
    }

    /// Méthode pour proposer l'extraction après un combat victorieux
    pub fn offer_extraction(defeated_enemies: &[Box<dyn HasEntity>]) {
        println!("\nVous avez gagné le combat!");
        println!("Souhaitez-vous tenter d'extraire un shadow? (o/n)");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Échec de la lecture de l'entrée");

        if choice.trim().eq_ignore_ascii_case("o") {
            match Self::extract_enemy(defeated_enemies) {
                Ok(_) => println!("Processus d'extraction terminé."),
                Err(e) => println!("Erreur lors de l'extraction: {}", e),
            }
        } else {
            println!("Vous avez refusé l'extraction.");
        }
    }
}
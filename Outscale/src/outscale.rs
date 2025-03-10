use rusqlite::{params, Connection, Result};

mod init_tables;
// struct qui  le jeu en 
// à l'appel on choisi un fichier de sauvegarde et on lance le jeu 
// Si le fichier n'existe pas on lance une nouvelle partie qui créera un fichier de sauvegarde

pub fn run() {

    let nb_partie: u32;
    let mut input = String::new();

    let conn = Connection::open("save.db").expect("Erreur lors de la connexion à la base de données");


}

fn lancement_mode_histoire() {
    // Implémentation du mode histoire
}
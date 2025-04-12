use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

// Fonction qui applique les styles aux balises du texte
fn apply_styles(text: &str) -> String {
    // Liste des styles avec leurs balises et styles correspondants
    let styles = vec![
        ("[italique]", "\x1b[3m", "[/italique]", "\x1b[0m"),
        ("[gras]", "\x1b[1m", "[/gras]", "\x1b[0m"),
        ("[bleu]", "\x1b[34m", "[/bleu]", "\x1b[0m"),
        ("[vert]", "\x1b[32m", "[/vert]", "\x1b[0m"),
        ("[rouge]", "\x1b[31m", "[/rouge]", "\x1b[0m"),
    ];

    let mut styled_text = text.to_string();

    // Remplacer les balises par les codes ANSI de style
    for (start_tag, start_style, end_tag, end_style) in styles {
        styled_text = styled_text.replace(start_tag, start_style).replace(end_tag, end_style);
    }

    styled_text
}

// Fonction qui demande une entrée à l'utilisateur
pub fn demander_au_joueur(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    input.trim().to_string()
}

// Fonction qui lit un fichier texte et applique les balises de style
pub fn redaction_histoire(fichier: &str) {
    // Vérification de l'existence du fichier
    if !Path::new(fichier).exists() {
        eprintln!("Erreur : Le fichier spécifié n'existe pas : {}", fichier);
        return;
    }

    // Lecture du contenu du fichier
    let contenu = match fs::read_to_string(fichier) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Erreur lors de la lecture du fichier : {}", e);
            return;
        }
    };

    // Applique les styles au texte du fichier
    let sortie = apply_styles(&contenu);

    // Affichage du texte modifié
    println!("{}", sortie);
}

// Test avec du texte statique
pub fn redaction_histoire_test() {
    println!("{}","ROOUUUUUGE".red());

}
//use crate::skills::inventaire::Inventaire;

use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Objet {
        pub id: i32,
        pub inventaire_id: i32,
        pub nom: String,
        pub degats: i32,
        pub degats_magique: i32,
        pub armure: i32,
        pub magic_resist: i32,
        pub mana: i32,
        pub taux_critique: i8,
        pub vitesse: i32,
        pub hp: i32,
        pub type_objet: String,
    }

    impl Objet {
        pub fn new(
            id: i32,
            inventaire_id : i32,
            nom: String,
            degats: i32,
            degats_magique: i32,
            armure: i32,
            magic_resist: i32,
            mana: i32,
            taux_critique: i8,
            vitesse: i32,
            hp: i32,
            type_objet: String,
        ) -> Self {
            Objet {
                id,
                inventaire_id,
                nom,
                degats,
                degats_magique,
                armure,
                magic_resist,
                mana,
                taux_critique,
                vitesse,
                hp,
                type_objet
            }
        }
        pub fn objet_vide() -> Self {
            Objet {
                id: -2, // pour la sauvegarde
                inventaire_id: -1,
                nom: "Aucun".to_string(),
                degats: 0,
                degats_magique: 0,
                armure: 0,
                magic_resist: 0,
                mana: 0,
                taux_critique: 0,
                vitesse: 0,
                hp: 0,
                type_objet: "aucun".to_string(),
            }
        }
    }
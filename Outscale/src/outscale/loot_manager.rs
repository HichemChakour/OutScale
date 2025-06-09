use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::IndexedMutRandom;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use crate::skills::object::Objet;

pub struct LootManager {
    items: Vec<Objet>,
}

impl LootManager {
    pub fn new(json_path: &str) -> Self {
        let file = File::open(json_path).expect("Impossible d'ouvrir le fichier item.json");
        let reader = BufReader::new(file);
        let items: Vec<Objet> = serde_json::from_reader(reader).expect("Erreur de parsing JSON");
        LootManager { items }
    }

    pub fn loot_random_item(&mut self) -> Objet {
        let mut rng = thread_rng();
        self.items
            .choose_mut(&mut rng)
            .expect("Aucun objet dans le loot")
            .clone()
    }
}
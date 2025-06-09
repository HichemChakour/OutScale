use crate::entities::entity::HasEntity;
use rusqlite::Connection;

pub struct LevelUpManager;

impl LevelUpManager {
    pub fn distribute_xp_to_player(player: &mut crate::entities::player::Player, xp: i32) -> String {
        let mut result = String::new();

        let current_xp = player.entity.xp;
        let new_xp = current_xp + xp;

        result.push_str(&format!(
            "\x1b[34m{}\x1b[0m gagne \x1b[33m{}\x1b[0m points d'expérience.\n",
            player.entity.name, xp
        ));

        player.entity.xp = new_xp;

        // Vérifier si le joueur monte de niveau
        if new_xp >= 100 {
            let level_ups = new_xp / 100;
            let remaining_xp = new_xp % 100;

            for _ in 0..level_ups {
                Self::level_up(&mut player.entity, &mut result);
            }

            player.entity.xp = remaining_xp;
        }

        // Ajouter l'XP aux ombres
        for ombre in &mut player.ombres {
            let current_xp = ombre.entity.xp;
            let new_xp = current_xp + xp;
            ombre.entity.xp = new_xp;

            if new_xp >= 100 {
                let level_ups = new_xp / 100;
                let remaining_xp = new_xp % 100;

                for _ in 0..level_ups {
                    Self::level_up(&mut ombre.entity, &mut result);
                }

                ombre.entity.xp = remaining_xp;
            }
        }

        result
    }

    fn level_up(entity: &mut crate::entities::entity::Entity, result: &mut String) {
        entity.level += 1;

        // Augmenter les stats en fonction de la classe
        match entity.classe_id {
            0 => Self::apply_necromancer_bonus(entity), // Nécromancien
            2 => Self::apply_warrior_bonus(entity),     // Guerrier
            3 => Self::apply_sorcerer_bonus(entity),    // Sorcier
            4 => Self::apply_ranger_bonus(entity),      // Rôdeur
            5 => Self::apply_tank_bonus(entity),        // Tank
            _ => Self::apply_default_bonus(entity),     // Par défaut
        }

        result.push_str(&format!(
            "\x1b[32m{}\x1b[0m passe au niveau \x1b[33m{}\x1b[0m!\n",
            entity.name, entity.level
        ));
    }

    fn apply_necromancer_bonus(entity: &mut crate::entities::entity::Entity) {
        entity.max_hp += 50;
        entity.hp = entity.max_hp;
        entity.max_mana += 69;
        entity.mana = entity.max_mana;
        entity.magic_dmg += 25;
        entity.attack_dmg += 25;
        entity.magic_resist += 19;
        entity.armor += 9;
    }

    fn apply_warrior_bonus(entity: &mut crate::entities::entity::Entity) {
        entity.max_hp += 15;
        entity.hp = entity.max_hp;
        entity.max_mana += 5;
        entity.mana = entity.max_mana;
        entity.attack_dmg += 8;
        entity.magic_dmg += 2;
        entity.armor += 5;
        entity.magic_resist += 2;
        entity.speed += 3;
    }

    fn apply_sorcerer_bonus(entity: &mut crate::entities::entity::Entity) {
        entity.max_hp += 8;
        entity.hp = entity.max_hp;
        entity.max_mana += 20;
        entity.mana = entity.max_mana;
        entity.magic_dmg += 10;
        entity.magic_resist += 5;
        entity.attack_dmg += 1;
        entity.armor += 1;
        entity.speed += 2;
    }

    fn apply_ranger_bonus(entity: &mut crate::entities::entity::Entity) {
        entity.max_hp += 12;
        entity.hp = entity.max_hp;
        entity.max_mana += 8;
        entity.mana = entity.max_mana;
        entity.attack_dmg += 6;
        entity.magic_dmg += 3;
        entity.armor += 3;
        entity.magic_resist += 2;
        entity.speed += 6;
        entity.dodge_chance += 0.5;
    }

    fn apply_tank_bonus(entity: &mut crate::entities::entity::Entity) {
        entity.max_hp += 25;
        entity.hp = entity.max_hp;
        entity.max_mana += 5;
        entity.mana = entity.max_mana;
        entity.armor += 8;
        entity.magic_resist += 6;
        entity.attack_dmg += 3;
        entity.speed += 1;
    }

    fn apply_default_bonus(entity: &mut crate::entities::entity::Entity) {
        entity.max_hp += 10;
        entity.hp = entity.max_hp;
        entity.max_mana += 10;
        entity.mana = entity.max_mana;
        entity.attack_dmg += 3;
        entity.magic_dmg += 3;
        entity.armor += 3;
        entity.magic_resist += 3;
        entity.speed += 3;
    }

    // Méthode pour afficher la progression vers le prochain niveau
    pub fn show_xp_progress(player: &crate::entities::player::Player) -> String {
        let mut result = String::new();
        result.push_str("--- Progression d'XP ---\n");

        // Afficher le joueur
        let current_xp = player.entity.xp;
        let needed_xp = 100;
        let percentage = (current_xp as f32 / needed_xp as f32) * 100.0;
        result.push_str(&format!(
            "\x1b[34m{}\x1b[0m (Nv. \x1b[33m{}\x1b[0m): \x1b[32m{}/{}\x1b[0m XP (\x1b[33m{:.1}%\x1b[0m)\n",
            player.entity.name, player.entity.level, current_xp, needed_xp, percentage
        ));

        // Afficher les ombres
        for ombre in &player.ombres {
            let current_xp = ombre.entity.xp;
            let percentage = (current_xp as f32 / needed_xp as f32) * 100.0;
            result.push_str(&format!(
                "\x1b[34m{}\x1b[0m (Ombre, Nv. \x1b[33m{}\x1b[0m): \x1b[32m{}/{}\x1b[0m XP (\x1b[33m{:.1}%\x1b[0m)\n",
                ombre.entity.name, ombre.entity.level, current_xp, needed_xp, percentage
            ));
        }

        result
    }
}
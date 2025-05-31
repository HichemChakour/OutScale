use crate::entities::entity::Entity;

#[derive(Debug, Clone,PartialEq)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub hp_refound: i32,
    pub mana_cost: i32,
    pub mana_refound: i32,
    pub magic_resist_debuff: i32,
    pub magic_resist_buff: i32,
    pub armor_debuff: i32,
    pub armor_buff: i32,
    pub attack_dmg: i32,
    pub attack_dmg_buff: i32,
    pub magic_dmg: i32,
    pub magic_dmg_buff: i32,
    pub for_allies: bool,
    pub entity_id: i32, // -1 pour le joueur
}

impl Skill {
    pub fn new(
         id : i32,
         name: String,
         description: String,
         hp_refound: i32,
         mana_cost: i32,
         mana_refound: i32,
         magic_resist_debuff: i32,
         magic_resist_buff: i32,
         armor_debuff: i32,
         armor_buff: i32,
         attack_dmg: i32,
         attack_dmg_buff: i32,
         magic_dmg: i32,
         magic_dmg_buff: i32,
         for_allies: bool,
         entity_id: i32,
    ) -> Self {
        Skill {
            id: 0,
            name,
            description,
            hp_refound,
            mana_cost,
            mana_refound,
            magic_resist_debuff,
            magic_resist_buff,
            armor_debuff,
            armor_buff,
            attack_dmg,
            attack_dmg_buff,
            magic_dmg,
            magic_dmg_buff,
            for_allies,
            entity_id,
        }
    }
    pub fn calculate_damage(&self, caster: &Entity, target: &Entity) -> i32 {
        let physical_damage = self.attack_dmg + caster.attack_dmg;
        let magic_damage = self.magic_dmg + caster.magic_dmg;

        // Réduction des dégâts physiques en fonction de l'armure de la cible
        let physical_reduction = target.armor as f32 / (target.armor as f32 + 100.0);
        let reduced_physical_damage = physical_damage as f32 * (1.0 - physical_reduction);

        // Réduction des dégâts magiques en fonction de la résistance magique de la cible
        let magic_reduction = target.magic_resist as f32 / (target.magic_resist as f32 + 100.0);
        let reduced_magic_damage = magic_damage as f32 * (1.0 - magic_reduction);

        // Dégâts totaux
        let total_damage = reduced_physical_damage + reduced_magic_damage;

        total_damage as i32
    }
    pub fn apply_effects(&self, caster: &mut Entity, target: &mut Entity) -> String {
        let mut result = String::new();

        caster.mana -= self.mana_cost;

        // Soins
        if self.hp_refound > 0 {
            target.hp += self.hp_refound;
            if target.hp > target.max_hp {
                target.hp = target.max_hp;
            }
            result.push_str(&format!(" \x1b[31m{} \x1b[0m healed for \x1b[32m{}\x1b[0m HP.\n", target.name, self.hp_refound));
        }

        // Restauration de mana
        if self.mana_refound > 0 {
            caster.mana += self.mana_refound;
            if caster.mana > caster.max_mana {
                caster.mana = caster.max_mana;
            }
            result.push_str(&format!("\x1b[31m{} \x1b[0m restored \x1b[32m{}\x1b[0m mana.\n", caster.name, self.mana_refound));
        }

        // Buffs et debuffs
        target.magic_resist += self.magic_resist_buff - self.magic_resist_debuff;
        target.armor += self.armor_buff - self.armor_debuff;
        target.attack_dmg += self.attack_dmg_buff;
        target.magic_dmg += self.magic_dmg_buff;

        if self.magic_resist_buff > 0 {
            result.push_str(&format!("\x1b[34m{} \x1b[0m gained \x1b[32m{}\x1b[0m magic resist.\n", target.name, self.magic_resist_buff));
        }
        if self.magic_resist_debuff > 0 {
            result.push_str(&format!("\x1b[34m{} \x1b[0m lost \x1b[32m{}\x1b[0m magic resist.\n", target.name, self.magic_resist_debuff));
        }
        if self.armor_buff > 0 {
            result.push_str(&format!("\x1b[34m{} \x1b[0m gained \x1b[32m{}\x1b[0m armor.\n", target.name, self.armor_buff));
        }
        if self.armor_debuff > 0 {
            result.push_str(&format!("\x1b[34m{} \x1b[0m lost \x1b[32m{}\x1b[0m armor.\n", target.name, self.armor_debuff));
        }
        if self.attack_dmg_buff > 0 {
            result.push_str(&format!("\x1b[34m{} \x1b[0m gained \x1b[32m{}\x1b[0m attack damage.\n", target.name, self.attack_dmg_buff));
        }
        if self.magic_dmg_buff > 0 {
            result.push_str(&format!("\x1b[34m{} \x1b[0m gained \x1b[32m{}\x1b[0m magic damage.\n", target.name, self.magic_dmg_buff));
        }

        // Calcul des dégâts
        if self.attack_dmg > 0 || self.magic_dmg > 0 {
            let damage = self.calculate_damage(caster, target);
            let damage_taken = target.apply_damage(damage);
            result.push_str(&format!("\x1b[34m{}\x1b[0m a subbit \x1b[33m{}\x1b[0m point de dégat\n", target.name, damage_taken));
        }

        result
    }
}
// equipment.rs
pub struct Sword {
    pub name: String,
    pub damage: i32,
    pub weight: f64,
}

impl Sword {
    pub fn new(name: String, damage: i32, weight: f64) -> Sword {
        Sword { name, damage, weight }
    }

    pub fn display_info(&self) {
        println!("Nom : {}", self.name);
        println!("Dégâts : {}", self.damage);
        println!("Poids : {}", self.weight);
    }

    pub fn calculate_power(&self) -> i32 {
        self.damage as i32
    }
}
mod items;

fn main() {
    // Importer les classes
    use items::equipment::Sword;

    // Créer une instance de Sword
    let sword = Sword::new("Épée de légende".to_string(), 10, 2.5);

    // Appeler les méthodes de Sword
    sword.display_info();
    println!("Puissance : {}", sword.calculate_power());
    println!("Le zizi de Amadou est vreuuuument énormimous bebou, puis-je le lécher goulûment ?")
}
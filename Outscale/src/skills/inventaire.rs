use crate::skills::object::Objet;

#[derive(Clone, Debug, PartialEq)]
pub struct Inventaire {
    pub id: i32,
    pub tete: Objet,
    pub jambes: Objet,
    pub torse: Objet,
    pub main1: Objet,
    pub main2: Objet,
    pub liste_objets: Vec<Objet>
}

impl Inventaire {
    pub fn new(
        id: i32,
        tete: Objet,
        jambes: Objet,
        torse: Objet,
        main1: Objet,
        main2: Objet,
        liste_objets: Vec<Objet>,
    ) -> Self {
        Inventaire {
            id,
            tete,
            jambes,
            torse,
            main1,
            main2,
            liste_objets,
        }
    }

    /*pub(crate) fn to_string(&self) -> String {
        format!(
            "Inventaire {{ id: {}, tete: {}, jambes: {}, torse: {}, main1: {}, main2: {} }}",
            self.id,
            self.tete.nom,
            self.jambes.nom,
            self.torse.nom,
            self.main1.nom,
            self.main2.nom
        )
    }*/
    /*pub(crate) fn to_string_liste_objet (&self) -> String {
        let objets_str: Vec<String> = self.liste_objets.iter().map(|o| o.nom.clone()).collect();
        format!("Liste des objets : {:?}", objets_str)
    }*/
}
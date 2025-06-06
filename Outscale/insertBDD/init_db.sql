-- Suppression des tables existantes
DROP TABLE IF EXISTS player;
DROP TABLE IF EXISTS ennemi;
DROP TABLE IF EXISTS shadow;
DROP TABLE IF EXISTS inventaire;
DROP TABLE IF EXISTS objet;
DROP TABLE IF EXISTS liste_objet;
DROP TABLE IF EXISTS inventaire_skills;
DROP TABLE IF EXISTS classe;
DROP TABLE IF EXISTS skills;
DROP TABLE IF EXISTS liste_skills;
DROP TABLE IF EXISTS type_objet;
Drop TABLE IF EXISTS action;
Drop TABLE IF EXISTS zone;
Drop TABLE IF EXISTS entite;
DROP TABLE IF EXISTS entity;
DROP TABLE IF EXISTS carte;
DROP TABLE IF EXISTS zones;
DROP TRIGGER IF EXISTS after_insert_player;
DROP TRIGGER IF EXISTS after_insert_ennemi;
DROP TRIGGER IF EXISTS after_insert_shadow;
DROP TABLE IF EXISTS journal;

-- Création des tables
CREATE TABLE IF NOT EXISTS player (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nom TEXT DEFAULT NULL,
    max_hp INTEGER DEFAULT NULL,
    hp INTEGER DEFAULT NULL,
    max_mana INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    armor INTEGER DEFAULT NULL,
    attack_damage INTEGER DEFAULT NULL,
    magic_damage INTEGER DEFAULT NULL,
    speed INTEGER DEFAULT NULL,
    dodge_chance FLOAT DEFAULT NULL,
    inventaire_id INTEGER DEFAULT NULL,
    level INTEGER DEFAULT 1,
    xp INTEGER DEFAULT 0,
    FOREIGN KEY(inventaire_id) REFERENCES inventaire(id)
);


CREATE TABLE IF NOT EXISTS entity (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nom TEXT DEFAULT NULL,
    enemy BOOLEAN DEFAULT NULL, --true = ennemi, false = shadow
    max_hp INTEGER DEFAULT NULL,
    hp INTEGER DEFAULT NULL,
    max_mana INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    armor INTEGER DEFAULT NULL,
    attack_damage INTEGER DEFAULT NULL,
    magic_damage INTEGER DEFAULT NULL,
    speed INTEGER DEFAULT NULL,
    dodge_chance FLOAT DEFAULT NULL,
    classe_id INTEGER NOT NULL,
    inventaire_id INTEGER DEFAULT NULL,
    xp INTEGER DEFAULT 0,
    level INTEGER DEFAULT 1,
    FOREIGN KEY(inventaire_id) REFERENCES inventaire(id)
);

CREATE TABLE IF NOT EXISTS inventaire (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_id INTEGER,
    equipement_tete INTEGER,
    equipement_torse INTEGER,
    equipement_jambe INTEGER,
    main1 INTEGER,
    main2 INTEGER
);

CREATE TABLE IF NOT EXISTS objet (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    inventaire_id INTEGER,
    nom TEXT DEFAULT NULL,
    degats INTEGER DEFAULT NULL,
    armure INTEGER DEFAULT NULL,
    taux_critique INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    vitesse INTEGER DEFAULT NULL,
    degats_magique INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    hp INTEGER DEFAULT NULL,
    type_objet TEXT DEFAULT NULL
);


CREATE TABLE IF NOT EXISTS classe (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nom TEXT DEFAULT NULL
);

-- table qui répertorie toutes les compétences de la bdd
CREATE TABLE IF NOT EXISTS skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT DEFAULT NULL,
    description TEXT DEFAULT NULL,
    hp_refound INTEGER DEFAULT NULL,
    mana_cost INTEGER DEFAULT NULL,
    mana_refound INTEGER DEFAULT NULL,
    magic_resist_debuff INTEGER DEFAULT NULL,
    magic_resist_buff INTEGER DEFAULT NULL,
    armor_debuff INTEGER DEFAULT NULL,
    armor_buff INTEGER DEFAULT NULL,
    attack_dmg INTEGER DEFAULT NULL,
    attack_dmg_buff INTEGER DEFAULT NULL,
    magic_dmg INTEGER DEFAULT NULL,
    magic_dmg_buff INTEGER DEFAULT NULL,
    for_allies BOOLEAN DEFAULT FALSE, -- true = pour les alliés, false = pour l'entité
    entity_id INTEGER DEFAULT NULL, -- ID de l'entité qui possède la compétence
    player_id INTEGER DEFAULT NULL -- ID du joueur qui possède la compétence
);

CREATE TABLE IF NOT EXISTS journal (
     id INTEGER PRIMARY KEY AUTOINCREMENT,
     texte TEXT DEFAULT NULL,
      date DATE DEFAULT (DATE('now'))
);

CREATE TABLE IF NOT EXISTS zones(
    nom TEXT PRIMARY KEY,
    description TEXT DEFAULT NULL,
    visited BOOLEAN DEFAULT FALSE
);

-- Création des déclencheurs
CREATE TRIGGER IF NOT EXISTS after_insert_player
AFTER INSERT ON player
FOR EACH ROW
BEGIN
    INSERT INTO inventaire (entity_id) VALUES (NEW.id);
    UPDATE player SET inventaire_id = (SELECT last_insert_rowid()) WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS after_insert_entity
AFTER INSERT ON entity
FOR EACH ROW
BEGIN
    INSERT INTO inventaire (entity_id) VALUES (NEW.inventaire_id);
    UPDATE entity SET inventaire_id = (SELECT last_insert_rowid()) WHERE nom = NEW.nom;
END;

-- Insertion des classes
INSERT INTO classe (nom) VALUES ('Necromancien');
INSERT INTO classe (nom) VALUES ('Guerrier');
INSERT INTO classe (nom) VALUES ('Sorcier');
INSERT INTO classe (nom) VALUES ('Rodeur');
INSERT INTO classe (nom) VALUES ('Tank');


--Insertion des zones

INSERT INTO zones (nom, description) VALUES ('AvignAura', 'Ville normalement paisible, mais qui est actuellement attaquée par des créatures maléfiques.');
INSERT INTO zones (nom, description) VALUES ('Rocher des Doms', 'Une forêt dense et sombre, peuplée de créatures mystérieuses et dangereuses.');
INSERT INTO zones (nom, description) VALUES ('MontFavé', 'Top 7 des montagnes qui se sont fait djoufara par des dragons');
INSERT INTO zones (nom, description) Values ('Shop', 'Boutique où les joueurs peuvent acheter et vendre des objets, des compétences et des équipements.');
INSERT INTO zones (nom, description) VALUES ('Les Remparts', 'Zone de défense de la ville, Là ou des hordes de monstres arrivent en boucles');
INSERT INTO zones (nom, description) VALUES ('Palais des Papes', 'Palais du Pape corrompu');

-- Insertion d'excaliburne
INSERT INTO objet(nom, degats, armure, taux_critique, mana, vitesse, degats_magique, magic_resist, hp, type_objet) VALUES
        ('Excaliburne', 100, 0, 0, 0, 0, 0, 0, 0, 'arme');

INSERT INTO entity(nom, enemy,  max_hp, hp, max_mana, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, classe_id) VALUES
        ('RAVUS', 0, 1000,1000,0, 0, 0, 0, 100, 0, 0, 0.1, (SELECT id FROM classe WHERE nom = 'Guerrier'));
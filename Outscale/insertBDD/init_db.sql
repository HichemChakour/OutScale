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
DROP TRIGGER IF EXISTS after_insert_player;
DROP TRIGGER IF EXISTS after_insert_ennemi;
DROP TRIGGER IF EXISTS after_insert_shadow;
-- Création des tables
CREATE TABLE IF NOT EXISTS player (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nom TEXT DEFAULT NULL,
    hp INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    armor INTEGER DEFAULT NULL,
    attack_damage INTEGER DEFAULT NULL,
    magic_damage INTEGER DEFAULT NULL,
    dodge_chance INTEGER DEFAULT NULL,
    inventaire_skills_id INTEGER DEFAULT NULL,
    inventaire_id INTEGER DEFAULT NULL,
    FOREIGN KEY(inventaire_skills_id) REFERENCES inventaire_skills(id),
    FOREIGN KEY(inventaire_id) REFERENCES inventaire(id)
);

CREATE TABLE IF NOT EXISTS ennemi (
    nom TEXT DEFAULT NULL,
    entite_name TEXT DEFAULT NULL,
    hp INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    armor INTEGER DEFAULT NULL,
    attack_damage INTEGER DEFAULT NULL,
    magic_damage INTEGER DEFAULT NULL,
    dodge_chance INTEGER DEFAULT NULL,
    xp INTEGER DEFAULT NULL,
    classe_id INTEGER NOT NULL,
    inventaire_skills_id INTEGER DEFAULT NULL,
    inventaire_id INTEGER DEFAULT NULL,
    FOREIGN KEY(classe_id) REFERENCES classe(classe_id),
    FOREIGN KEY(inventaire_skills_id) REFERENCES inventaire_skills(id),
    FOREIGN KEY(inventaire_id) REFERENCES inventaire(id)
);

CREATE TABLE IF NOT EXISTS shadow (
    nom TEXT DEFAULT NULL,
    entite_name TEXT DEFAULT NULL,
    hp INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    armor INTEGER DEFAULT NULL,
    attack_damage INTEGER DEFAULT NULL,
    magic_damage INTEGER DEFAULT NULL,
    dodge_chance INTEGER DEFAULT NULL,
    liste_skills TEXT DEFAULT NULL,
    classe_id INTEGER NOT NULL,
    inventaire_skills_id INTEGER DEFAULT NULL,
    inventaire_id INTEGER DEFAULT NULL,
    FOREIGN KEY(classe_id) REFERENCES classe(classe_id),
    FOREIGN KEY(inventaire_skills_id) REFERENCES inventaire_skills(id),
    FOREIGN KEY(inventaire_id) REFERENCES inventaire(id)
);

CREATE TABLE IF NOT EXISTS inventaire (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entite_id INTEGER,
    equipement_tete INTEGER,
    equipement_torse INTEGER,
    equipement_jambe INTEGER,
    main1 INTEGER,
    main2 INTEGER,
    FOREIGN KEY(entite_id) REFERENCES entite(id)
);

CREATE TABLE IF NOT EXISTS objet (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nom TEXT DEFAULT NULL,
    degats INTEGER DEFAULT NULL,
    armure INTEGER DEFAULT NULL,
    taux_critique INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    vitesse INTEGER DEFAULT NULL,
    degats_magique INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    nombre_main INTEGER DEFAULT NULL,
    hp INTEGER DEFAULT NULL,
    type_objet_id INTEGER,
    inventaire_skills_id INTEGER DEFAULT NULL,
    FOREIGN KEY(type_objet_id) REFERENCES type_objet(id),
    FOREIGN KEY(inventaire_skills_id) REFERENCES inventaire_skills(id)
);

CREATE TABLE IF NOT EXISTS liste_objet (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    objet_id INTEGER,
    inventaire_id INTEGER,
    FOREIGN KEY(objet_id) REFERENCES objet(id),
    FOREIGN KEY(inventaire_id) REFERENCES inventaire(id)
);

CREATE TABLE IF NOT EXISTS inventaire_skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE IF NOT EXISTS classe (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nom TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nom TEXT DEFAULT NULL,
    desc_skill TEXT DEFAULT NULL,
    degats INTEGER DEFAULT NULL,
    armure INTEGER DEFAULT NULL,
    taux_critique INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    degats_magique INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    hp INTEGER DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS liste_skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    inventaire_skills_id INTEGER,
    FOREIGN KEY(inventaire_skills_id) REFERENCES inventaire_skills(id),
    FOREIGN KEY(id) REFERENCES skills(id)
);

CREATE TABLE IF NOT EXISTS type_objet (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type_objet TEXT DEFAULT NULL,
    desc_objet TEXT DEFAULT NULL
);

-- Création des déclencheurs
CREATE TRIGGER IF NOT EXISTS after_insert_player
AFTER INSERT ON player
FOR EACH ROW
BEGIN
    INSERT INTO inventaire_skills (id) VALUES (NULL);
    UPDATE player SET inventaire_skills_id = (SELECT last_insert_rowid()) WHERE id = NEW.id;
    INSERT INTO inventaire (entite_id) VALUES (NEW.id);
    UPDATE player SET inventaire_id = (SELECT last_insert_rowid()) WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS after_insert_ennemi
AFTER INSERT ON ennemi
FOR EACH ROW
BEGIN
    INSERT INTO inventaire_skills (id) VALUES (NULL);
    UPDATE ennemi SET inventaire_skills_id = (SELECT last_insert_rowid()) WHERE nom = NEW.nom;
    INSERT INTO inventaire (entite_id) VALUES (NEW.inventaire_id);
    UPDATE ennemi SET inventaire_id = (SELECT last_insert_rowid()) WHERE nom = NEW.nom;
END;

CREATE TRIGGER IF NOT EXISTS after_insert_shadow
AFTER INSERT ON shadow
FOR EACH ROW
BEGIN
    INSERT INTO inventaire_skills (id) VALUES (NULL);
    UPDATE shadow SET inventaire_skills_id = (SELECT last_insert_rowid()) WHERE nom = NEW.nom;
    INSERT INTO inventaire (entite_id) VALUES (NEW.inventaire_id);
    UPDATE shadow SET inventaire_id = (SELECT last_insert_rowid()) WHERE nom = NEW.nom;
END;

CREATE TRIGGER IF NOT EXISTS after_insert_objet
AFTER INSERT ON objet
FOR EACH ROW
BEGIN
    INSERT INTO liste_objet (id, inventaire_id) VALUES (NEW.id, NEW.id);
END;

-- Insertion des classes
INSERT INTO classe (nom) VALUES ('Guerrier');
INSERT INTO classe (nom) VALUES ('Sorcier');
INSERT INTO classe (nom) VALUES ('Rodeur');
INSERT INTO classe (nom) VALUES ('Tank');

-- Insertion des types d'objets
INSERT INTO type_objet (type_objet, desc_objet) VALUES ('Armes', 'Objets utilisés pour infliger des dégâts');
INSERT INTO type_objet (type_objet, desc_objet) VALUES ('Casque', 'Équipement de tête offrant une protection');
INSERT INTO type_objet (type_objet, desc_objet) VALUES ('Plastron', 'Équipement de torse offrant une protection');
INSERT INTO type_objet (type_objet, desc_objet) VALUES ('Jambiere', 'Équipement de jambes offrant une protection');
INSERT INTO type_objet (type_objet, desc_objet) VALUES ('Consomable', 'Objets consommables offrant divers effets');
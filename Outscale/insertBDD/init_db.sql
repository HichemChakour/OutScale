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
DROP TRIGGER IF EXISTS after_insert_player;
DROP TRIGGER IF EXISTS after_insert_ennemi;
DROP TRIGGER IF EXISTS after_insert_shadow;
DROP TABLE IF EXISTS journal;

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
    speed INTEGER DEFAULT NULL,
    dodge_chance INTEGER DEFAULT NULL,
    inventaire_skills_id INTEGER DEFAULT NULL,
    inventaire_id INTEGER DEFAULT NULL,
    FOREIGN KEY(inventaire_skills_id) REFERENCES inventaire_skills(id),
    FOREIGN KEY(inventaire_id) REFERENCES inventaire(id)
);


CREATE TABLE IF NOT EXISTS entity (
    nom TEXT DEFAULT NULL,
    entity_id INTEGER PRIMARY KEY AUTOINCREMENT,
    enemy BOOLEAN DEFAULT NULL, --true = ennemi, false = shadow
    hp INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    armor INTEGER DEFAULT NULL,
    attack_damage INTEGER DEFAULT NULL,
    magic_damage INTEGER DEFAULT NULL,
    speed INTEGER DEFAULT NULL,
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



CREATE TABLE IF NOT EXISTS inventaire_skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE IF NOT EXISTS classe (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nom TEXT DEFAULT NULL
);

-- table qui répertorie toutes les compétences de la bdd
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

-- table qui répertorie toutes les compétences de acquises de tout les personnages de la bdd
CREATE TABLE IF NOT EXISTS liste_skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    inventaire_skills_id INTEGER,
    FOREIGN KEY(inventaire_skills_id) REFERENCES inventaire_skills(id),
    FOREIGN KEY(id) REFERENCES skills(id)
);

CREATE TABLE IF NOT EXISTS journal (
     id INTEGER PRIMARY KEY AUTOINCREMENT,
     texte TEXT DEFAULT NULL,
      date DATE DEFAULT (DATE('now'))
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

CREATE TRIGGER IF NOT EXISTS after_insert_entity
AFTER INSERT ON entity
FOR EACH ROW
BEGIN
    INSERT INTO inventaire_skills (id) VALUES (NULL);
    UPDATE entity SET inventaire_skills_id = (SELECT last_insert_rowid()) WHERE nom = NEW.nom;
    INSERT INTO inventaire (entite_id) VALUES (NEW.inventaire_id);
    UPDATE entity SET inventaire_id = (SELECT last_insert_rowid()) WHERE nom = NEW.nom;
END;



-- Insertion des classes
INSERT INTO classe (nom) VALUES ('Guerrier');
INSERT INTO classe (nom) VALUES ('Sorcier');
INSERT INTO classe (nom) VALUES ('Rodeur');
INSERT INTO classe (nom) VALUES ('Tank');

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
DROP TRIGGER IF EXISTS after_insert_player;
DROP TRIGGER IF EXISTS after_insert_ennemi;
DROP TRIGGER IF EXISTS after_insert_shadow;
-- Création des tables
CREATE TABLE IF NOT EXISTS player (
    p_n_id INTEGER PRIMARY KEY AUTOINCREMENT,
    va_nom TEXT DEFAULT NULL,
    hp INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    armor INTEGER DEFAULT NULL,
    attack_damage INTEGER DEFAULT NULL,
    magic_damage INTEGER DEFAULT NULL,
    dodge_chance INTEGER DEFAULT NULL,
    iv_skills_id INTEGER DEFAULT NULL,
    i_n_id INTEGER DEFAULT NULL,
    FOREIGN KEY(iv_skills_id) REFERENCES inventaire_skills(iv_skills_id),
    FOREIGN KEY(i_n_id) REFERENCES inventaire(i_n_id)
);

CREATE TABLE IF NOT EXISTS ennemi (
    va_nom TEXT DEFAULT NULL,
    entite_name TEXT DEFAULT NULL,
    hp INTEGER DEFAULT NULL,
    mana INTEGER DEFAULT NULL,
    magic_resist INTEGER DEFAULT NULL,
    armor INTEGER DEFAULT NULL,
    attack_damage INTEGER DEFAULT NULL,
    magic_damage INTEGER DEFAULT NULL,
    dodge_chance INTEGER DEFAULT NULL,
    n_xp INTEGER DEFAULT NULL,
    classe_id INTEGER NOT NULL,
    iv_skills_id INTEGER DEFAULT NULL,
    i_n_id INTEGER DEFAULT NULL,
    FOREIGN KEY(classe_id) REFERENCES classe(classe_id),
    FOREIGN KEY(iv_skills_id) REFERENCES inventaire_skills(iv_skills_id),
    FOREIGN KEY(i_n_id) REFERENCES inventaire(i_n_id)
);

CREATE TABLE IF NOT EXISTS shadow (
    va_nom TEXT DEFAULT NULL,
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
    iv_skills_id INTEGER DEFAULT NULL,
    i_n_id INTEGER DEFAULT NULL,
    FOREIGN KEY(classe_id) REFERENCES classe(classe_id),
    FOREIGN KEY(iv_skills_id) REFERENCES inventaire_skills(iv_skills_id),
    FOREIGN KEY(i_n_id) REFERENCES inventaire(i_n_id)
);

CREATE TABLE IF NOT EXISTS inventaire (
    i_n_id INTEGER PRIMARY KEY AUTOINCREMENT,
    e_id INTEGER,
    equipement_tete INTEGER,
    equipement_torse INTEGER,
    equipement_jambe INTEGER,
    main1 INTEGER,
    main2 INTEGER,
    de_qui TEXT DEFAULT NULL,
    FOREIGN KEY(e_id) REFERENCES entite(e_id)
);

CREATE TABLE IF NOT EXISTS objet (
    o_id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    iv_skills_id INTEGER DEFAULT NULL,
    FOREIGN KEY(type_objet_id) REFERENCES type_objet(type_objet_id),
    FOREIGN KEY(iv_skills_id) REFERENCES inventaire_skills(iv_skills_id)
);

CREATE TABLE IF NOT EXISTS liste_objet (
    liste_objet_id INTEGER PRIMARY KEY AUTOINCREMENT,
    o_id INTEGER,
    i_n_id INTEGER,
    FOREIGN KEY(o_id) REFERENCES objet(o_id),
    FOREIGN KEY(i_n_id) REFERENCES inventaire(i_n_id)
);

CREATE TABLE IF NOT EXISTS inventaire_skills (
    iv_skills_id INTEGER PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE IF NOT EXISTS classe (
    classe_id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    liste_skills_id INTEGER PRIMARY KEY AUTOINCREMENT,
    iv_skills_id INTEGER,
    id INTEGER,
    FOREIGN KEY(iv_skills_id) REFERENCES inventaire_skills(iv_skills_id),
    FOREIGN KEY(id) REFERENCES skills(id)
);

CREATE TABLE IF NOT EXISTS type_objet (
    type_objet_id INTEGER PRIMARY KEY AUTOINCREMENT,
    type_objet TEXT DEFAULT NULL,
    desc_objet TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS zones(
    nom VARCHAR2(70) PRIMARY KEY,
    cleared BOOLEAN DEFAULT FALSE
);

-- Création des déclencheurs
CREATE TRIGGER IF NOT EXISTS after_insert_player
AFTER INSERT ON player
FOR EACH ROW
BEGIN
    INSERT INTO inventaire_skills (iv_skills_id) VALUES (NULL);
    UPDATE player SET iv_skills_id = (SELECT last_insert_rowid()) WHERE p_n_id = NEW.p_n_id;
    INSERT INTO inventaire (e_id, de_qui) VALUES (NEW.p_n_id, NEW.va_nom);
    UPDATE player SET i_n_id = (SELECT last_insert_rowid()) WHERE p_n_id = NEW.p_n_id;
END;

CREATE TRIGGER IF NOT EXISTS after_insert_ennemi
AFTER INSERT ON ennemi
FOR EACH ROW
BEGIN
    INSERT INTO inventaire_skills (iv_skills_id) VALUES (NULL);
    UPDATE ennemi SET iv_skills_id = (SELECT last_insert_rowid()) WHERE va_nom = NEW.va_nom;
    INSERT INTO inventaire (e_id, de_qui) VALUES (NEW.i_n_id, NEW.va_nom);
    UPDATE ennemi SET i_n_id = (SELECT last_insert_rowid()) WHERE va_nom = NEW.va_nom;
END;

CREATE TRIGGER IF NOT EXISTS after_insert_shadow
AFTER INSERT ON shadow
FOR EACH ROW
BEGIN
    INSERT INTO inventaire_skills (iv_skills_id) VALUES (NULL);
    UPDATE shadow SET iv_skills_id = (SELECT last_insert_rowid()) WHERE va_nom = NEW.va_nom;
    INSERT INTO inventaire (e_id, de_qui) VALUES (NEW.i_n_id, NEW.va_nom);
    UPDATE shadow SET i_n_id = (SELECT last_insert_rowid()) WHERE va_nom = NEW.va_nom;
END;

CREATE TABLE IF NOT EXISTS journal (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    texte TEXT DEFAULT NULL,
    date DATE DEFAULT (DATE('now'))
);

CREATE TRIGGER IF NOT EXISTS after_insert_objet
AFTER INSERT ON objet
FOR EACH ROW
BEGIN
    INSERT INTO liste_objet (o_id, i_n_id) VALUES (NEW.o_id, NEW.i_n_id);
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
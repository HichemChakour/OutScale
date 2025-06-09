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
    classe_id INTEGER NOT NULL,
    FOREIGN KEY(inventaire_id) REFERENCES inventaire(id)
);


CREATE TABLE IF NOT EXISTS entity (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nom TEXT DEFAULT NULL,
    enemy BOOLEAN DEFAULT NULL,
    used BOOLEAN DEFAULT FALSE,
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
    discovered BOOLEAN DEFAULT FALSE, -- true = compétence découverte, false = compétence non découverte
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
INSERT INTO classe (nom) VALUES ('Guerrier');
INSERT INTO classe (nom) VALUES ('Sorcier');
INSERT INTO classe (nom) VALUES ('Rodeur');
INSERT INTO classe (nom) VALUES ('Tank');
INSERT INTO classe (nom) VALUES ('Drake');



--Insertion des zones

INSERT INTO zones (nom, description) VALUES ('AvignAura', 'Ville normalement paisible, mais qui est actuellement attaquée par des créatures maléfiques.');
INSERT INTO zones (nom, description) VALUES ('Rocher des Doms', 'Une forêt dense et sombre, peuplée de créatures mystérieuses et dangereuses.');
INSERT INTO zones (nom, description) VALUES ('MontFavé', 'Top 7 des montagnes qui se sont fait djoufara par des dragons');
INSERT INTO zones (nom, description) Values ('Shop', 'Boutique où les joueurs peuvent acheter et vendre des objets, des compétences et des équipements.');
INSERT INTO zones (nom, description) VALUES ('Les Remparts', 'Zone de défense de la ville, Là ou des hordes de monstres arrivent en boucles');
INSERT INTO zones (nom, description) VALUES ('Palais des Papes', 'Palais du Pape corrompu');

-- Insertion d'excaliburne
INSERT INTO objet(nom, degats, armure, taux_critique, mana, vitesse, degats_magique, magic_resist, hp, type_objet) VALUES
        ('Excalibur', 100, 0, 0, 0, 0, 0, 0, 0, 'arme');

INSERT INTO entity(nom, enemy, used, max_hp, hp, max_mana, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, classe_id, xp, level) VALUES
        ('Gardien Corrompu', 1,0, 20,20,0, 0, 0, 0, 20, 0, 5, 0.1, (SELECT id FROM classe WHERE nom = 'Guerrier'),100, 1);

/*Zone2 Le roché des doms*/
INSERT INTO entity(nom, enemy, used, max_hp, hp, max_mana, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, classe_id, xp, level) VALUES
        ('Gardien du temple', 1,0, 200, 200, 300, 70, 50, 50, 25, 5, 10, 0.1, (SELECT id FROM classe WHERE nom = 'Guerrier'), 75, 2);

INSERT INTO  entity(nom, enemy, used, max_hp, hp, max_mana, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, classe_id, xp, level) VALUES
        ('Le prêtre', 1,0, 150, 150, 300, 500, 20, 20, 7, 70, 0, 0.2, (SELECT id FROM classe WHERE nom = 'Sorcier'), 75, 2);

INSERT INTO  entity(nom, enemy, used,  max_hp, hp, max_mana, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, classe_id, xp, level) VALUES
    ('L`imame', 1,0, 150, 150, 300, 300, 20, 20, 7, 70, 0, 0.2, (SELECT id FROM classe WHERE nom = 'Sorcier'), 75, 2);

INSERT INTO skills (name, description, hp_refound, mana_cost, mana_refound, magic_resist_debuff, magic_resist_buff, armor_debuff, armor_buff, attack_dmg, attack_dmg_buff, magic_dmg, magic_dmg_buff, for_allies, entity_id, player_id) VALUES
    ('Gros cout de massue', 'Inflige de lourds dégâts physiques à l`ennemi.', 0, 25, 0, 0, 0, 0, 0, 20, 0, 0, 0, FALSE, (SELECT id FROM entity WHERE nom = 'Gardien du temple'), NULL),
    ('Rappel Religieux', 'Soigne une partie des points de vie de l`entité.', 100, 30, 0, 0, 5, 0, 5, 0, 5, 0, 5, TRUE, (SELECT id FROM entity WHERE nom = 'L`imame'), NULL),
    ('Bouclier magique', 'Augmente la résistance magique de l`entité.', 0, 40, 0, 0, 20, 0, 0, 0, 0, 0,0, TRUE, (SELECT id FROM entity WHERE nom = 'Le prêtre'), NULL);

/*Zone4 Le mont favé*/
INSERT INTO entity(nom, enemy, used,  max_hp, hp, max_mana, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, classe_id, xp, level) VALUES
    ('Le dragon noir', 1,0, 1000, 1000, 300, 300, 150, 150, 90, 100, 150, 0.1, (SELECT id FROM classe WHERE nom = 'Drake'), 300, 10);

INSERT INTO skills (name, description, hp_refound, mana_cost, mana_refound, magic_resist_debuff, magic_resist_buff, armor_debuff, armor_buff, attack_dmg, attack_dmg_buff, magic_dmg, magic_dmg_buff, for_allies, entity_id, player_id) VALUES
    ('Souffle de mafé', 'Inflige de lourds dégâts magiques à tous les ennemis.', 0, 100, 0, 0, 0, 0, 0, 70, 0, 0, 0, FALSE, (SELECT id FROM entity WHERE nom = 'Le dragon noir'), NULL),
    ('Charge du dragon', 'Inflige des dégâts physiques à l`ennemi et le repousse.', 0, 80, 0, 0, 0, 0, 0, 50, 0, 0, 0, FALSE, (SELECT id FROM entity WHERE nom = 'Le dragon noir'), NULL),
    ('Grosse carapace', 'Soigne une partie des points de vie du dragon.', 200, 50, 0, 0, 10, 0, 10, 0, 0, 0, 0, FALSE, (SELECT id FROM entity WHERE nom = 'Le dragon noir'), NULL);

/*Zone5 palais des papes*/
INSERT INTO entity(nom, enemy, used, max_hp, hp, max_mana, mana, magic_resist, armor, attack_damage, magic_damage, speed, dodge_chance, classe_id, xp, level) VALUES
    ('Pape corrompu', 1,0, 1500, 1500, 500, 500, 200, 200, 120, 150, 100, 0.1, (SELECT id FROM classe WHERE nom = 'Sorcier'), 300, 15);

INSERT INTO skills (name, description, hp_refound, mana_cost, mana_refound, magic_resist_debuff, magic_resist_buff, armor_debuff, armor_buff, attack_dmg, attack_dmg_buff, magic_dmg, magic_dmg_buff, for_allies, entity_id, player_id) VALUES
    ('Foudre divine', 'Inflige de lourds dégâts magiques à l`ennemi.', 0, 120, 0, 0, 0, 0, 0, 30, 0, 60, 0, FALSE, (SELECT id FROM entity WHERE nom = 'Pape corrompu'), NULL),
    ('Bénédiction du pape', 'Soigne une partie des points de vie de l`entité et augmente sa résistance magique.', 200, 0, 200, 20, 0, 0, 0, 0, 0, 0,0, TRUE, (SELECT id FROM entity WHERE nom = 'Pape corrompu'), NULL),
    ('Châtiment divin', 'Inflige des dégâts magiques à tous les ennemis et réduit leur armure.', 0, 150, 0, -30, 0, -30, 0, 50, 0, 50,0, FALSE, (SELECT id FROM entity WHERE nom = 'Pape corrompu'), NULL);


INSERT INTO skills (name, discovered, description, hp_refound, mana_cost, mana_refound, magic_resist_debuff, magic_resist_buff, armor_debuff, armor_buff, attack_dmg, attack_dmg_buff, magic_dmg, magic_dmg_buff, for_allies, entity_id, player_id) VALUES
    ('Coup de pied', true,'Inflige des dégâts physiques à l`ennemi.', 0, 20, 0, 0, 0, 0, 0, 30, 0, 0, 0, FALSE, NULL, NULL),
    ('Soin rapide', true,'Soigne une partie des points de vie de l`entité.', 50, 10, 0, 0, 5, 0, 5, 0, 5, 0, 5, TRUE, NULL, NULL),
    ('Bouclier de protection', true,'Augmente la résistance magique de l`entité.', 0, 15, 0, 0, 10, 0, 0, 0, 0, 0,0, FALSE, NULL, NULL),
    ('Charge', true,'Inflige des dégâts physiques à l`ennemi et augmente la vitesse de l`entité.', 0, 30, 0, 0, 0, 0, 0, 50, 0, 0, 20, FALSE, NULL, 1);

/*Randon 30 skills to collect*/
INSERT INTO skills (name, discovered, description, hp_refound, mana_cost, mana_refound, magic_resist_debuff, magic_resist_buff, armor_debuff, armor_buff, attack_dmg, attack_dmg_buff, magic_dmg, magic_dmg_buff, for_allies, entity_id, player_id) VALUES
    ('Coup de bouclier', 0, 'Inflige des dégâts physiques à l`ennemi et le repousse.', 0, 30, 0, 0, 0, 0, 0, 40, 0, 0, 0, FALSE, NULL, NULL),
    ('Charge héroïque',0, 'Inflige des dégâts physiques à l`ennemi et augmente la vitesse de l`entité.', 0, 50, 0, 0, 0, 0, 0, 60, 0, 0, 20, FALSE, NULL, NULL),
    ('Frappe rapide', 0,'Inflige des dégâts physiques à l`ennemi avec une chance de coup critique.', 0, 25, 0, 0, 0, 10, 0, 50, 20, 0, 10, FALSE, NULL, NULL),
    ('Soin puissant', 0,'Soigne une partie importante des points de vie de l`entité.', 200, 40, 0, 0, 10, 0, 10, 0, 10, 0 ,10 , TRUE , NULL , NULL),
    ('Bouclier magique',0, 'Augmente la résistance magique de l`entité pour un court laps de temps.', 0 ,30 ,0 ,20 ,20 ,0 ,0 ,0 ,0 ,20 ,20 , FALSE , NULL , NULL),
    ('Attaque tourbillonnante', 0,'Inflige des dégâts physiques à tous les ennemis proches.', 50 ,60 ,0 ,10 ,10 ,10 ,10 ,70 ,30 ,30 ,30 , FALSE , NULL , NULL),
    ('Rage du guerrier', 0,'Augmente les dégâts d`attaque de l`entité pendant un court laps de temps.', -50 ,80 ,50 ,-20 ,-20 ,-20 ,-20 ,-100 ,-50 ,-50 ,-50 ,- FALSE ,- NULL ,NULL),
    ('Coup de grâce', 0,'Inflige des dégâts massifs à l`ennemi, mais coûte beaucoup de mana.', 0, 100, 0, 0, 0, 0, 0, 150, 0, 0, 0, FALSE, NULL, NULL),
    ('Saut héroïque', 0,'Permet à l`entité de sauter sur un ennemi et d`infliger des dégâts.', 0, 40, 0, 0, 0, 0, 0, 80, 0, 0, 20, FALSE, NULL, NULL),
    ('Provocation', 0,'Force les ennemis à attaquer l`entité pendant un court laps de temps.', -50 ,20 ,0 ,30 ,30 ,30 ,30 ,0 ,0 ,0 ,0 , TRUE , NULL , NULL),
    ('Coup de tonnerre', 0,'Inflige des dégâts magiques à tous les ennemis proches.', 0 ,70 ,0 ,10 ,10 ,10 ,10 ,60 ,20 ,40 ,20 , FALSE , NULL , NULL),
    ('Soin en chaîne', 0,'Soigne une partie des points de vie de l`entité et de ses alliés proches.', -100 ,50 ,100 ,-10 ,-10 ,-10 ,-10 ,-50 ,-20 ,-20 ,-20 ,- TRUE ,- NULL ,NULL),
    ('Frappe du vent',0, 'Inflige des dégâts physiques à l`ennemi et augmente la vitesse de l`entité.', -50 ,30 ,0 ,-10 ,-10 ,-10 ,-10 ,-40 ,-20 ,-20 ,-20 ,- FALSE ,- NULL ,NULL),
    ('Bouclier d`énergie', 0,'Augmente la résistance magique de l`entité pour un court laps de temps.', -50 ,40 ,50 ,20 ,20 ,0 ,0 ,0 ,0 ,20 ,20 , FALSE ,- NULL ,NULL),
    ('Coup de pied sauté', 0,'Inflige des dégâts physiques à l`ennemi et le repousse.', -30 ,30 ,0 ,-10 ,-10 ,-10 ,-10 ,-40 ,-20 ,-20 ,-20 ,- FALSE ,- NULL ,NULL),
    ('Charge de la tempête', 0,'Inflige des dégâts physiques à l`ennemi et augmente la vitesse de l`entité.', -50 ,50 ,0 ,-10 ,-10 ,-10 ,-10 ,-60 ,-30 ,-30 ,-30 ,- FALSE ,- NULL ,NULL),
    ('Frappe de feu', 0,'Inflige des dégâts magiques à l`ennemi et le brûle pendant un court laps de temps.', 0 ,60 ,0 ,10 ,10 ,0 ,0 ,70 ,20 ,40 ,20 , FALSE ,- NULL ,NULL),
    ('Soin régénérateur', 0,'Soigne une partie des points de vie de l`entité sur la durée.', -100 ,50 ,100 ,0 ,0 ,0 ,0 ,50 ,20 ,20 ,20 , TRUE ,- NULL ,NULL),
    ('Coup de bouclier renforcé', 0,'Inflige des dégâts physiques à l`ennemi et augmente la défense de l`entité.', -50 ,40 ,0 ,-20 ,-20 ,-20 ,-20 ,-80 ,-40 ,-40 ,-40 ,- FALSE ,- NULL ,NULL),
    ('Attaque en rafale', 0,'Inflige plusieurs coups rapides à l`ennemi.', 0, 70, 0, 0, 0, 30, 0, 80, 30, 0, 10, FALSE, NULL, NULL),
    ('Saut d`attaque', 0,'Permet à l`entité de sauter sur un ennemi et d`infliger des dégâts.', 0, 40, 0, 0, 0, 0, 0, 70, 0, 0, 20, FALSE, NULL, NULL),
    ('Provocation renforcée', 0,'Force les ennemis à attaquer l`entité pendant un court laps de temps et augmente sa défense.', -50 ,30 ,0 ,40 ,40 ,40 ,40 ,0 ,0 ,0 ,0 , TRUE , NULL , NULL),
    ('Coup de tonnerre puissant', 0,'Inflige des dégâts magiques à tous les ennemis proches et les étourdit.', 0 ,80 ,0 ,20 ,20 ,20 ,20 ,70 ,30 ,50 ,30 , FALSE , NULL , NULL),
    ('Soin en chaîne amélioré', 0,'Soigne une partie des points de vie de l`entité et de ses alliés proches sur la durée.', -150 ,60 ,150 ,-20 ,-20 ,-20 ,-20 ,-60 ,-30 ,-30 ,-30 ,- TRUE ,- NULL ,NULL),
    ('Frappe du vent renforcée', 0,'Inflige des dégâts physiques à l`ennemi et augmente la vitesse de l`entité.', -100 ,40 ,0 ,-20 ,-20 ,-20 ,-20 ,-50 ,-30 ,-30 ,-30 ,- FALSE ,- NULL ,NULL),
    ('Bouclier d`énergie renforcé', 0,'Augmente la résistance magique de l`entité pour un court laps de temps et réduit les dégâts subis.', -100 ,50 ,100 ,30 ,30 ,0 ,0 ,0 ,0 ,30 ,30 , FALSE ,- NULL ,NULL);




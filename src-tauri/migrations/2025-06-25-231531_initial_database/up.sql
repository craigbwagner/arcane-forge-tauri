CREATE TABLE IF NOT EXISTS characters (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            creator TEXT NOT NULL,
            basic_description TEXT NOT NULL,
            levels TEXT NOT NULL,
            combat_stats TEXT NOT NULL,
            languages TEXT NOT NULL,
            ability_scores TEXT NOT NULL,
            skills TEXT NOT NULL,
            kill_list TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

CREATE TABLE IF NOT EXISTS items (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            total_charges INTEGER NOT NULL,
            value INTEGER NOT NULL,
            weight REAL NOT NULL,
            rarity TEXT NOT NULL,
            item_type TEXT NOT NULL,
            attunement BOOLEAN NOT NULL,
            is_magical BOOLEAN NOT NULL,
            acquired_through TEXT NOT NULL
        );

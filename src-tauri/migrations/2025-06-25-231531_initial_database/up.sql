CREATE TABLE IF NOT EXISTS characters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            levels TEXT,
            creator TEXT,
            basic_description TEXT,
            classes TEXT,
            languages TEXT,
            ability_scores TEXT,
            combat_stats TEXT,
            additional_features TEXT,
            skills TEXT,
            items TEXT,
            kill_list TEXT,
            created_at TEXT,
            updated_at TEXT
        )

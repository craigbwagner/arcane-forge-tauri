CREATE TABLE IF NOT EXISTS characters (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            levels TEXT NOT NULL,
            creator TEXT NOT NULL,
            basic_description TEXT NOT NULL,
            classes TEXT NOT NULL,
            languages TEXT NOT NULL,
            ability_scores TEXT NOT NULL,
            combat_stats TEXT NOT NULL,
            additional_features TEXT NOT NULL,
            skills TEXT NOT NULL,
            items TEXT NOT NULL,
            kill_list TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )

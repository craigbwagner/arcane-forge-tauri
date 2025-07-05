// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Integer,
        name -> Text,
        creator -> Text,
        basic_description -> Text,
        levels -> Text,
        combat_stats -> Text,
        languages -> Text,
        ability_scores -> Text,
        skills -> Text,
        kill_list -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

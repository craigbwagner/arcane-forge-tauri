// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Integer,
        name -> Text,
        creator -> Text,
        basic_description -> Text,
        combat_stats -> Text,
        languages -> Text,
        ability_scores -> Text,
        skills -> Text,
        kill_list -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        total_charges -> Integer,
        value -> Integer,
        weight -> Float,
        rarity -> Text,
        item_type -> Text,
        attunement -> Bool,
        is_magical -> Bool,
        acquired_through -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    items,
);

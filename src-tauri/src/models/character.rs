use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Alignment::LawfulGood => "Lawful Good",
                Alignment::NeutralGood => "Neutral Good",
                Alignment::ChaoticGood => "Chaotic Good",
                Alignment::LawfulNeutral => "Lawful Neutral",
                Alignment::TrueNeutral => "Neutral",
                Alignment::ChaoticNeutral => "Chaotic Neutral",
                Alignment::LawfulEvil => "Lawful Evil",
                Alignment::NeutralEvil => "Neutral Evil",
                Alignment::ChaoticEvil => "Chaotic Evil",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Sex {
    Male,
    Female,
    Other,
    Unspecified,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum HitDiceType {
    D6,
    D8,
    D10,
    D12,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
    FreeAction,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum UseReset {
    Daily,
    LongRest,
    ShortRest,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MagicItemRarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub creator: String,
    pub basic_description: BasicDescription,
    pub classes: Vec<Class>,
    pub languages: Vec<String>,
    pub ability_scores: [AbilityScore; 6],
    pub combat_stats: CombatStats,
    pub additional_features: Vec<Feature>,
    pub skills: [Skill; 18],
    pub items: Vec<Item>,
    pub weapons: Vec<Weapon>,
    pub kill_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Character {
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            creator: String::new(),
            basic_description: BasicDescription::default(),
            classes: Vec::new(),
            languages: Vec::new(),
            ability_scores: [
                AbilityScore::new(Ability::Strength, "STR"),
                AbilityScore::new(Ability::Dexterity, "DEX"),
                AbilityScore::new(Ability::Constitution, "CON"),
                AbilityScore::new(Ability::Intelligence, "INT"),
                AbilityScore::new(Ability::Wisdom, "WIS"),
                AbilityScore::new(Ability::Charisma, "CHA"),
            ],
            combat_stats: CombatStats::default(),
            additional_features: Vec::new(),
            skills: [
                Skill::new("Acrobatics", Ability::Dexterity),
                Skill::new("Animal Handling", Ability::Wisdom),
                Skill::new("Arcana", Ability::Intelligence),
                Skill::new("Athletics", Ability::Strength),
                Skill::new("Deception", Ability::Charisma),
                Skill::new("History", Ability::Intelligence),
                Skill::new("Insight", Ability::Wisdom),
                Skill::new("Intimidation", Ability::Charisma),
                Skill::new("Investigation", Ability::Intelligence),
                Skill::new("Medicine", Ability::Wisdom),
                Skill::new("Nature", Ability::Intelligence),
                Skill::new("Perception", Ability::Wisdom),
                Skill::new("Performance", Ability::Charisma),
                Skill::new("Persuasion", Ability::Charisma),
                Skill::new("Religion", Ability::Intelligence),
                Skill::new("Sleight of Hand", Ability::Dexterity),
                Skill::new("Stealth", Ability::Dexterity),
                Skill::new("Survival", Ability::Wisdom),
            ],
            items: Vec::new(),
            weapons: Vec::new(),
            kill_list: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BasicDescription {
    pub race: String,
    pub sex: Sex,
    pub size: Size,
    pub age: u16,
    pub height: String,
    pub weight: u16,
    pub alignment: Alignment,
}

impl Default for BasicDescription {
    fn default() -> Self {
        Self {
            race: String::new(),
            sex: Sex::Unspecified,
            size: Size::Medium,
            age: 0,
            height: String::new(),
            weight: 0,
            alignment: Alignment::TrueNeutral,
        }
    }
}

impl Character {
    pub fn total_level(&self) -> u8 {
        self.classes.iter().map(|c| c.level).sum()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CombatStats {
    pub initiative_mods: u8,
    pub speed: u8,
    pub speed_mods: u8,
    pub max_hp: u16,
    pub current_hp: u16,
    pub temp_hp: u8,
    pub hit_dice_remaining: u8,
    pub hit_dice_type: HitDiceType,
    pub hit_dice_total: u8,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            initiative_mods: 0,
            speed: 30,
            speed_mods: 0,
            max_hp: 1,
            current_hp: 1,
            temp_hp: 0,
            hit_dice_remaining: 1,
            hit_dice_type: HitDiceType::D8,
            hit_dice_total: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Class {
    pub name: String,
    pub subclass: String,
    pub level: u8,
    pub class_features: Vec<Feature>,
    pub subclass_features: Vec<Feature>,
}

impl Default for Class {
    fn default() -> Self {
        Self {
            name: String::new(),
            subclass: String::new(),
            level: 1,
            class_features: Vec::new(),
            subclass_features: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AbilityScore {
    pub name: String,
    pub short_name: String,
    pub is_proficient: bool,
    pub score: u8,
}

impl AbilityScore {
    pub fn new(name: Ability, short_name: &str) -> Self {
        Self {
            name: format!("{:?}", name),
            short_name: String::from(short_name),
            is_proficient: false,
            score: 10,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Skill {
    pub name: String,
    pub is_proficient: bool,
    pub has_expertise: bool,
    pub ability_name: Ability,
}

impl Skill {
    pub fn new(name: &str, ability: Ability) -> Self {
        Self {
            name: String::from(name),
            is_proficient: false,
            has_expertise: false,
            ability_name: ability,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Feature {
    pub name: Option<String>,
    pub description: Option<String>,
    pub uses: Option<u8>,
    pub uses_reset_on: Option<UseReset>,
    pub action_type: Option<ActionType>,
    pub duration: Option<String>,
    pub source: Option<String>,
    pub level_acquired: Option<u8>,
    pub is_passive: bool,
}

impl Default for Feature {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            uses: None,
            uses_reset_on: None,
            action_type: None,
            duration: None,
            source: None,
            level_acquired: None,
            is_passive: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Item {
    pub name: Option<String>,
    pub description: Option<String>,
    pub charges: Option<u8>,
    pub value: Option<u16>,
    pub weight: Option<f32>,
    pub rarity: Option<MagicItemRarity>,
    pub item_type: Option<String>,
    pub properties: Option<Vec<String>>,
    pub attunement: Option<bool>,
    pub is_magical: Option<bool>,
    pub source: Option<String>,
    pub acquired_through: Option<String>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            charges: None,
            value: None,
            weight: None,
            rarity: None,
            item_type: None,
            properties: None,
            attunement: None,
            is_magical: None,
            source: None,
            acquired_through: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Weapon {
    pub name: Option<String>,
    pub weight: Option<u8>,
    pub rarity: Option<MagicItemRarity>,
    pub details: Option<String>,
    pub damage_type: DamageType,
    pub reach: Option<u8>,
    pub value: Option<u16>,
    pub is_equipped: bool,
    pub to_hit_override: Option<u8>,
    pub to_hit_bonus: Option<u8>,
    pub damage_bonus: Option<u8>,
    pub source: Option<String>,
    pub acquired_through: Option<String>,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            name: None,
            weight: None,
            rarity: None,
            details: None,
            damage_type: DamageType::Slashing,
            reach: None,
            value: None,
            is_equipped: false,
            to_hit_override: None,
            to_hit_bonus: None,
            damage_bonus: None,
            source: None,
            acquired_through: None,
        }
    }
}

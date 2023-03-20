use eo::{
    data::{EOChar, EOInt, EOShort},
    protocol::{
        client::character::Create, AdminLevel, BigCoords, CharacterBaseStats2, CharacterMapInfo,
        CharacterSecondaryStats, CharacterStats2, Coords, Direction, Gender, Item, PaperdollFull,
        SitState, Skin, Spell, PaperdollIcon,
    },
};

use chrono::prelude::*;
use mysql_async::{prelude::*, Conn, Params, Row, TxOpts};

use crate::{
    player::PlayerHandle,
    utils::{self, full_to_b000a0hsw},
    SETTINGS,
};

#[derive(Debug, Clone, Default)]
pub struct Character {
    pub player_id: Option<EOShort>,
    pub player: Option<PlayerHandle>,
    pub id: EOInt,
    pub account_id: EOInt,
    pub name: String,
    pub title: Option<String>,
    pub home: String,
    pub fiance: Option<String>,
    pub partner: Option<String>,
    pub admin_level: AdminLevel,
    pub class: EOChar,
    pub gender: Gender,
    pub skin: Skin,
    pub hair_style: EOShort,
    pub hair_color: EOShort,
    pub bank_max: EOInt,
    pub gold_bank: EOInt,
    pub guild_name: Option<String>,
    pub guild_tag: Option<String>,
    pub guild_rank_id: Option<EOChar>,
    pub guild_rank_string: Option<String>,
    pub paperdoll: PaperdollFull,
    pub level: EOChar,
    pub experience: EOInt,
    pub hp: EOShort,
    pub max_hp: EOShort,
    pub tp: EOShort,
    pub max_tp: EOShort,
    pub max_sp: EOShort,
    pub weight: EOInt,
    pub max_weight: EOInt,
    pub base_strength: EOShort,
    pub base_intelligence: EOShort,
    pub base_wisdom: EOShort,
    pub base_agility: EOShort,
    pub base_constitution: EOShort,
    pub base_charisma: EOShort,
    pub adj_strength: EOShort,
    pub adj_intelligence: EOShort,
    pub adj_wisdom: EOShort,
    pub adj_agility: EOShort,
    pub adj_constitution: EOShort,
    pub adj_charisma: EOShort,
    pub stat_points: EOShort,
    pub skill_points: EOShort,
    pub karma: EOShort,
    pub usage: EOInt,
    pub min_damage: EOShort,
    pub max_damage: EOShort,
    pub accuracy: EOShort,
    pub evasion: EOShort,
    pub armor: EOShort,
    pub map_id: EOShort,
    pub coords: Coords,
    pub direction: Direction,
    pub sit_state: SitState,
    pub hidden: bool,
    pub items: Vec<Item>,
    pub bank: Vec<Item>,
    pub spells: Vec<Spell>,
    pub logged_in_at: Option<DateTime<Utc>>,
}

impl Character {
    pub fn from_creation(account_id: EOInt, create: &Create) -> Self {
        Character {
            account_id,
            gender: create.gender,
            hair_style: create.hairstyle,
            hair_color: create.haircolor,
            skin: create.skin,
            name: create.name.clone(),
            ..Default::default()
        }
    }

    pub fn get_icon(&self) -> PaperdollIcon {
        // TODO: group stuff

        match self.admin_level {
            AdminLevel::Player | AdminLevel::Guide | AdminLevel::Guardian => {
                PaperdollIcon::Player
            },
            AdminLevel::Gm => PaperdollIcon::Gm,
            AdminLevel::Hgm | AdminLevel::God => PaperdollIcon::Hgm,
        }
    }

    pub fn is_in_range(&self, coords: Coords) -> bool {
        utils::in_range(
            self.coords.x.into(),
            self.coords.y.into(),
            coords.x.into(),
            coords.y.into(),
        )
    }

    pub fn to_map_info(&self) -> CharacterMapInfo {
        CharacterMapInfo {
            name: self.name.clone(),
            id: self.player_id.expect("Character has no player id"),
            map_id: self.map_id,
            coords: BigCoords {
                x: self.coords.x.into(),
                y: self.coords.y.into(),
            },
            direction: self.direction,
            class_id: self.class,
            guild_tag: match self.guild_tag {
                Some(ref tag) => tag.to_string(),
                None => String::new(),
            },
            level: self.level,
            gender: self.gender,
            hairstyle: self.hair_style as EOChar,
            haircolor: self.hair_color as EOChar,
            skin_id: self.skin,
            max_hp: self.max_hp,
            hp: self.hp,
            max_tp: self.max_tp,
            tp: self.tp,
            paperdoll: full_to_b000a0hsw(&self.paperdoll),
            sit_state: self.sit_state,
            invisible: EOChar::from(self.hidden),
            animation: None,
        }
    }

    pub async fn load(
        conn: &mut Conn,
        id: EOInt,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut character = Character::default();
        let mut row = match conn
            .exec_first::<Row, &str, Params>(
                include_str!("sql/get_character.sql"),
                params! {
                    "character_id" => id,
                },
            )
            .await?
        {
            Some(row) => row,
            _ => {
                panic!(
                    "Attempting to load character that doesn't exist! ID: {}",
                    id
                );
            }
        };

        character.id = id;
        character.account_id = row.take("account_id").unwrap();
        character.name = row.take("name").unwrap();
        character.title = row.take("title").unwrap();
        character.home = row.take("home").unwrap();
        character.fiance = row.take("fiance").unwrap();
        character.partner = row.take("partner").unwrap();
        character.admin_level = AdminLevel::from_char(row.take("admin_level").unwrap()).unwrap();
        character.class = row.take("class").unwrap();
        character.gender = Gender::from_char(row.take("gender").unwrap()).unwrap();
        character.skin = Skin::from_char(row.take("race").unwrap()).unwrap();
        character.hair_style = row.take("hair_style").unwrap();
        character.hair_color = row.take("hair_color").unwrap();
        character.bank_max = row.take("bank_max").unwrap();
        character.gold_bank = row.take("gold_bank").unwrap();
        character.guild_rank_id = row.take("guild_rank_id").unwrap();
        character.guild_rank_string = row.take("guild_rank_string").unwrap();
        character.paperdoll.boots = row.take("boots").unwrap();
        character.paperdoll.accessory = row.take("accessory").unwrap();
        character.paperdoll.gloves = row.take("gloves").unwrap();
        character.paperdoll.belt = row.take("belt").unwrap();
        character.paperdoll.armor = row.take("armor").unwrap();
        character.paperdoll.hat = row.take("hat").unwrap();
        character.paperdoll.shield = row.take("shield").unwrap();
        character.paperdoll.weapon = row.take("weapon").unwrap();
        character.paperdoll.ring[0] = row.take("ring").unwrap();
        character.paperdoll.ring[1] = row.take("ring2").unwrap();
        character.paperdoll.armlet[0] = row.take("armlet").unwrap();
        character.paperdoll.armlet[1] = row.take("armlet2").unwrap();
        character.paperdoll.bracer[0] = row.take("bracer").unwrap();
        character.paperdoll.bracer[1] = row.take("bracer2").unwrap();
        character.level = row.take("level").unwrap();
        character.experience = row.take("experience").unwrap();
        character.hp = row.take("hp").unwrap();
        character.tp = row.take("tp").unwrap();
        character.base_strength = row.take("strength").unwrap();
        character.base_intelligence = row.take("intelligence").unwrap();
        character.base_wisdom = row.take("wisdom").unwrap();
        character.base_agility = row.take("agility").unwrap();
        character.base_constitution = row.take("constitution").unwrap();
        character.base_charisma = row.take("charisma").unwrap();
        character.stat_points = row.take("stat_points").unwrap();
        character.skill_points = row.take("skill_points").unwrap();
        character.karma = row.take("karma").unwrap();
        character.usage = row.take("usage").unwrap();
        character.map_id = row.take("map").unwrap();
        character.coords.x = row.take("x").unwrap();
        character.coords.y = row.take("y").unwrap();
        character.direction = Direction::from_char(row.take("direction").unwrap()).unwrap();
        character.sit_state = SitState::from_char(row.take("sitting").unwrap()).unwrap();
        character.hidden = row.take::<u32, &str>("hidden").unwrap() == 1;
        character.guild_name = row.take("guild_name").unwrap();
        character.guild_tag = row.take("tag").unwrap();

        character.items = conn
            .exec_map(
                include_str!("sql/get_character_inventory.sql"),
                params! {
                    "character_id" => id,
                },
                |mut row: Row| Item {
                    id: row.take(0).unwrap(),
                    amount: row.take(1).unwrap(),
                },
            )
            .await?;

        character.bank = conn
            .exec_map(
                include_str!("sql/get_character_bank.sql"),
                params! {
                    "character_id" => id,
                },
                |mut row: Row| Item {
                    id: row.take(0).unwrap(),
                    amount: row.take(1).unwrap(),
                },
            )
            .await?;

        character.spells = conn
            .exec_map(
                include_str!("sql/get_character_spells.sql"),
                params! {
                    "character_id" => id,
                },
                |mut row: Row| Spell {
                    id: row.take(0).unwrap(),
                    level: row.take(1).unwrap(),
                },
            )
            .await?;

        Ok(character)
    }

    pub async fn save(
        &mut self,
        conn: &mut Conn,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.id > 0 {
            self.update(conn).await
        } else {
            self.create(conn).await
        }
    }

    async fn create(
        &mut self,
        conn: &mut Conn,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = conn.start_transaction(TxOpts::default()).await?;

        tx.exec_drop(
            include_str!("./sql/create_character.sql"),
            params! {
                "account_id" => &self.account_id,
                "name" => &self.name,
                "home" => &SETTINGS.new_character.home,
                "gender" => &(self.gender as u32),
                "race" => &(self.skin as u32),
                "hair_style" => &(self.hair_style as u32),
                "hair_color" => &(self.hair_color as u32),
                "bank_max" => &0_u32, // TODO: figure out bank max
            },
        )
        .await?;

        self.id = tx.last_insert_id().unwrap() as EOInt;

        tx.exec_drop(
            r"INSERT INTO `Paperdoll` (
                `character_id`
            ) VALUES (:character_id);",
            params! {
                "character_id" => &self.id,
            },
        )
        .await?;

        tx.exec_drop(
            r"INSERT INTO `Position` (
                `character_id`,
                `map`,
                `x`,
                `y`,
                `direction`
            ) VALUES (
                :character_id,
                :map,
                :x,
                :y,
                :direction
            );",
            params! {
                "character_id" => &self.id,
                "map" => &SETTINGS.new_character.spawn_map,
                "x" => &SETTINGS.new_character.spawn_x,
                "y" => &SETTINGS.new_character.spawn_y,
                "direction" => &SETTINGS.new_character.spawn_direction,
            },
        )
        .await?;

        tx.exec_drop(
            r" INSERT INTO `Stats` (`character_id`)
            VALUES (:character_id);",
            params! {
                "character_id" => &self.id,
            },
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn update(
        &self,
        conn: &mut Conn,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = conn.start_transaction(TxOpts::default()).await?;

        tx.exec_drop(
            include_str!("./sql/update_character.sql"),
            params! {
                "character_id" => self.id,
                "title" => &self.title,
                "home" => &self.home,
                "fiance" => &self.fiance,
                "partner" => &self.partner,
                "admin_level" => self.admin_level as u32,
                "class" => self.class as u32,
                "gender" => self.gender as u32,
                "race" => self.skin as u32,
                "hair_style" => self.hair_style as u32,
                "hair_color" => self.hair_color as u32,
                "bank_max" => self.bank_max,
                "gold_bank" => self.gold_bank,
            },
        )
        .await?;

        tx.exec_drop(
            include_str!("./sql/update_paperdoll.sql"),
            params! {
                "character_id" => self.id,
                "boots" => self.paperdoll.boots as u32,
                "accessory" => self.paperdoll.accessory as u32,
                "gloves" => self.paperdoll.gloves as u32,
                "belt" => self.paperdoll.belt as u32,
                "armor" => self.paperdoll.armor as u32,
                "necklace" => self.paperdoll.necklace as u32,
                "hat" => self.paperdoll.hat as u32,
                "shield" => self.paperdoll.shield as u32,
                "weapon" => self.paperdoll.weapon as u32,
                "ring" => self.paperdoll.ring[0] as u32,
                "ring2" => self.paperdoll.ring[1] as u32,
                "armlet" => self.paperdoll.armlet[0] as u32,
                "armlet2" => self.paperdoll.armlet[1] as u32,
                "bracer" => self.paperdoll.bracer[0] as u32,
                "bracer2" => self.paperdoll.bracer[1] as u32,
            },
        )
        .await?;

        tx.exec_drop(
            include_str!("./sql/update_position.sql"),
            params! {
                "character_id" => self.id,
                "map_id" => self.map_id as u32,
                "x" => self.coords.x as u32,
                "y" => self.coords.y as u32,
                "direction" => self.direction as u32,
                "sitting" => self.sit_state as u32,
                "hidden" => EOInt::from(self.hidden),
            },
        )
        .await?;

        tx.exec_drop(
            include_str!("./sql/update_stats.sql"),
            params! {
                "character_id" => self.id,
                "level" => self.level as u32,
                "experience" => self.experience,
                "hp" => self.hp as u32,
                "tp" => self.tp as u32,
                "strength" => self.base_strength as u32,
                "intelligence" => self.base_intelligence as u32,
                "wisdom" => self.base_wisdom as u32,
                "agility" => self.base_agility as u32,
                "constitution" => self.base_constitution as u32,
                "charisma" => self.base_charisma as u32,
                "stat_points" => self.stat_points as u32,
                "skill_points" => self.skill_points as u32,
                "karma" => self.karma as u32,
                "usage" => self.usage,
            },
        )
        .await?;

        // TODO: save inventory/bank/spells

        tx.commit().await?;

        Ok(())
    }

    pub async fn delete(
        &self,
        conn: &mut Conn,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = conn.start_transaction(TxOpts::default()).await?;

        tx.exec_drop(
            r"DELETE FROM `Stats` WHERE `character_id` = :character_id;",
            params! {
                "character_id" => &self.id,
            },
        )
        .await?;

        tx.exec_drop(
            r"DELETE FROM `Spell` WHERE `character_id` = :character_id;",
            params! {
                "character_id" => &self.id,
            },
        )
        .await?;

        tx.exec_drop(
            r"DELETE FROM `Position` WHERE `character_id` = :character_id;",
            params! {
                "character_id" => &self.id,
            },
        )
        .await?;

        tx.exec_drop(
            r"DELETE FROM `Paperdoll` WHERE `character_id` = :character_id;",
            params! {
                "character_id" => &self.id,
            },
        )
        .await?;

        tx.exec_drop(
            r"DELETE FROM `Inventory` WHERE `character_id` = :character_id;",
            params! {
                "character_id" => &self.id,
            },
        )
        .await?;

        tx.exec_drop(
            r"DELETE FROM `Bank` WHERE `character_id` = :character_id;",
            params! {
                "character_id" => &self.id,
            },
        )
        .await?;

        tx.exec_drop(
            r"DELETE FROM `Character` WHERE `id` = :character_id;",
            params! {
                "character_id" => &self.id,
            },
        )
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub fn get_character_stats_2(&self) -> CharacterStats2 {
        CharacterStats2 {
            hp: self.hp,
            max_hp: self.max_hp,
            tp: self.tp,
            max_tp: self.max_tp,
            max_sp: self.max_sp,
            stat_points: self.stat_points,
            skill_points: self.skill_points,
            karma: self.karma,
            secondary: CharacterSecondaryStats {
                mindam: self.min_damage,
                maxdam: self.max_damage,
                accuracy: self.accuracy,
                evade: self.evasion,
                armor: self.armor,
            },
            base: CharacterBaseStats2 {
                str: self.adj_strength,
                intl: self.adj_intelligence,
                wis: self.adj_wisdom,
                agi: self.adj_agility,
                con: self.adj_constitution,
                cha: self.adj_charisma,
            },
        }
    }
}

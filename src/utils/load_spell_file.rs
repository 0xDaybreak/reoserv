use crc::{Crc, CRC_32_CKSUM};
use glob::glob;
use serde_json::Value;

use std::{fs::File, io::Read};

use bytes::Bytes;
use eo::{
    data::{
        decode_number, encode_number, EOChar, EOShort, EOThree, Serializeable, StreamBuilder,
        StreamReader,
    },
    pubs::{
        EsfFile, EsfSkillNature, EsfSpell, EsfSpellTargetRestrict, EsfSpellTargetType, EsfSpellType,
    },
};

use crate::SETTINGS;

use super::save_pub_file;

pub const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);

pub fn load_spell_file() -> Result<EsfFile, Box<dyn std::error::Error>> {
    if SETTINGS.server.generate_pub {
        load_json()
    } else {
        load_pub()
    }
}

fn load_json() -> Result<EsfFile, Box<dyn std::error::Error>> {
    let mut esf_file = EsfFile::default();
    esf_file.magic = "ESF".to_string();

    for entry in glob("pub/spells/*.json")? {
        let path = entry?;
        let mut file = File::open(path)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;

        let v: Value = serde_json::from_str(&json)?;
        let name = v["name"].as_str().unwrap_or_default().to_string();
        let shout = v["shout"].as_str().unwrap_or_default().to_string();
        let record = EsfSpell {
            name_length: name.len() as EOChar,
            shout_length: shout.len() as EOChar,
            name,
            shout,
            icon_id: v["iconId"].as_u64().unwrap_or(0) as EOShort,
            graphic_id: v["graphicId"].as_u64().unwrap_or(0) as EOShort,
            tp_cost: v["tpCost"].as_u64().unwrap_or(0) as EOShort,
            sp_cost: v["spCost"].as_u64().unwrap_or(0) as EOShort,
            cast_time: v["castTime"].as_u64().unwrap_or(0) as EOChar,
            nature: EsfSkillNature::from_char(v["nature"].as_u64().unwrap_or(0) as EOChar)
                .unwrap_or_default(),
            r#type: EsfSpellType::from_three(v["type"].as_u64().unwrap_or(0) as EOThree)
                .unwrap_or_default(),
            element: v["element"].as_u64().unwrap_or(0) as EOChar,
            element_power: v["elementPower"].as_u64().unwrap_or(0) as EOShort,
            target_restrict: EsfSpellTargetRestrict::from_char(
                v["targetRestrict"].as_u64().unwrap_or(0) as EOChar,
            )
            .unwrap_or_default(),
            target_type: EsfSpellTargetType::from_char(
                v["targetType"].as_u64().unwrap_or(0) as EOChar
            )
            .unwrap_or_default(),
            target_time: v["targetTime"].as_u64().unwrap_or(0) as EOChar,
            max_skill_level: v["maxSkillLevel"].as_u64().unwrap_or(0) as EOShort,
            min_damage: v["minDamage"].as_u64().unwrap_or(0) as EOShort,
            max_damage: v["maxDamage"].as_u64().unwrap_or(0) as EOShort,
            accuracy: v["accuracy"].as_u64().unwrap_or(0) as EOShort,
            evade: v["evade"].as_u64().unwrap_or(0) as EOShort,
            armor: v["armor"].as_u64().unwrap_or(0) as EOShort,
            return_damage: v["returnDamage"].as_u64().unwrap_or(0) as EOChar,
            hp_heal: v["healHp"].as_u64().unwrap_or(0) as EOShort,
            tp_heal: v["healTp"].as_u64().unwrap_or(0) as EOShort,
            sp_heal: v["healSp"].as_u64().unwrap_or(0) as EOChar,
            str: v["str"].as_u64().unwrap_or(0) as EOShort,
            intl: v["intl"].as_u64().unwrap_or(0) as EOShort,
            wis: v["wis"].as_u64().unwrap_or(0) as EOShort,
            agi: v["agi"].as_u64().unwrap_or(0) as EOShort,
            con: v["con"].as_u64().unwrap_or(0) as EOShort,
            cha: v["cha"].as_u64().unwrap_or(0) as EOShort,
        };
        esf_file.spells.push(record);
        esf_file.num_spells += 1;
    }

    esf_file.spells.push(EsfSpell {
        name_length: 3,
        name: "eof".to_string(),
        ..Default::default()
    });
    esf_file.num_spells += 1;

    let mut builder = StreamBuilder::new();
    esf_file.serialize(&mut builder);
    let buf = builder.get();

    let mut digest = CRC32.digest();
    digest.update(&buf[7..]);

    let checksum = digest.finalize();

    let encoded = encode_number(checksum);

    esf_file.rid = [
        decode_number(&encoded[0..=1]) as EOShort,
        decode_number(&encoded[2..=3]) as EOShort,
    ];

    save_pub_file(&esf_file, "pub/dsl001.esf")?;

    Ok(esf_file)
}

fn load_pub() -> Result<EsfFile, Box<dyn std::error::Error>> {
    let mut file = File::open("pub/dsl001.esf")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let bytes = Bytes::from(buf);
    let reader = StreamReader::new(bytes);

    let mut spell_file = EsfFile::default();
    spell_file.deserialize(&reader);
    Ok(spell_file)
}
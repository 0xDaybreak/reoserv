use std::{fs::File, io::Read};

use bytes::Bytes;
use eo::{
    data::{i32, EOInt, i32, Serializeable, StreamReader},
    pubs::{Skill, SkillMaster, SkillMasterFile},
};
use glob::glob;
use serde_json::Value;

use crate::SETTINGS;

use super::save_pub_file;

pub fn load_skill_master_file() -> Result<SkillMasterFile, Box<dyn std::error::Error>> {
    if SETTINGS.server.generate_pub {
        load_json()
    } else {
        load_pub()
    }
}

fn load_json() -> Result<SkillMasterFile, Box<dyn std::error::Error>> {
    let mut skill_master_file = SkillMasterFile::default();
    skill_master_file.magic = "EMF".to_string();

    for entry in glob("pub/skill_masters/*.json")? {
        let path = entry?;
        let mut file = File::open(path)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;

        let v: Value = serde_json::from_str(&json)?;

        let skills = v["skills"].as_array().unwrap();

        skill_master_file.skill_masters.push(SkillMaster {
            vendor_id: v["behaviorId"].as_u64().unwrap_or(0) as i32,
            name: v["name"].as_str().unwrap_or_default().to_string(),
            min_level: v["minLevel"].as_u64().unwrap_or(0) as i32,
            max_level: v["maxLevel"].as_u64().unwrap_or(0) as i32,
            class_req: v["classReq"].as_u64().unwrap_or(0) as i32,
            num_skills: skills.len() as i32,
            skills: skills
                .iter()
                .map(|v| Skill {
                    skill_id: v["id"].as_u64().unwrap_or(0) as i32,
                    min_level: v["minLevel"].as_u64().unwrap_or(0) as i32,
                    class_req: v["classReq"].as_u64().unwrap_or(0) as i32,
                    price: v["price"].as_u64().unwrap_or(0) as EOInt,
                    skill_id_req1: v["skillIdReq1"].as_u64().unwrap_or(0) as i32,
                    skill_id_req2: v["skillIdReq2"].as_u64().unwrap_or(0) as i32,
                    skill_id_req3: v["skillIdReq3"].as_u64().unwrap_or(0) as i32,
                    skill_id_req4: v["skillIdReq4"].as_u64().unwrap_or(0) as i32,
                    str_req: v["strReq"].as_u64().unwrap_or(0) as i32,
                    int_req: v["intReq"].as_u64().unwrap_or(0) as i32,
                    wis_req: v["wisReq"].as_u64().unwrap_or(0) as i32,
                    agi_req: v["agiReq"].as_u64().unwrap_or(0) as i32,
                    con_req: v["conReq"].as_u64().unwrap_or(0) as i32,
                    cha_req: v["chaReq"].as_u64().unwrap_or(0) as i32,
                })
                .collect(),
        });
    }

    save_pub_file(&skill_master_file, "pub/dsm001.emf")?;

    Ok(skill_master_file)
}

fn load_pub() -> Result<SkillMasterFile, Box<dyn std::error::Error>> {
    let mut file = File::open("pub/dsm001.emf")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let bytes = Bytes::from(buf);
    let reader = StreamReader::new(bytes);

    let mut skill_master_file = SkillMasterFile::default();
    skill_master_file.deserialize(&reader);
    Ok(skill_master_file)
}

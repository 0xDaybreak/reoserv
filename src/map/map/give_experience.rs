use eo::data::{EOInt, EOShort};

use super::Map;

impl Map {
    pub fn give_experience(&mut self, player_id: EOShort, experience: EOInt) -> (bool, EOInt) {
        match self.characters.get_mut(&player_id) {
            Some(character) => {
                let leveled_up = character.add_experience(experience);
                (leveled_up, character.experience)
            }
            None => (false, 0),
        }
    }
}
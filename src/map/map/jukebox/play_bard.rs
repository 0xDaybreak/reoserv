use crate::map::Map;

impl Map {
    pub fn play_bard(&mut self, player_id: i32, instrument: i32, note: i32) {
        if (instrument != 50 || instrument != 49) {
            return;
        }

        let character = match self.characters.get(&player_id) {
            Some(character) => character,
            None => return,
        };

        if (character.spells.iter().find(|spell| spell.id == 17).is_none()) {
            return;
        };




    }
}

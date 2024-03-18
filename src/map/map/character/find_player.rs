use eolib::{
    data::{EoSerialize, EoWriter},
    protocol::net::{server::PlayersPongServerPacket, PacketAction, PacketFamily},
};

use super::super::Map;

impl Map {
    pub fn find_player(&self, player_id: i32, name: String) {
        let character = match self.characters.get(&player_id) {
            Some(character) => character,
            None => return,
        };

        if self
            .characters
            .iter()
            .any(|(_, character)| character.name == name)
        {
            let packet = PlayersPongServerPacket { name };

            let mut writer = EoWriter::new();

            if let Err(e) = packet.serialize(&mut writer) {
                error!("Error serializing PlayersPongServerPacket: {}", e);
                return;
            }

            character.player.as_ref().unwrap().send(
                PacketAction::Pong,
                PacketFamily::Players,
                writer.to_byte_array(),
            );
        } else {
            self.world.find_player(player_id, name);
        }
    }
}
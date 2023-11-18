use eo::{
    data::{EOShort, StreamBuilder},
    protocol::{PacketAction, PacketFamily},
};

use super::super::World;

impl World {
    pub fn disband_party(&mut self, leader_id: EOShort) {
        let party_index = match self.parties.iter().position(|p| p.leader == leader_id) {
            Some(index) => index,
            None => return,
        };

        let party = self.parties.remove(party_index);

        let mut builder = StreamBuilder::new();
        builder.add_short(leader_id);

        let buf = builder.get();

        for member_id in &party.members {
            let member = match self.players.get(member_id) {
                Some(member) => member,
                None => continue,
            };

            member.send(PacketAction::Remove, PacketFamily::Party, buf.clone());
        }
    }
}
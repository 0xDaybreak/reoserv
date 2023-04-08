use bytes::Bytes;
use eo::{
    data::{Serializeable, StreamReader},
    protocol::client::paperdoll,
};

use crate::player::PlayerHandle;

pub async fn request(buf: Bytes, player: PlayerHandle) {
    let reader = StreamReader::new(buf);
    let mut packet = paperdoll::Request::default();
    packet.deserialize(&reader);

    debug!("{:?}", packet);

    let player_id = match player.get_player_id().await {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to get player id: {}", e);
            return;
        }
    };

    let map = match player.get_map().await {
        Ok(map) => map,
        Err(e) => {
            error!("Failed to get map: {}", e);
            return;
        }
    };

    map.request_paperdoll(player_id, packet.player_id);
}

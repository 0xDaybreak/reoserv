use eo::{
    data::{EOChar, EOInt, EOShort},
    protocol::{
        client,
        server::{account, character, init, login, welcome},
        FileType, OnlinePlayers,
    },
};
use tokio::sync::oneshot;

use crate::{character::Character, map::MapHandle, player::PlayerHandle};

use super::WorldHandle;

#[derive(Debug)]
pub enum Command {
    AcceptPartyRequest {
        player_id: EOShort,
        target_player_id: EOShort,
        request_type: EOChar,
    },
    AddPlayer {
        respond_to: oneshot::Sender<()>,
        player_id: EOShort,
        player: PlayerHandle,
    },
    BroadcastAdminMessage {
        name: String,
        message: String,
    },
    BroadcastAnnouncement {
        name: String,
        message: String,
    },
    BroadcastGlobalMessage {
        target_player_id: EOShort,
        name: String,
        message: String,
    },
    _BroadcastServerMessage {
        message: String,
    },
    BroadcastPartyMessage {
        player_id: EOShort,
        message: String,
    },
    CreateAccount {
        player: PlayerHandle,
        details: client::account::Create,
        respond_to:
            oneshot::Sender<Result<account::Reply, Box<dyn std::error::Error + Send + Sync>>>,
    },
    CreateCharacter {
        details: client::character::Create,
        player: PlayerHandle,
        respond_to:
            oneshot::Sender<Result<character::Reply, Box<dyn std::error::Error + Send + Sync>>>,
    },
    DeleteCharacter {
        session_id: EOShort,
        character_id: EOInt,
        player: PlayerHandle,
        respond_to:
            oneshot::Sender<Result<character::Reply, Box<dyn std::error::Error + Send + Sync>>>,
    },
    DropPlayer {
        player_id: EOShort,
        account_id: EOInt,
        character_name: String,
        respond_to: oneshot::Sender<()>,
    },
    EnterGame {
        session_id: EOShort,
        player: PlayerHandle,
        respond_to:
            oneshot::Sender<Result<welcome::Reply, Box<dyn std::error::Error + Send + Sync>>>,
    },
    GetCharacterByName {
        name: String,
        respond_to:
            oneshot::Sender<Result<Box<Character>, Box<dyn std::error::Error + Sync + Send>>>,
    },
    GetFile {
        file_type: FileType,
        session_id: EOShort,
        file_id: Option<EOChar>,
        player: PlayerHandle,
        respond_to: oneshot::Sender<Result<init::Init, Box<dyn std::error::Error + Send + Sync>>>,
    },
    GetMap {
        map_id: EOShort,
        respond_to: oneshot::Sender<Result<MapHandle, Box<dyn std::error::Error + Send + Sync>>>,
    },
    GetNextPlayerId {
        respond_to: oneshot::Sender<EOShort>,
    },
    GetOnlineList {
        respond_to: oneshot::Sender<Vec<OnlinePlayers>>,
    },
    GetPlayerCount {
        respond_to: oneshot::Sender<usize>,
    },
    LoadMapFiles {
        world: WorldHandle,
        respond_to: oneshot::Sender<()>,
    },
    Login {
        name: String,
        password: String,
        player: PlayerHandle,
        respond_to: oneshot::Sender<Result<login::Reply, Box<dyn std::error::Error + Send + Sync>>>,
    },
    PingPlayers,
    ReportPlayer {
        player_id: EOShort,
        reportee_name: String,
        message: String,
    },
    RequestAccountCreation {
        name: String,
        player: PlayerHandle,
        respond_to:
            oneshot::Sender<Result<account::Reply, Box<dyn std::error::Error + Send + Sync>>>,
    },
    RequestCharacterCreation {
        player: PlayerHandle,
        respond_to:
            oneshot::Sender<Result<character::Reply, Box<dyn std::error::Error + Send + Sync>>>,
    },
    RequestCharacterDeletion {
        character_id: EOInt,
        player: PlayerHandle,
        respond_to:
            oneshot::Sender<Result<character::Player, Box<dyn std::error::Error + Send + Sync>>>,
    },
    Save,
    SelectCharacter {
        character_id: EOInt,
        player: PlayerHandle,
        respond_to:
            oneshot::Sender<Result<welcome::Reply, Box<dyn std::error::Error + Send + Sync>>>,
    },
    SendAdminMessage {
        player_id: EOShort,
        message: String,
    },
    SendPrivateMessage {
        from: PlayerHandle,
        to: String,
        message: String,
    },
    Shutdown {
        respond_to: oneshot::Sender<()>,
    },
    Tick,
}

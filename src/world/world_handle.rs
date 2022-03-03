

use eo::data::EOShort;
use tokio::sync::{mpsc, oneshot};

use crate::player::PlayerHandle;

use super::{world::World, Command};

#[derive(Debug, Clone)]
pub struct WorldHandle {
    tx: mpsc::UnboundedSender<Command>,
    pub is_alive: bool,
}

impl WorldHandle {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let world = World::new(rx);
        tokio::task::Builder::new()
            .name("run_world")
            .spawn(run_world(world));

        Self { tx, is_alive: true }
    }

    pub async fn start_ping_timer(&self) {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(Command::StartPingTimer { respond_to: tx });
        rx.await.unwrap();
    }

    pub async fn get_player_count(
        &self,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(Command::GetPlayerCount { respond_to: tx });
        Ok(rx.await.unwrap())
    }

    pub async fn get_next_player_id(
        &self,
    ) -> Result<EOShort, Box<dyn std::error::Error + Send + Sync>> {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(Command::GetNextPlayerId { respond_to: tx });
        Ok(rx.await.unwrap())
    }

    pub async fn add_player(
        &mut self,
        player_id: EOShort,
        player: PlayerHandle,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(Command::AddPlayer {
            player_id,
            player,
            respond_to: tx,
        });
        rx.await.unwrap();
        Ok(())
    }

    pub async fn drop_player(
        &mut self,
        player_id: EOShort,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(Command::DropPlayer {
            respond_to: tx,
            player_id,
        });
        rx.await.unwrap();
        Ok(())
    }

    pub async fn load_maps(&self) {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(Command::LoadMapFiles { respond_to: tx });
        rx.await.unwrap();
    }

    pub async fn load_pubs(&self) {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(Command::LoadPubFiles { respond_to: tx });
        rx.await.unwrap();
    }
}

async fn run_world(mut world: World) {
    loop {
        if let Some(command) = world.rx.recv().await {
            world.handle_command(command).await;
        }
    }
}
use eolib::{
    data::{EoReader, EoSerialize},
    encrypt::{generate_swap_multiple, server_verification_hash},
    packet::{generate_sequence_start, get_init_sequence_bytes},
    protocol::net::{
        client::InitInitClientPacket,
        server::{
            InitBanType, InitInitServerPacket, InitInitServerPacketReplyCodeData,
            InitInitServerPacketReplyCodeDataBanned,
            InitInitServerPacketReplyCodeDataBannedBanTypeData,
            InitInitServerPacketReplyCodeDataBannedBanTypeDataTemporary,
            InitInitServerPacketReplyCodeDataOk, InitInitServerPacketReplyCodeDataOutOfDate,
            InitReply,
        },
        PacketAction, PacketFamily, Version,
    },
};

use crate::{player::ClientState, SETTINGS};

use super::super::Player;

impl Player {
    async fn init_request(&mut self, reader: EoReader) {
        let request = match InitInitClientPacket::deserialize(&reader) {
            Ok(request) => request,
            Err(e) => {
                self.close(format!("Failed to deserialize InitInitClientPacket: {}", e))
                    .await;
                return;
            }
        };

        if let Some(duration) = self.get_ban_duration().await {
            let _ = self
                .bus
                .send(
                    PacketAction::Init,
                    PacketFamily::Init,
                    InitInitServerPacket {
                        reply_code: InitReply::Banned,
                        reply_code_data: Some(InitInitServerPacketReplyCodeData::Banned(
                            if duration > 0 {
                                InitInitServerPacketReplyCodeDataBanned {
                                    ban_type: InitBanType::Temporary,
                                    ban_type_data: Some(
                                        InitInitServerPacketReplyCodeDataBannedBanTypeData::Temporary(
                                            InitInitServerPacketReplyCodeDataBannedBanTypeDataTemporary {
                                                minutes_remaining: duration as u8,
                                            },
                                        ),
                                    ),
                                }
                            } else {
                                InitInitServerPacketReplyCodeDataBanned {
                                    ban_type: InitBanType::Permanent,
                                    ban_type_data: None,
                                }
                            },
                        )),
                    },
                )
                .await;

            self.close("IP Banned".to_string()).await;
            return;
        }

        if let Ok(version_compare::Cmp::Gt) = version_compare::compare(
            format!(
                "{}.{}.{}",
                request.version.major, request.version.minor, request.version.patch
            ),
            &SETTINGS.server.max_version,
        ) {
            let versions = SETTINGS
                .server
                .max_version
                .split('.')
                .map(|c| c.parse::<i32>().unwrap_or_default())
                .collect::<Vec<i32>>();

            let _ = self
                .bus
                .send(
                    PacketAction::Init,
                    PacketFamily::Init,
                    InitInitServerPacket {
                        reply_code: InitReply::OutOfDate,
                        reply_code_data: Some(InitInitServerPacketReplyCodeData::OutOfDate(
                            InitInitServerPacketReplyCodeDataOutOfDate {
                                version: Version {
                                    major: versions[0],
                                    minor: versions[1],
                                    patch: versions[2],
                                },
                            },
                        )),
                    },
                )
                .await;

            self.close("Client too new".to_string()).await;

            return;
        }

        if let Ok(version_compare::Cmp::Lt) = version_compare::compare(
            format!(
                "{}.{}.{}",
                request.version.major, request.version.minor, request.version.patch
            ),
            &SETTINGS.server.min_version,
        ) {
            let versions = SETTINGS
                .server
                .min_version
                .split('.')
                .map(|c| c.parse::<i32>().unwrap_or_default())
                .collect::<Vec<i32>>();

            let _ = self
                .bus
                .send(
                    PacketAction::Init,
                    PacketFamily::Init,
                    InitInitServerPacket {
                        reply_code: InitReply::OutOfDate,
                        reply_code_data: Some(InitInitServerPacketReplyCodeData::OutOfDate(
                            InitInitServerPacketReplyCodeDataOutOfDate {
                                version: Version {
                                    major: versions[0],
                                    minor: versions[1],
                                    patch: versions[2],
                                },
                            },
                        )),
                    },
                )
                .await;

            self.close("Client too old".to_string()).await;

            return;
        }

        self.version = request.version;

        let sequence_start = generate_sequence_start();
        let sequence_bytes = get_init_sequence_bytes(sequence_start);
        self.bus.sequencer.set_start(sequence_start);

        let challenge_response = server_verification_hash(request.challenge);

        self.bus.client_enryption_multiple = generate_swap_multiple();
        self.bus.server_enryption_multiple = generate_swap_multiple();
        self.state = ClientState::Initialized;

        let _ = self
            .bus
            .send(
                PacketAction::Init,
                PacketFamily::Init,
                InitInitServerPacket {
                    reply_code: InitReply::OK,
                    reply_code_data: Some(InitInitServerPacketReplyCodeData::OK(
                        InitInitServerPacketReplyCodeDataOk {
                            seq1: sequence_bytes[0] as u8,
                            seq2: sequence_bytes[1] as u8,
                            server_encryption_multiple: self.bus.server_enryption_multiple,
                            client_encryption_multiple: self.bus.client_enryption_multiple,
                            challenge_response,
                            player_id: self.id,
                        },
                    )),
                },
            )
            .await;
    }

    pub async fn handle_init(&mut self, action: PacketAction, reader: EoReader) {
        match action {
            PacketAction::Init => self.init_request(reader).await,
            _ => error!("Unhandled packet Init_{:?}", action),
        }
    }
}

use chrono::NaiveDateTime;
use eo::{
    data::{i32, i32, StreamBuilder, EO_BREAK_CHAR},
    protocol::{PacketAction, PacketFamily},
};
use mysql_async::{params, prelude::Queryable, Row};

use crate::{
    utils::{format_duration, get_board_tile_spec},
    SETTINGS,
};

use super::super::Map;

struct BoardPost {
    id: i32,
    author: String,
    subject: String,
    created_at: NaiveDateTime,
}

impl Map {
    pub fn open_board(&mut self, player_id: i32, board_id: i32) {
        let character = match self.characters.get(&player_id) {
            Some(character) => character,
            None => return,
        };

        let board_tile_spec = match get_board_tile_spec(board_id) {
            Some(spec) => spec,
            None => return,
        };

        if !self.player_in_range_of_tile(player_id, board_tile_spec) {
            return;
        }

        if board_id == SETTINGS.board.admin_board as i32 && character.admin_level.to_char() < 1
        {
            return;
        }

        let player = match &character.player {
            Some(player) => player.clone(),
            None => return,
        };

        player.set_board_id(board_id);

        let pool = self.pool.clone();
        tokio::spawn(async move {
            let mut builder = StreamBuilder::new();

            let mut conn = pool.get_conn().await.unwrap();
            let limit = if board_id == SETTINGS.board.admin_board as i32 {
                SETTINGS.board.admin_max_posts
            } else {
                SETTINGS.board.max_posts
            };

            let posts = conn
                .exec_map(
                    include_str!("../../../sql/get_board_posts.sql"),
                    params! {
                        "board_id" => board_id,
                        "limit" => limit,
                    },
                    |mut row: Row| BoardPost {
                        id: row.take("id").unwrap(),
                        author: row.take("author").unwrap(),
                        subject: row.take("subject").unwrap(),
                        created_at: row.take("created_at").unwrap(),
                    },
                )
                .await
                .unwrap();

            builder.add_char(board_id as i32);
            builder.add_char(posts.len() as i32);

            for post in posts {
                let subject = if SETTINGS.board.date_posts {
                    format!("{} ({})", post.subject, format_duration(&post.created_at))
                } else {
                    post.subject
                };

                builder.add_short(post.id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&post.author);
                builder.add_break_string(&subject);
            }

            player.send(PacketAction::Open, PacketFamily::Board, builder.get());
        });
    }
}

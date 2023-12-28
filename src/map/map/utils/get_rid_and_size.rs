use eo::data::{EOInt, i32};
use tokio::sync::oneshot;

use super::super::Map;

impl Map {
    pub fn get_rid_and_size(&self, respond_to: oneshot::Sender<([i32; 2], EOInt)>) {
        let _ = respond_to.send((self.file.rid, self.file_size));
    }
}

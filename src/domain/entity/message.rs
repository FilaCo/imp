use crate::domain::entity::Peer;
use chrono::prelude::*;

pub struct Message {
    author: Peer,
    created_at: DateTime<Local>,
    updated_at: Option<DateTime<Local>>,
}

impl Message {
    pub fn new(
        author: Peer,
        created_at: DateTime<Local>,
        updated_at: Option<DateTime<Local>>,
    ) -> Self {
        Self {
            author,
            created_at,
            updated_at,
        }
    }

    pub fn edit(&mut self) {}

    pub fn delete(&mut self) {}
}

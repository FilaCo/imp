use crate::domain::entity::message::Message;

pub struct Peer {}

impl Peer {
    pub async fn send_message(&mut self, message: Message) {}
}

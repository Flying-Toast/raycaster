use crate::net::Responder;


pub struct Client {
    pub responder: Responder,
}

impl Client {
    pub fn new(responder: Responder) -> Self {
        Self {
            responder,
        }
    }
}

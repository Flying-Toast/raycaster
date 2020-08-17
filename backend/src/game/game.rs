use std::collections::HashMap;
use rand::{seq::IteratorRandom, thread_rng};
use crate::net::{Responder, ClientID};
use common::protocol::ClientMessage;
use common::map::{Map, TileType};
use common::protocol::payloads::*;
use crate::game::client::Client;
use common::entity::{Entity, EntityID};
use common::protocol::payload::BuiltPayload;
use common::vector::Vector;
use common::gamestate::GameState;


pub struct Game {
    state: GameState,
    clients: HashMap<ClientID, Client>,
    next_entity_id: u32,
}

impl Game {
    pub fn new() -> Self {
        let mapstring = std::fs::read_to_string("../maps/default").expect("Error reading default map");
        let map = Map::from_str(&mapstring).unwrap();
        Self {
            state: GameState::new(map),
            clients: HashMap::new(),
            next_entity_id: 0,
        }
    }

    pub fn on_client_disconnect(&mut self, client_id: ClientID) {
        if let Some(_) = self.remove_client(client_id) {
            eprintln!("Client {:?} disconnected unexpectedly", client_id);
        }
    }

    pub fn on_client_connect(&mut self, client_id: ClientID, mut responder: Responder) {
        let ent_id = self.gen_entity_id();
        responder.send(&YourIDPayload::assemble(ent_id));
        let entity = Entity::new(ent_id, self.state.choose_spawnpoint());
        self.announce_entity(&entity);
        self.state.add_entity(ent_id, entity);
        let mut client = Client::new(responder, ent_id);
        self.update_new_client(&mut client);
        self.clients.insert(client_id, client);
    }

    pub fn on_client_message(&mut self, client_id: ClientID, message: ClientMessage) {
        // ignore the message if we don't know a client with this id
        if !self.clients.contains_key(&client_id) {
            eprintln!("Ignoring a message from client #{:?} because they are not in the game", client_id);
            return;
        }

        match message {
            ClientMessage::Hello(payload) => {
                println!("Got ClientHelloPayload: {:?}", payload);
            },
        }
    }

    pub fn tick(&mut self, dt: u128) {
        self.send_queued_messages();
    }

    /// Flushes the outgoing message queues of all clients
    fn send_queued_messages(&mut self) {
        for client in self.clients.values_mut() {
            client.responder.flush();
        }
    }

    fn gen_entity_id(&mut self) -> EntityID {
        self.next_entity_id = self.next_entity_id.wrapping_add(1);

        EntityID::new(self.next_entity_id)
    }

    fn remove_entity(&mut self, entity_id: EntityID) {
        if let Some(_) = self.state.remove_entity(entity_id) {
            self.broadcast_message(&RemoveEntityPayload::assemble(entity_id));
        }
    }

    /// Sends `message` to all connected clients
    fn broadcast_message(&mut self, message: &BuiltPayload) {
        for client in self.clients.values_mut() {
            client.responder.send(message);
        }
    }

    /// Removes the specified client, and its player entity.
    /// Returns the removed `Client` (if one was removed).
    /// NOTE: this does not close the client's connection (see `close_and_remove_client()`).
    fn remove_client(&mut self, client_id: ClientID) -> Option<Client> {
        if let Some(client) = self.clients.remove(&client_id) {
            // remove the client's player
            self.remove_entity(client.player_entity);

            Some(client)
        } else {
            None
        }
    }

    /// The same as `remove_client()`, but also closes the client's connection.
    fn close_and_remove_client(&mut self, client_id: ClientID) {
        if let Some(client) = self.remove_client(client_id) {
            client.responder.close();
        }
    }

    /// Tells all clients about a new entity
    fn announce_entity(&mut self, entity: &Entity) {
        self.broadcast_message(&NewEntityPayload::assemble(entity));
    }

    /// Tells the entire current game state to `client`
    fn update_new_client(&mut self, client: &mut Client) {
        for entity in self.state.entities() {
            client.responder.send(&NewEntityPayload::assemble(entity));
        }
    }
}

trait GameStateExt {
    fn choose_spawnpoint(&self) -> Vector;
}

impl GameStateExt for GameState {
    fn choose_spawnpoint(&self) -> Vector {
        let (x, y) =
            self.map
                .tiles()
                .iter()
                .flatten()
                .filter(|tile| matches!(tile.tile_type(), TileType::SpawnPoint))
                .map(|tile| (tile.location().x, tile.location().y))
                .choose(&mut thread_rng())
                .unwrap_or((0.0, 0.0));

        Vector::new(x + 0.5, y + 0.5)
    }
}

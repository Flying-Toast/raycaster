use std::collections::HashMap;
use crate::net::{Responder, ClientID};
use crate::client::Client;
use common::protocol::ClientMessage;
use common::map::Map;
use common::protocol::payloads::*;
use common::entity::{Entity, EntityID};
use common::protocol::payload::BuiltPayload;
use common::vector::Vector;
use common::gamestate::GameState;


pub struct Game {
    state: GameState,
    clients: HashMap<ClientID, Client>,
    next_entity_id: u16,
}

impl Game {
    pub fn new() -> Self {
        let mapstring = std::fs::read_to_string("../maps/default").expect("Error reading default map");
        let map = Map::from_string(mapstring).unwrap();
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

    pub fn on_client_connect(&mut self, client_id: ClientID, responder: Responder) {
        let mut client = Client::new(responder, self.gen_entity_id());

        // TODO: actual spawpoints
        let entity = Entity::new(client.player_entity(), Vector::new(2.5, 2.5));
        self.announce_entity(&entity);
        self.state.add_entity(entity);
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
            ClientMessage::Input(payload) => {
                let client = self.clients.get_mut(&client_id).unwrap();
                let player_entity = client.player_entity();
                self.state.apply_input(player_entity, payload.input.as_foreign());
                client.last_processed_input = payload.input.seq_id();

                client.unbroadcast_inputs.push(payload.input.into_foreign());
            },
        }
    }

    pub fn tick(&mut self, dt: u128) {
        let mut foreign_inputs: Vec<(ClientID, BuiltPayload)> = Vec::new();
        for (id, client) in self.clients.iter_mut() {
            foreign_inputs.push((
                *id,
                ForeignInputsPayload::assemble(&client.player_entity(), &client.unbroadcast_inputs)
            ));
            client.unbroadcast_inputs.clear();

            if client.last_processed_input != client.last_acknowledged_input && client.last_processed_input != 0 {
                client.send(&LastProcessedInputPayload::assemble(&client.last_processed_input));
                client.last_acknowledged_input = client.last_processed_input;
            }
        }

        for (id, payload) in foreign_inputs {
            self.broadcast_message_except_to(&payload, id);
        }

        self.send_queued_messages();
    }

    /// Flushes the outgoing message queues of all clients
    fn send_queued_messages(&mut self) {
        for client in self.clients.values_mut() {
            client.flush_messages();
        }
    }

    fn gen_entity_id(&mut self) -> EntityID {
        self.next_entity_id = self.next_entity_id.wrapping_add(1);

        EntityID::new(self.next_entity_id)
    }

    fn remove_entity(&mut self, entity_id: EntityID) {
        if let Some(_) = self.state.remove_entity(entity_id) {
            self.broadcast_message(&RemoveEntityPayload::assemble(&entity_id));
        }
    }

    /// Sends `message` to all connected clients
    fn broadcast_message(&mut self, message: &BuiltPayload) {
        for client in self.clients.values_mut() {
            client.send(message);
        }
    }

    /// Sends `message` to all connected clients except `excluded_client`
    fn broadcast_message_except_to(&mut self, message: &BuiltPayload, excluded_client: ClientID) {
        for (id, client) in self.clients.iter_mut() {
            if *id != excluded_client {
                client.send(message);
            }
        }
    }

    /// Removes the specified client, and its player entity.
    /// Returns the removed `Client` (if one was removed).
    /// NOTE: this does not close the client's connection (see `close_and_remove_client()`).
    fn remove_client(&mut self, client_id: ClientID) -> Option<Client> {
        if let Some(client) = self.clients.remove(&client_id) {
            // remove the client's player
            self.remove_entity(client.player_entity());

            Some(client)
        } else {
            None
        }
    }

    /// The same as `remove_client()`, but also closes the client's connection.
    fn close_and_remove_client(&mut self, client_id: ClientID) {
        if let Some(client) = self.remove_client(client_id) {
            client.disconnect();
        }
    }

    /// Tells all clients about a new entity
    fn announce_entity(&mut self, entity: &Entity) {
        self.broadcast_message(&NewEntityPayload::assemble(entity));
    }

    /// Tells the entire current game state to `client`
    fn update_new_client(&mut self, client: &mut Client) {
        client.send(&YourIDPayload::assemble(&client.player_entity()));
        for entity in self.state.entities() {
            client.send(&NewEntityPayload::assemble(entity));
        }
        client.send(&SetMapPayload::assemble(self.state.map()));

        client.send(&ReadyPayload::assemble());
    }
}

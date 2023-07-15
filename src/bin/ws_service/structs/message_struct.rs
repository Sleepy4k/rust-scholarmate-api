use serde_json::json;
use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};

use crate::structs::{
  ws_struct::{Connect, Disconnect},
  main_struct::{Message, ListRooms}
};

#[doc = "ClientMessage is the struct that will be used to send message to the server from client."]
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
  pub id: i32,
  pub msg: String,
  pub room: String,
}

#[doc = "Join is the struct that will be used to join the server from client."]
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
  pub id: i32,
  pub name: String,
}

#[doc = "ChatServer is the struct that will be used to chat server in websocket."]
#[derive(Debug)]
pub struct ChatServer {
  sessions: HashMap<i32, Recipient<Message>>,
  rooms: HashMap<String, HashSet<i32>>,
  rng: ThreadRng,
}

impl ChatServer {
  #[doc = "new is the function that will be used to create new chat server."]
  pub fn new() -> ChatServer {
    let mut rooms = HashMap::new();
    rooms.insert("main".to_string(), HashSet::new());

    Self {
      sessions: HashMap::new(),
      rooms,
      rng: rand::thread_rng()
    }
  }

  #[doc = "send_message is the function that will be used to send message to the server from client."]
  fn send_message(&self, room: &str, message: &str, skip_id: i32) {
    if let Some(sessions) = self.rooms.get(room) {
      for id in sessions {
        if *id != skip_id {
          if let Some(addr) = self.sessions.get(id) {
            addr.do_send(Message(message.to_owned()));
          }
        }
      }
    }
  }
}

impl Actor for ChatServer {
  type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
  type Result = i32;

  #[doc = "handle is the function that will be used to handle the connect message."]
  fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
    let id = self.rng.gen::<i32>();
    self.sessions.insert(id, msg.addr);
    self.rooms
      .entry("main".to_string())
      .or_insert_with(HashSet::new)
      .insert(id);
    self.send_message("main", &json!({
      "value": vec![format!("{}", id)],
      "status": "connected",
    }).to_string(), 0);
    id
  }
}

impl Handler<Disconnect> for ChatServer {
  type Result = ();

  #[doc = "handle is the function that will be used to handle the disconnect message."]
  fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
    let mut rooms: Vec<String> = vec![];

    if self.sessions.remove(&msg.id).is_some() {
      for (name, sessions) in &mut self.rooms {
        if sessions.remove(&msg.id) {
          rooms.push(name.to_owned());
        }
      }
    }

    for room in rooms {
      self.send_message("main", &json!({
          "room": room,
          "value": vec![format!("Someone disconnect!")],
          "status": "disconnected",
      }).to_string(), 0);
    }
  }
}

impl Handler<ClientMessage> for ChatServer {
  type Result = ();

  #[doc = "handle is the function that will be used to handle the client message."]
  fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
    self.send_message(&msg.room, &msg.msg, msg.id);
  }
}

impl Handler<ListRooms> for ChatServer {
  type Result = MessageResult<ListRooms>;

  #[doc = "handle is the function that will be used to handle the list rooms message."]
  fn handle(&mut self, _: ListRooms, _: &mut Self::Context) -> Self::Result {
    let mut rooms = vec![];

    for key in self.rooms.keys() {
      rooms.push(key.to_owned());
    }

    MessageResult(rooms)
  }
}

impl Handler<Join> for ChatServer {
  type Result = ();

  #[doc = "handle is the function that will be used to handle the join message."]
  fn handle(&mut self, msg: Join, _: &mut Self::Context) -> Self::Result {
    let Join {id, name} = msg;
    let mut rooms = vec![];

    for (n, sessions) in &mut self.rooms {
      if sessions.remove(&id) {
        rooms.push(n.to_owned());
      }
    }

    for room in rooms {
      self.send_message(&room, &json!({
        "room": room,
        "value": vec![format!("Someone disconnect!")],
        "status": "disconnected"
      }).to_string(), 0);
    }

    self.rooms
      .entry(name.clone())
      .or_insert_with(HashSet::new)
      .insert(id);
  }
}
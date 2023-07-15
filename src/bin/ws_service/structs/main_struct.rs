use actix::prelude::*;
use sqlx::{Pool, Postgres};

use crate::structs::message_struct::ChatServer;

#[doc = "ListRooms is the struct that will be used to list rooms in the server."]
pub struct ListRooms;

#[doc = "Message is the struct that will be used to send message to the server from client."]
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[doc = "WSAppState is the struct that will be used to store the state of the server."]
pub struct WSAppState {
  pub db: Pool<Postgres>,
  pub srv: Addr<ChatServer>
}

impl actix::Message for ListRooms {
  type Result = Vec<String>;
}

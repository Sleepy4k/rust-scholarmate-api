use actix::prelude::*;
use sqlx::{Pool, Postgres};

use crate::structs::message_struct::ChatServer;

pub struct ListRooms;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct WSAppState {
  pub db: Pool<Postgres>,
  pub srv: Addr<ChatServer>
}

impl actix::Message for ListRooms {
  type Result = Vec<String>;
}

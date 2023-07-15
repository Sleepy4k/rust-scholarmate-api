use actix::prelude::*;

use crate::structs::main_struct::Message;

#[doc = "Connect is the struct that will be used to connect to the server and to the client."]
#[derive(Message)]
#[rtype(i32)]
pub struct Connect {
  pub addr: Recipient<Message>,
}

#[doc = "Disconnect is the struct that will be used to disconnect from the server and to the client."]
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub id: i32,
} 
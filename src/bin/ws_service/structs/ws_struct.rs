use actix::prelude::*;

use crate::structs::main_struct::Message;

#[derive(Message)]
#[rtype(i32)]
pub struct Connect {
  pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub id: i32,
}
use crate::{protocols::gptp::{header::Header, message_type::MessageType}, utils::Octet};

pub struct Signaling
{
  header: Header,
  target_port_identity: [Octet; 10],
  // TODO: implement last field
}

impl Signaling
{
  pub fn header(&self) -> Header
  { return self.header; }

  pub fn msg_type(&self) -> MessageType
  { return self.header().message_type(); }

  pub fn new(payload: &[u8]) -> Self
  {
    return Self
    {
      header: Header::new(MessageType::Signaling, payload),
      target_port_identity: [
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
      ],
    };
  }
}

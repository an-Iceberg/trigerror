use crate::{protocols::gptp::{header::Header, message_type::MessageType}, utils::Octet};

pub struct Sync2Step
{
  header: Header,
  reserved: [Octet; 10]
}

impl Sync2Step
{
  pub fn header(&self) -> Header
  { return self.header; }

  pub fn new(payload: &[u8]) -> Self
  {
    return Self
    {
      header: Header::new(MessageType::Sync2Step, payload),
      reserved: [
        payload[34],
        payload[35],
        payload[36],
        payload[37],
        payload[38],
        payload[39],
        payload[40],
        payload[41],
        payload[42],
        payload[43],
      ]
    };
  }
}

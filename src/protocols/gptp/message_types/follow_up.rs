use crate::{protocols::gptp::{header::Header, message_type::MessageType}, utils::Octet};

pub struct FollowUp
{
  header: Header,
  precise_origin_timestamp: [Octet; 10],
  // TODO: the rest of the fields (there's a lot) (+ validation)
}

impl FollowUp
{
  pub fn header(&self) -> Header
  { return self.header; }

  pub fn new(payload: &[u8]) -> Self
  {
    return Self
    {
      header: Header::new(MessageType::FollowUp, payload),
      precise_origin_timestamp: [
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

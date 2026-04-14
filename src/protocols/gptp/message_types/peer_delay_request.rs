use crate::{protocols::gptp::{header::Header, message_type::MessageType}, utils::Octet};

pub struct PeerDelayRequest
{
  header: Header,
  reserved1: [Octet; 10],
  reserved2: [Octet; 10],
}

impl PeerDelayRequest
{
  pub fn header(&self) -> Header
  { return self.header; }

  pub fn new(payload: &[u8]) -> Self
  {
    return Self
    {
      header: Header::new(MessageType::PeerDelayRequest, payload),
      reserved1: [
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
      ],
      reserved2: [
        payload[44],
        payload[45],
        payload[46],
        payload[47],
        payload[48],
        payload[49],
        payload[50],
        payload[51],
        payload[52],
        payload[53],
      ]
    };
  }
}

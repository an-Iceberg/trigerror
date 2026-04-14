use crate::{protocols::gptp::{header::Header, message_type::MessageType}, utils::Octet};

pub struct PeerDelayResponse
{
  header: Header,
  request_receipt_timestamp: [Octet; 10],
  requesting_port_identity: [Octet; 10],
}

impl PeerDelayResponse
{
  pub fn header(&self) -> Header
  { return self.header; }

  pub fn new(payload: &[u8]) -> Self
  {
    return Self
    {
      header: Header::new(MessageType::PeerDelayResponse, payload),
      request_receipt_timestamp: [
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
      requesting_port_identity:  [
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
      ],
    };
  }
}

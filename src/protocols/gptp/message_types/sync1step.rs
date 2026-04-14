use crate::{protocols::gptp::{header::Header, message_type::MessageType}, utils::{Octet, bytes_to_u16}};

pub struct Sync1Step
{
  header: Header,
  origin_timestamp: [Octet; 10],
  tlv_type: u16,
  length_field: u16,
  organization_id: [Octet; 3],
  organization_sub_type: [Octet; 3],
  cumulative_scaled_rate_offset: [Octet; 4],
  gm_time_base_indicator: [Octet; 2],
  last_gm_phase_change: [Octet; 12],
  scaled_last_gm_frequency_change: [Octet; 4],
}

impl Sync1Step
{
  pub fn header(&self) -> Header
  { return self.header; }

  pub fn new(payload: &[u8]) -> Self
  {
    return Self
    {
      header: Header::new(MessageType::Sync1Step, payload),
      origin_timestamp: [
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
      tlv_type: bytes_to_u16(payload[44], payload[45]),
      length_field: bytes_to_u16(payload[46], payload[47]),
      organization_id: [
        payload[48],
        payload[49],
        payload[50],
      ],
      organization_sub_type: [
        payload[51],
        payload[52],
        payload[53],
      ],
      cumulative_scaled_rate_offset: [
        payload[54],
        payload[55],
        payload[56],
        payload[57],
      ],
      gm_time_base_indicator: [
        payload[58],
        payload[59],
      ],
      last_gm_phase_change: [
        payload[60],
        payload[61],
        payload[62],
        payload[63],
        payload[64],
        payload[65],
        payload[66],
        payload[67],
        payload[68],
        payload[69],
        payload[70],
        payload[71],
      ],
      scaled_last_gm_frequency_change: [
        payload[72],
        payload[73],
        payload[74],
        payload[75],
      ],
    };
  }
}

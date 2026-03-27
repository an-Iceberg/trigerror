use super::{Octet, header::Header};

pub enum GPTPMesage
{
  Announce
  {
    header: Header,
    current_utc_offset: u16,
    grandmaster_priority_1: Octet,
    grandmaster_clock_quality: [Octet; 4],
    grandmaster_priority_2: Octet,
    grandmaster_identity: [Octet; 8],
    steps_removed: u16,
    time_source: Octet,
    tlv_type: [Octet; 2],
    length_field: [Octet; 2],
    path_sequence: Vec<Octet>,
  },
  Signaling
  {
    header: Header,
    target_port_identity: [Octet; 10],
    // TODO: implement last field
  },
  Sync1Step
  {
    // TODO: header 11.4.2
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
  },
  Sync2Step
  {
    // TODO: header 11.4.2
    header: Header,
    reserved: [Octet; 10]
  },
  FollowUp
  {
    // TODO: header 11.4.2
    header: Header,
    // TODO: different versions
    // TODO: 11.4.4.1-3
  },
  PdelayReq
  {
    // TODO: header 11.4.2
    header: Header,
  },
  PdelayResp
  {
    // TODO: header 11.4.2
    header: Header,
    request_receipt_timestamp: [Octet; 10],
    requesting_port_identity: [Octet; 10],
  },
  PdelayRespFollowUp
  {
    // TODO: header 11.4.2
    header: Header,
    response_origin_timestamp: [Octet; 10],
    requesting_port_identity: [Octet; 10],
  }
}

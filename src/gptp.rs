type Octet = u8;

enum MessageType
{
  Sync =               0x0,
  PdelayReq =          0x2,
  PdelayResp =         0x3,
  FollowUp =           0x8,
  PdelayRespFollowUp = 0xA,
  Announce =           0xB,
  Signaling =          0xC,
}

enum Flags
{
  AlternateMaster =       0b00000000_00000000,
  TwoStep =               0b00000001_00000000,
  Unicast =               0b00000010_00000000,
  PTPProfileSpecific1 =   0b00100000_00000000,
  PTPProfileSpecific2 =   0b01000000_00000000,
  Leap61 =                0b00000000_00000001,
  Leap59 =                0b00000000_00000010,
  CurrentUTCOffsetValid = 0b00000000_00000100,
  PTPTimescale =          0b00000000_00001000,
  TimeTraceable =         0b00000000_00010000,
  FrequencyTraceable =    0b00000000_00100000,
  Reserved,
}

struct Header
{
  major_s_dold: Octet,
  message_type: MessageType,
  minor_version_ptp: Octet,
  version_ptp: Octet,
  message_length: u16,
  domain_number: Octet,
  minor_s_dold: Octet,
  flags: Flags,
  correction_field: [Octet; 8],
  message_type_specific: [Octet; 4],
  source_port_identity: [Octet; 10],
  sequence_id: [Octet; 2],
  control_field: Octet,
  log_message_interval: Octet,
}

// TODO: header 11.4.2

struct GPTP
{
  header: Header
}

enum GPTPMesage
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

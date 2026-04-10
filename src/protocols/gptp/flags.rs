use std::fmt::Display;

use crate::utils::get_bit;

#[derive(Debug)]
pub struct Flags
{
  alternate_master: bool,
  two_step: bool,
  unicast: bool,
  ptp_profile_specific_1: bool,
  ptp_profile_specific_2: bool,
  leap_61: bool,
  leap_59: bool,
  current_utc_offset_valid: bool,
  ptp_timescale: bool,
  time_traceable: bool,
  frequency_traceable: bool,
}

impl Flags
{
  pub fn new(first_byte: u8, second_byte: u8) -> Self
  {
    return Flags
    {
      alternate_master: get_bit(first_byte, 0),
      two_step: get_bit(first_byte, 1),
      unicast: get_bit(first_byte, 2),
      ptp_profile_specific_1: get_bit(first_byte, 5),
      ptp_profile_specific_2: get_bit(first_byte, 6),
      leap_61: get_bit(second_byte, 0),
      leap_59: get_bit(second_byte, 1),
      current_utc_offset_valid: get_bit(second_byte, 2),
      ptp_timescale: get_bit(second_byte, 3),
      time_traceable: get_bit(second_byte, 4),
      frequency_traceable: get_bit(second_byte, 5),
    };
  }

  pub fn alternate_master(&self) -> bool { return self.alternate_master; }
  pub fn two_step(&self) -> bool { return self.two_step; }
  pub fn unicast(&self) -> bool { return self.unicast; }
  pub fn ptp_profile_specific_1(&self) -> bool { return self.ptp_profile_specific_1; }
  pub fn ptp_profile_specific_2(&self) -> bool { return self.ptp_profile_specific_2; }
  pub fn leap_61(&self) -> bool { return self.leap_61; }
  pub fn leap_59(&self) -> bool { return self.leap_59; }
  pub fn current_utc_offset_valid(&self) -> bool { return self.current_utc_offset_valid; }
  pub fn ptp_timescale(&self) -> bool { return self.ptp_timescale; }
  pub fn time_traceable(&self) -> bool { return self.time_traceable; }
  pub fn frequency_traceable(&self) -> bool { return self.frequency_traceable; }
}

impl Display for Flags
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    todo!()
  }
}

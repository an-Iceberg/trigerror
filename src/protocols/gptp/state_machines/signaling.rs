use crate::protocols::gptp::state_machines::{mac_validator::MACValidator, time_validator::TimeValidator};

/// State machine for verifying Signaling messages.
#[derive(Default)]
pub struct SignalingSM
{
  mac_validator: MACValidator,
  time_validator: TimeValidator,
}

use crate::protocols::gptp::state_machines::{mac_validator::MACValidator, time_validator::TimeValidator};

/// State machine for verifying PeerDelay messages.
#[derive(Default)]
pub struct PeerDelaySM
{
  mac_validator: MACValidator,
  time_validator: TimeValidator,
}

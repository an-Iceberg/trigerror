use std::time::Duration;

pub struct MDSyncReceiveSM
{
  state: State,

  follow_up_receipt_timeout_time: (),
  received_sync: bool,
  received_follow_up: bool,
  received_sync_pointer: (),
  received_follow_up_pointer: (),
  tx_md_sync_received_pointer_mdsr: (),
  upstream_sync_interval: (),
  temp: i16,

  // Taken from table 10-1.
  BEGIN: bool,
  clock_time_transmitter_sync_interval: Duration,
  clock_time_receiver_time: Duration,
  sync_receipt_time: Duration,
  sync_receipt_local_time: Duration,
  clock_source_frequency_offset: f64,
  clock_source_phase_offset: Duration,
  clock_source_time_base_indicator: u16,
  clock_source_time_base_indicator_old: u16,
  clock_source_last_gm_phase_change: u64,
  clock_source_last_gm_frequency_change: f64,
  current_time: Duration,
  gm_present: bool,
  gm_rate_ratio: f64,
  gm_time_base_indicator: u16,
  last_gm_phase_change: Duration,
  last_gm_frequency_change: f64,
  local_clock_tick_interval: Duration, // TimeInterval
  local_time: Duration,
  selected_state: (),
  time_transmitter_time: Duration,
  this_clock: u64, // ClockIdentity
  parent_log_sync_interval: i8,
  instance_enable: bool,
  sync_receipt_timeout_time: Duration,
  as_capable: bool,
  asymmetry_measurement_mode: bool,
  sync_receipt_timeout_time_interval: Duration,
  current_log_sync_interval: i8,
  initial_log_sync_interval: i8,
  sync_interval: Duration,
  neighbor_rate_ratio: f64,
  mean_link_delay: Duration,
  delay_asymmetry: Duration,
  compute_neighbor_rate_ratio: bool,
  compute_mean_link_delay: bool,
  port_operational: bool,
  ptp_port_enabled: bool,
  this_port: u16,
  sync_locked: bool,
  neighbor_gptp_capable: bool,
  sync_slowdown: bool,
  old_sync_interval: Duration,
  gptp_capable_message_slowdown: bool,
  gptp_capable_message_interval: Duration,
  old_gptp_capable_message_interval: Duration,
  current_log_gptp_capable_message_interval: i8,
  sync_grandmaster_identity: u64, // ClockIdentity
  sync_steps_removed: u16,
  drift_tracking_tlv_support: bool,
  received_ps_sync_css: bool,
  received_local_clock_tick_cxx: bool,
  rate_ratio_drift: i32,
}

enum State { Discard, WaitingForFollowUp, WaitingForSync }

impl MDSyncReceiveSM
{
  pub fn set_md_sync_receive_mdsr(&mut self) -> Self
  {
    todo!()
  }

  pub fn tx_md_sync_receive(&mut self, tx_md_sync_receive_pointer_mdsr: ())
  {
    todo!()
  }
}

use std::time::Duration;

pub struct MDSyncReceiveSM
{
  state: State,

  // follow_up_receipt_timeout_time: (),
  // received_sync: bool,
  // received_follow_up: bool,
  // received_sync_pointer: (),
  // received_follow_up_pointer: (),
  // tx_md_sync_received_pointer_mdsr: (),
  // upstream_sync_interval: (),
  // temp: i16,

  // Sync and follow up
  // TODO: wait for sync messages (messageType == Sync) and then for follow up (messageType == follow up)
  // NOTE: log_message_interval
  // NOTE: if log_message_interval changes, error and set value of erroneous packet to new now value
  // TODO: time margin 30%

}

enum State { WaitingForFollowUp, WaitingForSync }

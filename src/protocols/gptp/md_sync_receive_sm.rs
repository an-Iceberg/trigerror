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

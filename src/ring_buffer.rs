use std::collections::VecDeque;
use crate::packet::Packet;

// This is basically the decorator pattern.
#[derive(Debug, Default)]
pub struct RingBuffer
{
  data: VecDeque<Packet>,
  pub count_before: u32,
  pub time_before: u32,
}

impl RingBuffer
{
  pub fn new() -> Self { return Self::default(); }

  pub fn with_capacity(capacity: usize) -> Self
  {
    return RingBuffer
    {
      data: VecDeque::with_capacity(capacity),
      ..Default::default()
    };
  }

  pub fn push(&mut self, frame: Packet)
  {
    self.data.push_back(frame);
    // Purge elements from the start of the queue that don't fulfill the criteria
    // of `count_before` or `time_before`.
    while self.data.len() >= self.count_before as usize
    { self.data.pop_front(); }
    // Purge all packets, that are older than `time_before`.
  }

  /// This returns the contents of the ring buffer as a list to be processed.
  pub fn drain(&mut self) -> Vec<Packet>
  { return Vec::from(self.data.clone()); }
}

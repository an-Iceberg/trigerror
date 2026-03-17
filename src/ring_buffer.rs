use std::collections::VecDeque;
use crate::packet::Packet;

// This is basically the decorator pattern.
#[derive(Debug, Default)]
pub struct RingBuffer
{
  data: VecDeque<Packet>,
  count_before: u32,
  time_before: u32,
  // INFO: these won't be implemented unless we know how to handle file size.
  // pub size: u32,
  // pub max_size: u32,
}

impl RingBuffer
{
  pub fn new(
    count_before: u32,
    time_before: u32,
    // max_size: u32,
  ) -> Self
  {
    return Self
    {
      count_before,
      time_before,
      // max_size,
      ..Default::default()
    };
  }

  pub fn with_capacity(capacity: usize) -> Self
  {
    return RingBuffer
    {
      data: VecDeque::with_capacity(capacity),
      ..Default::default()
    };
  }

  pub fn push(&mut self, packet: Packet)
  {
    self.data.push_back(packet);
    // Purge all packets, that are older than `time_before`.
    // NOTE: while oldest packet older than (newest packet - time_before): dequeue packet
    // Purge elements from the start of the queue that don't fulfill the criteria
    // of `count_before` or `time_before`.
    while self.data.len() > self.count_before as usize
    { self.data.pop_front(); }
    // dbg!{self.data.len()};
  }

  /// FIX: this does not "drain" the buffer.
  /// This returns the contents of the ring buffer as a list to be processed.
  pub fn drain(&mut self) -> Vec<Packet>
  { return Vec::from(self.data.clone()); }

  pub fn len(&mut self) -> usize { return self.data.iter().len(); }

  pub fn is_empty(&mut self) -> bool { return self.data.is_empty(); }
}

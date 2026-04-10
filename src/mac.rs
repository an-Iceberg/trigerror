use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MAC
{
  address: (u8, u8, u8, u8, u8, u8)
}

impl Display for MAC
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    return formatter.write_str(format!(
      "{:02X?}-{:02X?}-{:02X?}-{:02X?}-{:02X?}-{:02X?}",
      self.address.0,
      self.address.1,
      self.address.2,
      self.address.3,
      self.address.4,
      self.address.5,
    ).as_str());
  }
}

impl MAC
{
  pub fn new() -> Self { return MAC::default(); }

  pub fn from_bytes(bytes: (u8, u8, u8, u8, u8, u8)) -> Self { return MAC { address: bytes }; }
}
